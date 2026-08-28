//! # Windowed or Flat Granmo/Tsetlin Machine NLP Classification Pipeline (Phase 1)
//!
//! Phase 1 of a research programme exploring whether Tsetlin Machines (TMs) can
//! be extended toward deep-learning-style context modelling while remaining
//! gradient-free. This phase implements the **convolution analogue**: clauses
//! evaluated over a sliding window of consecutive word positions, giving the
//! model local order sensitivity (e.g., "not" immediately followed by "good")
//! that flat Bag-of-Words TMs structurally cannot represent. This is expected
//! to matter for context-dependent targets such as toxicity classification of
//! social-media text.
//!
//! ## Architecture
//! - `VanillaTM`   — the flat BOW baseline, retained for head-to-head ablation.
//! - `WindowedTM`  — the new windowed clause bank. Literals are indexed by
//!   `(relative_slot, word_id)` plus mirrored negated literals. Per-clause,
//!   per-window firings are pooled (`AnyFire` OR-pool / `CountFire` sum-pool
//!   capped by `clause_vote_cap`) into a document-level vote, and Type I/II
//!   feedback is computed against that pooled outcome (aggregate-outcome
//!   attribution), with literal-level reinforcement applied to one randomly
//!   selected fired window (standard Convolutional-TM practice).
//! - Short documents are handled deliberately: a document shorter than the
//!   window width yields exactly one right-PAD-padded window. PAD (and
//!   out-of-vocabulary tokens) behave as "no word present": they fail positive
//!   literals and satisfy negated literals.
//!
//! ## Removed (per hand-off decision)
//! The Stage-1 semantic pre-training pipeline (`KMeans`, `NonNegatedTM`,
//! `SemanticEnricher`, TF-IDF vectorisation) has been physically deleted: the
//! clustering signal was self-referential over the same vocabulary space and
//! added no external information. Serialized models from the previous version
//! are therefore **not** loadable by this version.
//!
//! ## Performance notes
//! - Inference clause evaluation is O(window_width) per window via incremental
//!   included-positive-literal counters, not O(window_width × vocab).
//! - Inference is parallelised with rayon (per-class sums in `predict`,
//!   per-record in `evaluate_pipeline`).
//! - Training is sequential and deterministic under a fixed seed; parallel
//!   training is future work (requires reproducible per-worker RNG streams).
//!
//! ## CLI
//! - `--mode train`   trains flat or windowed model, evaluates, optionally saves.
//! - `--mode predict` loads a model and classifies one text with decoded,
//!   position-annotated logic rules (explainability trace).

#![forbid(unsafe_code)]

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
// ---------------------------------------------------------------------------
// Error Handling Architecture
// ---------------------------------------------------------------------------

/// Unified error type for every fallible operation in the pipeline, so that
/// `main` and library consumers receive one coherent, matchable error surface.
#[derive(Debug)]
pub enum PipelineError {
    IoError {
        path: PathBuf,
        source: std::io::Error,
    },
    CsvError {
        path: PathBuf,
        source: csv::Error,
    },
    JsonError {
        path: PathBuf,
        source: serde_json::Error,
    },
    DatasetInvalid(String),
    ColumnNotFound(String),
    DimensionMismatch {
        expected: usize,
        found: usize,
        context: String,
    },
    NumericalError(String),
    InvalidConfiguration(String),
    CliError(String),
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError { path, source } => {
                write!(f, "I/O error at '{}': {}", path.display(), source)
            }
            Self::CsvError { path, source } => {
                write!(f, "CSV error at '{}': {}", path.display(), source)
            }
            Self::JsonError { path, source } => {
                write!(f, "JSON error at '{}': {}", path.display(), source)
            }
            Self::DatasetInvalid(msg) => write!(f, "Invalid dataset: {msg}"),
            Self::ColumnNotFound(col) => write!(f, "Column '{col}' not found in dataset"),
            Self::DimensionMismatch {
                expected,
                found,
                context,
            } => write!(
                f,
                "Dimension mismatch in {context}: expected {expected}, found {found}"
            ),
            Self::NumericalError(msg) => write!(f, "Numerical computation error: {msg}"),
            Self::InvalidConfiguration(msg) => write!(f, "Configuration error: {msg}"),
            Self::CliError(msg) => write!(f, "CLI argument error: {msg}"),
        }
    }
}

impl std::error::Error for PipelineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError { source, .. } => Some(source),
            Self::CsvError { source, .. } => Some(source),
            Self::JsonError { source, .. } => Some(source),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// PRNG (xorshift64 — deterministic, dependency-free, integer-state)
// ---------------------------------------------------------------------------

/// Deterministic pseudo-random number generator used for stochastic Tsetlin
/// feedback and dataset shuffling. Seeded explicitly so every training run is
/// reproducible, which is essential for the flat-vs-windowed ablation study.
#[derive(Debug, Clone)]
pub struct FastRng {
    state: u64,
}

impl FastRng {
    /// Creates a generator from an explicit seed. A zero seed is remapped to a
    /// fixed non-zero constant because xorshift has an absorbing state at zero.
    pub fn seed(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x853c_49e6_748f_ea9b
            } else {
                seed
            },
        }
    }

    /// Advances the xorshift64 state and returns the next raw 64-bit value.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Uniform draw in [0, 1). Note: `f64` is used only for stochastic
    /// feedback probability comparisons, matching the pre-existing convention;
    /// no new floating-point state is introduced by the windowed model.
    pub fn gen_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Uniform integer draw in [start, end). Errors on an empty range rather
    /// than panicking, per the crate-wide no-panic policy.
    pub fn gen_range(&mut self, start: usize, end: usize) -> Result<usize, PipelineError> {
        if start >= end {
            return Err(PipelineError::InvalidConfiguration(format!(
                "Invalid range: start ({start}) must be strictly less than end ({end})"
            )));
        }
        let range = (end - start) as u64;
        let offset = (self.next_u64() % range) as usize;
        Ok(start + offset)
    }
}

// ---------------------------------------------------------------------------
// Dataset Ingestion (CSV / JSONL)
// ---------------------------------------------------------------------------

/// One labelled training/evaluation example.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextRecord {
    pub text: String,
    pub label: String,
}

/// A loaded labelled corpus plus its label vocabulary. Label IDs are assigned
/// in first-seen order and shared between train/test via `align_labels_with`.
#[derive(Debug, Clone)]
pub struct Dataset {
    pub records: Vec<TextRecord>,
    pub label_to_id: HashMap<String, usize>,
    pub id_to_label: Vec<String>,
}

impl Dataset {
    /// Loads a dataset from a delimited file with a header row. The path must
    /// be absolute (crate-wide policy) so error messages and reproducibility
    /// logs are unambiguous.
    pub fn from_csv<P: AsRef<Path>>(
        absolute_path: P,
        delimiter: u8,
        text_column_name: &str,
        label_column_name: &str,
    ) -> Result<Self, PipelineError> {
        let path_buf = absolute_path.as_ref().to_path_buf();
        if !path_buf.is_absolute() {
            return Err(PipelineError::DatasetInvalid(format!(
                "Path must be absolute: {}",
                path_buf.display()
            )));
        }

        let file = File::open(&path_buf).map_err(|e| PipelineError::IoError {
            path: path_buf.clone(),
            source: e,
        })?;

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .flexible(true)
            .from_reader(file);

        let headers = rdr
            .headers()
            .map_err(|e| PipelineError::CsvError {
                path: path_buf.clone(),
                source: e,
            })?
            .clone();

        let text_idx = headers
            .iter()
            .position(|h| h.trim() == text_column_name)
            .ok_or_else(|| PipelineError::ColumnNotFound(text_column_name.to_string()))?;

        let label_idx = headers
            .iter()
            .position(|h| h.trim() == label_column_name)
            .ok_or_else(|| PipelineError::ColumnNotFound(label_column_name.to_string()))?;

        let mut records = Vec::new();
        let mut label_to_id = HashMap::new();
        let mut id_to_label = Vec::new();

        for result in rdr.records() {
            let record = result.map_err(|e| PipelineError::CsvError {
                path: path_buf.clone(),
                source: e,
            })?;

            let text_val = record.get(text_idx).unwrap_or("").trim().to_string();
            let label_val = record.get(label_idx).unwrap_or("").trim().to_string();

            if text_val.is_empty() || label_val.is_empty() {
                continue;
            }

            if !label_to_id.contains_key(&label_val) {
                let id = id_to_label.len();
                label_to_id.insert(label_val.clone(), id);
                id_to_label.push(label_val.clone());
            }

            records.push(TextRecord {
                text: text_val,
                label: label_val,
            });
        }

        if records.is_empty() {
            return Err(PipelineError::DatasetInvalid(format!(
                "No valid records in '{}'",
                path_buf.display()
            )));
        }

        Ok(Self {
            records,
            label_to_id,
            id_to_label,
        })
    }

    /// Loads a dataset from a JSON-Lines file (one JSON object per line).
    /// Numeric labels are stringified so that e.g. toxicity datasets with
    /// 0/1 labels ingest cleanly.
    pub fn from_jsonl<P: AsRef<Path>>(
        absolute_path: P,
        text_field_name: &str,
        label_field_name: &str,
    ) -> Result<Self, PipelineError> {
        let path_buf = absolute_path.as_ref().to_path_buf();
        if !path_buf.is_absolute() {
            return Err(PipelineError::DatasetInvalid(format!(
                "Path must be absolute: {}",
                path_buf.display()
            )));
        }

        let file = File::open(&path_buf).map_err(|e| PipelineError::IoError {
            path: path_buf.clone(),
            source: e,
        })?;

        let reader = BufReader::new(file);
        let mut records = Vec::new();
        let mut label_to_id = HashMap::new();
        let mut id_to_label = Vec::new();

        for line_res in reader.lines() {
            let line = line_res.map_err(|e| PipelineError::IoError {
                path: path_buf.clone(),
                source: e,
            })?;
            let line_str = line.trim();
            if line_str.is_empty() {
                continue;
            }

            let val: serde_json::Value =
                serde_json::from_str(line_str).map_err(|e| PipelineError::JsonError {
                    path: path_buf.clone(),
                    source: e,
                })?;

            let text_val = match val.get(text_field_name) {
                Some(serde_json::Value::String(s)) => s.trim().to_string(),
                _ => continue,
            };

            let label_val = match val.get(label_field_name) {
                Some(serde_json::Value::String(s)) => s.trim().to_string(),
                Some(serde_json::Value::Number(n)) => n.to_string(),
                _ => continue,
            };

            if text_val.is_empty() || label_val.is_empty() {
                continue;
            }

            if !label_to_id.contains_key(&label_val) {
                let id = id_to_label.len();
                label_to_id.insert(label_val.clone(), id);
                id_to_label.push(label_val.clone());
            }

            records.push(TextRecord {
                text: text_val,
                label: label_val,
            });
        }

        if records.is_empty() {
            return Err(PipelineError::DatasetInvalid(format!(
                "No records in JSONL '{}'",
                path_buf.display()
            )));
        }

        Ok(Self {
            records,
            label_to_id,
            id_to_label,
        })
    }

    /// Re-keys this dataset's label maps to match a reference (typically the
    /// training split), so class IDs are consistent across train and test.
    pub fn align_labels_with(&mut self, reference_label_to_id: &HashMap<String, usize>) {
        self.label_to_id = reference_label_to_id.clone();
        let mut id_to_label = vec![String::new(); reference_label_to_id.len()];
        for (label, &id) in reference_label_to_id {
            if id < id_to_label.len() {
                id_to_label[id] = label.clone();
            }
        }
        self.id_to_label = id_to_label;
    }

