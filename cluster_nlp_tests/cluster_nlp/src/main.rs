//! # Semantic Pre-trained Tsetlin Machine NLP Pipeline
//!
//! Dual-mode CLI tool for NLP text classification:
//! - `--mode train`: Pre-computes features once, trains Stage 1 + Stage 2, evaluates, and saves model.
//! - `--mode predict`: Loads the model from disk and classifies a single text with white-box logic rules.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Error Handling Architecture
// ---------------------------------------------------------------------------

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
            } => {
                write!(
                    f,
                    "Dimension mismatch in {context}: expected {expected}, found {found}"
                )
            }
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
// PRNG
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct FastRng {
    state: u64,
}

impl FastRng {
    pub fn seed(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x853c49e6748fea9b } else { seed },
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    pub fn gen_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

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
// Dataset Ingestion
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextRecord {
    pub text: String,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct Dataset {
    pub records: Vec<TextRecord>,
    pub label_to_id: HashMap<String, usize>,
    pub id_to_label: Vec<String>,
}

impl Dataset {
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
// Vocabulary & TF-IDF Vectorizer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vocabulary {
    pub word_to_id: HashMap<String, usize>,
    pub id_to_word: Vec<String>,
    pub idf: Vec<f64>,
}

impl Vocabulary {
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
        let total_docs = corpus.len() as f64;

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

        filtered.sort_by(|a, b| b.1.cmp(&a.1));

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
        let mut idf = Vec::new();

        for (word, df) in filtered {
            let id = id_to_word.len();
            word_to_id.insert(word.clone(), id);
            id_to_word.push(word);

            let term_idf = ((1.0 + total_docs) / (1.0 + df as f64)).ln() + 1.0;
            idf.push(term_idf);
        }

        Ok(Self {
            word_to_id,
            id_to_word,
            idf,
        })
    }

    pub fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| s.len() > 1)
            .map(|s| s.to_string())
            .collect()
    }

    pub fn text_to_bow(&self, text: &str) -> Vec<bool> {
        let mut bow = vec![false; self.id_to_word.len()];
        for token in Self::tokenize(text) {
            if let Some(&id) = self.word_to_id.get(&token) {
                bow[id] = true;
            }
        }
        bow
    }

    pub fn text_to_tfidf(&self, text: &str) -> Vec<f64> {
        let mut vec = vec![0.0; self.id_to_word.len()];
        let tokens = Self::tokenize(text);
        if tokens.is_empty() {
            return vec;
        }

        for token in &tokens {
            if let Some(&id) = self.word_to_id.get(token) {
                vec[id] += 1.0;
            }
        }

        let mut norm_sq = 0.0;
        for i in 0..vec.len() {
            vec[i] = (vec[i] / tokens.len() as f64) * self.idf[i];
            norm_sq += vec[i] * vec[i];
        }

        if norm_sq > 0.0 {
            let norm = norm_sq.sqrt();
            for val in &mut vec {
                *val /= norm;
            }
        }
        vec
    }
}

// ---------------------------------------------------------------------------
// K-Means Clustering
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KMeans {
    pub k: usize,
    pub centroids: Vec<Vec<f64>>,
}

impl KMeans {
    pub fn new(k: usize) -> Result<Self, PipelineError> {
        if k == 0 {
            return Err(PipelineError::InvalidConfiguration(
                "K must be > 0".to_string(),
            ));
        }
        Ok(Self {
            k,
            centroids: Vec::new(),
        })
    }

