

**Reading rule for the list:** for a struct/enum, include the type definition with *all* its fields/variants and its doc block. For an `impl`, include **only the methods named** — anything not named is not in this crate. Every listed item is copied verbatim from the full crate (docs and comments included) unless it is marked `→ REWRITTEN Rn`.

---

## A. What to include, in file order

### Header
- Module doc block → **REWRITTEN R0**
- `#![forbid(unsafe_code)]`

### Section 1 — Errors
- `enum GranmoModelError` (all variants) → **add two variants, R1**
- `impl GranmoModelError`: `is_retryable` (→ **add two arms, R1**), `code`
- `impl core::fmt::Display for GranmoModelError` (cfg debug/test)

### Section 3 — Enforced newtypes
- `struct NgramLength`; `impl`: `MIN`, `MAX`, `new`, `get`
- `const PROBABILITY_FIXED_POINT_UNIT`
- `struct VoteThreshold`; `impl`: `MIN`, `MAX`, `new`, `get`
- `struct MaxScanBytes`; `impl`: `MIN`, `MAX`, `new`, `get`
- `struct PatchSize`; `impl`: `MIN`, `MAX`, `new`, `get`
- `struct StrideLen`; `impl`: `MIN`, `MAX`, `new`, `get`
- `struct ClauseCount`; `impl`: `MAX`, `new`, `get`
- `struct StatesPerAction`; `impl`: `MIN`, `MAX`, `new`, `get`
- `struct SpecificityThresholds`; `impl`: `from_raw_thresholds`, `from_specificity`, `get`

### Section 4 — Preprocess
- `enum PreprocessStage`
- `const PREPROCESS_DEFINED_STAGE_BITS`
- `struct PreprocessProfile`; `impl`: `from_bits`, `get_bits`, `preset_p0`, `preset_p2`
- `fn is_ascii_punct_not_space`
- `fn leet_fold_byte`
- `struct BytePreprocessor`; `impl`: `new`, `reset`, `process_byte`, `process_document`

### Section 5 — ByteConvTM (evaluation surface)
- `const PAD_BYTE`, `const BYTE_ALPHABET_SIZE`, `const MASK_WORDS`
- `fn derive_allowed_mask_from_states`
- `struct ByteConvTM` (all fields). Add `#[allow(dead_code)]` to exactly three fields: `forget_threshold_u16`, `reinforce_threshold_u16`, `guarded_include` — with this comment above each group: `// Persisted training hyper-parameter: validated at load, stored so the struct is the complete model; read only by the training crate.`
- `impl ByteConvTM` (this is one impl block; take these methods from the several impl blocks in the full crate):
  `conv_build_probability_lut`, `new`, `literals_per_clause`, `positive_local_index`, `negated_local_index`, `global_state_index`, `global_slot_index`, `depth_for_clause`, `scan_plan`, `window_byte`, `clause_fires_at`, `clause_fires_any`, `conv_vote_sum`, `conv_predict`, `conv_fired_window_positions`, `conv_describe_clause`, `compute_mask_from_states`, `conv_validate_internal_consistency`, `conv_clause_count`, `rebuild_caches_from_states` → **REWRITTEN R2**

### Section 5B — ClassifierEngine
- `enum ClassifierEngine` (3 variants)
- `impl ClassifierEngine`: `engine_name`, `engine_vote_sum`, `engine_predict`, `engine_validate_internal_consistency`, `engine_build_probability_lut`

### Section 6 — Self-check
- `fn run_self_check`

### Section 7 — Probability
- `struct ProbabilityLut`; `impl`: `build`, `probability_u16`, `lut_validity_recheck`, `probability_for_report`

### Section 8 — Artifact load
- Format banner comment (the whole `// Binary format, version 2 …` block)
- `const ARTIFACT_MAGIC`, `ARTIFACT_FORMAT_VERSION`, `ARTIFACT_KIND_BYTECONV_FULL_TRAINING`, `ARTIFACT_KIND_BYTEBAG_FULL_TRAINING`, `ARTIFACT_KIND_SEQ_FREQ_HYBRID_FULL_TRAINING`
- `fn fnv1a_64`
- `struct ByteCursor`; `impl`: `new`, `take`, `read_u8`, `read_u16_le`, `read_i16_le`, `read_u32_le`
- `fn read_byte_conv_full_body`
- `fn read_byte_bag_full_body` → **REWRITTEN R3**
- `fn read_seq_freq_hybrid_full_body`
- `struct ModelArtifact`; `impl`: `load_from_file`
- `impl ByteBagTM`: `bag_rebuild_masks_from_states` → **REWRITTEN R4**