    /// Shuffles (Fisher–Yates, seeded RNG) and splits into train/test subsets.
    pub fn split(
        &self,
        train_ratio: f64,
        rng: &mut FastRng,
    ) -> Result<(Dataset, Dataset), PipelineError> {
        if !(0.0..=1.0).contains(&train_ratio) {
            return Err(PipelineError::InvalidConfiguration(
                "train_ratio must be between 0.0 and 1.0".to_string(),
            ));
        }

        let mut shuffled = self.records.clone();
        for i in (1..shuffled.len()).rev() {
            let j = rng.gen_range(0, i + 1)?;
            shuffled.swap(i, j);
        }

        let train_size = ((shuffled.len() as f64) * train_ratio).round() as usize;
        let (train_recs, test_recs) = shuffled.split_at(train_size);

        let train_set = Dataset {
            records: train_recs.to_vec(),
            label_to_id: self.label_to_id.clone(),
            id_to_label: self.id_to_label.clone(),
        };

        let test_set = Dataset {
            records: test_recs.to_vec(),
            label_to_id: self.label_to_id.clone(),
            id_to_label: self.id_to_label.clone(),
        };

        Ok((train_set, test_set))
    }
}

// ---------------------------------------------------------------------------
// Vocabulary (word-level; BOW for the flat baseline, ID-sequences for the
// windowed model). TF-IDF was deleted with the K-means stage — it served no
// remaining consumer.
// ---------------------------------------------------------------------------

/// Word-level vocabulary built from the training corpus. Provides two feature
/// views over the same vocabulary space:
/// - `text_to_bow`            → flat presence vector for `VanillaTM` (baseline);
/// - `text_to_token_sequence` → ordered word-ID sequence for `WindowedTM`,
///   where `None` marks an out-of-vocabulary token ("no known word here"),
///   deliberately preserving positional adjacency instead of collapsing it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vocabulary {
    pub word_to_id: HashMap<String, usize>,
    pub id_to_word: Vec<String>,
}

impl Vocabulary {
    /// Builds the vocabulary from the training corpus: document-frequency
    /// filtering (`min_df`), then most-frequent-first truncation to
    /// `max_features` if given.
    pub fn build(
        corpus: &[String],
        min_df: usize,
        max_features: Option<usize>,
    ) -> Result<Self, PipelineError> {
        if corpus.is_empty() {
            return Err(PipelineError::DatasetInvalid(
                "Cannot build vocabulary from empty corpus".to_string(),
            ));
        }

        let mut doc_freq: HashMap<String, usize> = HashMap::new();
        for doc in corpus {
            let unique_tokens: HashSet<String> = Self::tokenize(doc).into_iter().collect();
            for tok in unique_tokens {
                *doc_freq.entry(tok).or_insert(0) += 1;
            }
        }

        let mut filtered: Vec<(String, usize)> = doc_freq
            .into_iter()
            .filter(|&(_, df)| df >= min_df)
            .collect();

        // Most-frequent-first; ties broken alphabetically for determinism.
        filtered.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        if let Some(cap) = max_features {
            filtered.truncate(cap);
        }

        if filtered.is_empty() {
            return Err(PipelineError::DatasetInvalid(format!(
                "Vocabulary is empty after min_df ({min_df}) filtering"
            )));
        }

        let mut word_to_id = HashMap::new();
        let mut id_to_word = Vec::new();

        for (word, _df) in filtered {
            let id = id_to_word.len();
            word_to_id.insert(word.clone(), id);
            id_to_word.push(word);
        }

        Ok(Self {
            word_to_id,
            id_to_word,
        })
    }

    /// Canonical tokenizer shared by all feature views: lowercase, split on
    /// non-alphanumeric characters, drop single-character fragments. Word-level
    /// tokenization is locked in for Phase 1 (byte-level is a future ablation).
    pub fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| s.len() > 1)
            .map(|s| s.to_string())
            .collect()
    }

    /// Flat Bag-of-Words presence vector for the `VanillaTM` baseline. This is
    /// the representation whose lack of order sensitivity motivated Phase 1.
    pub fn text_to_bow(&self, text: &str) -> Vec<bool> {
        let mut bow = vec![false; self.id_to_word.len()];
        for token in Self::tokenize(text) {
            if let Some(&id) = self.word_to_id.get(&token) {
                bow[id] = true;
            }
        }
        bow
    }

    /// Ordered word-ID sequence for the `WindowedTM`. Out-of-vocabulary tokens
    /// become `None` ("no known word here") rather than being dropped, so that
    /// adjacency relations between the remaining known words are not falsified
    /// (dropping OOV tokens would make non-adjacent words appear adjacent).
    /// `None` has identical semantics to PAD: fails positive literals,
    /// satisfies negated literals.
    pub fn text_to_token_sequence(&self, text: &str) -> Vec<Option<usize>> {
        Self::tokenize(text)
            .iter()
            .map(|token| self.word_to_id.get(token).copied())
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Flat Baseline: Vanilla Tsetlin Machine over BOW (retained for ablation)
// ---------------------------------------------------------------------------

/// The flat Bag-of-Words Tsetlin Machine baseline. Retained unchanged (modulo
/// documentation) so the windowed model can be benchmarked head-to-head on
/// identical splits, isolating the effect of windowing alone.
///
/// Layout: `ta_states[class][clause][literal]` with `2 * num_features`
/// literals per clause — positive literals at `[0, num_features)`, negated
/// literals at `[num_features, 2*num_features)`. A literal is *included* in a
/// clause iff its state exceeds `states_per_action`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VanillaTM {
    pub num_classes: usize,
    pub num_clauses_per_class: usize,
    pub num_features: usize,
    pub threshold: i32,
    pub states_per_action: i32,
    pub specificity: f64,
    pub ta_states: Vec<Vec<Vec<i32>>>,
}

impl VanillaTM {
    /// Constructs a flat TM with every automaton initialised on the exclude
    /// side of the boundary (state == `states_per_action`).
    pub fn new(
        num_classes: usize,
        num_clauses_per_class: usize,
        num_features: usize,
        threshold: i32,
        states_per_action: i32,
        specificity: f64,
    ) -> Result<Self, PipelineError> {
        if num_classes == 0 || num_clauses_per_class == 0 || num_features == 0 {
            return Err(PipelineError::InvalidConfiguration(
                "VanillaTM: num_classes, num_clauses_per_class and num_features must be > 0"
                    .to_string(),
            ));
        }
        if threshold <= 0 || states_per_action <= 0 {
            return Err(PipelineError::InvalidConfiguration(
                "VanillaTM: threshold and states_per_action must be > 0".to_string(),
            ));
        }
        if specificity <= 1.0 {
            return Err(PipelineError::InvalidConfiguration(
                "VanillaTM: specificity must be > 1.0".to_string(),
            ));
        }
        let n = states_per_action;
        let ta_states = vec![vec![vec![n; 2 * num_features]; num_clauses_per_class]; num_classes];

        Ok(Self {
            num_classes,
            num_clauses_per_class,
            num_features,
            threshold,
            states_per_action,
            specificity,
            ta_states,
        })
    }

    /// Evaluates one clause as a conjunction over its included literals.
    #[inline(always)]
    fn evaluate_clause(&self, class: usize, clause: usize, x: &[bool]) -> bool {
        let n = self.states_per_action;
        let states = &self.ta_states[class][clause];

        for k in 0..self.num_features {
            if states[k] > n && !x[k] {
                return false;
            }
            if states[self.num_features + k] > n && x[k] {
                return false;
            }
        }
        true
    }

    /// Signed vote sum for one class: even-indexed clauses vote +1 on firing,
    /// odd-indexed clauses vote -1 (standard polarity convention).
    pub fn calculate_class_sum(&self, class: usize, x: &[bool]) -> i32 {
        let mut score = 0;
        for c in 0..self.num_clauses_per_class {
            if self.evaluate_clause(class, c, x) {
                if c % 2 == 0 {
                    score += 1;
                } else {
                    score -= 1;
                }
            }
        }
        score
    }

    /// Argmax-of-class-sums prediction. Ties resolve to the lowest class ID
    /// (deterministic).
    pub fn predict(&self, x: &[bool]) -> Result<usize, PipelineError> {
        if x.len() != self.num_features {
            return Err(PipelineError::DimensionMismatch {
                expected: self.num_features,
                found: x.len(),
                context: "VanillaTM predict".to_string(),
            });
        }

        let mut best_class = 0;
        let mut max_score = i32::MIN;
        for class in 0..self.num_classes {
            let score = self.calculate_class_sum(class, x);
            if score > max_score {
                max_score = score;
                best_class = class;
            }
        }
        Ok(best_class)
    }