    pub fn fit(
        &mut self,
        data: &[Vec<f64>],
        max_iter: usize,
        rng: &mut FastRng,
    ) -> Result<Vec<usize>, PipelineError> {
        if data.is_empty() {
            return Err(PipelineError::DatasetInvalid(
                "Data cannot be empty".to_string(),
            ));
        }
        if data.len() < self.k {
            return Err(PipelineError::InvalidConfiguration(format!(
                "Samples ({}) must be >= k ({})",
                data.len(),
                self.k
            )));
        }

        let dim = data[0].len();
        self.centroids = Vec::with_capacity(self.k);
        for _ in 0..self.k {
            let idx = rng.gen_range(0, data.len())?;
            self.centroids.push(data[idx].clone());
        }

        let mut assignments = vec![0; data.len()];

        for _ in 0..max_iter {
            let mut changed = false;

            for (i, x) in data.iter().enumerate() {
                let mut best_dist = f64::MAX;
                let mut best_cluster = 0;
                for (c_idx, c) in self.centroids.iter().enumerate() {
                    let dist: f64 = x.iter().zip(c.iter()).map(|(a, b)| (a - b).powi(2)).sum();
                    if dist < best_dist {
                        best_dist = dist;
                        best_cluster = c_idx;
                    }
                }
                if assignments[i] != best_cluster {
                    assignments[i] = best_cluster;
                    changed = true;
                }
            }

            if !changed {
                break;
            }

            let mut counts = vec![0usize; self.k];
            let mut new_centroids = vec![vec![0.0; dim]; self.k];

            for (x, &cluster) in data.iter().zip(assignments.iter()) {
                counts[cluster] += 1;
                for (d, &val) in x.iter().enumerate() {
                    new_centroids[cluster][d] += val;
                }
            }

            for c in 0..self.k {
                if counts[c] > 0 {
                    for d in 0..dim {
                        new_centroids[c][d] /= counts[c] as f64;
                    }
                } else {
                    let rand_idx = rng.gen_range(0, data.len())?;
                    new_centroids[c] = data[rand_idx].clone();
                }
            }
            self.centroids = new_centroids;
        }

        Ok(assignments)
    }

    pub fn predict(&self, x: &[f64]) -> Result<usize, PipelineError> {
        if self.centroids.is_empty() {
            return Err(PipelineError::NumericalError(
                "KMeans not fitted".to_string(),
            ));
        }

        let mut best_dist = f64::MAX;
        let mut best_cluster = 0;
        for (c_idx, c) in self.centroids.iter().enumerate() {
            let dist: f64 = x.iter().zip(c.iter()).map(|(a, b)| (a - b).powi(2)).sum();
            if dist < best_dist {
                best_dist = dist;
                best_cluster = c_idx;
            }
        }
        Ok(best_cluster)
    }
}

// ---------------------------------------------------------------------------
// Stage 1: Non-Negated TM (NTM)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonNegatedTM {
    pub num_classes: usize,
    pub num_clauses_per_class: usize,
    pub num_features: usize,
    pub threshold: i32,
    pub states_per_action: i32,
    pub ta_states: Vec<Vec<Vec<i32>>>,
}

impl NonNegatedTM {
    pub fn new(
        num_classes: usize,
        num_clauses_per_class: usize,
        num_features: usize,
        threshold: i32,
        states_per_action: i32,
    ) -> Result<Self, PipelineError> {
        if num_classes == 0 || num_clauses_per_class == 0 || num_features == 0 {
            return Err(PipelineError::InvalidConfiguration(
                "TM params must be > 0".to_string(),
            ));
        }
        let n = states_per_action;
        let ta_states = vec![vec![vec![n; num_features]; num_clauses_per_class]; num_classes];

        Ok(Self {
            num_classes,
            num_clauses_per_class,
            num_features,
            threshold,
            states_per_action,
            ta_states,
        })
    }