### Section 11 — Predict CLI → **REWRITTEN R5** (entire section)
- `struct PredictCliArgs`; `impl`: `parse_predict_cliargs`
- `fn resolve_predict_input_bytes`
- `fn print_predict_help`
- `fn handle_predict`
- `fn print_conv_explain_trace` (verbatim from full crate)
- `fn print_bag_explain_trace` (verbatim from full crate)
- `pub fn run_predict_main`

### Section 12 — ByteBagVocabulary
- `const BAG_NGRAM_MAX_LEN`
- `struct ByteBagVocabulary`; `impl`: `from_flat_bytes`, `vocabulary_len`, `ngram_at_rank`, `rebuild_lookup_order`, `lookup`, `extract_presence_bits`, `vocab_validity_recheck`

### Section 12B — ByteBagTM (evaluation surface)
- Section banner comment (verbatim), prefixed with the one-paragraph note in **R6**
- `struct ByteBagTM` → **REWRITTEN R6** (guard fields removed)
- `impl ByteBagTM`: `new_with_vocabulary` → **REWRITTEN R6**; verbatim: `bag_feature_count`, `bag_literals_per_clause`, `bag_mask_words_per_clause`, `bag_state_index`, `bag_mask_word_range`, `bag_depth_for_clause`, `bag_document_presence_bits`, `bag_clause_fires`, `bag_vote_sum`, `bag_predict`, `bag_fired_clause_bits`, `bag_describe_clause`, `bag_clause_count`, `bag_build_probability_lut`; `bag_validate_internal_consistency` → **REWRITTEN R7**

### Section 12C — HybridTM (evaluation surface)
- Section banner comment (verbatim)
- `struct HybridTM` (all fields)
- `impl HybridTM`: `new_from_sub_engines`, `hyb_conv_engine_ref`, `hyb_bag_engine_ref`, `hyb_clause_count`, `hyb_vote_sum`, `hyb_predict`, `hyb_validate_internal_consistency`, `hyb_build_probability_lut`

### Tests (`#[cfg(test)] mod tests`) — verbatim, all reference only items above
- helpers: `make_engine`, `temp_artifact_path`
- `engine_rejects_stride_exceeding_patch`
- `fresh_engine_fires_everywhere_and_votes_zero` — delete its last two lines (the `conv_fired_clause_bits` assertion; that method is not in this crate)
- `error_codes_are_two_bytes_and_copy`
- `patch_size_bounds_and_recheck`, `clause_count_must_be_even_nonzero`, `specificity_thresholds_integer_exactness`
- `profile_rejects_reserved_bits`
- `p0_folds_dedupes_trims_lowercases`, `p2_leet_fold_merges_obfuscated_spelling`, `streaming_equals_batch_processing`, `reset_isolates_documents`
- `probability_lut_is_monotone_sigmoid_centered_at_zero`, `probability_lut_rejects_out_of_range_votes`
- `artifact_rejects_foreign_files_and_relative_paths`, `artifact_rejects_unsupported_version_and_kind`, `specificity_from_raw_thresholds_enforces_invariant`

Round-trip equivalence is verified by saving with the full crate and loading here (the acceptance test stated in R0); this crate has no writer.

### `main.rs` → **R8**

---

## B. Rewritten / new code

### R0 — Module header