    /// One stochastic training update (Type I / Type II feedback) for one
    /// sample against all classes, gated by the resource-allocation threshold.
    pub fn train_step(
        &mut self,
        x: &[bool],
        target_class: usize,
        rng: &mut FastRng,
    ) -> Result<(), PipelineError> {
        if x.len() != self.num_features {
            return Err(PipelineError::DimensionMismatch {
                expected: self.num_features,
                found: x.len(),
                context: "VanillaTM train_step".to_string(),
            });
        }
        if target_class >= self.num_classes {
            return Err(PipelineError::InvalidConfiguration(format!(
                "VanillaTM train_step: target_class {} out of range ({} classes)",
                target_class, self.num_classes
            )));
        }

        let n = self.states_per_action;
        let t = self.threshold;
        let s = self.specificity;

        for class in 0..self.num_classes {
            let class_sum = self.calculate_class_sum(class, x).clamp(-t, t);

            let update_prob = if class == target_class {
                (t - class_sum) as f64 / (2.0 * t as f64)
            } else {
                (t + class_sum) as f64 / (2.0 * t as f64)
            };

            if rng.gen_f64() > update_prob {
                continue;
            }

            let is_target = class == target_class;

            for c in 0..self.num_clauses_per_class {
                let is_positive_clause = c % 2 == 0;
                let clause_output = self.evaluate_clause(class, c, x);

                if (is_target && is_positive_clause) || (!is_target && !is_positive_clause) {
                    // Type I feedback: reinforce patterns (Ia) or erase (Ib).
                    if clause_output {
                        for k in 0..self.num_features {
                            if x[k] {
                                if rng.gen_f64() <= (s - 1.0) / s
                                    && self.ta_states[class][c][k] < 2 * n
                                {
                                    self.ta_states[class][c][k] += 1;
                                }
                            } else if rng.gen_f64() <= 1.0 / s && self.ta_states[class][c][k] > 1 {
                                self.ta_states[class][c][k] -= 1;
                            }

                            let neg_k = self.num_features + k;
                            if !x[k] {
                                if rng.gen_f64() <= (s - 1.0) / s
                                    && self.ta_states[class][c][neg_k] < 2 * n
                                {
                                    self.ta_states[class][c][neg_k] += 1;
                                }
                            } else if rng.gen_f64() <= 1.0 / s
                                && self.ta_states[class][c][neg_k] > 1
                            {
                                self.ta_states[class][c][neg_k] -= 1;
                            }
                        }
                    } else {
                        for lit in 0..2 * self.num_features {
                            if rng.gen_f64() <= 1.0 / s && self.ta_states[class][c][lit] > 1 {
                                self.ta_states[class][c][lit] -= 1;
                            }
                        }
                    }
                } else if clause_output {
                    // Type II feedback: block spurious firing by including
                    // literals that are false in this sample.
                    for k in 0..self.num_features {
                        if !x[k] && self.ta_states[class][c][k] < 2 * n {
                            self.ta_states[class][c][k] += 1;
                        }
                        let neg_k = self.num_features + k;
                        if x[k] && self.ta_states[class][c][neg_k] < 2 * n {
                            self.ta_states[class][c][neg_k] += 1;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Phase 1 core: Windowed ("convolutional") Tsetlin Machine
// ---------------------------------------------------------------------------

/// How per-window clause firings are pooled into one document-level signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoolingStrategy {
    /// OR/max-pool: clause contributes 1 vote if it fired at any window.
    AnyFire,
    /// Sum-pool: clause contributes `min(fire_count, clause_vote_cap)` votes.
    /// With `clause_vote_cap == 1` this degenerates to `AnyFire`.
    CountFire,
}

/// The windowed clause bank: the Phase 1 convolution analogue.
///
/// Each clause is a conjunction over literals indexed by
/// `(relative_slot, word_id)`:
/// - positive literal `(w, v)`: "the word at window slot `w` is exactly `v`";
/// - negated literal `(w, v)`: "the word at window slot `w` is not `v`"
///   (satisfied by PAD/OOV slots, which carry no word).
///
/// Literal layout within `ta_states[class][clause]` (length `2 * W * V`):
/// - positive `(w, v)` at index `w * V + v`;
/// - negated  `(w, v)` at index `W * V + w * V + v`.
///
/// ## O(W) evaluation invariant
/// `included_positive_counts[class][clause][slot]` caches how many positive
/// literals are currently included at each slot. At most one word occupies a
/// slot, so a slot passes its positive-literal constraints iff
/// `count == 0`, or `count == 1` and the included literal is the observed
/// word's. This reduces per-window clause evaluation from O(W×V) to O(W).
/// The cache is updated on every include/exclude boundary crossing and is
/// re-validated on model load.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowedTM {
    pub num_classes: usize,
    pub num_clauses_per_class: usize,
    pub vocab_size: usize,
    pub window_width: usize,
    pub threshold: i32,
    pub states_per_action: i32,
    pub specificity: f64,
    pub pooling: PoolingStrategy,
    pub clause_vote_cap: u32,
    pub ta_states: Vec<Vec<Vec<i32>>>,
    /// Cached count of included positive literals per (class, clause, slot).
    /// Serialized alongside the states; consistency is verified on load.
    included_positive_counts: Vec<Vec<Vec<u32>>>,
}

impl WindowedTM {
    /// Constructs a windowed TM with all automata initialised at the exclude
    /// boundary (state == `states_per_action`), hence zero included literals
    /// and all-zero counters — a consistent starting invariant.
    ///
    /// Memory note: each clause holds `2 * window_width * vocab_size` i32
    /// states. E.g. V=4000, W=3, 80 clauses, 2 classes ≈ 15 MB.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        num_classes: usize,
        num_clauses_per_class: usize,
        vocab_size: usize,
        window_width: usize,
        threshold: i32,
        states_per_action: i32,
        specificity: f64,
        pooling: PoolingStrategy,
        clause_vote_cap: u32,
    ) -> Result<Self, PipelineError> {
        if num_classes == 0 || num_clauses_per_class == 0 || vocab_size == 0 || window_width == 0 {
            return Err(PipelineError::InvalidConfiguration(
                "WindowedTM: classes, clauses, vocab_size and window_width must be > 0".to_string(),
            ));
        }
        if threshold <= 0 || states_per_action <= 0 {
            return Err(PipelineError::InvalidConfiguration(
                "WindowedTM: threshold and states_per_action must be > 0".to_string(),
            ));
        }
        if specificity <= 1.0 {
            return Err(PipelineError::InvalidConfiguration(
                "WindowedTM: specificity must be > 1.0".to_string(),
            ));
        }
        if clause_vote_cap == 0 {
            return Err(PipelineError::InvalidConfiguration(
                "WindowedTM: clause_vote_cap must be >= 1".to_string(),
            ));
        }

        let n = states_per_action;
        let literals = 2 * window_width * vocab_size;
        let ta_states = vec![vec![vec![n; literals]; num_clauses_per_class]; num_classes];
        let included_positive_counts =
            vec![vec![vec![0u32; window_width]; num_clauses_per_class]; num_classes];

        Ok(Self {
            num_classes,
            num_clauses_per_class,
            vocab_size,
            window_width,
            threshold,
            states_per_action,
            specificity,
            pooling,
            clause_vote_cap,
            ta_states,
            included_positive_counts,
        })
    }

    // --- Literal index helpers (see struct-level layout documentation) ---

    #[inline(always)]
    fn positive_literal_index(&self, slot: usize, word_id: usize) -> usize {
        slot * self.vocab_size + word_id
    }

    #[inline(always)]
    fn negated_literal_index(&self, slot: usize, word_id: usize) -> usize {
        self.window_width * self.vocab_size + slot * self.vocab_size + word_id
    }

    #[inline(always)]
    fn literals_per_clause(&self) -> usize {
        2 * self.window_width * self.vocab_size
    }

    /// Number of sliding-window positions for a document. Documents shorter
    /// than the window width (including empty documents after OOV filtering)
    /// deliberately yield exactly one right-PAD-padded window — the explicit
    /// short-document policy from the hand-off, never a skip or a panic.
    fn num_windows(token_count: usize, window_width: usize) -> usize {
        if token_count >= window_width {
            token_count - window_width + 1
        } else {
            1
        }
    }

    /// Reads the token at an absolute document position; positions beyond the
    /// document end are PAD (`None`), unifying PAD and OOV semantics.
    #[inline(always)]
    fn token_at(tokens: &[Option<usize>], position: usize) -> Option<usize> {
        tokens.get(position).copied().flatten()
    }

    /// O(window_width) clause evaluation at one window position, using the
    /// included-positive-count cache (see struct docs for the invariant).
    fn evaluate_clause_at_window(
        &self,
        class: usize,
        clause: usize,
        tokens: &[Option<usize>],
        window_start: usize,
    ) -> bool {
        let n = self.states_per_action;
        let states = &self.ta_states[class][clause];
        let required_counts = &self.included_positive_counts[class][clause];

        for slot in 0..self.window_width {
            match Self::token_at(tokens, window_start + slot) {
                Some(word_id) => {
                    // Positive constraint: every included positive literal at
                    // this slot must name the observed word. Fails iff there
                    // exists an included positive literal other than the
                    // observed word's own.
                    let own_included = states[self.positive_literal_index(slot, word_id)] > n;
                    if required_counts[slot] > u32::from(own_included) {
                        return false;
                    }
                    // Negated constraint: the observed word must not be
                    // negatively included at this slot.
                    if states[self.negated_literal_index(slot, word_id)] > n {
                        return false;
                    }
                }
                None => {
                    // PAD/OOV: fails any positive literal; satisfies all
                    // negated literals (confirmed decision #4).
                    if required_counts[slot] > 0 {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// All window start positions at which the given clause fires. Public so
    /// the explainability layer can report firing positions.
    pub fn fired_window_positions(
        &self,
        class: usize,
        clause: usize,
        tokens: &[Option<usize>],
    ) -> Result<Vec<usize>, PipelineError> {
        if class >= self.num_classes || clause >= self.num_clauses_per_class {
            return Err(PipelineError::InvalidConfiguration(format!(
                "fired_window_positions: class {class}/clause {clause} out of range"
            )));
        }
        let windows = Self::num_windows(tokens.len(), self.window_width);
        Ok((0..windows)
            .filter(|&p| self.evaluate_clause_at_window(class, clause, tokens, p))
            .collect())
    }

    /// Maps a raw per-clause fire count to its pooled vote magnitude under
    /// the configured pooling strategy (confirmed decisions #5 and default
    /// `CountFire`).
    #[inline(always)]
    fn pooled_vote_magnitude(&self, fire_count: u32) -> u32 {
        match self.pooling {
            PoolingStrategy::AnyFire => u32::from(fire_count > 0),
            PoolingStrategy::CountFire => fire_count.min(self.clause_vote_cap),
        }
    }

    /// Signed, pooled vote sum for one class. Sequential by design: it is the
    /// inner loop of training. `predict` parallelises across classes instead.
    pub fn calculate_class_sum(
        &self,
        class: usize,
        tokens: &[Option<usize>],
    ) -> Result<i32, PipelineError> {
        if class >= self.num_classes {
            return Err(PipelineError::InvalidConfiguration(format!(
                "calculate_class_sum: class {class} out of range ({} classes)",
                self.num_classes
            )));
        }
        let windows = Self::num_windows(tokens.len(), self.window_width);
        let mut score: i32 = 0;
        for clause in 0..self.num_clauses_per_class {
            let fire_count = (0..windows)
                .filter(|&p| self.evaluate_clause_at_window(class, clause, tokens, p))
                .count() as u32;
            let magnitude = self.pooled_vote_magnitude(fire_count) as i32;
            if clause % 2 == 0 {
                score += magnitude;
            } else {
                score -= magnitude;
            }
        }
        Ok(score)
    }

    /// Argmax prediction over pooled class sums, parallelised across classes
    /// with rayon (production/inference efficiency priority). Ties resolve to
    /// the lowest class ID for determinism.
    pub fn predict(&self, tokens: &[Option<usize>]) -> Result<usize, PipelineError> {
        let sums: Result<Vec<(usize, i32)>, PipelineError> = (0..self.num_classes)
            .into_par_iter()
            .map(|class| Ok((class, self.calculate_class_sum(class, tokens)?)))
            .collect();
        let sums = sums?;

        let mut best_class = 0usize;
        let mut best_score = i32::MIN;
        for (class, score) in sums {
            // Strict '>' plus ascending class order => lowest class ID wins ties.
            if score > best_score {
                best_score = score;
                best_class = class;
            }
        }
        Ok(best_class)
    }

    // --- Automaton transitions with counter maintenance -------------------
    // Every state change routes through these two methods so the O(W)
    // evaluation invariant (included_positive_counts) can never drift.

    /// Increments a TA state (bounded at `2n`), updating the positive-literal
    /// counter when the state crosses the exclude→include boundary.
    fn increment_ta_state(&mut self, class: usize, clause: usize, literal: usize) {
        let n = self.states_per_action;
        let state = self.ta_states[class][clause][literal];
        if state < 2 * n {
            self.ta_states[class][clause][literal] = state + 1;
            // Crossing n -> n+1 means this literal just became included.
            if state == n && literal < self.window_width * self.vocab_size {
                let slot = literal / self.vocab_size;
                self.included_positive_counts[class][clause][slot] += 1;
            }
        }
    }

    /// Decrements a TA state (bounded at 1), updating the positive-literal
    /// counter when the state crosses the include→exclude boundary.
    fn decrement_ta_state(&mut self, class: usize, clause: usize, literal: usize) {
        let n = self.states_per_action;
        let state = self.ta_states[class][clause][literal];
        if state > 1 {
            self.ta_states[class][clause][literal] = state - 1;
            // Crossing n+1 -> n means this literal just became excluded.
            if state == n + 1 && literal < self.window_width * self.vocab_size {
                let slot = literal / self.vocab_size;
                // saturating_sub as defence-in-depth; validated invariant
                // means this should never actually saturate.
                self.included_positive_counts[class][clause][slot] =
                    self.included_positive_counts[class][clause][slot].saturating_sub(1);
            }
        }
    }

    // --- Feedback (aggregate-outcome attribution, confirmed decision) -----

    /// Type Ia feedback: the clause fired (pooled) and should have — reinforce
    /// the literal pattern of ONE randomly chosen fired window (standard
    /// Convolutional-TM credit assignment under aggregate-outcome feedback).
    /// True literals strengthen with probability (s-1)/s; false literals decay
    /// with probability 1/s. Training cost is O(W×V) per event — acceptable,
    /// since production inference (not training) is the efficiency priority.
    fn apply_type_ia_feedback(
        &mut self,
        class: usize,
        clause: usize,
        tokens: &[Option<usize>],
        window_start: usize,
        rng: &mut FastRng,
    ) {
        let s = self.specificity;
        let reinforce_prob = (s - 1.0) / s;
        let forget_prob = 1.0 / s;

        for slot in 0..self.window_width {
            let token = Self::token_at(tokens, window_start + slot);
            for word_id in 0..self.vocab_size {
                let positive_literal_true = token == Some(word_id);
                let pos_lit = self.positive_literal_index(slot, word_id);
                let neg_lit = self.negated_literal_index(slot, word_id);

                if positive_literal_true {
                    if rng.gen_f64() <= reinforce_prob {
                        self.increment_ta_state(class, clause, pos_lit);
                    }
                } else if rng.gen_f64() <= forget_prob {
                    self.decrement_ta_state(class, clause, pos_lit);
                }

                // Negated literal truth is the complement (PAD/None => true).
                if !positive_literal_true {
                    if rng.gen_f64() <= reinforce_prob {
                        self.increment_ta_state(class, clause, neg_lit);
                    }
                } else if rng.gen_f64() <= forget_prob {
                    self.decrement_ta_state(class, clause, neg_lit);
                }
            }
        }
    }

    /// Type Ib feedback: the clause should have fired but did not at any
    /// window — gently erase (all literals decay with probability 1/s),
    /// freeing the clause to learn a new pattern.
    fn apply_type_ib_feedback(&mut self, class: usize, clause: usize, rng: &mut FastRng) {
        let forget_prob = 1.0 / self.specificity;
        for literal in 0..self.literals_per_clause() {
            if rng.gen_f64() <= forget_prob {
                self.decrement_ta_state(class, clause, literal);
            }
        }
    }

    /// Type II feedback: the clause fired (pooled) but should not have —
    /// increment every literal that is FALSE in one randomly chosen fired
    /// window, pushing the clause toward including a blocking literal
    /// (mirrors `VanillaTM`'s Type II semantics).
    fn apply_type_ii_feedback(
        &mut self,
        class: usize,
        clause: usize,
        tokens: &[Option<usize>],
        window_start: usize,
    ) {
        for slot in 0..self.window_width {
            let token = Self::token_at(tokens, window_start + slot);
            for word_id in 0..self.vocab_size {
                let positive_literal_true = token == Some(word_id);
                if !positive_literal_true {
                    let pos_lit = self.positive_literal_index(slot, word_id);
                    self.increment_ta_state(class, clause, pos_lit);
                } else {
                    let neg_lit = self.negated_literal_index(slot, word_id);
                    self.increment_ta_state(class, clause, neg_lit);
                }
            }
        }
    }

    /// One stochastic training update for one tokenized document. Shape
    /// mirrors `VanillaTM::train_step`: per-class resource-allocation gating
    /// on the (pooled) class sum, then Type I / Type II feedback per clause,
    /// with the pooled any-window firing as the clause outcome.
    pub fn train_step(
        &mut self,
        tokens: &[Option<usize>],
        target_class: usize,
        rng: &mut FastRng,
    ) -> Result<(), PipelineError> {
        if target_class >= self.num_classes {
            return Err(PipelineError::InvalidConfiguration(format!(
                "WindowedTM train_step: target_class {} out of range ({} classes)",
                target_class, self.num_classes
            )));
        }

        let t = self.threshold;
        let windows = Self::num_windows(tokens.len(), self.window_width);

        for class in 0..self.num_classes {
            let class_sum = self.calculate_class_sum(class, tokens)?.clamp(-t, t);

            let update_prob = if class == target_class {
                (t - class_sum) as f64 / (2.0 * t as f64)
            } else {
                (t + class_sum) as f64 / (2.0 * t as f64)
            };

            if rng.gen_f64() > update_prob {
                continue;
            }

            let is_target = class == target_class;

            for clause in 0..self.num_clauses_per_class {
                let is_positive_clause = clause % 2 == 0;

                // Collect fired windows BEFORE mutating states, both because
                // feedback needs a concrete window to attribute against and
                // because evaluation borrows immutably.
                let fired: Vec<usize> = (0..windows)
                    .filter(|&p| self.evaluate_clause_at_window(class, clause, tokens, p))
                    .collect();
                let clause_fired = !fired.is_empty();

                if (is_target && is_positive_clause) || (!is_target && !is_positive_clause) {
                    if clause_fired {
                        let chosen = fired[rng.gen_range(0, fired.len())?];
                        self.apply_type_ia_feedback(class, clause, tokens, chosen, rng);
                    } else {
                        self.apply_type_ib_feedback(class, clause, rng);
                    }
                } else if clause_fired {
                    let chosen = fired[rng.gen_range(0, fired.len())?];
                    self.apply_type_ii_feedback(class, clause, tokens, chosen);
                }
            }
        }
        Ok(())
    }

    /// Recomputes the included-positive-literal counters from raw TA states
    /// and verifies they match the cache. Called after deserialization and in
    /// tests, guarding the O(W) evaluation invariant against corrupt or
    /// hand-edited model files.
    pub fn validate_internal_consistency(&self) -> Result<(), PipelineError> {
        let n = self.states_per_action;
        for class in 0..self.num_classes {
            for clause in 0..self.num_clauses_per_class {
                for slot in 0..self.window_width {
                    let recomputed = (0..self.vocab_size)
                        .filter(|&w| {
                            self.ta_states[class][clause][self.positive_literal_index(slot, w)] > n
                        })
                        .count() as u32;
                    let cached = self.included_positive_counts[class][clause][slot];
                    if recomputed != cached {
                        return Err(PipelineError::NumericalError(format!(
                            "WindowedTM counter inconsistency at class {class}, clause {clause}, \
                                    slot {slot}: cached {cached}, recomputed {recomputed}"
                        )));
                    }
                }
            }
        }
        Ok(())
    }

    /// Decodes one clause into a human-readable positional pattern, e.g.
    /// `[w+0] not ∧ [w+1] good ∧ [w+2] ¬great`. Rendering is capped to keep
    /// explainability output readable when many negated literals accumulate.
    /// Returns an empty string for an empty clause (fires everywhere; not a
    /// meaningful rule).
    pub fn describe_clause_pattern(
        &self,
        class: usize,
        clause: usize,
        vocab: &Vocabulary,
        max_rendered_literals: usize,
    ) -> Result<String, PipelineError> {
        if class >= self.num_classes || clause >= self.num_clauses_per_class {
            return Err(PipelineError::InvalidConfiguration(format!(
                "describe_clause_pattern: class {class}/clause {clause} out of range"
            )));
        }
        let n = self.states_per_action;
        let states = &self.ta_states[class][clause];
        let mut parts: Vec<String> = Vec::new();
        let mut omitted = 0usize;

        for slot in 0..self.window_width {
            for word_id in 0..self.vocab_size {
                let word = vocab
                    .id_to_word
                    .get(word_id)
                    .cloned()
                    .unwrap_or_else(|| format!("word#{word_id}"));

                if states[self.positive_literal_index(slot, word_id)] > n {
                    if parts.len() < max_rendered_literals {
                        parts.push(format!("[w+{slot}] {word}"));
                    } else {
                        omitted += 1;
                    }
                }
                if states[self.negated_literal_index(slot, word_id)] > n {
                    if parts.len() < max_rendered_literals {
                        parts.push(format!("[w+{slot}] ¬{word}"));
                    } else {
                        omitted += 1;
                    }
                }
            }
        }

        if parts.is_empty() {
            return Ok(String::new());
        }
        let mut rendered = parts.join(" ∧ ");
        if omitted > 0 {
            rendered.push_str(&format!(" … (+{omitted} more literals)"));
        }
        Ok(rendered)
    }

    /// Test-only helper: forces a literal fully into the include region via
    /// the counter-maintaining transition path, so tests can construct exact
    /// clauses without breaking the evaluation invariant.
    #[cfg(test)]
    fn test_force_include_literal(&mut self, class: usize, clause: usize, literal: usize) {
        while self.ta_states[class][clause][literal] < 2 * self.states_per_action {
            self.increment_ta_state(class, clause, literal);
        }
    }
}

// ---------------------------------------------------------------------------
// Evaluation Report
// ---------------------------------------------------------------------------

/// Standard multi-class evaluation metrics plus confusion matrix, used for
/// the flat-vs-windowed head-to-head benchmark.
#[derive(Debug, Clone)]
pub struct EvaluationReport {
    pub accuracy: f64,
    pub macro_precision: f64,
    pub macro_recall: f64,
    pub macro_f1: f64,
    pub confusion_matrix: Vec<Vec<usize>>,
    pub class_labels: Vec<String>,
    pub total_samples: usize,
    pub train_duration: Option<Duration>,
}

impl EvaluationReport {
    /// Computes accuracy, macro precision/recall/F1 and the confusion matrix
    /// from parallel prediction/ground-truth vectors.
    pub fn compute(
        predictions: &[usize],
        ground_truth: &[usize],
        class_labels: &[String],
    ) -> Result<Self, PipelineError> {
        if predictions.len() != ground_truth.len() {
            return Err(PipelineError::DimensionMismatch {
                expected: ground_truth.len(),
                found: predictions.len(),
                context: "EvaluationReport::compute".to_string(),
            });
        }
        if ground_truth.is_empty() {
            return Err(PipelineError::DatasetInvalid(
                "Empty predictions".to_string(),
            ));
        }

        let num_classes = class_labels.len();
        let mut cm = vec![vec![0usize; num_classes]; num_classes];
        let mut correct = 0usize;

        for (&pred, &actual) in predictions.iter().zip(ground_truth.iter()) {
            if pred < num_classes && actual < num_classes {
                cm[actual][pred] += 1;
                if pred == actual {
                    correct += 1;
                }
            }
        }

        let accuracy = correct as f64 / ground_truth.len() as f64;
        let mut precisions = Vec::with_capacity(num_classes);
        let mut recalls = Vec::with_capacity(num_classes);
        let mut f1s = Vec::with_capacity(num_classes);

        for c in 0..num_classes {
            let tp = cm[c][c] as f64;
            let fp: f64 = (0..num_classes)
                .filter(|&i| i != c)
                .map(|i| cm[i][c] as f64)
                .sum();
            let fn_val: f64 = (0..num_classes)
                .filter(|&j| j != c)
                .map(|j| cm[c][j] as f64)
                .sum();

            let prec = if (tp + fp) > 0.0 { tp / (tp + fp) } else { 0.0 };
            let rec = if (tp + fn_val) > 0.0 {
                tp / (tp + fn_val)
            } else {
                0.0
            };
            let f1 = if (prec + rec) > 0.0 {
                2.0 * (prec * rec) / (prec + rec)
            } else {
                0.0
            };

            precisions.push(prec);
            recalls.push(rec);
            f1s.push(f1);
        }

        let macro_precision = precisions.iter().sum::<f64>() / num_classes as f64;
        let macro_recall = recalls.iter().sum::<f64>() / num_classes as f64;
        let macro_f1 = f1s.iter().sum::<f64>() / num_classes as f64;

        Ok(Self {
            accuracy,
            macro_precision,
            macro_recall,
            macro_f1,
            confusion_matrix: cm,
            class_labels: class_labels.to_vec(),
            total_samples: ground_truth.len(),
            train_duration: None,
        })
    }

    /// Prints the report in a fixed-width console layout.
    pub fn print_summary(&self) {
        println!("\n============================================================");
        println!("               Classification Evaluation Report             ");
        println!("============================================================");
        println!("  Evaluated Samples: {}", self.total_samples); // <--- SPOT INSERTION B
        if let Some(dur) = self.train_duration {
            println!("  Training Time:     {:.2?}", dur); // <--- SPOT INSERTION A
        }
        println!("  Accuracy:        {:.2}%", self.accuracy * 100.0);
        println!("  Macro Precision: {:.4}", self.macro_precision);
        println!("  Macro Recall:    {:.4}", self.macro_recall);
        println!("  Macro F1-Score:  {:.4}", self.macro_f1);
        println!("------------------------------------------------------------");
        println!("Confusion Matrix (Rows: Actual, Columns: Predicted):");

        print!("{:<15}", "");
        for label in &self.class_labels {
            print!("{:<14}", label);
        }
        println!();

        for (i, actual_label) in self.class_labels.iter().enumerate() {
            print!("{:<15}", actual_label);
            for j in 0..self.class_labels.len() {
                print!("{:<14}", self.confusion_matrix[i][j]);
            }
            println!();
        }
        println!("============================================================\n");
    }
}

// ---------------------------------------------------------------------------
// Pipeline Artifact, Explainability & Inference
// ---------------------------------------------------------------------------

/// A decoded, human-readable clause that fired during inference — the
/// white-box explainability trace. Extended (not replaced) for Phase 1:
/// `fired_positions` reports the window start positions (token indices) at
/// which a windowed clause fired; it is empty for flat-model rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiredRule {
    pub clause_index: usize,
    pub class_name: String,
    /// +1 for a positive (vote-for) clause, -1 for a negative (veto) clause.
    pub polarity: i32,
    /// Rendered conjunction. Flat: `good ∧ ¬bad`. Windowed (positional):
    /// `[w+0] not ∧ [w+1] good`.
    pub rule: String,
    /// Window start positions where the clause fired (windowed model only).
    #[serde(default)]
    pub fired_positions: Vec<usize>,
}

/// Full structured result of a single-text prediction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionOutput {
    pub text: String,
    pub label: String,
    pub class_id: usize,
    /// Vote gap between the winning class and the runner-up.
    pub margin: i32,
    pub class_votes: HashMap<String, i32>,
    pub rules: Vec<FiredRule>,
}

/// Which classifier a trained pipeline artifact contains. The enum makes the
/// flat baseline and the Phase 1 windowed model share one save/load/predict
/// surface, per the confirmed restructuring decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassifierModel {
    Flat(VanillaTM),
    Windowed(WindowedTM),
}

/// The persisted, self-contained model artifact: vocabulary + classifier +
/// label maps. NOTE: not compatible with artifacts from the deleted
/// semantic-pretraining version (breaking change, accepted for the MVP).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineModel {
    pub vocab: Vocabulary,
    pub classifier: ClassifierModel,
    pub id_to_label: Vec<String>,
    pub label_to_id: HashMap<String, usize>,
}

/// Cap on rendered literals per explainability rule, keeping traces readable
/// when clauses accumulate many negated literals.
const MAX_RENDERED_LITERALS_PER_RULE: usize = 12;

impl PipelineModel {
    /// Serializes the artifact as pretty JSON at an absolute path, creating
    /// parent directories as needed.
    pub fn save_to_file<P: AsRef<Path>>(&self, absolute_path: P) -> Result<(), PipelineError> {
        let path_buf = absolute_path.as_ref().to_path_buf();
        if !path_buf.is_absolute() {
            return Err(PipelineError::InvalidConfiguration(format!(
                "Path must be absolute: {}",
                path_buf.display()
            )));
        }

        if let Some(parent) = path_buf.parent() {
            std::fs::create_dir_all(parent).map_err(|e| PipelineError::IoError {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        let file = File::create(&path_buf).map_err(|e| PipelineError::IoError {
            path: path_buf.clone(),
            source: e,
        })?;

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self).map_err(|e| PipelineError::JsonError {
            path: path_buf,
            source: e,
        })?;

        Ok(())
    }

    /// Deserializes an artifact from an absolute path. For windowed models,
    /// the O(W) evaluation counter cache is re-validated against raw TA
    /// states before the model is accepted.
    pub fn load_from_file<P: AsRef<Path>>(absolute_path: P) -> Result<Self, PipelineError> {
        let path_buf = absolute_path.as_ref().to_path_buf();
        if !path_buf.is_absolute() {
            return Err(PipelineError::InvalidConfiguration(format!(
                "Path must be absolute: {}",
                path_buf.display()
            )));
        }

        let file = File::open(&path_buf).map_err(|e| PipelineError::IoError {
            path: path_buf.clone(),
            source: e,
        })?;

        let reader = BufReader::new(file);
        let model: Self =
            serde_json::from_reader(reader).map_err(|e| PipelineError::JsonError {
                path: path_buf,
                source: e,
            })?;

        // Guard the evaluation invariant against corrupt/edited files.
        if let ClassifierModel::Windowed(tm) = &model.classifier {
            tm.validate_internal_consistency()?;
        }

        Ok(model)
    }

    /// Classifies one text and returns the label, per-class votes, winning
    /// margin, and decoded fired rules (explainability trace).
    pub fn predict_one(
        &self,
        text: &str,
        max_rules: usize,
    ) -> Result<PredictionOutput, PipelineError> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Err(PipelineError::DatasetInvalid(
                "Input text cannot be empty".to_string(),
            ));
        }

        // Gather signed vote sums per class, model-specifically.
        let votes: Vec<i32> = match &self.classifier {
            ClassifierModel::Flat(tm) => {
                let bow = self.vocab.text_to_bow(trimmed);
                (0..tm.num_classes)
                    .map(|c| tm.calculate_class_sum(c, &bow))
                    .collect()
            }
            ClassifierModel::Windowed(tm) => {
                let tokens = self.vocab.text_to_token_sequence(trimmed);
                let collected: Result<Vec<i32>, PipelineError> = (0..tm.num_classes)
                    .into_par_iter()
                    .map(|c| tm.calculate_class_sum(c, &tokens))
                    .collect();
                collected?
            }
        };

        if votes.is_empty() {
            return Err(PipelineError::NumericalError(
                "Model has no classes".to_string(),
            ));
        }

        // Argmax with runner-up tracking for the margin.
        let mut best_class = 0usize;
        let mut max_score = i32::MIN;
        let mut second_score = i32::MIN;
        for (class, &score) in votes.iter().enumerate() {
            if score > max_score {
                second_score = max_score;
                max_score = score;
                best_class = class;
            } else if score > second_score {
                second_score = score;
            }
        }
        let margin = if votes.len() > 1 {
            max_score - second_score
        } else {
            max_score
        };

        let label = self
            .id_to_label
            .get(best_class)
            .cloned()
            .unwrap_or_else(|| format!("Class_{best_class}"));

        let class_votes: HashMap<String, i32> = self
            .id_to_label
            .iter()
            .enumerate()
            .map(|(i, name)| (name.clone(), *votes.get(i).unwrap_or(&0)))
            .collect();

        let rules = match &self.classifier {
            ClassifierModel::Flat(tm) => {
                let bow = self.vocab.text_to_bow(trimmed);
                self.extract_fired_rules_flat(tm, best_class, &bow, max_rules)
            }
            ClassifierModel::Windowed(tm) => {
                let tokens = self.vocab.text_to_token_sequence(trimmed);
                self.extract_fired_rules_windowed(tm, best_class, &tokens, max_rules)?
            }
        };

        Ok(PredictionOutput {
            text: trimmed.to_string(),
            label,
            class_id: best_class,
            margin,
            class_votes,
            rules,
        })
    }

    /// Decodes fired flat-BOW clauses for the winning class into
    /// co-occurrence rules (pre-existing behaviour, preserved).
    fn extract_fired_rules_flat(
        &self,
        tm: &VanillaTM,
        class_id: usize,
        bow: &[bool],
        max_rules: usize,
    ) -> Vec<FiredRule> {
        let n = tm.states_per_action;
        let num_features = tm.num_features;
        let class_name = self.id_to_label.get(class_id).cloned().unwrap_or_default();
        let mut fired = Vec::new();

        for c in 0..tm.num_clauses_per_class {
            if fired.len() >= max_rules {
                break;
            }

            let mut clause_satisfied = true;
            let mut included = Vec::new();

            for k in 0..num_features {
                if tm.ta_states[class_id][c][k] > n {
                    if !bow[k] {
                        clause_satisfied = false;
                        break;
                    }
                    if let Some(word) = self.vocab.id_to_word.get(k) {
                        included.push(word.clone());
                    }
                }
                let neg_k = num_features + k;
                if tm.ta_states[class_id][c][neg_k] > n {
                    if bow[k] {
                        clause_satisfied = false;
                        break;
                    }
                    if let Some(word) = self.vocab.id_to_word.get(k) {
                        included.push(format!("¬{word}"));
                    }
                }
            }

            if clause_satisfied && !included.is_empty() {
                included.truncate(MAX_RENDERED_LITERALS_PER_RULE);
                fired.push(FiredRule {
                    clause_index: c,
                    class_name: class_name.clone(),
                    polarity: if c % 2 == 0 { 1 } else { -1 },
                    rule: included.join(" ∧ "),
                    fired_positions: Vec::new(),
                });
            }
        }

        fired
    }

    /// Decodes fired windowed clauses for the winning class into positional
    /// n-gram rules, including WHERE in the document each clause fired —
    /// the Phase 1 explainability improvement over flat co-occurrence rules.
    fn extract_fired_rules_windowed(
        &self,
        tm: &WindowedTM,
        class_id: usize,
        tokens: &[Option<usize>],
        max_rules: usize,
    ) -> Result<Vec<FiredRule>, PipelineError> {
        let class_name = self.id_to_label.get(class_id).cloned().unwrap_or_default();
        let mut fired_rules = Vec::new();

        for clause in 0..tm.num_clauses_per_class {
            if fired_rules.len() >= max_rules {
                break;
            }

            let positions = tm.fired_window_positions(class_id, clause, tokens)?;
            if positions.is_empty() {
                continue;
            }

            let pattern = tm.describe_clause_pattern(
                class_id,
                clause,
                &self.vocab,
                MAX_RENDERED_LITERALS_PER_RULE,
            )?;
            // An empty pattern means an empty clause (fires everywhere) —
            // not a meaningful explanation; skip it, as the flat path does.
            if pattern.is_empty() {
                continue;
            }

            fired_rules.push(FiredRule {
                clause_index: clause,
                class_name: class_name.clone(),
                polarity: if clause % 2 == 0 { 1 } else { -1 },
                rule: pattern,
                fired_positions: positions,
            });
        }

        Ok(fired_rules)
    }
}

// ---------------------------------------------------------------------------
// Training Configuration & Engine
// ---------------------------------------------------------------------------

/// Which classifier architecture to train (the ablation axis of Phase 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    /// Flat BOW `VanillaTM` baseline.
    Flat,
    /// Phase 1 windowed (convolution-analogue) model. Default.
    Windowed,
}

/// All training hyperparameters. Windowed-specific fields are ignored by the
/// flat baseline path.
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub train_ratio: f64,
    pub min_df: usize,
    pub max_features: Option<usize>,
    pub model_type: ModelType,
    /// Sliding-window width in tokens (confirmed hyperparameter; default 3).
    pub window_width: usize,
    /// Pooling of per-window firings into a document vote (default CountFire).
    pub pooling: PoolingStrategy,
    /// Vote cap for CountFire pooling (1 degenerates to AnyFire; default 3).
    pub clause_vote_cap: u32,
    pub tm_clauses: usize,
    pub tm_threshold: i32,
    pub specificity: f64,
    pub states_per_action: i32,
    pub epochs: usize,
    pub seed: u64,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            train_ratio: 0.8,
            min_df: 2,
            max_features: Some(4000),
            model_type: ModelType::Windowed,
            window_width: 3,
            pooling: PoolingStrategy::CountFire,
            clause_vote_cap: 3,
            tm_clauses: 80,
            tm_threshold: 50,
            specificity: 5.0,
            states_per_action: 100,
            epochs: 25,
            seed: 42,
        }
    }
}

/// Trains a pipeline of the configured architecture. Features (BOW vectors or
/// word-ID sequences) are pre-computed ONCE before the epoch loop so the loop
/// performs no tokenization or allocation — the established performance
/// pattern from the previous pipeline, carried over to sequences.
pub fn train_pipeline(
    train_set: &Dataset,
    config: &BenchmarkConfig,
) -> Result<PipelineModel, PipelineError> {
    let mut rng = FastRng::seed(config.seed);

    if train_set.records.is_empty() {
        return Err(PipelineError::DatasetInvalid(
            "Training set contains no records".to_string(),
        ));
    }
    let num_classes = train_set.id_to_label.len();
    if num_classes == 0 {
        return Err(PipelineError::DatasetInvalid(
            "Training set has no labels".to_string(),
        ));
    }

    // 1. Vocabulary over the training corpus only (no test leakage).
    println!(
        "[1/3] Building vocabulary across {} documents...",
        train_set.records.len()
    );
    let corpus: Vec<String> = train_set.records.iter().map(|r| r.text.clone()).collect();
    let vocab = Vocabulary::build(&corpus, config.min_df, config.max_features)?;
    println!("  Active vocabulary features: {}", vocab.id_to_word.len());

    // 2 + 3. Pre-compute features once, then run the epoch loop.
    let classifier = match config.model_type {
        ModelType::Flat => {
            println!("[2/3] Pre-computing flat BOW vectors (baseline path)...");
            let samples: Vec<(Vec<bool>, usize)> = train_set
                .records
                .iter()
                .filter_map(|record| {
                    train_set
                        .label_to_id
                        .get(&record.label)
                        .map(|&label_id| (vocab.text_to_bow(&record.text), label_id))
                })
                .collect();
            if samples.is_empty() {
                return Err(PipelineError::DatasetInvalid(
                    "No trainable samples after label mapping".to_string(),
                ));
            }

            let mut tm = VanillaTM::new(
                num_classes,
                config.tm_clauses,
                vocab.id_to_word.len(),
                config.tm_threshold,
                config.states_per_action,
                config.specificity,
            )?;

            println!(
                "[3/3] Training flat VanillaTM ({} epochs, {} classes)...",
                config.epochs, num_classes
            );
            for _ in 0..config.epochs {
                for (bow, label_id) in &samples {
                    tm.train_step(bow, *label_id, &mut rng)?;
                }
            }
            ClassifierModel::Flat(tm)
        }
        ModelType::Windowed => {
            println!("[2/3] Pre-computing word-ID token sequences (windowed path)...");
            let samples: Vec<(Vec<Option<usize>>, usize)> = train_set
                .records
                .iter()
                .filter_map(|record| {
                    train_set
                        .label_to_id
                        .get(&record.label)
                        .map(|&label_id| (vocab.text_to_token_sequence(&record.text), label_id))
                })
                .collect();
            if samples.is_empty() {
                return Err(PipelineError::DatasetInvalid(
                    "No trainable samples after label mapping".to_string(),
                ));
            }

            let mut tm = WindowedTM::new(
                num_classes,
                config.tm_clauses,
                vocab.id_to_word.len(),
                config.window_width,
                config.tm_threshold,
                config.states_per_action,
                config.specificity,
                config.pooling,
                config.clause_vote_cap,
            )?;

            println!(
                "[3/3] Training WindowedTM (width {}, {:?} pooling, cap {}, {} epochs, {} classes)...",
                config.window_width,
                config.pooling,
                config.clause_vote_cap,
                config.epochs,
                num_classes
            );
            for _ in 0..config.epochs {
                for (tokens, label_id) in &samples {
                    tm.train_step(tokens, *label_id, &mut rng)?;
                }
            }
            ClassifierModel::Windowed(tm)
        }
    };

    Ok(PipelineModel {
        vocab,
        classifier,
        id_to_label: train_set.id_to_label.clone(),
        label_to_id: train_set.label_to_id.clone(),
    })
}

/// Evaluates a trained pipeline on a test dataset. Inference is pure (no RNG),
/// so records are scored in parallel with rayon; results remain deterministic.
/// Records whose label is unknown to the model are skipped.
pub fn evaluate_pipeline(
    model: &PipelineModel,
    test_set: &Dataset,
) -> Result<EvaluationReport, PipelineError> {
    let scored: Result<Vec<Option<(usize, usize)>>, PipelineError> = test_set
        .records
        .par_iter()
        .map(|record| {
            let label_id = match model.label_to_id.get(&record.label) {
                Some(&id) => id,
                None => return Ok(None),
            };
            let predicted = match &model.classifier {
                ClassifierModel::Flat(tm) => tm.predict(&model.vocab.text_to_bow(&record.text))?,
                ClassifierModel::Windowed(tm) => {
                    tm.predict(&model.vocab.text_to_token_sequence(&record.text))?
                }
            };
            Ok(Some((predicted, label_id)))
        })
        .collect();

    let mut predictions = Vec::new();
    let mut ground_truth = Vec::new();
    for pair in scored?.into_iter().flatten() {
        predictions.push(pair.0);
        ground_truth.push(pair.1);
    }

    EvaluationReport::compute(&predictions, &ground_truth, &model.id_to_label)
}

// ---------------------------------------------------------------------------
// CLI Execution Layer
// ---------------------------------------------------------------------------

/// Parsed command-line arguments. Unknown flags and malformed numeric values
/// are hard errors (fail-fast) rather than being silently defaulted.
struct CliArgs {
    mode: String,
    train_file: Option<PathBuf>,
    test_file: Option<PathBuf>,
    model_path: Option<PathBuf>,
    predict_text: Option<String>,
    text_col: String,
    label_col: String,
    is_jsonl: bool,
    model_type: ModelType,
    window_width: usize,
    pooling: PoolingStrategy,
    clause_vote_cap: u32,
    epochs: usize,
    clauses: usize,
    max_features: usize,
    min_df: usize,
    train_ratio: f64,
    specificity: f64,
    threshold: i32,
}

/// Parses one numeric CLI value with a clear, flag-attributed error message
/// (never a silent default on malformed input).
fn parse_cli_number<T: std::str::FromStr>(flag: &str, raw: &str) -> Result<T, PipelineError> {
    raw.parse::<T>().map_err(|_| {
        PipelineError::CliError(format!("Flag '{flag}' received an invalid value: '{raw}'"))
    })
}

impl CliArgs {
    fn parse(args: &[String]) -> Result<Self, PipelineError> {
        let mut mode = String::new();
        let mut train_file = None;
        let mut test_file = None;
        let mut model_path = None;
        let mut predict_text = None;
        let mut text_col = "text".to_string();
        let mut label_col = "label".to_string();
        let mut is_jsonl = false;
        let mut model_type = ModelType::Windowed;
        let mut window_width: usize = 3;
        let mut pooling = PoolingStrategy::CountFire;
        let mut clause_vote_cap: u32 = 3;
        let mut epochs: usize = 25;
        let mut clauses: usize = 80;
        let mut max_features: usize = 4000;
        let mut min_df: usize = 2;
        let mut train_ratio: f64 = 0.8;
        let mut specificity: f64 = 5.0;
        let mut threshold: i32 = 50;

        /// Fetches the value following a flag, erroring if it is missing.
        fn take_value<'a>(
            args: &'a [String],
            index: &mut usize,
            flag: &str,
        ) -> Result<&'a str, PipelineError> {
            *index += 1;
            args.get(*index)
                .map(|s| s.as_str())
                .ok_or_else(|| PipelineError::CliError(format!("Flag '{flag}' requires a value")))
        }