    #[inline(always)]
    fn evaluate_clause(&self, class: usize, clause: usize, x: &[bool]) -> bool {
        let n = self.states_per_action;
        let states = &self.ta_states[class][clause];
        for k in 0..self.num_features {
            if states[k] > n && !x[k] {
                return false;
            }
        }
        true
    }

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
                context: "NTM train_step".to_string(),
            });
        }

        let n = self.states_per_action;
        let t = self.threshold;

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
                    if clause_output {
                        for k in 0..self.num_features {
                            if x[k] && self.ta_states[class][c][k] < 2 * n {
                                self.ta_states[class][c][k] += 1;
                            }
                        }
                    } else {
                        for k in 0..self.num_features {
                            if self.ta_states[class][c][k] > 1 {
                                self.ta_states[class][c][k] -= 1;
                            }
                        }
                    }
                } else if clause_output {
                    for k in 0..self.num_features {
                        if !x[k] && self.ta_states[class][c][k] < 2 * n {
                            self.ta_states[class][c][k] += 1;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn extract_cluster_keywords(
        &self,
        cluster_id: usize,
        vocab: &Vocabulary,
        top_m: usize,
    ) -> Vec<(String, i32)> {
        let n = self.states_per_action;
        let mut confidence_scores = vec![0i32; self.num_features];

        for c in (0..self.num_clauses_per_class).step_by(2) {
            for k in 0..self.num_features {
                let state = self.ta_states[cluster_id][c][k];
                if state > n {
                    confidence_scores[k] += state - n;
                }
            }
        }

        let mut ranked: Vec<(usize, i32)> = confidence_scores.into_iter().enumerate().collect();
        ranked.sort_by(|a, b| b.1.cmp(&a.1));

        ranked
            .into_iter()
            .take(top_m)
            .filter(|&(_, score)| score > 0)
            .map(|(id, score)| (vocab.id_to_word[id].clone(), score))
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Stage 2: Vanilla TM
// ---------------------------------------------------------------------------

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
                "TM params must be > 0".to_string(),
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
// Semantic Feature Enrichment
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticEnricher {
    pub cluster_keywords: HashMap<usize, Vec<usize>>,
}

impl SemanticEnricher {
    pub fn new(ntm: &NonNegatedTM, vocab: &Vocabulary, top_m: usize) -> Self {
        let mut cluster_keywords = HashMap::new();
        for c in 0..ntm.num_classes {
            let kws = ntm.extract_cluster_keywords(c, vocab, top_m);
            let ids = kws
                .into_iter()
                .filter_map(|(w, _)| vocab.word_to_id.get(&w).copied())
                .collect();
            cluster_keywords.insert(c, ids);
        }
        Self { cluster_keywords }
    }

    pub fn enrich(&self, raw_bow: &[bool], cluster_id: usize) -> Vec<bool> {
        let mut enriched = raw_bow.to_vec();
        if let Some(keywords) = self.cluster_keywords.get(&cluster_id) {
            for &kw_id in keywords {
                if kw_id < enriched.len() {
                    enriched[kw_id] = true;
                }
            }
        }
        enriched
    }
}

// ---------------------------------------------------------------------------
// Evaluation Report
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct EvaluationReport {
    pub accuracy: f64,
    pub macro_precision: f64,
    pub macro_recall: f64,
    pub macro_f1: f64,
    pub confusion_matrix: Vec<Vec<usize>>,
    pub class_labels: Vec<String>,
}

impl EvaluationReport {
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
        })
    }

    pub fn print_summary(&self) {
        println!("\n============================================================");
        println!("               Classification Evaluation Report             ");
        println!("============================================================");
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
// Pipeline Artifact & Inference
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiredRule {
    pub clause_index: usize,
    pub class_name: String,
    pub polarity: i32,
    pub rule: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionOutput {
    pub text: String,
    pub label: String,
    pub class_id: usize,
    pub margin: i32,
    pub cluster_id: usize,
    pub class_votes: HashMap<String, i32>,
    pub rules: Vec<FiredRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineModel {
    pub vocab: Vocabulary,
    pub kmeans: KMeans,
    pub enricher: SemanticEnricher,
    pub tm: VanillaTM,
    pub id_to_label: Vec<String>,
    pub label_to_id: HashMap<String, usize>,
}

impl PipelineModel {
    pub fn save_to_file<P: AsRef<Path>>(&self, absolute_path: P) -> Result<(), PipelineError> {
        let path_buf = absolute_path.as_ref().to_path_buf();
        if !path_buf.is_absolute() {
            return Err(PipelineError::InvalidConfiguration(format!(
                "Path must be absolute: {}",
                path_buf.display()
            )));
        }

        if let Some(parent) = path_buf.parent() {
            let _ = std::fs::create_dir_all(parent);
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

        Ok(model)
    }

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

        let raw_bow = self.vocab.text_to_bow(trimmed);
        let tfidf = self.vocab.text_to_tfidf(trimmed);
        let cluster_id = self.kmeans.predict(&tfidf)?;
        let enriched = self.enricher.enrich(&raw_bow, cluster_id);

        let mut class_votes = HashMap::new();
        let mut best_class = 0;
        let mut max_score = i32::MIN;
        let mut second_score = i32::MIN;

        for (c_idx, c_name) in self.id_to_label.iter().enumerate() {
            let score = self.tm.calculate_class_sum(c_idx, &enriched);
            class_votes.insert(c_name.clone(), score);

            if score > max_score {
                second_score = max_score;
                max_score = score;
                best_class = c_idx;
            } else if score > second_score {
                second_score = score;
            }
        }

        let margin = if self.id_to_label.len() > 1 {
            max_score - second_score
        } else {
            max_score
        };

        let label = self
            .id_to_label
            .get(best_class)
            .cloned()
            .unwrap_or_else(|| format!("Class_{best_class}"));

        let rules = self.extract_fired_rules(best_class, &enriched, max_rules);

        Ok(PredictionOutput {
            text: trimmed.to_string(),
            label,
            class_id: best_class,
            margin,
            cluster_id,
            class_votes,
            rules,
        })
    }

    fn extract_fired_rules(
        &self,
        class_id: usize,
        enriched_bow: &[bool],
        max_rules: usize,
    ) -> Vec<FiredRule> {
        let n = self.tm.states_per_action;
        let num_features = self.tm.num_features;
        let class_name = self.id_to_label.get(class_id).cloned().unwrap_or_default();
        let mut fired = Vec::new();

        for c in 0..self.tm.num_clauses_per_class {
            if fired.len() >= max_rules {
                break;
            }

            let mut clause_satisfied = true;
            let mut included = Vec::new();

            for k in 0..num_features {
                if self.tm.ta_states[class_id][c][k] > n {
                    if !enriched_bow[k] {
                        clause_satisfied = false;
                        break;
                    }
                    if let Some(word) = self.vocab.id_to_word.get(k) {
                        included.push(word.clone());
                    }
                }
                let neg_k = num_features + k;
                if self.tm.ta_states[class_id][c][neg_k] > n {
                    if enriched_bow[k] {
                        clause_satisfied = false;
                        break;
                    }
                    if let Some(word) = self.vocab.id_to_word.get(k) {
                        included.push(format!("¬{word}"));
                    }
                }
            }

            if clause_satisfied && !included.is_empty() {
                let polarity = if c % 2 == 0 { 1 } else { -1 };
                fired.push(FiredRule {
                    clause_index: c,
                    class_name: class_name.clone(),
                    polarity,
                    rule: included.join(" ∧ "),
                });
            }
        }

        fired
    }
}

// ---------------------------------------------------------------------------
// High-Performance Pipeline Training Engine (Pre-computed Feature Vectors)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub train_ratio: f64,
    pub min_df: usize,
    pub max_features: Option<usize>,
    pub num_clusters: usize,
    pub ntm_clauses: usize,
    pub ntm_threshold: i32,
    pub tm_clauses: usize,
    pub tm_threshold: i32,
    pub specificity: f64,
    pub states_per_action: i32,
    pub top_keywords_per_cluster: usize,
    pub epochs: usize,
    pub seed: u64,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            train_ratio: 0.8,
            min_df: 2,
            max_features: Some(4000),
            num_clusters: 2,
            ntm_clauses: 40,
            ntm_threshold: 40,
            tm_clauses: 80,
            tm_threshold: 50,
            specificity: 5.0,
            states_per_action: 100,
            top_keywords_per_cluster: 4,
            epochs: 25,
            seed: 42,
        }
    }
}

/// Trains the pipeline with pre-computed BoW feature vectors for maximum execution speed.
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

    // 1. Build Vocabulary
    println!(
        "[1/4] Building vocabulary across {} documents...",
        train_set.records.len()
    );
    let all_train_texts: Vec<String> = train_set.records.iter().map(|r| r.text.clone()).collect();
    let vocab = Vocabulary::build(&all_train_texts, config.min_df, config.max_features)?;
    println!("  Active vocabulary features: {}", vocab.id_to_word.len());

    // 2. Pre-calculate TF-IDF & Fit K-Means
    println!(
        "[2/4] Fitting K={} semantic clusters on TF-IDF space...",
        config.num_clusters
    );
    let tfidf_data: Vec<Vec<f64>> = all_train_texts
        .iter()
        .map(|text| vocab.text_to_tfidf(text))
        .collect();

    let mut kmeans = KMeans::new(config.num_clusters)?;
    let cluster_assignments = kmeans.fit(&tfidf_data, 25, &mut rng)?;

    // 3. Pre-train NTM on pre-computed BoW vectors
    println!(
        "[3/4] Pre-training NTM on semantic clusters ({} epochs)...",
        config.epochs
    );
    let mut ntm = NonNegatedTM::new(
        config.num_clusters,
        config.ntm_clauses,
        vocab.id_to_word.len(),
        config.ntm_threshold,
        config.states_per_action,
    )?;

    let train_bows: Vec<Vec<bool>> = all_train_texts
        .iter()
        .map(|text| vocab.text_to_bow(text))
        .collect();

    for _ in 0..config.epochs {
        for (bow, &cluster) in train_bows.iter().zip(cluster_assignments.iter()) {
            ntm.train_step(bow, cluster, &mut rng)?;
        }
    }

    let enricher = SemanticEnricher::new(&ntm, &vocab, config.top_keywords_per_cluster);

    // 4. Pre-compute enriched vectors ONCE for ultra-fast Stage 2 training
    println!(
        "[4/4] Supervised Vanilla TM training ({} epochs across {} classes)...",
        config.epochs,
        train_set.id_to_label.len()
    );

    // Pre-calculate all enriched training vectors
    let mut precomputed_training_samples: Vec<(Vec<bool>, usize)> =
        Vec::with_capacity(train_set.records.len());
    for (i, record) in train_set.records.iter().enumerate() {
        if let Some(&label_id) = train_set.label_to_id.get(&record.label) {
            let enriched = enricher.enrich(&train_bows[i], cluster_assignments[i]);
            precomputed_training_samples.push((enriched, label_id));
        }
    }

    let num_classes = train_set.id_to_label.len();
    let mut tm = VanillaTM::new(
        num_classes,
        config.tm_clauses,
        vocab.id_to_word.len(),
        config.tm_threshold,
        config.states_per_action,
        config.specificity,
    )?;

    // Ultra-fast epoch loop: pure slice updates, 0 allocations, 0 tokenizations
    for _ in 0..config.epochs {
        for (enriched_bow, label_id) in &precomputed_training_samples {
            tm.train_step(enriched_bow, *label_id, &mut rng)?;
        }
    }

    Ok(PipelineModel {
        vocab,
        kmeans,
        enricher,
        tm,
        id_to_label: train_set.id_to_label.clone(),
        label_to_id: train_set.label_to_id.clone(),
    })
}

/// Evaluates a trained pipeline on a test dataset.
pub fn evaluate_pipeline(
    model: &PipelineModel,
    test_set: &Dataset,
) -> Result<EvaluationReport, PipelineError> {
    let mut predictions = Vec::with_capacity(test_set.records.len());
    let mut ground_truth = Vec::with_capacity(test_set.records.len());

    for record in &test_set.records {
        let label_id = match model.label_to_id.get(&record.label) {
            Some(&id) => id,
            None => continue,
        };
        let raw_bow = model.vocab.text_to_bow(&record.text);
        let tfidf = model.vocab.text_to_tfidf(&record.text);
        let cluster_id = model.kmeans.predict(&tfidf)?;
        let enriched = model.enricher.enrich(&raw_bow, cluster_id);

        let pred = model.tm.predict(&enriched)?;
        predictions.push(pred);
        ground_truth.push(label_id);
    }

    EvaluationReport::compute(&predictions, &ground_truth, &model.id_to_label)
}

// ---------------------------------------------------------------------------
// CLI Execution Layer
// ---------------------------------------------------------------------------

struct CliArgs {
    mode: String,
    train_file: Option<PathBuf>,
    test_file: Option<PathBuf>,
    model_path: Option<PathBuf>,
    predict_text: Option<String>,
    text_col: String,
    label_col: String,
    is_jsonl: bool,
    epochs: usize,
    clauses: usize,
    max_features: usize,
    min_df: usize,
    clusters: usize,
    train_ratio: f64,
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
        let mut epochs = 25;
        let mut clauses = 80;
        let mut max_features = 4000;
        let mut min_df = 2;
        let mut clusters = 2;
        let mut train_ratio = 0.8;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--mode" => {
                    i += 1;
                    if i < args.len() {
                        mode = args[i].clone();
                    }
                }
                "--train-file" | "--train" => {
                    i += 1;
                    if i < args.len() {
                        let p = PathBuf::from(&args[i]);
                        train_file = Some(
                            p.canonicalize()
                                .map_err(|e| PipelineError::IoError { path: p, source: e })?,
                        );
                    }
                }
                "--test-file" | "--test" => {
                    i += 1;
                    if i < args.len() {
                        let p = PathBuf::from(&args[i]);
                        test_file = Some(
                            p.canonicalize()
                                .map_err(|e| PipelineError::IoError { path: p, source: e })?,
                        );
                    }
                }
                "--model-path" | "--model" => {
                    i += 1;
                    if i < args.len() {
                        let p = PathBuf::from(&args[i]);
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
                }
                "--text" | "--predict-text" => {
                    i += 1;
                    if i < args.len() {
                        predict_text = Some(args[i].clone());
                    }
                }
                "--text-col" => {
                    i += 1;
                    if i < args.len() {
                        text_col = args[i].clone();
                    }
                }
                "--label-col" => {
                    i += 1;
                    if i < args.len() {
                        label_col = args[i].clone();
                    }
                }
                "--jsonl" => {
                    is_jsonl = true;
                }
                "--epochs" => {
                    i += 1;
                    if i < args.len() {
                        epochs = args[i].parse::<usize>().unwrap_or(25);
                    }
                }
                "--clauses" => {
                    i += 1;
                    if i < args.len() {
                        clauses = args[i].parse::<usize>().unwrap_or(80);
                    }
                }
                "--max-features" => {
                    i += 1;
                    if i < args.len() {
                        max_features = args[i].parse::<usize>().unwrap_or(4000);
                    }
                }
                "--min-df" => {
                    i += 1;
                    if i < args.len() {
                        min_df = args[i].parse::<usize>().unwrap_or(2);
                    }
                }
                "--clusters" => {
                    i += 1;
                    if i < args.len() {
                        clusters = args[i].parse::<usize>().unwrap_or(2);
                    }
                }
                "--train-ratio" => {
                    i += 1;
                    if i < args.len() {
                        train_ratio = args[i].parse::<f64>().unwrap_or(0.8);
                    }
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                _ => {}
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
            epochs,
            clauses,
            max_features,
            min_df,
            clusters,
            train_ratio,
        })
    }
}