```rust
//! # Granmo-Predict: Inference-Only Loader/Predictor for Granmo Model Artifacts
//! — stripped-down companion to the Disco-Granmo Phase 4 crate
//!
//! Observing: https://github.com/lineality/rust_lang_rules
//!
//! This module contains EXACTLY the code needed to run/predict on a saved
//! model file (`.gmb`, Section 8 format v2, kinds 1/3/4): the error-code
//! system, the enforced configuration newtypes the loader re-bounds header
//! values through, the byte-stream preprocessor (replayed from the profile
//! persisted INSIDE the artifact, locked decision §10.9), the three engines'
//! evaluation surfaces, the probability LUT, the four-gate artifact loader,
//! and a minimal `predict` CLI.
//!
//! ## What is deliberately absent (lives in the full crate)
//! Training (feedback/transition code, RNG, worker parallelism, fire guard),
//! dataset ingestion, the experiment harness, batch modes, reports, and
//! artifact WRITING. This crate never writes a model file.
//!
//! ## Contract with the full crate
//! Every function here is copied verbatim (same name, same body) from the
//! training crate except those marked REWRITTEN in the hand-off inventory.
//! ACCEPTANCE TEST: a model saved by the full crate and loaded here must
//! produce IDENTICAL vote sums on every document to the full crate's own
//! `--mode predict`.
//!
//! ## Section map
//! S1 errors · S3 enforced newtypes · S4 preprocess · S5 ByteConvTM (eval)
//! · S5B ClassifierEngine · S6 self-check · S7 probability · S8 artifact
//! load · S11 predict CLI · S12 bag vocabulary · S12B ByteBagTM (eval) ·
//! S12C HybridTM (eval) · tests
//!
//! ## Modes (per the Mode & Case Handling framework)
//! - Production-release: never panics, error paths allocate nothing.
//! - Debug: `eprintln!` diagnostics gated `#[cfg(debug_assertions)]`.
//! - Test: `#[cfg(test)]` cargo tests use `assert!` freely.

#![forbid(unsafe_code)]
```

### R1 — Error enum additions

Add to `GranmoModelError`, at the end of the `800–899` block (after `CliPredictionRecordMalformed = 814`):

```rust
    /// `--text-file` path was not absolute (crate-wide absolute-path policy).
    CliTextFilePathNotAbsolute = 815,
    /// Reading the `--text-file` input failed (filesystem detail dropped per
    /// no-PII policy). RETRYABLE: may be a transient filesystem state.
    CliTextFileReadFailed = 816,
```

In `is_retryable`: add `| Self::CliTextFileReadFailed` to the `true` arm and `| Self::CliTextFilePathNotAbsolute` to the `false` arm.

Add this attribute and note directly above the enum (the codes are append-only and must keep decoding identically to the full crate's logs, so variants no path here produces are retained by design):

```rust
/// INFERENCE-ONLY BUILD NOTE: variants for training, dataset, harness and
/// parallelism codes are not produced by any path in this module. They are
/// retained because codes are append-only and permanent: a code seen in a
/// full-crate log or exit status must decode to the same meaning here.
#[allow(dead_code)]
```

### R2 — `ByteConvTM::rebuild_caches_from_states`

```rust
    /// Rebuilds the positive-include counts and allowed-bytes masks purely
    /// from raw automaton states. Artifact-load path: derived caches are
    /// never trusted from disk (they are not even stored). After this,
    /// `conv_validate_internal_consistency` must pass by construction — it
    /// is still run as the final load gate (defence-in-depth against a bug
    /// in THIS function). Depth is read per clause through
    /// `depth_for_clause` (the M-Hetero seam), matching the validation
    /// loop's structure.
    ///
    /// INFERENCE-ONLY BUILD: the full crate writes the mask through the
    /// clause work-view so live training and the load rebuild share one
    /// implementation. That view is training code and is absent here; the
    /// mask is written directly from `compute_mask_from_states`, which is
    /// the SAME single source of truth (`derive_allowed_mask_from_states`)
    /// the view called — the semantics cannot differ.
    fn rebuild_caches_from_states(&mut self) -> Result<(), GranmoModelError> {
        for clause in 0..self.n_clauses {
            let depth_n = self.depth_for_clause(clause);
            for slot in 0..self.patch_size {
                // Recompute the include count for this slot.
                let pos_start =
                    self.global_state_index(clause, self.positive_local_index(slot, 0))?;
                let pos_end = pos_start
                    .checked_add(BYTE_ALPHABET_SIZE)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)?;
                let recomputed_count = self
                    .ta_states
                    .get(pos_start..pos_end)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)?
                    .iter()
                    .filter(|&&s| s > depth_n)
                    .count() as u16;

                // Recompute the mask from states (read-only helper), then
                // store both derived values for this slot.
                let recomputed_mask = self.compute_mask_from_states(clause, slot)?;
                let slot_idx = self.global_slot_index(clause, slot)?;
                *self
                    .positive_include_counts
                    .get_mut(slot_idx)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)? = recomputed_count;
                *self
                    .allowed_masks
                    .get_mut(slot_idx)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)? = recomputed_mask;
            }
        }
        Ok(())
    }