        let mut i = 1;
        while i < args.len() {
            let flag = args[i].as_str();
            match flag {
                "--mode" => {
                    mode = take_value(args, &mut i, flag)?.to_string();
                }
                "--train-file" | "--train" => {
                    let raw = take_value(args, &mut i, flag)?;
                    let p = PathBuf::from(raw);
                    train_file = Some(
                        p.canonicalize()
                            .map_err(|e| PipelineError::IoError { path: p, source: e })?,
                    );
                }
                "--test-file" | "--test" => {
                    let raw = take_value(args, &mut i, flag)?;
                    let p = PathBuf::from(raw);
                    test_file = Some(
                        p.canonicalize()
                            .map_err(|e| PipelineError::IoError { path: p, source: e })?,
                    );
                }
                "--model-path" | "--model" => {
                    let raw = take_value(args, &mut i, flag)?;
                    let p = PathBuf::from(raw);
                    let abs = if p.is_absolute() {
                        p
                    } else {
                        std::env::current_dir()
                            .map_err(|e| PipelineError::IoError {
                                path: p.clone(),
                                source: e,
                            })?
                            .join(&p)
                    };
                    model_path = Some(abs);
                }
                "--text" | "--predict-text" => {
                    predict_text = Some(take_value(args, &mut i, flag)?.to_string());
                }
                "--text-col" => {
                    text_col = take_value(args, &mut i, flag)?.to_string();
                }
                "--label-col" => {
                    label_col = take_value(args, &mut i, flag)?.to_string();
                }
                "--jsonl" => {
                    is_jsonl = true;
                }
                "--model-type" => {
                    let raw = take_value(args, &mut i, flag)?.to_lowercase();
                    model_type = match raw.as_str() {
                        "flat" => ModelType::Flat,
                        "windowed" => ModelType::Windowed,
                        other => {
                            return Err(PipelineError::CliError(format!(
                                "--model-type must be 'flat' or 'windowed', got '{other}'"
                            )));
                        }
                    };
                }
                "--window-width" => {
                    window_width = parse_cli_number(flag, take_value(args, &mut i, flag)?)?;
                }
                "--pooling" => {
                    let raw = take_value(args, &mut i, flag)?.to_lowercase();
                    pooling = match raw.as_str() {
                        "countfire" => PoolingStrategy::CountFire,
                        "anyfire" => PoolingStrategy::AnyFire,
                        other => {
                            return Err(PipelineError::CliError(format!(
                                "--pooling must be 'countfire' or 'anyfire', got '{other}'"
                            )));
                        }
                    };
                }
                "--vote-cap" => {
                    clause_vote_cap = parse_cli_number(flag, take_value(args, &mut i, flag)?)?;
                }
                "--epochs" => {
                    epochs = parse_cli_number(flag, take_value(args, &mut i, flag)?)?;
                }
                "--clauses" => {
                    clauses = parse_cli_number(flag, take_value(args, &mut i, flag)?)?;
                }
                "--max-features" => {
                    max_features = parse_cli_number(flag, take_value(args, &mut i, flag)?)?;
                }
                "--min-df" => {
                    min_df = parse_cli_number(flag, take_value(args, &mut i, flag)?)?;
                }
                "--train-ratio" => {
                    train_ratio = parse_cli_number(flag, take_value(args, &mut i, flag)?)?;
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                "--specificity" | "-s" => {
                    specificity = parse_cli_number(flag, take_value(args, &mut i, flag)?)?;
                }
                "--threshold" | "-t" => {
                    threshold = parse_cli_number(flag, take_value(args, &mut i, flag)?)?;
                }
                unknown => {
                    return Err(PipelineError::CliError(format!(
                        "Unknown flag '{unknown}'. Run with --help for usage."
                    )));
                }
            }
            i += 1;
        }

        Ok(Self {
            mode,
            train_file,
            test_file,
            model_path,
            predict_text,
            text_col,
            label_col,
            is_jsonl,
            model_type,
            window_width,
            pooling,
            clause_vote_cap,
            epochs,
            clauses,
            max_features,
            min_df,
            train_ratio,
            specificity,
            threshold,
        })
    }
}