fn print_help() {
    println!("Semantic Tsetlin Machine NLP Classification CLI");
    println!("===============================================");
    println!("\nTRAIN MODE:");
    println!("  cargo run --release -- --mode train --train-file <PATH> [OPTIONS]");
    println!("  Options:");
    println!("    --test-file <PATH>     Optional test split path (evaluates split if omitted)");
    println!("    --model-path <PATH>    Destination path to save trained .json model");
    println!("    --text-col <NAME>      Text column name (default: 'text')");
    println!("    --label-col <NAME>     Label column name (default: 'label')");
    println!("    --jsonl                Parse input as JSONL instead of CSV");
    println!("    --epochs <N>           Training epochs (default: 25)");
    println!("    --clauses <N>          Clauses per class (default: 80)");
    println!("    --max-features <N>     Vocabulary size cap (default: 4000)");
    println!("    --min-df <N>           Minimum document frequency (default: 2)");
    println!("    --clusters <N>         Unsupervised semantic clusters (default: 2)");
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

    let config = BenchmarkConfig {
        train_ratio: args.train_ratio,
        min_df: args.min_df,
        max_features: Some(args.max_features),
        num_clusters: args.clusters,
        ntm_clauses: args.clauses / 2,
        tm_clauses: args.clauses,
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
        test_dataset.align_labels_with(&train_dataset.label_to_id);

        let model = train_pipeline(&train_dataset, &config)?;
        let report = evaluate_pipeline(&model, &test_dataset)?;
        (model, report)
    } else {
        println!(
            "Splitting dataset into {:.0}% train / {:.0}% test...",
            args.train_ratio * 100.0,
            (1.0 - args.train_ratio) * 100.0
        );
        let mut rng = FastRng::seed(config.seed);
        let (train_split, test_split) = train_dataset.split(args.train_ratio, &mut rng)?;
        let model = train_pipeline(&train_split, &config)?;
        let report = evaluate_pipeline(&model, &test_split)?;
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
    println!("Assigned Cluster:  Cluster {}", output.cluster_id);
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
                "+1 vote"
            } else {
                "-1 veto"
            };
            println!(
                "  [Clause #{:<2} ({sign})] IF ( {} ) THEN {}",
                rule.clause_index, rule.rule, rule.class_name
            );
        }
    }
    println!("============================================================\n");

    Ok(())
}