```

### R3 — `read_byte_bag_full_body`

```rust
/// Parses the kind-3 (ByteBag full-training) body: every header value
/// passes through its enforced-type constructor (load gate 2); the
/// vocabulary is reconstructed from its flat bytes with its lookup order
/// rebuilt and structurally validated; raw states are read and the include
/// masks rebuilt from them (gate 3). Gate 4 — full consistency validation —
/// runs in the COMMON load path via the engine enum.
fn read_byte_bag_full_body(cursor: &mut ByteCursor<'_>) -> Result<ByteBagTM, GranmoModelError> {
    let ngram_len = NgramLength::new(cursor.read_u8()?)?;
    if cursor.read_u8()? != 0 {
        return Err(GranmoModelError::ArtHeaderFieldInvalid);
    }
    let vocab_count = cursor.read_u16_le()?;
    if vocab_count == 0 {
        return Err(GranmoModelError::ArtHeaderFieldInvalid);
    }
    let clause_count = ClauseCount::new(cursor.read_u16_le()?)?;
    let vote_target = VoteThreshold::new(cursor.read_i16_le()?)?;
    let automaton_depth = StatesPerAction::new(cursor.read_i16_le()?)?;
    let forget_threshold_raw = cursor.read_u16_le()?;
    let reinforce_threshold_raw = cursor.read_u16_le()?;
    let specificity =
        SpecificityThresholds::from_raw_thresholds(forget_threshold_raw, reinforce_threshold_raw)?;
    let scan_cap = MaxScanBytes::new(cursor.read_u32_le()?)?;

    // Vocabulary flat bytes: fixed width, exactly vocab_count * n bytes.
    let vocabulary_flat_len = usize::from(vocab_count)
        .checked_mul(usize::from(ngram_len.get()?))
        .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
    let vocabulary_flat_slice = cursor.take(vocabulary_flat_len)?;
    let restored_vocabulary =
        ByteBagVocabulary::from_flat_bytes(ngram_len, vocabulary_flat_slice.to_vec())?;

    // Engine shell from validated header values, then state overwrite.
    // Fire guard: not part of this inference-only build. Artifact kind 3
    // never carried guard state (limit, streaks, reset counters are
    // ephemeral training-session state — Drop 4.1 decision of record), so
    // nothing is lost at load and the format is unchanged.
    let mut bag_engine = ByteBagTM::new_with_vocabulary(
        restored_vocabulary,
        clause_count,
        vote_target,
        automaton_depth,
        specificity,
        scan_cap,
    )?;

    let expected_state_count = bag_engine.bag_ta_states.len();
    let state_bytes = cursor.take(
        expected_state_count
            .checked_mul(2)
            .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?,
    )?;
    for (state_slot, chunk) in bag_engine
        .bag_ta_states
        .iter_mut()
        .zip(state_bytes.chunks_exact(2))
    {
        *state_slot = i16::from_le_bytes([chunk[0], chunk[1]]);
    }
    bag_engine.bag_rebuild_masks_from_states()?;
    Ok(bag_engine)
}
```

### R4 — `ByteBagTM::bag_rebuild_masks_from_states`

```rust
impl ByteBagTM {
    /// Rebuilds both include masks purely from raw automaton states
    /// (artifact-load path, gate 3). Masks are zeroed and re-derived
    /// bit-by-bit from the include rule of record (state > N ⇒ included).
    /// After this, `bag_validate_internal_consistency` must pass by
    /// construction — it is still run as the final load gate
    /// (defence-in-depth, mirroring `rebuild_caches_from_states`).
    ///
    /// INFERENCE-ONLY BUILD: the full crate routes this through the clause
    /// work-view's `view_bag_set_include_bit` so training and load share
    /// one implementation. That view is training code and is absent here;
    /// the bit rule below is the same rule the validator recomputes, so a
    /// divergence would fail gate 4 immediately.
    fn bag_rebuild_masks_from_states(&mut self) -> Result<(), GranmoModelError> {
        for word in self.bag_positive_include_masks.iter_mut() {
            *word = 0;
        }
        for word in self.bag_negated_include_masks.iter_mut() {
            *word = 0;
        }
        let feature_count = self.bag_feature_count();
        for clause in 0..self.bag_clause_total {
            let clause_depth = self.bag_depth_for_clause(clause);
            let (range_start, _range_end) = self.bag_mask_word_range(clause)?;
            for rank in 0..feature_count {
                // Negated twin lives at M + rank; sums stay far inside usize
                // (M <= 65000) and bag_state_index re-validates the range.
                let positive_state = *self
                    .bag_ta_states
                    .get(self.bag_state_index(clause, rank)?)
                    .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
                let negated_state = *self
                    .bag_ta_states
                    .get(self.bag_state_index(clause, feature_count + rank)?)
                    .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
                let word_index = range_start
                    .checked_add(rank >> 6)
                    .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
                let bit = 1u64 << (rank & 63);
                if positive_state > clause_depth {
                    let word = self
                        .bag_positive_include_masks
                        .get_mut(word_index)
                        .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
                    *word |= bit;
                }
                if negated_state > clause_depth {
                    let word = self
                        .bag_negated_include_masks
                        .get_mut(word_index)
                        .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
                    *word |= bit;
                }
            }
        }
        Ok(())
    }
}
```

### R5 — Section 11: predict CLI (replaces the whole section)

`print_conv_explain_trace` and `print_bag_explain_trace` are copied verbatim from the full crate and are not repeated here.

```rust
// ===========================================================================
// SECTION 11: Predict CLI (fail-fast parsing; single mode)
// ===========================================================================