fn print_help() {
    println!("Windowed Tsetlin Machine NLP Classification CLI (Phase 1)");
    println!("=========================================================");
    println!("\nTRAIN MODE:");
    println!("  cargo run --release -- --mode train --train-file <PATH> [OPTIONS]");
    println!("  Options:");
    println!("    --test-file <PATH>     Optional test split (auto-splits if omitted)");
    println!("    --model-path <PATH>    Destination for the trained .json model");
    println!("    --model-type <T>       'windowed' (default) or 'flat' baseline");
    println!("    --window-width <N>     Sliding window width in tokens (default: 3)");
    println!("    --pooling <P>          'countfire' (default) or 'anyfire'");
    println!("    --vote-cap <N>         CountFire per-clause vote cap (default: 3)");
    println!("    --text-col <NAME>      Text column/field name (default: 'text')");
    println!("    --label-col <NAME>     Label column/field name (default: 'label')");
    println!("    --jsonl                Parse input as JSONL instead of CSV");
    println!("    --epochs <N>           Training epochs (default: 25)");
    println!("    --clauses <N>          Clauses per class (default: 80)");
    println!("    --threshold <N>        Threshold target (default: 50)");
    println!("    --specificity <F>      Specificity parameter (default: 5.0)");
    println!("    --max-features <N>     Vocabulary size cap (default: 4000)");
    println!("    --min-df <N>           Minimum document frequency (default: 2)");
    println!("    --train-ratio <F>      Auto-split ratio (default: 0.8)");
    println!("\nPREDICT MODE:");
    println!("  cargo run --release -- --mode predict --model-path <PATH> --text \"<STRING>\"");
}