fn run_demo() -> Result<(), PipelineError> {
    println!("No mode flag passed. Running built-in self-test demo...\n");

    let temp_dir = std::env::temp_dir();
    let demo_csv = temp_dir.join("nlp_demo.csv");
    let model_file = temp_dir.join("nlp_demo_model.json");

    {
        let mut file = File::create(&demo_csv).map_err(|e| PipelineError::IoError {
            path: demo_csv.clone(),
            source: e,
        })?;
        writeln!(file, "sentence,category").map_err(|e| PipelineError::IoError {
            path: demo_csv.clone(),
            source: e,
        })?;
        writeln!(
            file,
            "premier league team secured championship victory in finals,Sports"
        )
        .map_err(|e| PipelineError::IoError {
            path: demo_csv.clone(),
            source: e,
        })?;
        writeln!(
            file,
            "basketball point guard scored decisive winning shots in tournament,Sports"
        )
        .map_err(|e| PipelineError::IoError {
            path: demo_csv.clone(),
            source: e,
        })?;
        writeln!(
            file,
            "striker scored remarkable winning goal in championship finals,Sports"
        )
        .map_err(|e| PipelineError::IoError {
            path: demo_csv.clone(),
            source: e,
        })?;
        writeln!(
            file,
            "tennis player celebrates historic grand slam championship victory,Sports"
        )
        .map_err(|e| PipelineError::IoError {
            path: demo_csv.clone(),
            source: e,
        })?;
        writeln!(
            file,
            "astronomers observe distant galaxy using deep space telescope,Science"
        )
        .map_err(|e| PipelineError::IoError {
            path: demo_csv.clone(),
            source: e,
        })?;
        writeln!(
            file,
            "nasa launched rocket mission orbiting mars planet observatory,Science"
        )
        .map_err(|e| PipelineError::IoError {
            path: demo_csv.clone(),
            source: e,
        })?;
        writeln!(
            file,
            "deep space satellite detects cosmic radiation signals in galaxy,Science"
        )
        .map_err(|e| PipelineError::IoError {
            path: demo_csv.clone(),
            source: e,
        })?;
        writeln!(
            file,
            "spacecraft maneuvers orbit around international space station,Science"
        )
        .map_err(|e| PipelineError::IoError {
            path: demo_csv.clone(),
            source: e,
        })?;
    }

    let dataset = Dataset::from_csv(&demo_csv, b',', "sentence", "category")?;
    let config = BenchmarkConfig {
        min_df: 1,
        max_features: Some(100),
        num_clusters: 2,
        epochs: 30,
        seed: 42,
        ..BenchmarkConfig::default()
    };

    println!("Training demo model...");
    let model = train_pipeline(&dataset, &config)?;
    model.save_to_file(&model_file)?;

    println!("Testing predict on serialized model...");
    let sample = "telescope observed ancient galaxy in outer space";
    let loaded_model = PipelineModel::load_from_file(&model_file)?;
    let result = loaded_model.predict_one(sample, 2)?;

    println!("\nInference Text: \"{}\"", sample);
    println!("Predicted Label: {}", result.label);
    assert_eq!(result.label, "Science");
    println!("Demo completed successfully. Run with '--help' to view full CLI options.");

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

    #[test]
    fn test_model_save_load_predict_consistency() {
        let temp_dir = std::env::temp_dir();
        let model_path = temp_dir.join("test_save_load_model.json");

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
            num_clusters: 2,
            epochs: 15,
            seed: 99,
            ..BenchmarkConfig::default()
        };

        let model = train_pipeline(&dataset, &config).expect("Training failed");
        model.save_to_file(&model_path).expect("Saving failed");

        let loaded = PipelineModel::load_from_file(&model_path).expect("Loading failed");

        let test_sentence = "astronomy telescope star orbit";
        let out = loaded
            .predict_one(test_sentence, 2)
            .expect("Prediction failed");

        assert_eq!(out.label, "science");
        assert_eq!(out.class_id, 1);
        assert!(!out.class_votes.is_empty());
    }

    #[test]
    fn test_dataset_jsonl_parsing() {
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join("test_dataset.jsonl");

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
}