/// Parsed CLI arguments for the inference binary. Unknown flags are hard
/// errors (fail-fast policy) — a typo must never silently run with defaults.
#[derive(Debug, Clone)]
struct PredictCliArgs {
    /// Accepted for command-line compatibility with the full crate; the
    /// only valid value is "predict" (the default when omitted).
    mode: String,
    model_in: Option<std::path::PathBuf>,
    /// Inline document text (UTF-8 from the shell; scored as bytes).
    predict_text: Option<String>,
    /// Absolute path to a file whose RAW BYTES are the document. The model
    /// is byte-level, so no decode is performed on file content.
    text_file: Option<std::path::PathBuf>,
}

impl PredictCliArgs {
    fn parse_predict_cliargs(args: &[String]) -> Result<Self, GranmoModelError> {
        let mut parsed = Self {
            mode: "predict".to_string(),
            model_in: None,
            predict_text: None,
            text_file: None,
        };

        /// Fetches the value after a flag; a flag at end-of-args is an error.
        fn take_value<'a>(
            args: &'a [String],
            index: &mut usize,
            _flag: &str,
        ) -> Result<&'a str, GranmoModelError> {
            *index += 1;
            match args.get(*index) {
                Some(value) => Ok(value.as_str()),
                None => {
                    #[cfg(debug_assertions)]
                    eprintln!("CLI-801: flag '{}' requires a value", _flag);
                    Err(GranmoModelError::CliFlagMissingValue)
                }
            }
        }

        let mut i = 1usize;
        while let Some(flag_owned) = args.get(i) {
            let flag = flag_owned.as_str();
            match flag {
                "--mode" => parsed.mode = take_value(args, &mut i, flag)?.to_string(),
                "--model-in" => {
                    parsed.model_in =
                        Some(std::path::PathBuf::from(take_value(args, &mut i, flag)?))
                }
                "--text" => parsed.predict_text = Some(take_value(args, &mut i, flag)?.to_string()),
                "--text-file" => {
                    parsed.text_file =
                        Some(std::path::PathBuf::from(take_value(args, &mut i, flag)?))
                }
                "--help" | "-h" => {
                    print_predict_help();
                    std::process::exit(0);
                }
                _unknown => {
                    #[cfg(debug_assertions)]
                    eprintln!("CLI-800: unknown flag '{}'", _unknown);
                    return Err(GranmoModelError::CliUnknownFlag);
                }
            }
            i += 1;
        }
        Ok(parsed)
    }
}

/// Resolves the input document bytes for one prediction: `--text` (inline)
/// or `--text-file` (raw bytes from an absolute path). Exactly one source
/// must be present: both absent is a missing-required-flag error; both
/// present is an invalid-value error (fail-fast: never silently prefer one).
fn resolve_predict_input_bytes(args: &PredictCliArgs) -> Result<Vec<u8>, GranmoModelError> {
    match (&args.predict_text, &args.text_file) {
        (Some(_), Some(_)) => {
            #[cfg(debug_assertions)]
            eprintln!("CLI-802: --text and --text-file are mutually exclusive");
            Err(GranmoModelError::CliInvalidValue)
        }
        (Some(text), None) => Ok(text.as_bytes().to_vec()),
        (None, S