fn handle_train(args: &CliArgs) -> Result<(), PipelineError> {
    let train_path = args.train_file.as_ref().ok_or_else(|| {
        PipelineError::CliError(
            "Train mode requires '--train-file <PATH>'. Run with --help for details.".to_string(),
        )
    })?;

    println!("Loading training dataset from: {}", train_path.display());
    let train_dataset = if args.is_jsonl {
        Dataset::from_jsonl(train_path, &args.text_col, &args.label_col)?
    } else {
        Dataset::from_csv(train_path, b',', &args.text_col, &args.label_col)?
    };
    println!("  Total records loaded: {}", train_dataset.records.len());
    let config = BenchmarkConfig {
        train_ratio: args.train_ratio,
        min_df: args.min_df,
        max_features: Some(args.max_features),
        model_type: args.model_type,
        window_width: args.window_width,
        pooling: args.pooling,
        clause_vote_cap: args.clause_vote_cap,
        tm_clauses: args.clauses,
        tm_threshold: args.threshold,
        specificity: args.specificity,
        epochs: args.epochs,
        ..BenchmarkConfig::default()
    };

    let (fitted_model, test_report) = if let Some(test_path) = &args.test_file {
        println!("Loading test dataset from: {}", test_path.display());
        let mut test_dataset = if args.is_jsonl {
            Dataset::from_jsonl(test_path, &args.text_col, &args.label_col)?
        } else {
            Dataset::from_csv(test_path, b',', &args.text_col, &args.label_col)?
        };
        println!("  Test records loaded: {}", test_dataset.records.len());
        test_dataset.align_labels_with(&train_dataset.label_to_id);

        let train_start = Instant::now();
        let model = train_pipeline(&train_dataset, &config)?;
        let train_duration = train_start.elapsed();

        let mut report = evaluate_pipeline(&model, &test_dataset)?;
        report.train_duration = Some(train_duration);
        (model, report)
    } else {
        println!(
            "Splitting dataset ({} total rows) into {:.0}% train / {:.0}% test...",
            train_dataset.records.len(),
            args.train_ratio * 100.0,
            (1.0 - args.train_ratio) * 100.0
        );
        let mut rng = FastRng::seed(config.seed);
        let (train_split, test_split) = train_dataset.split(args.train_ratio, &mut rng)?;
        println!(
            "  Split: {} train rows, {} test rows",
            train_split.records.len(),
            test_split.records.len()
        );

        let train_start = Instant::now();
        let model = train_pipeline(&train_split, &config)?;
        let train_duration = train_start.elapsed();

        let mut report = evaluate_pipeline(&model, &test_split)?;
        report.train_duration = Some(train_duration);
        (model, report)
    };

    test_report.print_summary();

    if let Some(save_path) = &args.model_path {
        fitted_model.save_to_file(save_path)?;
        println!(
            "Successfully saved trained model artifact to: {}",
            save_path.display()
        );
    }

    Ok(())
}

fn handle_predict(args: &CliArgs) -> Result<(), PipelineError> {
    let model_path = args.model_path.as_ref().ok_or_else(|| {
        PipelineError::CliError(
            "Predict mode requires '--model-path <PATH>' to load the model.".to_string(),
        )
    })?;

    let text = args.predict_text.as_ref().ok_or_else(|| {
        PipelineError::CliError(
            "Predict mode requires '--text \"<INPUT STRING>\"' to classify.".to_string(),
        )
    })?;

    println!("Loading model from: {}", model_path.display());
    let model = PipelineModel::load_from_file(model_path)?;

    let output = model.predict_one(text, 3)?;

    println!("\n============================================================");
    println!("                    Single Text Inference                   ");
    println!("============================================================");
    println!("Input Text:        \"{}\"", output.text);
    println!("Predicted Label:   {}", output.label);
    println!("Voting Margin:     +{} votes", output.margin);
    println!("------------------------------------------------------------");
    println!("Class Vote Totals:");
    for (name, score) in &output.class_votes {
        println!("  - {:<16}: {:>4} votes", name, score);
    }
    println!("------------------------------------------------------------");
    println!("Triggered Logic Rules (Explainability Trace):");
    if output.rules.is_empty() {
        println!("  [Default Baseline Rule Active]");
    } else {
        for rule in &output.rules {
            let sign = if rule.polarity > 0 {
                "+ vote"
            } else {
                "- veto"
            };
            let positions = if rule.fired_positions.is_empty() {
                String::new()
            } else {
                format!(" @ token positions {:?}", rule.fired_positions)
            };
            println!(
                "  [Clause #{:<2} ({sign})] IF ( {} ) THEN {}{}",
                rule.clause_index, rule.rule, rule.class_name, positions
            );
        }
    }
    println!("============================================================\n");

    Ok(())
}

/// Built-in self-test demo, run when no CLI flags are given. Deliberately a
/// NEGATION dataset: both classes contain the word "good", so a flat BOW model
/// cannot separate them — only the windowed model's local order sensitivity
/// ("not" immediately before "good") can. This exercises exactly the property
/// Phase 1 exists to add, and previews the context-dependence expected in the
/// toxicity benchmark.
fn run_demo() -> Result<(), PipelineError> {
    println!("No mode flag passed. Running built-in negation self-test demo...\n");

    let temp_dir = std::env::temp_dir();
    let demo_csv = temp_dir.join("windowed_tm_demo.csv");
    let model_file = temp_dir.join("windowed_tm_demo_model.json");

    // Write the demo CSV; every negative example contains "not good" adjacent,
    // every positive example contains "good" NOT preceded by "not".
    {
        let mut file = File::create(&demo_csv).map_err(|e| PipelineError::IoError {
            path: demo_csv.clone(),
            source: e,
        })?;
        let rows = [
            "sentence,category",
            "the movie was really good and fun,positive",
            "this film was good with great acting,positive",
            "an honestly good and enjoyable story,positive",
            "the ending was good and satisfying,positive",
            "such good pacing and good writing,positive",
            "the movie was not good at all,negative",
            "this film was not good despite great acting,negative",
            "honestly not good and not enjoyable,negative",
            "the ending was not good sadly,negative",
            "such pacing was not good and boring,negative",
        ];
        for row in rows {
            writeln!(file, "{row}").map_err(|e| PipelineError::IoError {
                path: demo_csv.clone(),
                source: e,
            })?;
        }
    }

    let dataset = Dataset::from_csv(&demo_csv, b',', "sentence", "category")?;
    let config = BenchmarkConfig {
        min_df: 1,
        max_features: Some(100),
        model_type: ModelType::Windowed,
        tm_clauses: 40,
        tm_threshold: 15,
        specificity: 3.0,
        epochs: 200,
        seed: 42,
        ..BenchmarkConfig::default()
    };

    println!("Training windowed demo model...");
    let model = train_pipeline(&dataset, &config)?;
    model.save_to_file(&model_file)?;

    println!("Testing predict on serialized model...");
    let sample = "the acting was not good";
    let loaded_model = PipelineModel::load_from_file(&model_file)?;
    let result = loaded_model.predict_one(sample, 3)?;

    println!("\nInference Text: \"{}\"", sample);
    println!("Predicted Label: {}", result.label);
    for rule in &result.rules {
        println!(
            "  Fired rule (clause #{}, positions {:?}): {}",
            rule.clause_index, rule.fired_positions, rule.rule
        );
    }

    // Fail-fast self-test via Result (never assert/panic in production paths).
    if result.label != "negative" {
        return Err(PipelineError::NumericalError(format!(
            "Demo self-test failed: expected 'negative' for negated sentiment, got '{}'",
            result.label
        )));
    }
    println!("\nDemo self-test passed (negation correctly detected).");
    println!("Run with '--help' to view full CLI options.");

    Ok(())
}

fn main() -> Result<(), PipelineError> {
    let raw_args: Vec<String> = std::env::args().collect();
    if raw_args.len() <= 1 {
        return run_demo();
    }

    let args = CliArgs::parse(&raw_args)?;

    match args.mode.as_str() {
        "train" => handle_train(&args),
        "predict" => handle_predict(&args),
        "" => run_demo(),
        other => Err(PipelineError::CliError(format!(
            "Unknown mode '{other}'. Must be '--mode train' or '--mode predict'."
        ))),
    }
}

// ---------------------------------------------------------------------------
// Unit Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Convenience: builds a small windowed TM for hand-constructed clauses.
    /// clause 0 is positive-polarity by convention.
    fn small_windowed_tm(num_clauses: usize, vocab_size: usize) -> WindowedTM {
        WindowedTM::new(
            1,
            num_clauses,
            vocab_size,
            3,
            15,
            100,
            3.0,
            PoolingStrategy::CountFire,
            3,
        )
        .expect("small TM construction must succeed")
    }

    /// THE core Phase 1 property test: a clause encoding "word 1 immediately
    /// followed by word 2" fires on adjacent order, and does NOT fire on the
    /// same words reversed or separated — exactly what flat BOW cannot express.
    #[test]
    fn test_windowed_order_sensitivity() {
        let mut tm = small_windowed_tm(2, 5);
        let pos_slot0_word1 = tm.positive_literal_index(0, 1);
        let pos_slot1_word2 = tm.positive_literal_index(1, 2);
        tm.test_force_include_literal(0, 0, pos_slot0_word1);
        tm.test_force_include_literal(0, 0, pos_slot1_word2);
        tm.validate_internal_consistency().unwrap();

        // Adjacent, correct order: fires at window 0.
        let adjacent = vec![Some(1), Some(2), Some(3)];
        assert_eq!(tm.fired_window_positions(0, 0, &adjacent).unwrap(), vec![0]);

        // Same words, reversed order: must NOT fire anywhere.
        let reversed = vec![Some(2), Some(1), Some(3)];
        assert!(
            tm.fired_window_positions(0, 0, &reversed)
                .unwrap()
                .is_empty()
        );

        // Same words, non-adjacent (word 3 interposed): must NOT fire.
        let separated = vec![Some(1), Some(3), Some(2)];
        assert!(
            tm.fired_window_positions(0, 0, &separated)
                .unwrap()
                .is_empty()
        );

        // Pattern at a later offset: window slides and finds it at position 1.
        let offset = vec![Some(0), Some(1), Some(2), Some(3)];
        assert_eq!(tm.fired_window_positions(0, 0, &offset).unwrap(), vec![1]);
    }

    /// Short-document policy: a document shorter than the window width yields
    /// exactly one right-PAD-padded window; PAD slots impose no positive
    /// requirement, and a positive literal aimed at a PAD slot fails.
    #[test]
    fn test_short_document_padding_semantics() {
        let mut tm = small_windowed_tm(4, 5);

        // Clause 0: requires words at slots 0 and 1 only; slot 2 unconstrained.
        tm.test_force_include_literal(0, 0, tm.positive_literal_index(0, 1));
        tm.test_force_include_literal(0, 0, tm.positive_literal_index(1, 2));
        // Clause 2 (also positive polarity): additionally requires a word at
        // slot 2, which PAD can never satisfy.
        tm.test_force_include_literal(0, 2, tm.positive_literal_index(0, 1));
        tm.test_force_include_literal(0, 2, tm.positive_literal_index(2, 4));
        tm.validate_internal_consistency().unwrap();

        // Two-token document, window width 3 => one padded window.
        let short_doc = vec![Some(1), Some(2)];
        assert_eq!(
            tm.fired_window_positions(0, 0, &short_doc).unwrap(),
            vec![0],
            "PAD slot with no positive requirement must not block firing"
        );
        assert!(
            tm.fired_window_positions(0, 2, &short_doc)
                .unwrap()
                .is_empty(),
            "positive literal aimed at a PAD slot must fail"
        );
    }

    /// PAD/OOV satisfies negated literals and fails positive ones (decision #4):
    /// a purely negated clause fires on an all-PAD (empty) document but is
    /// blocked when the forbidden word actually appears.
    #[test]
    fn test_pad_satisfies_negated_and_fails_positive() {
        let mut tm = small_windowed_tm(2, 5);
        tm.test_force_include_literal(0, 0, tm.negated_literal_index(0, 0));
        tm.validate_internal_consistency().unwrap();

        // Empty document => one all-PAD window; ¬word0 at slot 0 is satisfied.
        let empty: Vec<Option<usize>> = Vec::new();
        assert_eq!(tm.fired_window_positions(0, 0, &empty).unwrap(), vec![0]);

        // Forbidden word present at slot 0 => blocked.
        let with_forbidden = vec![Some(0)];
        assert!(
            tm.fired_window_positions(0, 0, &with_forbidden)
                .unwrap()
                .is_empty()
        );

        // OOV token (None) mid-document behaves exactly like PAD.
        let with_oov = vec![None, Some(1), Some(2)];
        assert_eq!(tm.fired_window_positions(0, 0, &with_oov).unwrap().len(), 1);
    }

    /// CountFire pooling sums per-window firings and clamps at the vote cap;
    /// AnyFire collapses to a single vote (decision #5).
    #[test]
    fn test_pooling_countfire_cap_and_anyfire() {
        // One clause (index 0, positive polarity) requiring word 1 at slot 0.
        let mut tm =
            WindowedTM::new(1, 1, 5, 3, 15, 100, 3.0, PoolingStrategy::CountFire, 3).unwrap();
        tm.test_force_include_literal(0, 0, tm.positive_literal_index(0, 1));
        tm.validate_internal_consistency().unwrap();

        // Six tokens of word 1 => 4 windows, all firing (slot 0 is word 1).
        let tokens = vec![Some(1); 6];
        assert_eq!(tm.fired_window_positions(0, 0, &tokens).unwrap().len(), 4);

        // CountFire with cap 3: pooled vote clamps 4 -> 3.
        assert_eq!(tm.calculate_class_sum(0, &tokens).unwrap(), 3);

        // AnyFire: same firings pool to a single vote.
        tm.pooling = PoolingStrategy::AnyFire;
        assert_eq!(tm.calculate_class_sum(0, &tokens).unwrap(), 1);

        // CountFire with cap 1 degenerates to AnyFire, as documented.
        tm.pooling = PoolingStrategy::CountFire;
        tm.clause_vote_cap = 1;
        assert_eq!(tm.calculate_class_sum(0, &tokens).unwrap(), 1);
    }

    /// End-to-end LEARNING test of the order-sensitivity property: both
    /// classes contain "good" (word 1), so only the adjacency of "not" (word 0)
    /// before "good" separates them — flat BOW is structurally blind here.
    /// Deterministic under the fixed seed.
    #[test]
    fn test_windowed_tm_learns_negation() {
        // Manual micro-vocabulary: 0=not, 1=good, 2=movie, 3=film, 4=really,
        // 5=acting, 6=story, 7=fun.
        let positives: Vec<Vec<Option<usize>>> = vec![
            vec![Some(1), Some(2)],          // good movie
            vec![Some(4), Some(1), Some(3)], // really good film
            vec![Some(1), Some(5)],          // good acting
            vec![Some(1), Some(6)],          // good story
            vec![Some(4), Some(1), Some(7)], // really good fun
        ];
        let negatives: Vec<Vec<Option<usize>>> = vec![
            vec![Some(0), Some(1), Some(2)], // not good movie
            vec![Some(4), Some(0), Some(1)], // really not good
            vec![Some(0), Some(1), Some(5)], // not good acting
            vec![Some(0), Some(1), Some(6)], // not good story
            vec![Some(0), Some(1), Some(7)], // not good fun
        ];

        let mut tm =
            WindowedTM::new(2, 20, 8, 3, 15, 100, 3.0, PoolingStrategy::CountFire, 3).unwrap();
        let mut rng = FastRng::seed(1234);

        for _ in 0..200 {
            for doc in &positives {
                tm.train_step(doc, 0, &mut rng).unwrap();
            }
            for doc in &negatives {
                tm.train_step(doc, 1, &mut rng).unwrap();
            }
        }

        // Invariant must survive a full training run.
        tm.validate_internal_consistency().unwrap();

        for doc in &positives {
            assert_eq!(
                tm.predict(doc).unwrap(),
                0,
                "positive doc misclassified: {doc:?}"
            );
        }
        for doc in &negatives {
            assert_eq!(
                tm.predict(doc).unwrap(),
                1,
                "negative doc misclassified: {doc:?}"
            );
        }
    }

    /// The counter cache backing O(W) evaluation must exactly match a fresh
    /// recomputation from raw TA states after arbitrary stochastic training.
    #[test]
    fn test_counter_consistency_after_training() {
        let mut tm =
            WindowedTM::new(2, 10, 6, 3, 10, 50, 3.0, PoolingStrategy::CountFire, 2).unwrap();
        let mut rng = FastRng::seed(7);

        let docs: Vec<(Vec<Option<usize>>, usize)> = vec![
            (vec![Some(0), Some(1), Some(2), Some(3)], 0),
            (vec![Some(3), Some(2), Some(1), Some(0)], 1),
            (vec![Some(4), None, Some(5)], 0),
            (vec![Some(5)], 1),
            (vec![], 0), // empty doc: all-PAD window, must train without error
        ];

        for _ in 0..50 {
            for (doc, label) in &docs {
                tm.train_step(doc, *label, &mut rng).unwrap();
            }
        }
        tm.validate_internal_consistency().unwrap();
    }

    /// Full pipeline round-trip for the WINDOWED model: train via
    /// `train_pipeline`, save, load (which re-validates the counter cache),
    /// and confirm the loaded model reproduces the original predictions and
    /// produces position-annotated rules.
    #[test]
    fn test_windowed_pipeline_save_load_consistency() {
        let temp_dir = std::env::temp_dir();
        let model_path = temp_dir.join("test_windowed_save_load_model.json");

        let dataset = Dataset {
            records: vec![
                TextRecord {
                    text: "the movie was really good and fun".to_string(),
                    label: "positive".to_string(),
                },
                TextRecord {
                    text: "an honestly good and enjoyable story".to_string(),
                    label: "positive".to_string(),
                },
                TextRecord {
                    text: "such good pacing and good writing".to_string(),
                    label: "positive".to_string(),
                },
                TextRecord {
                    text: "the movie was not good at all".to_string(),
                    label: "negative".to_string(),
                },
                TextRecord {
                    text: "honestly not good and not enjoyable".to_string(),
                    label: "negative".to_string(),
                },
                TextRecord {
                    text: "such pacing was not good and boring".to_string(),
                    label: "negative".to_string(),
                },
            ],
            label_to_id: [("positive".to_string(), 0), ("negative".to_string(), 1)]
                .into_iter()
                .collect(),
            id_to_label: vec!["positive".to_string(), "negative".to_string()],
        };

        let config = BenchmarkConfig {
            min_df: 1,
            max_features: Some(50),
            model_type: ModelType::Windowed,
            tm_clauses: 40,
            tm_threshold: 15,
            specificity: 3.0,
            epochs: 200,
            seed: 42,
            ..BenchmarkConfig::default()
        };

        let model = train_pipeline(&dataset, &config).expect("Training failed");
        model.save_to_file(&model_path).expect("Saving failed");
        let loaded = PipelineModel::load_from_file(&model_path).expect("Loading failed");

        // Loaded and in-memory models must agree on every training text.
        for record in &dataset.records {
            let original = model.predict_one(&record.text, 3).expect("predict failed");
            let reloaded = loaded.predict_one(&record.text, 3).expect("predict failed");
            assert_eq!(original.label, reloaded.label);
            assert_eq!(original.class_votes, reloaded.class_votes);
        }

        // Negation generalisation + explainability shape check.
        let out = loaded
            .predict_one("the story was not good", 3)
            .expect("Prediction failed");
        assert_eq!(out.label, "negative");
        for rule in &out.rules {
            assert!(
                !rule.fired_positions.is_empty(),
                "windowed rules must report firing positions"
            );
            assert!(
                rule.rule.contains("[w+"),
                "windowed rules must be position-annotated: {}",
                rule.rule
            );
        }
    }

    /// The flat baseline path must remain fully functional (it is the ablation
    /// control), including artifact round-tripping.
    #[test]
    fn test_flat_baseline_pipeline_save_load() {
        let temp_dir = std::env::temp_dir();
        let model_path = temp_dir.join("test_flat_save_load_model.json");

        let dataset = Dataset {
            records: vec![
                TextRecord {
                    text: "basketball tournament team victory".to_string(),
                    label: "sports".to_string(),
                },
                TextRecord {
                    text: "soccer championship points player".to_string(),
                    label: "sports".to_string(),
                },
                TextRecord {
                    text: "space telescope galaxy orbit".to_string(),
                    label: "science".to_string(),
                },
                TextRecord {
                    text: "satellite rocket astronaut mars".to_string(),
                    label: "science".to_string(),
                },
            ],
            label_to_id: [("sports".to_string(), 0), ("science".to_string(), 1)]
                .into_iter()
                .collect(),
            id_to_label: vec!["sports".to_string(), "science".to_string()],
        };

        let config = BenchmarkConfig {
            min_df: 1,
            max_features: Some(50),
            model_type: ModelType::Flat,
            epochs: 30,
            seed: 99,
            ..BenchmarkConfig::default()
        };

        let model = train_pipeline(&dataset, &config).expect("Training failed");
        model.save_to_file(&model_path).expect("Saving failed");
        let loaded = PipelineModel::load_from_file(&model_path).expect("Loading failed");

        let out = loaded
            .predict_one("telescope galaxy orbit", 2)
            .expect("Prediction failed");
        assert_eq!(out.label, "science");
        // Flat rules carry no positional data by design.
        for rule in &out.rules {
            assert!(rule.fired_positions.is_empty());
        }
    }

    /// OOV tokens must map to `None` (PAD semantics) while preserving the
    /// positions of known words — adjacency must not be falsified by dropping.
    #[test]
    fn test_token_sequence_oov_preserves_positions() {
        let corpus = vec!["alpha beta gamma".to_string()];
        let vocab = Vocabulary::build(&corpus, 1, None).expect("vocab build failed");

        let seq = vocab.text_to_token_sequence("alpha zzznotinvocab beta");
        assert_eq!(seq.len(), 3, "OOV token must occupy a position, not vanish");
        assert!(seq[0].is_some());
        assert!(seq[1].is_none(), "OOV must be None (PAD semantics)");
        assert!(seq[2].is_some());
        assert_ne!(seq[0], seq[2]);
    }

    /// JSONL ingestion regression test (pre-existing behaviour, preserved).
    #[test]
    fn test_dataset_jsonl_parsing() {
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join("test_dataset_phase1.jsonl");

        {
            let mut file = File::create(&temp_path).expect("failed to create temp jsonl");
            writeln!(
                file,
                r#"{{"text": "football match victory", "label": "sports"}}"#
            )
            .unwrap();
            writeln!(
                file,
                r#"{{"text": "telescope detects star", "label": "science"}}"#
            )
            .unwrap();
        }

        let dataset =
            Dataset::from_jsonl(&temp_path, "text", "label").expect("JSONL parsing failed");
        assert_eq!(dataset.records.len(), 2);
        assert_eq!(dataset.id_to_label.len(), 2);
    }

    /// Invalid configurations must be rejected at construction, never at
    /// first use.
    #[test]
    fn test_windowed_tm_configuration_validation() {
        assert!(
            WindowedTM::new(0, 10, 10, 3, 15, 100, 3.0, PoolingStrategy::CountFire, 3).is_err()
        );
        assert!(
            WindowedTM::new(2, 10, 10, 0, 15, 100, 3.0, PoolingStrategy::CountFire, 3).is_err()
        );
        assert!(
            WindowedTM::new(2, 10, 10, 3, 15, 100, 1.0, PoolingStrategy::CountFire, 3).is_err()
        );
        assert!(
            WindowedTM::new(2, 10, 10, 3, 15, 100, 3.0, PoolingStrategy::CountFire, 0).is_err()
        );
        assert!(WindowedTM::new(2, 10, 10, 3, 15, 100, 3.0, PoolingStrategy::CountFire, 3).is_ok());
    }
}
