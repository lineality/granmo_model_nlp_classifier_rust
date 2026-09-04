//! # Byte Granmo Model w/ Parallal Train/Predict Threading — Phase 2 Crate
//! NLP text classification oriented, for binary (2-class) models only
//!
//! Observing: https://github.com/lineality/rust_lang_rules
//!
//! Binary classification of short social-media text (hate/bullying
//! detection) using byte-level Granmo Models (per Granmo 2018, the
//! bandit-driven propositional-logic learning architecture), built to a
//! production profile: integer-only hot paths, no-heap no-panic error
//! handling, streaming-capable preprocessing, kind-dispatched binary
//! artifacts with four-gate load validation.
//!
//! ## Engines (both live behind the `ClassifierEngine` dispatch enum)
//! - `ByteConvTM` (Section 5, `conv_` methods): byte-level convolutional
//!   engine — K-byte windows, allowed-bytes masks, OR-pooling.
//! - `ByteBagTM` (Section 12B, `bag_` methods): flat bag-of-byte-n-grams
//!   engine — the §8 scientific control isolating positional windowing.
//!
//! ## Naming rule (project law)
//! Every function/method/type name is globally unique in this crate.
//! Engine methods carry their engine prefix (`conv_`, `bag_`); the plain
//! dispatch names exist only on `ClassifierEngine`. Sole recorded
//! exception pending ruling: the framework-uniform `new`/`get` pair on the
//! enforced newtypes (see the handoff document, open decisions).
//!
//! ## Section map
//! S1 errors · S2 RNG · S3 enforced newtypes · S4 preprocess ·
//! S5 ByteConvTM · S5B ClassifierEngine · S5C shared training math ·
//! S6 self-check · S7 probability/sweep · S8 artifacts · S9 dataset+split ·
//! S9B JSONL · S10 experiment runner · S11 CLI · S12 bag vocabulary ·
//! S12B ByteBagTM · main · tests
//!
//! ## Modes (per the Mode & Case Handling framework)
//! - Production-release: never panics, error paths allocate nothing.
//! - Debug: `eprintln!` diagnostics gated `#[cfg(debug_assertions)]`.
//! - Test: `#[cfg(test)]` cargo tests use `assert!` freely.

/*
TODO: Is this correct?

                           ┌───────────────────────────┐
                           │    Corpus Size (D_train)  │
                           └─────────────┬─────────────┘
                                         │
                 ┌───────────────────────┴───────────────────────┐
                 ▼                                               ▼
      Small / Noisy (D < 20k)                         Large / Clean (D > 50k)
┌───────────────────────────────────┐           ┌───────────────────────────────────┐
│ States (N):       40 – 80         │           │ States (N):       120 – 200       │
│ Epochs:           10 – 20         │           │ Epochs:           8 – 15          │
│ Vote Target (T):  C / 4           │           │ Vote Target (T):  C / 3           │
│ Specificity (s):  2.5 – 3.5       │           │ Specificity (s):  3.0 – 5.0       │
│ Vocab Size (M):   1000 – 2000     │           │ Vocab Size (M):   4000 – 8000     │
└───────────────────────────────────┘           └───────────────────────────────────┘


PROBLEM DOMAIN ALIGNMENT
┌──────────────────────────────────────────────┬──────────────────────────────────────────────┐
│                  ByteBagTM                   │                  ByteConvTM                  │
├──────────────────────────────────────────────┼──────────────────────────────────────────────┤
│ • Social media text & bullying detection     │ • Network packet payload inspection          │
│ • Document topic & sentiment classification  │ • Binary executable malware header detection │
│ • Multi-lingual / code-switched text         │ • Low-power IoT time-series anomaly detection│
│ • Long-range bag-of-tokens semantics         │ • Audio edge-detection & frame analysis      │
└──────────────────────────────────────────────┴──────────────────────────────────────────────┘


Heterogeneous Clause Bank Layout:
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│ Clause Index:   0 ... 399 (Positive Polarity)        │ 400 ... 599 (Negative Polarity)  │
├──────────────────────────┬───────────────────────────┼──────────────────┬───────────────┤
│ Species A: "Anchors"     │ Species B: "Scouts"       │ Species A:       │ Species B:    │
│ 70% (N=160, s=4.5)       │ 30% (N=40, s=2.0)         │ "Anchors" (70%)  │ "Scouts" (30%)│
│ Slow, noise-resistant    │ Fast, plastic, exploratory│                  │               │
└──────────────────────────┴───────────────────────────┴──────────────────┴───────────────┘
*/

/*

#### rust_lang_rules
(Production-Rust rules of thumb)

#### Also see [Mode & Case-Handling](https://github.com/lineality/modes_and_case_handling/blob/main/mode_case_handling_framework_summary.md)

# 🦀 Rust Guidelines 🦀:
- Always best practice
- Never ~unsafe code
- Always extensive doc strings: what the code is doing with project-context
- Always clear comments
- Include cargo tests (where possible)
- Functions should return result (in real life everything will fail at some point)
- Never remove (still-current) documentation
- Always clear, meaningful, globally unique names (e.g. variables, functions, etc.): Do not give two things one name, do not give one thing two names.
- Always absolute file paths
- Always error handling
- '?' should be avoided where it would obscure an error
- Never use unwrap (in production-release builds; use in cargo-tests is obviously valid)
- Always handle modes (test, debug, production-release) and cases/errors appropriately for that project
- Always follow boy-scout values
- Single-Flat: Default to a single-flat-file module x.rs. The crate src/ contains two files: main.rs and x.rs. main.rs uses "mod x; use x::FUNCTIONNAME;" to run the module. The single-flat-file module x.rs (where x = the name of the module) is portable to be added to other crates and projects).
- Vanilla-Rust: No third party crates.

# 🦀 10 Rust Rules 🦀:
1. Avoid Risky Methodologies:
- no recursion
- no goto
- no fancy pointer use
- no preprocessor branching
('unsafe' code blocks in Rust may be unavoidable)

2. Loops: either firmly bounded or unbounded w/ recovery

3. Attempt to Pre-allocate all memory (stack, not heap)
- case by case, avoid heap in production-release-mode where sound

4. Clear Function Scope and Data Ownership

5. Mode & Case Handling, & Defensive-Programming:
- Modes: test-mode, debug-mode, production-release-mode;
- continual state-recovery (without panic/halt) in production-release-mode
- See Example/Default Framework: [github.com/lineality/modes_and_case_handling](https://github.com/lineality/modes_and_case_handling)

6. Manage ownership and borrowing

7. Manage return values

8. Manage conditional compilation

9. Communicate:
- Use doc strings; use comments.
- Document use-cases, edge-cases, policies, intent: features vs. bugs, etc.
- Rather than let _ =, allow that result to be shown in debugging
- log errors (MVP: append log file in executable-parent dir)

10. Use state-less operations when possible

#### References & Links:
- https://en.wikipedia.org/wiki/The_Power_of_10:_Rules_for_Developing_Safety-Critical_Code
- https://spinroot.com/gerard/pdf/P10.pdf
- https://spinroot.com/static/index.html
- https://web.eecs.umich.edu/~imarkov/10rules.pdf
- https://en.wikipedia.org/wiki/Static_program_analysis
- https://www.oreilly.com/library/view/designing-data-intensive-applications/9781491903063/
- https://www.oreilly.com/library/view/rust-atomics-and/9781098119430/
- Books by https://en.wikipedia.org/wiki/P._J._Plauger

#### mode_case_handling_framework_summary



# Mode & Case Handling Bullet Points Summary
## Framework: Three Modes & Rules

### Three Modes of Operation:
1. Production-Release Mode:
- Never panics, halts, or leaks data.
- Uses 2-byte error codes (no heap, no strings, no PII).
- Smoothly handles all cases via "Let It Fail & Recover" (with optional logging).
- Uses a Three-Level Recovery Hierarchy

2. Debug Mode:
- Similar to production but logs verbose diagnostics (heap allowed).
- Uses `eprintln!` for errors and `debug_assert!` for internal invariants (gated to avoid running in test/production).
- Optional debug printing/logging for inspection.

3. Test Mode:
- Uses `assert!` in test functions.
- Deliberately crashes/panics to generate stack traces for debugging.



### Rules:

1. All functions return `Result<T, YourProjectError>`. The error payload is always the 2-byte `YourProjectError` enum. All operations are expected to eventually fail in some way; all such failures must be smoothly handled.

2. Production-Release Mode (Result-Error case-handling):
- never panics
- no heap in error-code-return from functions
- no print or log of that code by the function before the code is returned

3. Separate Debug-Mode Assertions from Cargo-Tests:
- `debug_assert!` must be gated with `#[cfg(all(debug_assertions, not(test)))]` to exclude from test mode and production mode builds.

4. Use Gated Verbose Debug-Mode Diagnostics:
- Use `eprintln!` (gated with `#[cfg(debug_assertions)]`) for debug-only output. Heap is allowed/needed here.

5. Test-Mode Isolation:
- Test code must be gated with `#[cfg(test)]`.
- Avoid using test-mode tests inside functions for stable code.

6. Use a Fieldless Enum error-code system.

7. Use Enforced-Custom-Types for Value-Integrity.

8. Use a Three-Level Recovery Hierarchy for recovery and state.


### Enforced-Custom-Types & Value-Integrity:
- Use `struct`/`enum`/`impl` to enforce value boundaries for inputs and intermediate values. This ensures invalid states (including bit-flips, corruption) are caught and handled as errors, not passed silently.
- required: add a validity-recheck into the .get() method
- Optionally use a .validity_recheck() method
- While a custom type in Rust can enforce and validate that a value is within the definition boundaries, this check only happens once when the constructor is run ( .new() ), assuming that the constructor was correctly written to carry out that check correctly. If memory-corruption happens after that initial check there are no automatic checks that will catch that the value is now invalid (e.g. as the value is returned from and accepted by another function). Additional .validity_recheck() methods can be made and used to manually check at specific points.
- Using custom types helps to manage compatibility within the design (e.g. narrowing scope of function inputs)
- vanilla custom types (without additional validity-checks) guard against design errors (coding-mistakes) and 'expected' errors.
- additional validity-rechecks can add guards against some electrical, hardware, and adversarial based errors, or 'unexpected errors.'
- Getting and mutation/updating use methods that must be manually designed to check & enforce boundary checks and validity.
#### Public & Private
- note: the individual rules for structs, enums, impls, and their combinations, vary.
- A struct CAN be safely pub (so other modules can use the type name in signatures).
- The struct's inner fields should NOT be pub (so other modules cannot bypass checks).
- Example of safety for combinations: if an enum variant carries data then do not put raw unbounded primitives inside that; rather, put a bounded struct inside.

### Three-Level Recovery Hierarchy ("Let It Fail & Recover")
Production-release failures should fit one of three bounded recovery tiers:
- Recovery Tier 1: Micro-Retry (Local) — If err.is_retryable() is true, a caller repeats the bounded operation (with backoff/sleep).
- Recovery Tier 2: Step Fallback / Safe Degradation (Subsystem) — This is at a level above which any retry-errors would be thrown. If non-retryable, or retry attempts are exhausted, the subsystem safely aborts the current command, and handles what to do next. This is highly case dependent, and might include trying another function, reverting to a previous state, exiting silently, etc. Some internal state may need to be reset. Ultimately move on with or without logging the case or "error."
- Recovery Tier 3: Macro Re-initialization — An outer loop reinitializes state and continues execution without halting. The largest case for Tier 3 reboots the entire program.

#### Recovery & State-Recovery:
Recovery tiers represent functional layers of 'state' in terms of 'recovery levels' to plan for, e.g. what to do when some state-values may not exist. Tier 1 covers the state in question. Tiers 2 and 3 range from small to large 'reboot/retry' scales up to the whole program. Full system restart will likely often have at least some state that it can be restarted with (resulting in no noticeable interruption), sometimes implying that state should be managed outside of retry-loops, to retain intact values.


### Error Handling System
- Single Fieldless Enum:
  A global `YourProjectError` enum (e.g., `#[repr(u16)]`) defines all error codes.
  Properties:
  - No heap (2-byte `Copy` values).
  - No `String` or `io::Error` payloads (prevents PII leaks).
  - Exhaustive `match` for retryability (`is_retryable()` method).
  - Append-only codes: Never reuse or renumber.

- Error Code Table Rules:
  1. Codes are unique and permanent (e.g., `Fs32tFromStrInputTooLong = 101`).
  2. Reserve blocks per module/feature (e.g., 100–199 for `FixedSize32Timestamp`).
  3. Variant names: `AcronymFunctionCondition` (e.g., `IoPermissionDenied`).
  4. Document codes in the enum’s doc comment.

- Display for Debug Only:
  `Display` impl (for human-readable text) is compiled only for debug/test builds.

#### Error Sites & Propagation:
- Unique codes permit use of '?' because the unique code of the root-cause error is preserved (not obscured) by propagating the error.
- If an error site is of note, that site needs to throw a specific error code.
- This acts as a proxy for manually checking for most internal-invariant type issues.


### Two Detection Patterns
1. If-Detection-Pattern:
   For direct condition checks (e.g., `if len > 32`).
   - Debug: `debug_assert!` (internal invariants only) + `eprintln!`.
   - Production: Return terse error code.

2. Match-A-Function-Call-Pattern:
   For handling `Result` from fallible calls (e.g., `std::str::from_utf8`).
   - Debug: `debug_assert!(false, ...)` (for internal invariants) + `eprintln!`.
   - Production: Drop callee’s error (to avoid heap/PII), return project error code.


## Key Principles
- No Heap in Production: Ban `String`, `format!`, `Box<dyn Error>`, etc.
- No Panics in Production: Use `checked_add`, `.get()`, etc., to avoid silent wraps/panics.
- Defensive Programming:
  - Input Validation: Expected issues (e.g., bad user input) → return error code + debug `eprintln!`.
  - Internal Invariant: "Should-not-happen" checks (e.g., bit-flips) → `debug_assert!` + production catch.
- Retry Logic: Defined per error code (not ranges) in `is_retryable()`.


### Example Code
#### Error Enum
```rust
#[repr(u16)]
pub enum YourProjectError {
    IoNotFound = 50,
    Fs32tFromStrInputTooLong = 101,
    RetryMaxAttemptsZero = 200,
}
```

#### If-Detection-Pattern
```rust
if !condition {
    #[cfg(debug_assertions)]
    eprintln!("ACRO-101: detail: {}", value);
    return Err(YourProjectError::AcroFnCondition);
}
```

#### Match-A-Function-Call-Pattern
```rust
match fallible_call(input) {
    Ok(v) => Ok(v),
    Err(_detail) => {
        #[cfg(debug_assertions)]
        eprintln!("ACRO-101: {}", _detail);
        Err(YourProjectError::AcroFnCondition)
    }
}
```

### Banned in Production
- `unwrap`/`expect`/`panic!`
- Heap allocations (`String`, `format!`, `Box<dyn Error>`)
- Error messages with PII (file/dir paths, user data, etc.)
- Unchecked Arithmetic: Arithmetic operators that can panic in production (use `checked_add`, etc.)


#### Suggestions (TODO: under construction; move to rules, for manageable use of codes?)
- Allow effective "blocks" of codes for functions, to allow coherent numbering and easy incrementing in case two developers collide. e.g. if the last two digits are internal to a function, u16 would allow for 654 functions to each have 99 internal errors (or depending on average function size, ten per function may be enough on average).
- alt phrasing: allow 10 or 100 error-codes per function to manage allocations and changes to codes

*/

/*

# Best Narrow

42
```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:13:55
------------------------------------------------------------
  Accuracy (@ V > 0): 84.68%
  Best-F1 Threshold:  V > 0
  Precision:          0.8497
  Recall:             0.8427
  F1-Score:           0.8462
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4216        739
Actual Pos (1)    780         4179
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 0/300 (0 vacuous, 0 specialized)  p25 21.3%  median 24.0%  p75 26.8%
  includes/clause: min 1  p25 33  median 38  p75 44  max 62  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1519 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb

```

128
```bash
$  cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt

    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:21:30
------------------------------------------------------------
  Accuracy (@ V > 0): 85.49%
  Best-F1 Threshold:  V > -1
  Precision:          0.8606
  Recall:             0.8485
  F1-Score:           0.8545
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4244        685
Actual Pos (1)    755         4230
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 0/300 (0 vacuous, 0 specialized)  p25 23.4%  median 25.7%  p75 28.1%
  includes/clause: min 1  p25 32  median 37  p75 42  max 58  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1439 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb

```

---

# Best Wide
- Type Wide

```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 105, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:30:00
------------------------------------------------------------
  Accuracy (@ V > 0): 84.95%
  Best-F1 Threshold:  V > 0
  Precision:          0.8408
  Recall:             0.8625
  F1-Score:           0.8515
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4145        810
Actual Pos (1)    682         4277
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 18.3%  median 20.5%  p75 22.7%
  includes/clause: min 1  p25 55  median 63  p75 71  max 97  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1492 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb

```

128:
```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 105, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:08:33
------------------------------------------------------------
  Accuracy (@ V > 0): 85.14%
  Best-F1 Threshold:  V > -1
  Precision:          0.8597
  Recall:             0.8383
  F1-Score:           0.8489
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4247        682
Actual Pos (1)    806         4179
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 19.9%  median 22.1%  p75 24.5%
  includes/clause: min 1  p25 52  median 59  p75 65  max 92  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1473 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb

```

*/

#![forbid(unsafe_code)]

// ===========================================================================
// SECTION 1: Error-Code System
// ===========================================================================

/// Crate-wide fieldless error-code enum. 2 bytes, `Copy`, no heap, no PII.
///
/// ## Code-block allocation (append-only; never renumber or reuse):
/// - 100–199  Preprocess pipeline (`Pp*`)
/// - 200–299  RNG (`Rng*`)
/// - 300–399  Enforced custom types / configuration (`Cfg*`)
/// - 400–499  ByteConvTM engine (`Bct*`)
/// - 500–599  Artifact serialization / I/O (`Art*`)
/// - 600–699  Probability output & reporting (`Prb*`)
///
/// Variant naming: `AcronymFunctionCondition`.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GranmoModelError {
    // --- 100–199: Preprocess ---
    /// A `PreprocessProfile` bit pattern uses reserved (undefined) stage bits.
    PpProfileReservedBitsSet = 100,
    /// Profile revalidation on `.get()` found reserved bits set
    /// (post-construction corruption or construction-bypass).
    PpProfileRecheckCorrupt = 101,

    // --- 200–299: RNG ---
    /// `gen_index` was called with an empty range (`len == 0`).
    RngGenIndexEmptyRange = 200,

    // --- 300–399: Enforced custom types / configuration ---
    /// `PatchSize` outside the supported bounds at construction.
    CfgPatchSizeOutOfBounds = 300,
    /// `PatchSize` revalidation on `.get()` failed (corruption).
    CfgPatchSizeRecheckCorrupt = 301,
    /// `StrideLen` outside the supported bounds at construction.
    CfgStrideOutOfBounds = 302,
    /// `StrideLen` revalidation on `.get()` failed (corruption).
    CfgStrideRecheckCorrupt = 303,
    /// `ClauseCount` not even, zero, or above the supported maximum.
    CfgClauseCountInvalid = 304,
    /// `ClauseCount` revalidation on `.get()` failed (corruption).
    CfgClauseCountRecheckCorrupt = 305,
    /// `StatesPerAction` outside supported bounds at construction.
    CfgStatesPerActionOutOfBounds = 306,
    /// `StatesPerAction` revalidation on `.get()` failed (corruption).
    CfgStatesPerActionRecheckCorrupt = 307,
    /// Specificity `s` was not strictly greater than 1.0, or was not finite.
    CfgSpecificityOutOfBounds = 308,
    /// `SpecificityThresholds` revalidation on `.get()` failed (corruption).
    CfgSpecificityRecheckCorrupt = 309,
    /// `VoteThreshold` (T) outside supported bounds at construction.
    CfgVoteThresholdOutOfBounds = 310,
    /// `VoteThreshold` revalidation on `.get()` failed (corruption).
    CfgVoteThresholdRecheckCorrupt = 311,
    /// `MaxScanBytes` outside supported bounds at construction.
    CfgMaxScanBytesOutOfBounds = 312,
    /// `MaxScanBytes` revalidation on `.get()` failed (corruption).
    CfgMaxScanBytesRecheckCorrupt = 313,
    /// Raw threshold pair given to `SpecificityThresholds::from_raw` does
    /// not satisfy the fixed-point invariant (sum == 65536, forget >= 1).
    /// Occurs when an artifact carries tampered/corrupt threshold fields.
    CfgSpecificityRawThresholdsInvalid = 314,
    /// `NgramLength` outside the supported bounds at construction.
    CfgNgramLengthOutOfBounds = 315,
    /// `NgramLength` revalidation on `.get()` failed (corruption).
    CfgNgramLengthRecheckCorrupt = 316,
    /// `VocabSize` outside the supported bounds at construction.
    CfgVocabSizeOutOfBounds = 317,
    /// `VocabSize` revalidation on `.get()` failed (corruption).
    CfgVocabSizeRecheckCorrupt = 318,
    /// `FireGuardStreakLimit` value outside supported bounds at
    /// construction: must be 0 (guard disabled) or within the active
    /// range. Sub-minimum active values (1..=15) are rejected as
    /// almost-certainly-typos — a limit of 1 would recycle every clause
    /// on its first specialized fire.
    CfgFireGuardLimitOutOfBounds = 319,
    /// `FireGuardStreakLimit` revalidation on `.get()` failed
    /// (post-construction corruption).
    CfgFireGuardLimitRecheckCorrupt = 320,
    // --- 400–499: ByteConvTM engine ---
    /// Stride exceeds patch size at engine construction (windows would skip
    /// bytes entirely — no valid configuration does this).
    BctStrideExceedsPatchSize = 400,
    /// An internal index (clause, literal, mask, or count) fell outside its
    /// backing storage. Internal-invariant violation: unreachable unless the
    /// model state is corrupt. One shared code for all such sites by design.
    BctIndexOutOfRange = 401,
    /// A stored allowed-bytes mask disagrees with recomputation from raw
    /// automaton states (`validate_internal_consistency`).
    BctMaskCacheInconsistent = 402,
    /// A cached positive-include count disagrees with recomputation from raw
    /// automaton states (`validate_internal_consistency`).
    BctCountCacheInconsistent = 403,
    /// An automaton state lies outside the legal band `[1, 2N]`.
    BctStateValueOutOfRange = 404,
    /// A checked arithmetic operation in the engine overflowed (allocation
    /// sizing, vote accumulation, or state stepping).
    BctArithmeticOverflow = 405,

    // --- 500–599: Artifact serialization / I/O ---
    /// Artifact path was not absolute (crate-wide absolute-path policy:
    /// reproducibility logs and error reports must be unambiguous).
    ArtPathNotAbsolute = 500,
    /// Creating/writing the artifact file failed (filesystem error dropped
    /// per no-PII policy). RETRYABLE: may be a transient filesystem state.
    ArtFileWriteFailed = 501,
    /// Opening/reading the artifact file failed (filesystem error dropped
    /// per no-PII policy). RETRYABLE: may be a transient filesystem state.
    ArtFileReadFailed = 502,
    /// File does not begin with the artifact magic bytes — not one of ours.
    ArtMagicMismatch = 503,
    /// Artifact format version is newer/older than this build supports.
    ArtVersionUnsupported = 504,
    /// File ended before the declared payload was fully read.
    ArtTruncated = 505,
    /// FNV-1a checksum over the payload does not match the stored checksum:
    /// bit-rot, truncation-with-padding, or tampering.
    ArtChecksumMismatch = 506,
    /// Artifact kind byte names a kind this build cannot load (e.g. a
    /// compact-inference artifact loaded by a trainer expecting full states).
    ArtKindUnsupported = 507,
    /// The stored automaton-state count disagrees with the count implied by
    /// the stored configuration header — internally inconsistent artifact.
    ArtStateCountMismatch = 508,
    /// A fixed-domain header field carries an out-of-domain value (a flag
    /// byte that is neither 0 nor 1, or a nonzero reserved byte). Distinct
    /// from `Cfg*` bound failures: this is artifact FORMAT domain
    /// validation, not configuration-range validation. (The v1 loader
    /// overloaded `ArtKindUnsupported` at the guard-flag site; v2 separates
    /// the conditions so diagnostics name the true cause.)
    ArtHeaderFieldInvalid = 509,
    // --- 600–699: Probability output & reporting ---
    /// Probability LUT construction produced an unusable table size.
    PrbLutSizeInvalid = 600,
    /// A vote sum passed to the LUT lies outside `[-C/2, +C/2]` — the model
    /// and LUT disagree about clause count (mismatched pairing).
    PrbVoteOutOfRange = 601,
    /// LUT revalidation failed (non-monotone or wrong endpoints): corruption
    /// after construction.
    PrbLutRecheckCorrupt = 602,
    /// Threshold sweep called with empty inputs.
    PrbSweepEmptyInput = 603,
    /// Threshold sweep called with votes/labels of different lengths.
    PrbSweepLengthMismatch = 604,
    // --- 700–799: Dataset ingestion ---
    /// Dataset path was not absolute (crate-wide absolute-path policy).
    DsPathNotAbsolute = 700,
    /// Reading the dataset file failed (filesystem detail dropped per
    /// no-PII policy). RETRYABLE: may be a transient filesystem state.
    DsFileReadFailed = 701,

    /// After parsing and filtering, zero usable labeled records remained.
    DsNoUsableRecords = 705,
    /// Train percentage outside 1..=99.
    DsSplitRatioInvalid = 706,
    /// The requested split left the train or test side empty.
    DsSplitEmptySide = 707,
    /// A JSONL line, after leading whitespace, did not begin a JSON object.
    /// The file is not line-delimited JSON objects — fail fast, not skip:
    /// the operator has pointed the harness at the wrong kind of file.
    DsJsonLineNotObject = 708,
    /// A JSON string ran to end-of-line without a closing quote. Raw
    /// newlines are illegal inside JSON strings, so line-splitting first is
    /// always safe: this code means the LINE is malformed, not the splitter.
    DsJsonUnterminatedString = 709,
    /// A backslash escape other than the JSON-defined set
    /// (\" \\ \/ \b \f \n \r \t \uXXXX) was found.
    DsJsonBadEscape = 710,
    /// A \uXXXX escape was malformed: short/non-hex digits, a lone or
    /// out-of-order surrogate, or an invalid resulting code point. Social
    /// media exports escape emoji as surrogate PAIRS (e.g. \uD83D\uDE00);
    /// this code fires when a pair is broken, never on a valid one.
    DsJsonBadUnicodeEscape = 711,
    /// Structural JSON error on a line: missing colon/comma, unbalanced
    /// nesting while skipping an unknown field, an empty/unrecognizable
    /// value token, or trailing content after the closing brace.
    DsJsonMalformedStructure = 712,
    /// `--folds` value outside the supported range `KFOLD_MIN..=KFOLD_MAX`.
    DsFoldCountInvalid = 713,
    /// Fewer usable documents than folds, or fold-boundary arithmetic
    /// produced an inconsistent partition (unreachable unless wiring is
    /// corrupt). One shared code for both by design: the operator-facing
    /// remedy is the same (more data or fewer folds).
    DsFoldGeometryFault = 714,

    // --- 800–899: CLI / harness ---
    /// An unrecognized flag was passed (fail-fast: never silently ignored).
    CliUnknownFlag = 800,
    /// A flag that requires a value was the last argument.
    CliFlagMissingValue = 801,
    /// A flag value failed to parse as its required type/range.
    CliInvalidValue = 802,
    /// A flag required by the selected mode was absent.
    CliMissingRequiredFlag = 803,
    /// `--mode` value was not one of train/predict/batch/batch-guard/row-audit.
    CliUnknownMode = 804,
    /// `--preset` value was not one of raw/p0/p1/p2/p3/p4/p5.
    CliUnknownPreset = 805,
    /// `--engine` value was not one of byte-conv/byte-bag.
    CliUnknownEngine = 806,
    /// Internal invariant fault in the harness fire-rate/vote fold over
    /// fired-clause bitsets (a bitset word or per-clause counter fell
    /// outside its storage, or a counter overflowed). Unreachable unless
    /// report wiring is corrupt; one shared code for these harness-internal
    /// sites by design.
    CliFireRateReportInternalFault = 807,
    /// The misprediction-log path, after default resolution, was not
    /// absolute (crate-wide absolute-path policy: log locations must be
    /// unambiguous for reproducibility). Occurs only when an operator
    /// supplies a relative `--log-out` AND the fallback resolution against
    /// the executable directory also fails.
    CliLogPathNotAbsolute = 808,
    /// Creating the misprediction-log parent directory failed (filesystem
    /// detail dropped per no-PII policy). RETRYABLE: may be transient.
    CliLogDirCreateFailed = 809,
    /// Opening/appending the misprediction-log file failed (filesystem
    /// detail dropped per no-PII policy). RETRYABLE: may be transient.
    CliLogWriteFailed = 810,
    /// The per-row prediction-record path was not absolute (crate-wide
    /// absolute-path policy).
    CliPredictionRecordPathNotAbsolute = 811,
    /// Creating/appending the prediction-record file failed (filesystem
    /// detail dropped per no-PII policy). RETRYABLE: may be transient.
    CliPredictionRecordWriteFailed = 812,
    /// Reading the prediction-record file for `row-audit` failed
    /// (filesystem detail dropped per no-PII policy). RETRYABLE.
    CliPredictionRecordReadFailed = 813,
    /// A prediction-record line was structurally malformed, or two records
    /// for the same line index carried different labels (records from
    /// different datasets were mixed). Fail-fast: an audit over mixed
    /// inputs would silently misattribute every statistic.
    CliPredictionRecordMalformed = 814,

    // --- 900–999: ByteBag baseline (`Bbg*`) — flat bag-of-byte-n-grams,
    //     the scientific control for "does convolution earn its complexity"
    //     (§8 success criteria). ---
    /// Vocabulary construction found zero shingles (empty training corpus).
    BbgVocabEmptyCorpus = 900,
    /// Checked arithmetic overflowed in ByteBag code (counting, sizing).
    BbgArithmeticOverflow = 901,
    /// A vocabulary rank/byte index fell outside its backing storage, or a
    /// lookup was called with a wrong-length shingle. Internal-invariant
    /// violation: unreachable unless state is corrupt or the caller is
    /// mis-wired. One shared code for all such sites by design.
    BbgVocabIndexOutOfRange = 902,
    /// Vocabulary revalidation failed (flat-storage length mismatch, or
    /// the sorted-lookup order is not a strict permutation): corruption
    /// after construction/load.
    BbgVocabRecheckCorrupt = 903,

    /// An internal ByteBagTM engine index (state slot, mask word, or
    /// literal) fell outside its backing storage. Internal-invariant
    /// violation: unreachable unless engine state is corrupt or a caller
    /// is mis-wired. One shared code for all such sites by design
    /// (mirrors 401 for the conv engine and 902 for the vocabulary).
    BbgEngineIndexOutOfRange = 904,
    /// Checked arithmetic overflowed in ByteBagTM engine code (allocation
    /// sizing, vote accumulation, or state stepping). Mirrors 405.
    BbgEngineArithmeticOverflow = 905,
    /// A ByteBagTM automaton state lies outside the legal band [1, 2N].
    /// Mirrors 404.
    BbgStateValueOutOfRange = 906,
    /// A stored ByteBagTM include mask (positive or negated) disagrees
    /// with recomputation from raw automaton states
    /// (`bag_validate_internal_consistency`). Mirrors 402.
    BbgIncludeMaskInconsistent = 907,
    /// A fire-guard streak or reset-count storage index fell outside its
    /// backing storage, or guard storage geometry disagrees with the
    /// clause count. Internal-invariant violation: unreachable unless
    /// engine state is corrupt. Mirrors 904 for the engine proper.
    BbgFireGuardIndexOutOfRange = 908,

    // --- 1000–1099: Shared engine training math (`Trn*`) — helpers used
    //     identically by every engine (feedback gates, etc.). ---
    /// Checked arithmetic overflowed while computing the integer feedback
    /// gates. Mathematically unreachable within `VoteThreshold` bounds
    /// (T <= 10000 bounds every intermediate at <= 20000); present as
    /// defence-in-depth per the checked-arithmetic rule.
    TrnGateArithmeticOverflow = 1000,

    // --- 1100–1199: Parallel execution (`Par*`) — worker-count validation
    //     and scoped-thread coordination faults. Parallelism in this crate
    //     is PERFORMANCE-ONLY: results are byte-identical at every worker
    //     count (per-clause derived RNG streams), so every code here is a
    //     wiring/invariant fault, never a semantic difference. ---
    /// `WorkerCount` outside supported bounds at construction.
    ParWorkerCountOutOfBounds = 1100,
    /// `WorkerCount` revalidation on `.get()` failed (corruption).
    ParWorkerCountRecheckCorrupt = 1101,
    /// A scoped worker thread's join returned the panic arm. Production
    /// code never panics, so this is internal-invariant detection: the
    /// step is reported failed and never retried (repeating an identical
    /// call cannot help).
    ParWorkerJoinFailed = 1102,
    /// Parallel evaluation's ordered merge found chunk geometry
    /// inconsistent with input lengths. Unreachable unless wiring is
    /// corrupt; one shared code for these merge sites by design.
    ParEvalMergeGeometryFault = 1103,
    /// Per-clause mutable view construction produced a slice whose length
    /// disagrees with the engine's declared geometry (states, masks, or
    /// counts). Unreachable unless engine storage is corrupt.
    ParClauseViewGeometryFault = 1104,

    // --- 1200–1299: Hybrid co-training engine (`Hyb*`) and late-fusion
    //     ensemble evaluation (`Ens*`). ---
    /// Checked arithmetic overflowed in HybridTM code (combined vote
    /// accumulation, combined fired-bitset sizing, clause-index offsetting).
    /// Mathematically unreachable within `ClauseCount` bounds; present per
    /// the checked-arithmetic rule.
    HybArithmeticOverflow = 1200,
    /// An internal HybridTM index (combined-bitset word, sub-bank clause
    /// offset) fell outside its backing storage. Internal-invariant
    /// violation; one shared code for such sites by design (mirrors
    /// 401 / 904).
    HybIndexOutOfRange = 1201,
    /// Late-fusion inputs disagree in LENGTH (vote vectors, label vector,
    /// raw/prepared test slices): the two reports were not produced on the
    /// same test split, or report wiring is corrupt.
    EnsVoteVectorMismatch = 1202,
    /// HybridTM sub-bank geometry is unusable: the conv bank's clause count
    /// is odd, which would break the parity-polarity rule over the
    /// concatenated clause index space (conv first, bag at offset
    /// conv_count) that the harness's single-pass evaluation relies on.
    HybClauseGeometryFault = 1203,
    /// Late-fusion inputs agree in length but the two reports' ground-truth
    /// label vectors DIFFER: the runs were evaluated on different test
    /// splits or orderings, so their votes cannot be summed
    /// document-for-document.
    EnsLabelVectorMismatch = 1204,
}

impl GranmoModelError {
    /// Retryability, defined per individual code (never by numeric range).
    ///
    /// Tier-1 micro-retry applies ONLY to filesystem read/write codes: those
    /// may fail transiently and legitimately succeed on a bounded retry with
    /// backoff. Every other code is either a caller-input error (fix the
    /// input) or a corruption/invariant detection (Tier-2 fallback: reject
    /// the model/file or abort the step — repeating the identical call
    /// cannot help).
    pub fn is_retryable(&self) -> bool {
        match self {
            // True Arm
            Self::ArtFileWriteFailed
            | Self::CliPredictionRecordWriteFailed
            | Self::CliPredictionRecordReadFailed
            | Self::ArtFileReadFailed
            | Self::DsFileReadFailed
            | Self::CliLogDirCreateFailed
            | Self::CliLogWriteFailed => true,

            // False Arm
            Self::PpProfileReservedBitsSet
            | Self::DsFoldCountInvalid
            | Self::DsFoldGeometryFault
            | Self::CliPredictionRecordPathNotAbsolute
            | Self::CliPredictionRecordMalformed
            | Self::HybArithmeticOverflow
            | Self::HybIndexOutOfRange
            | Self::EnsVoteVectorMismatch
            | Self::HybClauseGeometryFault
            | Self::EnsLabelVectorMismatch
            | Self::CfgFireGuardLimitOutOfBounds
            | Self::CfgFireGuardLimitRecheckCorrupt
            | Self::BbgFireGuardIndexOutOfRange
            | Self::CliLogPathNotAbsolute
            | Self::PpProfileRecheckCorrupt
            | Self::ParWorkerCountOutOfBounds
            | Self::ParWorkerCountRecheckCorrupt
            | Self::ParWorkerJoinFailed
            | Self::ParEvalMergeGeometryFault
            | Self::ParClauseViewGeometryFault
            | Self::RngGenIndexEmptyRange
            | Self::CfgPatchSizeOutOfBounds
            | Self::CfgPatchSizeRecheckCorrupt
            | Self::CfgStrideOutOfBounds
            | Self::ArtHeaderFieldInvalid
            | Self::CfgStrideRecheckCorrupt
            | Self::CfgClauseCountInvalid
            | Self::CfgClauseCountRecheckCorrupt
            | Self::CfgStatesPerActionOutOfBounds
            | Self::CfgStatesPerActionRecheckCorrupt
            | Self::CfgSpecificityOutOfBounds
            | Self::CfgSpecificityRecheckCorrupt
            | Self::CfgVoteThresholdOutOfBounds
            | Self::CfgVoteThresholdRecheckCorrupt
            | Self::CfgMaxScanBytesOutOfBounds
            | Self::CfgMaxScanBytesRecheckCorrupt
            | Self::CfgSpecificityRawThresholdsInvalid
            | Self::BctStrideExceedsPatchSize
            | Self::BctIndexOutOfRange
            | Self::BctMaskCacheInconsistent
            | Self::BctCountCacheInconsistent
            | Self::BctStateValueOutOfRange
            | Self::BctArithmeticOverflow
            | Self::ArtPathNotAbsolute
            | Self::ArtMagicMismatch
            | Self::ArtVersionUnsupported
            | Self::ArtTruncated
            | Self::ArtChecksumMismatch
            | Self::ArtKindUnsupported
            | Self::ArtStateCountMismatch
            | Self::PrbLutSizeInvalid
            | Self::PrbVoteOutOfRange
            | Self::PrbLutRecheckCorrupt
            | Self::PrbSweepEmptyInput
            | Self::PrbSweepLengthMismatch
            | Self::DsPathNotAbsolute
            | Self::DsNoUsableRecords
            | Self::DsSplitRatioInvalid
            | Self::DsSplitEmptySide
            | Self::CliUnknownFlag
            | Self::CliFlagMissingValue
            | Self::CliInvalidValue
            | Self::CliMissingRequiredFlag
            | Self::CliUnknownMode
            | Self::DsJsonLineNotObject
            | Self::DsJsonUnterminatedString
            | Self::DsJsonBadEscape
            | Self::DsJsonBadUnicodeEscape
            | Self::DsJsonMalformedStructure
            | Self::CfgNgramLengthOutOfBounds
            | Self::CfgNgramLengthRecheckCorrupt
            | Self::CfgVocabSizeOutOfBounds
            | Self::CfgVocabSizeRecheckCorrupt
            | Self::BbgVocabEmptyCorpus
            | Self::BbgArithmeticOverflow
            | Self::BbgVocabIndexOutOfRange
            | Self::BbgVocabRecheckCorrupt
            | Self::TrnGateArithmeticOverflow
            | Self::BbgEngineIndexOutOfRange
            | Self::BbgEngineArithmeticOverflow
            | Self::BbgStateValueOutOfRange
            | Self::BbgIncludeMaskInconsistent
            | Self::CliUnknownEngine
            | Self::CliFireRateReportInternalFault
            | Self::CliUnknownPreset => false,
        }
    }

    /// Numeric code accessor for exit statuses / logging endpoints.
    pub fn code(&self) -> u16 {
        *self as u16
    }
}

/// Human-readable rendering: debug and test builds ONLY (production builds
/// carry codes, never strings — no heap, no PII).
#[cfg(any(debug_assertions, test))]
impl core::fmt::Display for GranmoModelError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GranmoModelError code {}: {:?}", self.code(), self)
    }
}

// ===========================================================================
// SECTION 2: Deterministic Integer-Only RNG
// ===========================================================================

/// Deterministic xorshift64 PRNG. Integer-only draw surface: the training
/// hot path compares raw `u16` draws against precomputed `u16` thresholds
/// (see `SpecificityThresholds`), and selects sampled windows with
/// `gen_index`. There is deliberately NO float-producing method: the compute
/// policy for the new engine is integer-only hot paths.
#[derive(Debug, Clone)]
pub struct FastRng {
    state: u64,
}

impl FastRng {
    /// Constructs from an explicit seed. Zero is remapped to a fixed non-zero
    /// constant because xorshift64 has an absorbing state at zero.
    pub fn seed(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x853c_49e6_748f_ea9b
            } else {
                seed
            },
        }
    }

    /// Advances the state; returns the next raw 64-bit value.
    #[inline(always)]
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Uniform `u16` draw (top 16 bits of the raw draw — xorshift's weakest
    /// bits are the low ones, so the high bits are taken).
    #[inline(always)]
    pub fn next_u16(&mut self) -> u16 {
        (self.next_u64() >> 48) as u16
    }

    /// Stochastic coin: true with probability `threshold / 65536`.
    /// This is the per-literal feedback decision primitive (compare one draw
    /// against a threshold precomputed at config time — no division, no
    /// floats at runtime).
    #[inline(always)]
    pub fn coin(&mut self, threshold_u16: u16) -> bool {
        self.next_u16() < threshold_u16
    }

    /// Uniform index draw in `[0, len)` via widening multiply
    /// (`(u128(draw) * len) >> 64`).
    ///
    /// Bias statement (documented per the coding rules): the widening-multiply
    /// method has bias bounded by `len / 2^64`. For all uses in this crate
    /// (window sampling, shuffles; `len` < 2^32) the bias is below 2^-32 per
    /// draw, which is accepted rather than paying for a rejection loop with
    /// an unbounded worst case.
    #[inline(always)]
    pub fn gen_index(&mut self, len: usize) -> Result<usize, GranmoModelError> {
        if len == 0 {
            return Err(GranmoModelError::RngGenIndexEmptyRange);
        }
        let draw = self.next_u64();
        let idx = ((u128::from(draw) * (len as u128)) >> 64) as usize;
        Ok(idx)
    }
}

// ---------------------------------------------------------------------------
// Per-clause derived RNG streams (locked decision, Drop 3.x)
// ---------------------------------------------------------------------------
//
// CONTRACT OF RECORD. Training randomness is indexed by WHAT IT IS FOR —
// the triple (training step, clause, purpose) — not by WHEN IT IS DRAWN.
// The master `FastRng` still drives per-epoch order shuffling and draws
// exactly ONE `next_u64()` per training step to produce that step's seed;
// each clause then runs its own `FastRng` seeded from that step seed.
//
// Consequences (all deliberate):
// - A run is reproducible from (dataset, split seed, run config, epochs)
//   and NOTHING else — not the worker count, not the thread schedule, not
//   the machine's core count.
// - `--workers 1` and `--workers 32` produce byte-identical artifacts, so
//   the single-threaded path is a valid oracle for the parallel path.
// - This REPLACES the pre-parallel contract (one stream consumed in clause
//   order). Trajectories from before this change are not reproducible by
//   this build; artifacts are regenerable experiment outputs, so there is
//   no migration path by design.
//
// The two purpose tags keep the window-scan stream and the feedback stream
// independent, so the two training passes may use different clause→worker
// assignments with no coupling between them.

/// Purpose tag: pass-1 reservoir window sampling.
const RNG_PURPOSE_WINDOW_SCAN: u64 = 0x5BF0_3635_1C31_9A7D;
/// Purpose tag: pass-2 gate draw and feedback coin draws.
const RNG_PURPOSE_CLAUSE_FEEDBACK: u64 = 0x2E1B_9C44_A70F_D318;

/// Purpose tag: derives the HybridTM bag bank's per-step seed from the
/// master step seed. Without this, bag clause c and conv clause c would
/// derive IDENTICAL streams from (step_seed, c, purpose) and be gated
/// in/out together every step — a silent coupling between the two banks
/// that would invalidate the shared-resource-allocation experiment.
/// The conv bank uses the master step seed unmodified (bank ordinal 0).
const RNG_PURPOSE_HYBRID_BAG_BANK: u64 = 0x7A4C_E921_5D08_B3F6;
/// Bank ordinal mixed into the hybrid bag-bank seed derivation.
const HYBRID_BAG_BANK_ORDINAL: u64 = 1;

/// Odd multiplier decorrelating adjacent clause indices before mixing.
const CLAUSE_INDEX_SCATTER_MULTIPLIER: u64 = 0xD139_5F14_7EF7_2F19;

/// Derives an independent stream seed for one (step, clause, purpose)
/// triple using the SplitMix64 finalizer.
///
/// `wrapping_mul`/`wrapping_add` are the DEFINED behavior of this mixer
/// (modular arithmetic), not unchecked arithmetic: wrapping is explicit,
/// intentional, and cannot panic — the same posture as `fnv1a_64`.
/// `FastRng::seed` remaps a zero result, so the xorshift absorbing state
/// is unreachable here.
#[inline(always)]
fn derive_clause_stream_seed(step_seed: u64, clause_index: u64, purpose_tag: u64) -> u64 {
    let mut mixed =
        step_seed ^ clause_index.wrapping_mul(CLAUSE_INDEX_SCATTER_MULTIPLIER) ^ purpose_tag;
    mixed ^= mixed >> 30;
    mixed = mixed.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    mixed ^= mixed >> 27;
    mixed = mixed.wrapping_mul(0x94D0_49BB_1331_11EB);
    mixed ^= mixed >> 31;
    mixed
}

// ===========================================================================
// SECTION 3: Enforced Custom Types (bounded config values)
// ===========================================================================
//
// Pattern (per the Enforced-Custom-Types rules): pub struct, PRIVATE inner
// field, bounds enforced in the constructor, bounds RE-verified in `.get()`
// so post-construction corruption is caught at the point of use and returned
// as an error code rather than silently propagating a bad value.

/// Shingle (byte n-gram) length for the ByteBag baseline. Bounds match
/// `PatchSize` (2..=16) deliberately: the default of record is n = K = 5 so
/// the bag and the convolutional model see the same maximal pattern length,
/// making the §8 comparison a single-variable experiment. A DISTINCT type
/// from `PatchSize` (not a reuse): the two values are independent knobs and
/// must never be silently interchangeable in a signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NgramLength {
    value: u8,
}

impl NgramLength {
    pub const MIN: u8 = 2;
    pub const MAX: u8 = 16;

    pub fn new(value: u8) -> Result<Self, GranmoModelError> {
        if value < Self::MIN || value > Self::MAX {
            #[cfg(debug_assertions)]
            eprintln!(
                "CFG-315: NgramLength {} outside {}..={}",
                value,
                Self::MIN,
                Self::MAX
            );
            return Err(GranmoModelError::CfgNgramLengthOutOfBounds);
        }
        Ok(Self { value })
    }

    /// Revalidating accessor (catches post-construction corruption).
    pub fn get(&self) -> Result<u8, GranmoModelError> {
        if self.value < Self::MIN || self.value > Self::MAX {
            return Err(GranmoModelError::CfgNgramLengthRecheckCorrupt);
        }
        Ok(self.value)
    }
}

/// Maximum vocabulary size M for the ByteBag baseline. Default of record:
/// 4000 — matching Phase 1's word-vocabulary scale, which keeps the
/// literal-space comparison auditable (bag: 2×M = 8,000 literals/clause vs.
/// conv: 2×K×256 = 2,560 at K=5). Upper bound keeps rank indices safely
/// inside u16 for the artifact format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VocabSize {
    value: u16,
}

impl VocabSize {
    pub const MIN: u16 = 2;
    pub const MAX: u16 = 65000;

    pub fn new(value: u16) -> Result<Self, GranmoModelError> {
        if value < Self::MIN || value > Self::MAX {
            #[cfg(debug_assertions)]
            eprintln!(
                "CFG-317: VocabSize {} outside {}..={}",
                value,
                Self::MIN,
                Self::MAX
            );
            return Err(GranmoModelError::CfgVocabSizeOutOfBounds);
        }
        Ok(Self { value })
    }

    pub fn get(&self) -> Result<u16, GranmoModelError> {
        if self.value < Self::MIN || self.value > Self::MAX {
            return Err(GranmoModelError::CfgVocabSizeRecheckCorrupt);
        }
        Ok(self.value)
    }
}

/// Consecutive-fire streak limit for the training-time always-fire guard
/// (ByteBag engine, Drop 4.1; conv port is recorded backlog).
///
/// ## Semantics
/// - `0` = guard DISABLED — the recorded-baseline default. The guard is
///   an ablation arm (like the conv engine's GuardedInclude flag): it is
///   never silently on, so every recorded baseline row stays comparable
///   across sessions.
/// - An active value `L` means: a clause that fires on `L` CONSECUTIVE
///   shuffled training documents, while non-vacuous (holding at least one
///   included literal), is judged pathologically always-firing and is
///   reset to fresh state (prune-and-respawn). Because the harness
///   re-shuffles training order every epoch, `L` consecutive fires is a
///   representative corpus sample.
///
/// ## Why a minimum active value exists
/// A tiny limit (e.g. 1) would reset every clause the moment it first
/// fires with a learned pattern — catastrophic, and almost certainly a
/// typo rather than an experiment. The floor forces the guard to observe
/// at least a modest consecutive sample before condemning a clause.
///
/// ## Ephemerality (design record)
/// This value and the per-clause streak state it governs are TRAINING
/// SESSION state: deliberately NOT persisted in artifact kind 3. Loaded
/// models resume training guard-off unless reconstructed with a limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FireGuardStreakLimit {
    value: u32,
}

impl FireGuardStreakLimit {
    /// Sentinel: guard disabled (the default of record).
    pub const DISABLED: u32 = 0;
    /// Smallest permitted ACTIVE limit (see doc block for rationale).
    pub const MIN_ACTIVE: u32 = 16;
    /// Largest permitted ACTIVE limit (sanity cap: beyond any real
    /// train-split size this crate targets; larger values are typos).
    pub const MAX_ACTIVE: u32 = 16_777_216;

    pub fn new(value: u32) -> Result<Self, GranmoModelError> {
        if value != Self::DISABLED && (value < Self::MIN_ACTIVE || value > Self::MAX_ACTIVE) {
            #[cfg(debug_assertions)]
            eprintln!(
                "CFG-319: FireGuardStreakLimit {} must be 0 (disabled) or in {}..={}",
                value,
                Self::MIN_ACTIVE,
                Self::MAX_ACTIVE
            );
            return Err(GranmoModelError::CfgFireGuardLimitOutOfBounds);
        }
        Ok(Self { value })
    }

    /// Revalidating accessor (catches post-construction corruption).
    pub fn get(&self) -> Result<u32, GranmoModelError> {
        if self.value != Self::DISABLED
            && (self.value < Self::MIN_ACTIVE || self.value > Self::MAX_ACTIVE)
        {
            return Err(GranmoModelError::CfgFireGuardLimitRecheckCorrupt);
        }
        Ok(self.value)
    }
}

/// Fixed-point unit for stochastic thresholds: probabilities are expressed
/// as `x / 65536` and decided by comparing one `u16` draw against `x`.
const PROBABILITY_FIXED_POINT_UNIT: u32 = 1 << 16;

/// Resource-allocation vote target T. Training clamps the vote sum V to
/// ±T and gates feedback with integer comparisons: draw r in [0, 2T),
/// apply iff r < (T - V) (target-consistent) or r < (T + V) (otherwise).
/// Default of record: 50.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoteThreshold {
    value: i16,
}

impl VoteThreshold {
    pub const MIN: i16 = 1;
    pub const MAX: i16 = 10000;

    pub fn new(value: i16) -> Result<Self, GranmoModelError> {
        if value < Self::MIN || value > Self::MAX {
            #[cfg(debug_assertions)]
            eprintln!(
                "CFG-310: VoteThreshold {} outside {}..={}",
                value,
                Self::MIN,
                Self::MAX
            );
            return Err(GranmoModelError::CfgVoteThresholdOutOfBounds);
        }
        Ok(Self { value })
    }

    pub fn get(&self) -> Result<i16, GranmoModelError> {
        if self.value < Self::MIN || self.value > Self::MAX {
            return Err(GranmoModelError::CfgVoteThresholdRecheckCorrupt);
        }
        Ok(self.value)
    }
}

/// Scan-length cap in bytes (locked decision §10.10): bounds inference and
/// training cost per document; with OR-pooling each clause still contributes
/// at most one vote regardless of length, so the cap affects only scan cost
/// and pattern reach, never vote semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxScanBytes {
    value: u32,
}

impl MaxScanBytes {
    pub const MIN: u32 = 16;
    pub const MAX: u32 = 1_048_576;

    pub fn new(value: u32) -> Result<Self, GranmoModelError> {
        if value < Self::MIN || value > Self::MAX {
            #[cfg(debug_assertions)]
            eprintln!(
                "CFG-312: MaxScanBytes {} outside {}..={}",
                value,
                Self::MIN,
                Self::MAX
            );
            return Err(GranmoModelError::CfgMaxScanBytesOutOfBounds);
        }
        Ok(Self { value })
    }

    pub fn get(&self) -> Result<u32, GranmoModelError> {
        if self.value < Self::MIN || self.value > Self::MAX {
            return Err(GranmoModelError::CfgMaxScanBytesRecheckCorrupt);
        }
        Ok(self.value)
    }
}

/// Worker-thread count for parallel training and evaluation.
///
/// PERFORMANCE-ONLY KNOB (locked decision): results are byte-identical at
/// every value in range, because randomness is indexed by (training step,
/// clause, purpose) rather than by consumption order — see
/// `derive_clause_stream_seed`. `WorkerCount::new(1)` is therefore not a
/// "strict mode"; it is the same computation on one thread, which makes
/// the single-threaded path a valid ORACLE for the multi-threaded path.
///
/// Upper bound is a sanity cap, not a machine property: an absurd worker
/// count is a typo, and oversubscription only wastes context switches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkerCount {
    value: u16,
}

impl WorkerCount {
    pub const MIN: u16 = 1;
    pub const MAX: u16 = 1024;

    pub fn new(value: u16) -> Result<Self, GranmoModelError> {
        if value < Self::MIN || value > Self::MAX {
            #[cfg(debug_assertions)]
            eprintln!(
                "PAR-1100: WorkerCount {} outside {}..={}",
                value,
                Self::MIN,
                Self::MAX
            );
            return Err(GranmoModelError::ParWorkerCountOutOfBounds);
        }
        Ok(Self { value })
    }

    pub fn get(&self) -> Result<u16, GranmoModelError> {
        if self.value < Self::MIN || self.value > Self::MAX {
            return Err(GranmoModelError::ParWorkerCountRecheckCorrupt);
        }
        Ok(self.value)
    }

    /// Resolves the machine's available parallelism, clamped into bounds.
    /// Total by construction (a failed query falls back to 1), so it
    /// returns `Self` rather than `Result` — same posture as the
    /// `PreprocessProfile::preset_*` constructors.
    pub fn resolve_automatic() -> Self {
        let detected = std::thread::available_parallelism()
            .map(|count| count.get())
            .unwrap_or(1);
        let clamped = detected.clamp(usize::from(Self::MIN), usize::from(Self::MAX));
        Self {
            value: clamped as u16,
        }
    }
}

/// Effective worker count for a fork-join over `work_item_total` items:
/// never more workers than items, never fewer than one. Splitting the
/// clause bank into empty ranges would spawn threads that do nothing.
fn resolve_effective_worker_count(
    requested_workers: WorkerCount,
    work_item_total: usize,
) -> Result<usize, GranmoModelError> {
    let requested = usize::from(requested_workers.get()?);
    Ok(requested.min(work_item_total.max(1)).max(1))
}

/// Items per worker chunk, rounded up so `chunks_mut(chunk_size)` yields
/// at most `worker_total` chunks covering every item exactly once.
fn resolve_work_chunk_size(
    work_item_total: usize,
    worker_total: usize,
) -> Result<usize, GranmoModelError> {
    if worker_total == 0 {
        return Err(GranmoModelError::ParClauseViewGeometryFault);
    }
    work_item_total
        .checked_add(worker_total)
        .and_then(|sum| sum.checked_sub(1))
        .map(|sum| sum / worker_total)
        .map(|size| size.max(1))
        .ok_or(GranmoModelError::ParClauseViewGeometryFault)
}

/// Convolution window width K in bytes. Bounds: 2..=16 (K=1 degenerates to a
/// unigram byte model; K>16 exceeds the intended short-pattern regime and
/// the mask-array cost model). Default of record: 5 (§11.1 resolved).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PatchSize {
    value: u8,
}

impl PatchSize {
    pub const MIN: u8 = 2;
    pub const MAX: u8 = 16;

    pub fn new(value: u8) -> Result<Self, GranmoModelError> {
        if value < Self::MIN || value > Self::MAX {
            #[cfg(debug_assertions)]
            eprintln!(
                "CFG-300: PatchSize {} outside {}..={}",
                value,
                Self::MIN,
                Self::MAX
            );
            return Err(GranmoModelError::CfgPatchSizeOutOfBounds);
        }
        Ok(Self { value })
    }

    /// Revalidating accessor (catches post-construction corruption).
    pub fn get(&self) -> Result<u8, GranmoModelError> {
        if self.value < Self::MIN || self.value > Self::MAX {
            return Err(GranmoModelError::CfgPatchSizeRecheckCorrupt);
        }
        Ok(self.value)
    }
}

/// Window stride S in bytes. Bounds: 1..=8 and (checked at engine wiring
/// time) S <= K. Default of record: 2 (§11.2 resolved).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrideLen {
    value: u8,
}

impl StrideLen {
    pub const MIN: u8 = 1;
    pub const MAX: u8 = 8;

    pub fn new(value: u8) -> Result<Self, GranmoModelError> {
        if value < Self::MIN || value > Self::MAX {
            #[cfg(debug_assertions)]
            eprintln!(
                "CFG-302: StrideLen {} outside {}..={}",
                value,
                Self::MIN,
                Self::MAX
            );
            return Err(GranmoModelError::CfgStrideOutOfBounds);
        }
        Ok(Self { value })
    }

    pub fn get(&self) -> Result<u8, GranmoModelError> {
        if self.value < Self::MIN || self.value > Self::MAX {
            return Err(GranmoModelError::CfgStrideRecheckCorrupt);
        }
        Ok(self.value)
    }
}

/// Total clause count in the single binary-task bank. Must be even (even
/// indices = positive polarity, odd = negative) and nonzero. Default of
/// record: 200.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClauseCount {
    value: u16,
}

impl ClauseCount {
    pub const MAX: u16 = 65534;

    pub fn new(value: u16) -> Result<Self, GranmoModelError> {
        if value == 0 || value % 2 != 0 || value > Self::MAX {
            #[cfg(debug_assertions)]
            eprintln!(
                "CFG-304: ClauseCount {} must be even, nonzero, <= {}",
                value,
                Self::MAX
            );
            return Err(GranmoModelError::CfgClauseCountInvalid);
        }
        Ok(Self { value })
    }

    pub fn get(&self) -> Result<u16, GranmoModelError> {
        if self.value == 0 || self.value % 2 != 0 || self.value > Self::MAX {
            return Err(GranmoModelError::CfgClauseCountRecheckCorrupt);
        }
        Ok(self.value)
    }
}

/// Automaton depth N per action (P4 noise-filter tuning; see design notes).
/// States occupy `[1, 2N]` in i16; include iff state > N. Bounds keep `2N`
/// inside i16 with headroom. Kept per-value (not global) so mixed-depth
/// team ablations are a config, not a rewrite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatesPerAction {
    value: i16,
}

impl StatesPerAction {
    pub const MIN: i16 = 8;
    pub const MAX: i16 = 16000;

    pub fn new(value: i16) -> Result<Self, GranmoModelError> {
        if value < Self::MIN || value > Self::MAX {
            #[cfg(debug_assertions)]
            eprintln!(
                "CFG-306: StatesPerAction {} outside {}..={}",
                value,
                Self::MIN,
                Self::MAX
            );
            return Err(GranmoModelError::CfgStatesPerActionOutOfBounds);
        }
        Ok(Self { value })
    }

    pub fn get(&self) -> Result<i16, GranmoModelError> {
        if self.value < Self::MIN || self.value > Self::MAX {
            return Err(GranmoModelError::CfgStatesPerActionRecheckCorrupt);
        }
        Ok(self.value)
    }
}

/// Precomputed integer thresholds for the specificity parameter `s`.
///
/// `s` exists as a float ONLY here, at config time. The training hot path
/// performs `rng.coin(threshold)` — one u16 compare, no floats, no division:
/// - `forget_threshold`    ≈ round(65536 * (1/s))       — probability 1/s
/// - `reinforce_threshold` = 65536 - forget_threshold    — probability (s-1)/s
///
/// Invariant (revalidated on `.get()`): the two thresholds sum exactly to
/// the fixed-point unit, and `forget_threshold` is nonzero (s finite).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpecificityThresholds {
    forget_threshold: u16,
    reinforce_threshold: u16,
}

impl SpecificityThresholds {
    /// Reconstructs the threshold pair from raw stored values (artifact
    /// load path). Unlike `from_specificity`, this involves NO float math:
    /// the artifact round-trips the exact integers used in training, so a
    /// loaded model reproduces training-time stochastic behavior bit-for-bit
    /// under the same seed. The fixed-point invariant is enforced here
    /// exactly as `.get()` enforces it, so a tampered artifact is rejected
    /// at reconstruction, not at first use.
    pub fn from_raw_thresholds(
        forget_threshold: u16,
        reinforce_threshold: u16,
    ) -> Result<Self, GranmoModelError> {
        let sum = u32::from(forget_threshold).checked_add(u32::from(reinforce_threshold));
        match sum {
            Some(total) if total == PROBABILITY_FIXED_POINT_UNIT && forget_threshold >= 1 => {
                Ok(Self {
                    forget_threshold,
                    reinforce_threshold,
                })
            }
            _ => {
                #[cfg(debug_assertions)]
                eprintln!(
                    "CFG-314: raw thresholds ({}, {}) violate fixed-point invariant",
                    forget_threshold, reinforce_threshold
                );
                Err(GranmoModelError::CfgSpecificityRawThresholdsInvalid)
            }
        }
    }

    /// Builds thresholds from `s`. Requires `1.0 < s` and `s` finite and
    /// small enough that `1/s` rounds to a nonzero u16 (s < 65536).
    pub fn from_specificity(s: f64) -> Result<Self, GranmoModelError> {
        if !s.is_finite() || s <= 1.0 || s >= 65536.0 {
            #[cfg(debug_assertions)]
            eprintln!(
                "CFG-308: specificity {} must be finite and in (1.0, 65536.0)",
                s
            );
            return Err(GranmoModelError::CfgSpecificityOutOfBounds);
        }
        // Offline config-time float math (permitted): round to fixed point.
        let forget = ((PROBABILITY_FIXED_POINT_UNIT as f64) / s).round() as u32;
        // s in (1, 65536) guarantees forget in [1, 65535]; clamp defensively
        // without unchecked arithmetic.
        let forget_u16 = forget.clamp(1, u32::from(u16::MAX)) as u16;
        let reinforce_u16 =
            (PROBABILITY_FIXED_POINT_UNIT.saturating_sub(u32::from(forget_u16))) as u16;
        Ok(Self {
            forget_threshold: forget_u16,
            reinforce_threshold: reinforce_u16,
        })
    }

    /// Revalidating accessor: returns `(forget_threshold, reinforce_threshold)`.
    pub fn get(&self) -> Result<(u16, u16), GranmoModelError> {
        let sum = u32::from(self.forget_threshold).checked_add(u32::from(self.reinforce_threshold));
        match sum {
            Some(total) if total == PROBABILITY_FIXED_POINT_UNIT && self.forget_threshold >= 1 => {
                Ok((self.forget_threshold, self.reinforce_threshold))
            }
            _ => Err(GranmoModelError::CfgSpecificityRecheckCorrupt),
        }
    }
}

// ===========================================================================
// SECTION 4: M-Preprocess — Byte-Stream Adapter Pipeline (stages 1–8)
// ===========================================================================

/// Stage bit positions inside a `PreprocessProfile` bitmask. Stage order of
/// application is FIXED to the canonical catalogue order (1..=8) regardless
/// of construction order; the bitmask only selects which stages are enabled.
/// Stage 9 (Porter stemmer) is dropped from Phase 2 per resolved §11.3.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreprocessStage {
    /// 1: `\n`, `\r`, `\t` -> space.
    WhitespaceFold = 1 << 0,
    /// 2: collapse runs of spaces to a single space.
    SpaceDedupe = 1 << 1,
    /// 3: drop spaces before the first non-space byte.
    LeadingTrim = 1 << 2,
    /// 4: ASCII bytes 65–90 -> +32.
    AsciiLowercase = 1 << 3,
    /// 5: leet fold: `@->a  $->s  0->o  1->l  3->e  !->i`.
    LeetFold = 1 << 4,
    /// 6: remove ASCII digits 0x30–0x39.
    DigitStrip = 1 << 5,
    /// 7: remove ASCII punctuation (space excluded).
    SymbolStrip = 1 << 6,
    /// 8: remove all spaces (expected harmful; kept as experiment P5).
    SpaceStrip = 1 << 7,
}

/// Bitmask of all defined stage bits (bits 0..8). Anything above is reserved.
const PREPROCESS_DEFINED_STAGE_BITS: u16 = 0x00FF;

/// The persisted preprocessing selection. This value travels INSIDE the model
/// artifact (locked decision §10.9): inference must replay the exact stage
/// set used at training time. It is a 2-byte Copy value with no heap, so it
/// is safe to embed in the compact inference artifact header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreprocessProfile {
    stage_bits: u16,
}

impl PreprocessProfile {
    /// Constructs from raw stage bits, rejecting reserved bits.
    pub fn from_bits(stage_bits: u16) -> Result<Self, GranmoModelError> {
        if stage_bits & !PREPROCESS_DEFINED_STAGE_BITS != 0 {
            #[cfg(debug_assertions)]
            eprintln!("PP-100: profile bits {:#06x} use reserved bits", stage_bits);
            return Err(GranmoModelError::PpProfileReservedBitsSet);
        }
        Ok(Self { stage_bits })
    }

    /// Revalidating accessor for the raw bits (artifact persistence).
    pub fn get_bits(&self) -> Result<u16, GranmoModelError> {
        if self.stage_bits & !PREPROCESS_DEFINED_STAGE_BITS != 0 {
            return Err(GranmoModelError::PpProfileRecheckCorrupt);
        }
        Ok(self.stage_bits)
    }

    /// Whether one stage is enabled (revalidates first).
    pub fn has_stage(&self, stage: PreprocessStage) -> Result<bool, GranmoModelError> {
        let bits = self.get_bits()?;
        Ok(bits & (stage as u16) != 0)
    }

    // --- Experiment presets of record (§5). Each preset is one training run.

    /// `raw`: no stages — the streamed-production always-reported case.
    pub fn preset_raw() -> Self {
        Self { stage_bits: 0 }
    }

    /// P0 baseline: whitespace fold + space dedupe + leading trim + lowercase.
    pub fn preset_p0() -> Self {
        Self {
            stage_bits: PreprocessStage::WhitespaceFold as u16
                | PreprocessStage::SpaceDedupe as u16
                | PreprocessStage::LeadingTrim as u16
                | PreprocessStage::AsciiLowercase as u16,
        }
    }

    /// P1: P0 minus lowercase (capitalization A/B — ALL-CAPS is signal).
    pub fn preset_p1() -> Self {
        Self {
            stage_bits: Self::preset_p0().stage_bits & !(PreprocessStage::AsciiLowercase as u16),
        }
    }

    /// P2: P0 + leet fold (top-priority experiment).
    pub fn preset_p2() -> Self {
        Self {
            stage_bits: Self::preset_p0().stage_bits | PreprocessStage::LeetFold as u16,
        }
    }

    /// P3: P2 + symbol strip.
    pub fn preset_p3() -> Self {
        Self {
            stage_bits: Self::preset_p2().stage_bits | PreprocessStage::SymbolStrip as u16,
        }
    }

    /// P4: P0 + digit strip.
    pub fn preset_p4() -> Self {
        Self {
            stage_bits: Self::preset_p0().stage_bits | PreprocessStage::DigitStrip as u16,
        }
    }

    /// P5: P0 + space strip (expected harmful; word boundaries are signal).
    pub fn preset_p5() -> Self {
        Self {
            stage_bits: Self::preset_p0().stage_bits | PreprocessStage::SpaceStrip as u16,
        }
    }
}

/// ASCII-punctuation test for stage 7 (space deliberately excluded — word
/// boundaries are signal for this task).
#[inline(always)]
fn is_ascii_punct_not_space(byte: u8) -> bool {
    matches!(byte, 0x21..=0x2F | 0x3A..=0x40 | 0x5B..=0x60 | 0x7B..=0x7E)
}

/// Default leet-fold map for stage 5 (§5 catalogue). Configurable maps are
/// a later extension; the artifact records the profile bits, and any future
/// custom map will get its own persisted field + new error codes.
#[inline(always)]
fn leet_fold_byte(byte: u8) -> u8 {
    match byte {
        b'@' => b'a',
        b'$' => b's',
        b'0' => b'o',
        b'1' => b'l',
        b'3' => b'e',
        b'!' => b'i',
        other => other,
    }
}

/// The streaming preprocessing pipeline: bytes in, bytes out, O(1) state.
///
/// Every catalogue stage (1–8) is a map-or-drop transform, so the whole
/// pipeline reduces to `process_byte: u8 -> Option<u8>` with exactly two
/// bits of internal state (dedupe memory + leading-trim latch). This makes
/// single-pass streaming inference (and the future offset-window stream
/// evaluator) structurally trivial: no buffering, no lookahead, no heap.
///
/// Non-ASCII bytes (>= 0x80) pass through all stages untouched (locked
/// decision §10.5): emoji and other scripts remain opaque-but-consistent
/// byte patterns.
#[derive(Debug, Clone)]
pub struct BytePreprocessor {
    profile: PreprocessProfile,
    /// Dedupe state: was the last EMITTED byte a space?
    previous_emitted_was_space: bool,
    /// Leading-trim state: has any non-space byte been emitted yet?
    emitted_nonspace_yet: bool,
}

impl BytePreprocessor {
    /// Creates a pipeline for the given (validated) profile.
    pub fn new(profile: PreprocessProfile) -> Result<Self, GranmoModelError> {
        // Revalidate at wiring time so a corrupted profile cannot silently
        // configure a mismatched pipeline (artifact-replay integrity).
        let _ = profile.get_bits()?;
        Ok(Self {
            profile,
            previous_emitted_was_space: false,
            emitted_nonspace_yet: false,
        })
    }

    /// Resets per-document state. Call between documents when reusing one
    /// pipeline instance across a corpus (the profile itself is immutable).
    pub fn reset(&mut self) {
        self.previous_emitted_was_space = false;
        self.emitted_nonspace_yet = false;
    }

    /// Processes one input byte; `Ok(Some(b))` = emit `b`, `Ok(None)` = drop.
    ///
    /// Stages apply in canonical catalogue order 1..=8. Note the documented
    /// ordering consequence: dedupe (stage 2) runs BEFORE the strip stages
    /// (6/7), so a strip that removes a byte between two spaces leaves a
    /// double space. This matches the specification of record; changing
    /// stage order would be a new profile semantic and a new artifact
    /// version, not a patch.
    pub fn process_byte(&mut self, input: u8) -> Result<Option<u8>, GranmoModelError> {
        let bits = self.profile.get_bits()?;
        let enabled = |stage: PreprocessStage| bits & (stage as u16) != 0;

        let mut byte = input;

        // Stage 1: whitespace fold.
        if enabled(PreprocessStage::WhitespaceFold) && matches!(byte, b'\n' | b'\r' | b'\t') {
            byte = b' ';
        }

        // Stage 2: space dedupe (drop a space if the last emitted byte was one).
        if enabled(PreprocessStage::SpaceDedupe) && byte == b' ' && self.previous_emitted_was_space
        {
            return Ok(None);
        }

        // Stage 3: leading trim (drop spaces until first non-space emission).
        if enabled(PreprocessStage::LeadingTrim) && byte == b' ' && !self.emitted_nonspace_yet {
            return Ok(None);
        }

        // Stage 4: ASCII lowercase.
        if enabled(PreprocessStage::AsciiLowercase) && byte.is_ascii_uppercase() {
            byte = byte.wrapping_add(32); // 65..=90 + 32 cannot wrap; explicit anyway
        }

        // Stage 5: leet fold.
        if enabled(PreprocessStage::LeetFold) {
            byte = leet_fold_byte(byte);
        }

        // Stage 6: digit strip.
        if enabled(PreprocessStage::DigitStrip) && byte.is_ascii_digit() {
            return Ok(None);
        }

        // Stage 7: symbol strip (space excluded).
        if enabled(PreprocessStage::SymbolStrip) && is_ascii_punct_not_space(byte) {
            return Ok(None);
        }

        // Stage 8: space strip.
        if enabled(PreprocessStage::SpaceStrip) && byte == b' ' {
            return Ok(None);
        }

        // Emit; update the two state bits from the EMITTED byte.
        self.previous_emitted_was_space = byte == b' ';
        if byte != b' ' {
            self.emitted_nonspace_yet = true;
        }
        Ok(Some(byte))
    }

    /// Batch convenience over a full document (research-harness path; the
    /// heap allocation here is in the DATA path, which is permitted — the
    /// no-heap rule binds error paths and the streaming production path,
    /// which uses `process_byte` directly).
    pub fn process_document(&mut self, input: &[u8]) -> Result<Vec<u8>, GranmoModelError> {
        self.reset();
        let mut output = Vec::with_capacity(input.len());
        for &input_byte in input {
            if let Some(emitted) = self.process_byte(input_byte)? {
                output.push(emitted);
            }
        }
        Ok(output)
    }
}

// ===========================================================================
// SECTION 5: ByteConvTM — Byte-Level Convolutional Granmo Model (M-Conv-Core)
// ===========================================================================

/// PAD byte for right-padding documents shorter than the patch size
/// (locked decision §10.4: 0x00, which never occurs in normal text).
const PAD_BYTE: u8 = 0x00;

/// Implicit one-hot alphabet size: the byte value IS the literal index.
const BYTE_ALPHABET_SIZE: usize = 256;

/// Words in one allowed-bytes bitmask: 256 bits = 4 × u64.
const MASK_WORDS: usize = 4;

/// Derives one slot's allowed-bytes mask from that slot's raw automaton
/// states. THE single source of truth for mask semantics (§4 of the
/// specification of record): the validator, the artifact-load rebuild, and
/// the live training path all call this one function, so they cannot drift.
///
/// Semantics:
/// - 0 positive includes: byte b allowed iff negated(b) not included;
/// - 1 positive include b*: only b* allowed, and only if negated(b*) is
///   not also included;
/// - >=2 positive includes: nothing allowed (structurally dead slot).
///
/// Slice indexing below is safe by construction: both inputs are length-
/// checked to exactly `BYTE_ALPHABET_SIZE` before any index is taken.
fn derive_allowed_mask_from_states(
    positive_states: &[i16],
    negated_states: &[i16],
    depth_n: i16,
) -> Result<[u64; MASK_WORDS], GranmoModelError> {
    if positive_states.len() != BYTE_ALPHABET_SIZE || negated_states.len() != BYTE_ALPHABET_SIZE {
        return Err(GranmoModelError::BctIndexOutOfRange);
    }
    let included_positive_total = positive_states.iter().filter(|&&s| s > depth_n).count();
    let mut mask = [0u64; MASK_WORDS];
    if included_positive_total <= 1 {
        for byte_value in 0..BYTE_ALPHABET_SIZE {
            let negated_included = negated_states[byte_value] > depth_n;
            let positive_ok = included_positive_total == 0 || positive_states[byte_value] > depth_n;
            if positive_ok && !negated_included {
                mask[byte_value >> 6] |= 1u64 << (byte_value & 63);
            }
        }
    }
    Ok(mask)
}

/// The byte-level convolutional Granmo Model of record (§4 of the hand-off).
///
/// ## Structure
/// One shared clause bank (binary task): even clause indices vote +1 for
/// class 1, odd indices vote −1. Each clause holds `2 × K × 256` automata
/// over literals `(slot k, byte b)`: positive literal at local index
/// `k*256 + b` ("byte at window offset k equals b"), negated at
/// `K*256 + k*256 + b` ("byte at offset k is not b"). States are i16 in
/// `[1, 2N]`; a literal is included iff its state > N.
///
/// ## Derived evaluation structure (design commitment, not an afterthought)
/// Per (clause, slot): a 256-bit allowed-bytes mask (`[u64; 4]`), bit b set
/// iff byte b passes BOTH constraint families at that slot:
/// - a positive-include of b' ≠ b disallows b (mutual exclusion);
/// - two or more positive includes at one slot disallow EVERYTHING
///   (structurally dead — see the GuardedInclude ablation flag);
/// - a negated-include of b disallows b.
/// A clause fires at a window iff all K observed (or PAD) bytes are allowed:
/// K bit-tests, zero floats. Masks are recomputed only when an automaton
/// crosses the include/exclude boundary; ALL state changes route through
/// `increment_ta_state` / `decrement_ta_state` so the cache cannot drift,
/// and `validate_internal_consistency` re-derives everything from raw
/// states (call after any future artifact load).
///
/// ## Compute policy
/// Integer-only hot paths: mask bit-tests for evaluation; u16-threshold coin
/// draws for stochastic feedback; the (T∓V)/2T gate as an integer compare of
/// one draw in [0, 2T). Floats exist only in `SpecificityThresholds`
/// construction (config time).
///
/// ## Automaton-team notes (design record)
/// The automata are standard two-action bounded counters (properties P1–P5
/// per the design analysis). `guarded_include = true` enables the
/// `GuardedInclude` variant: a positive literal is refused the
/// exclude→include boundary crossing while another positive literal at the
/// same slot is already included, eliminating a-priori-dead joint states.
/// OFF by default; it is an M-Ablate arm, not the recorded baseline.
#[derive(Debug, Clone)]
pub struct ByteConvTM {
    /// K: window width in bytes (validated at construction via `PatchSize`).
    patch_size: usize,
    /// S: window stride in bytes (validated; S <= K enforced).
    stride: usize,
    /// Total clauses; even = positive polarity, odd = negative.
    n_clauses: usize,
    /// T, as i32 for clamp/gate arithmetic.
    vote_threshold: i32,
    /// N: automaton depth per action; states live in [1, 2N].
    states_per_action: i16,
    /// P(forget) = forget_threshold/65536 ≈ 1/s.
    forget_threshold_u16: u16,
    /// P(reinforce) = reinforce_threshold/65536 ≈ (s−1)/s.
    reinforce_threshold_u16: u16,
    /// Document scan cap in bytes.
    max_scan_bytes: usize,
    /// GuardedInclude automaton variant flag (ablation arm; default false).
    guarded_include: bool,
    /// Raw automaton states: `clause * literals_per_clause + local_literal`.
    ta_states: Vec<i16>,
    /// Allowed-bytes masks: `clause * patch_size + slot`.
    allowed_masks: Vec<[u64; MASK_WORDS]>,
    /// Cached included-positive-literal count per (clause, slot); mirrors
    /// Phase 1's `included_positive_counts` pattern.
    positive_include_counts: Vec<u16>,
}

/// One clause's P4-tier properties, snapshotted through the per-clause
/// accessors BEFORE mutable borrows are taken (the accessors need `&self`).
/// This snapshot is precisely where a future TeamCompositionPalette enters
/// the parallel path — the M-Hetero seam survives parallelization intact.
#[derive(Debug, Clone, Copy)]
struct ConvClauseProperties {
    depth_n: i16,
    forget_threshold: u16,
    reinforce_threshold: u16,
    guarded_include: bool,
}

/// Exclusive mutable access to ONE clause's automaton states, allowed-bytes
/// masks, and positive-include counts.
///
/// ## Why this type exists
/// Clause state in `ByteConvTM` is disjoint and contiguous: clause c owns
/// `ta_states[c*L..(c+1)*L]`, `allowed_masks[c*K..(c+1)*K]`, and
/// `positive_include_counts[c*K..(c+1)*K]`, and NO transition or feedback
/// operation ever touches another clause. Producing these views by zipping
/// `chunks_mut` over the three storage vectors makes the borrow checker
/// itself prove the ranges are disjoint — so clause ranges can be handed
/// to different threads with no `unsafe`, no locks, and no atomics.
///
/// ## Single source of truth
/// ALL state mutation lives here (not on the engine), so the mask/count
/// caches cannot drift: the sequential path and the parallel path execute
/// byte-identical code, differing only in how many views one thread holds.
/// Indices are LOCAL to the clause (`0..2*K*256`).
pub struct ConvClauseWorkView<'engine> {
    clause_index: usize,
    patch_size: usize,
    depth_n: i16,
    forget_threshold: u16,
    reinforce_threshold: u16,
    guarded_include: bool,
    states: &'engine mut [i16],
    masks: &'engine mut [[u64; MASK_WORDS]],
    positive_counts: &'engine mut [u16],
}

impl<'engine> ConvClauseWorkView<'engine> {
    /// This clause's index in the bank (polarity = index parity).
    #[inline(always)]
    fn view_clause_index(&self) -> usize {
        self.clause_index
    }

    #[inline(always)]
    fn view_positive_local_index(&self, slot: usize, byte_value: usize) -> usize {
        slot * BYTE_ALPHABET_SIZE + byte_value
    }

    #[inline(always)]
    fn view_negated_local_index(&self, slot: usize, byte_value: usize) -> usize {
        self.patch_size * BYTE_ALPHABET_SIZE + slot * BYTE_ALPHABET_SIZE + byte_value
    }

    /// Recomputes and stores one slot's mask from this clause's raw states.
    /// Called ONLY from the two transition methods on boundary crossings —
    /// the invariant that keeps evaluation O(K) bit-tests.
    fn view_recompute_mask(&mut self, slot: usize) -> Result<(), GranmoModelError> {
        let positive_start = self.view_positive_local_index(slot, 0);
        let negated_start = self.view_negated_local_index(slot, 0);
        let positive_end = positive_start
            .checked_add(BYTE_ALPHABET_SIZE)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let negated_end = negated_start
            .checked_add(BYTE_ALPHABET_SIZE)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let fresh_mask = {
            let positive_states = self
                .states
                .get(positive_start..positive_end)
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            let negated_states = self
                .states
                .get(negated_start..negated_end)
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            derive_allowed_mask_from_states(positive_states, negated_states, self.depth_n)?
        };
        let stored = self
            .masks
            .get_mut(slot)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        *stored = fresh_mask;
        Ok(())
    }

    /// Increments one automaton state (saturating at 2N), maintaining the
    /// positive-include count cache and recomputing the affected slot mask
    /// on an exclude→include crossing. GuardedInclude semantics unchanged:
    /// a positive literal is refused the crossing while another positive
    /// literal at the same slot is already included.
    fn view_increment_state(&mut self, local_literal: usize) -> Result<(), GranmoModelError> {
        let twice_n = self
            .depth_n
            .checked_mul(2)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        let state = *self
            .states
            .get(local_literal)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        if state >= twice_n {
            return Ok(()); // saturated deep-include; no change
        }

        let positive_region_len = self.patch_size * BYTE_ALPHABET_SIZE;
        let is_positive_literal = local_literal < positive_region_len;
        let crossing = state == self.depth_n;
        let slot = if is_positive_literal {
            local_literal / BYTE_ALPHABET_SIZE
        } else {
            (local_literal - positive_region_len) / BYTE_ALPHABET_SIZE
        };

        if crossing && is_positive_literal && self.guarded_include {
            let current_count = *self
                .positive_counts
                .get(slot)
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            if current_count > 0 {
                return Ok(()); // guard refuses the crossing; clamp at boundary
            }
        }

        let new_state = state
            .checked_add(1)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        *self
            .states
            .get_mut(local_literal)
            .ok_or(GranmoModelError::BctIndexOutOfRange)? = new_state;

        if crossing {
            if is_positive_literal {
                let count = self
                    .positive_counts
                    .get_mut(slot)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)?;
                *count = count
                    .checked_add(1)
                    .ok_or(GranmoModelError::BctArithmeticOverflow)?;
            }
            self.view_recompute_mask(slot)?;
        }
        Ok(())
    }

    /// Decrements one automaton state (floor at 1), maintaining the count
    /// cache and recomputing the slot mask on an include→exclude crossing.
    fn view_decrement_state(&mut self, local_literal: usize) -> Result<(), GranmoModelError> {
        let boundary_plus_one = self
            .depth_n
            .checked_add(1)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        let state = *self
            .states
            .get(local_literal)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        if state <= 1 {
            return Ok(()); // floor; no change
        }

        let new_state = state
            .checked_sub(1)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        *self
            .states
            .get_mut(local_literal)
            .ok_or(GranmoModelError::BctIndexOutOfRange)? = new_state;

        // Crossing include→exclude happens when the OLD state was N+1.
        if state == boundary_plus_one {
            let positive_region_len = self.patch_size * BYTE_ALPHABET_SIZE;
            let is_positive_literal = local_literal < positive_region_len;
            let slot = if is_positive_literal {
                local_literal / BYTE_ALPHABET_SIZE
            } else {
                (local_literal - positive_region_len) / BYTE_ALPHABET_SIZE
            };
            if is_positive_literal {
                let count = self
                    .positive_counts
                    .get_mut(slot)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)?;
                // saturating_sub as defence-in-depth; a validated invariant
                // means this can never actually saturate.
                *count = count.saturating_sub(1);
            }
            self.view_recompute_mask(slot)?;
        }
        Ok(())
    }

    /// Type Ia: clause fired and should have — reinforce the literal
    /// pattern of ONE sampled fired window. Semantics and coin-draw ORDER
    /// are byte-identical to the pre-parallel implementation; only the
    /// stream identity changed (per-clause, see `derive_clause_stream_seed`).
    fn view_apply_type_ia_feedback(
        &mut self,
        document: &[u8],
        effective_len: usize,
        window_start: usize,
        rng: &mut FastRng,
    ) -> Result<(), GranmoModelError> {
        let reinforce_threshold = self.reinforce_threshold;
        let forget_threshold = self.forget_threshold;
        for slot in 0..self.patch_size {
            let observed = ByteConvTM::window_byte(document, effective_len, window_start, slot);
            for byte_value in 0..BYTE_ALPHABET_SIZE {
                let positive_local = self.view_positive_local_index(slot, byte_value);
                let negated_local = self.view_negated_local_index(slot, byte_value);
                let positive_literal_true = byte_value == usize::from(observed);

                if positive_literal_true {
                    if rng.coin(reinforce_threshold) {
                        self.view_increment_state(positive_local)?;
                    }
                } else if rng.coin(forget_threshold) {
                    self.view_decrement_state(positive_local)?;
                }

                // Negated literal truth is the complement.
                if !positive_literal_true {
                    if rng.coin(reinforce_threshold) {
                        self.view_increment_state(negated_local)?;
                    }
                } else if rng.coin(forget_threshold) {
                    self.view_decrement_state(negated_local)?;
                }
            }
        }
        Ok(())
    }

    /// Type Ib: clause should have fired but did not at ANY window —
    /// input-independent uniform decay with P ≈ 1/s.
    fn view_apply_type_ib_feedback(&mut self, rng: &mut FastRng) -> Result<(), GranmoModelError> {
        let forget_threshold = self.forget_threshold;
        let literals_total = self.states.len();
        for local_literal in 0..literals_total {
            if rng.coin(forget_threshold) {
                self.view_decrement_state(local_literal)?;
            }
        }
        Ok(())
    }

    /// Type II: clause fired but should not have — deterministically
    /// increment every literal FALSE in one sampled fired window.
    fn view_apply_type_ii_feedback(
        &mut self,
        document: &[u8],
        effective_len: usize,
        window_start: usize,
    ) -> Result<(), GranmoModelError> {
        for slot in 0..self.patch_size {
            let observed = usize::from(ByteConvTM::window_byte(
                document,
                effective_len,
                window_start,
                slot,
            ));
            for byte_value in 0..BYTE_ALPHABET_SIZE {
                if byte_value != observed {
                    let positive_local = self.view_positive_local_index(slot, byte_value);
                    self.view_increment_state(positive_local)?;
                }
            }
            let negated_local = self.view_negated_local_index(slot, observed);
            self.view_increment_state(negated_local)?;
        }
        Ok(())
    }
}

/// Pass-1 result for one clause: did it fire anywhere, and which fired
/// window did the reservoir sample. `Copy` so worker chunks can be merged
/// and re-read in pass 2 with no allocation.
#[derive(Debug, Clone, Copy)]
struct ConvClauseScanOutcome {
    fired: bool,
    sampled_window_start: usize,
}

/// Pass 1 body for a contiguous clause range (read-only against the
/// engine). Used by BOTH the single-threaded path and each worker thread,
/// so the two cannot diverge.
fn conv_scan_clause_range(
    engine: &ByteConvTM,
    document: &[u8],
    step_seed: u64,
    first_clause: usize,
    outcomes: &mut [ConvClauseScanOutcome],
) -> Result<(), GranmoModelError> {
    for (offset, outcome_slot) in outcomes.iter_mut().enumerate() {
        let clause = first_clause
            .checked_add(offset)
            .ok_or(GranmoModelError::ParClauseViewGeometryFault)?;
        let mut clause_rng = FastRng::seed(derive_clause_stream_seed(
            step_seed,
            clause as u64,
            RNG_PURPOSE_WINDOW_SCAN,
        ));
        let (fired_count, sampled_start) =
            engine.scan_clause_reservoir(clause, document, &mut clause_rng)?;
        *outcome_slot = ConvClauseScanOutcome {
            fired: fired_count > 0,
            sampled_window_start: sampled_start,
        };
    }
    Ok(())
}

/// Pass 2 body for a contiguous range of clause views. Used by BOTH the
/// single-threaded path and each worker thread.
fn conv_apply_feedback_to_view_range(
    views: &mut [ConvClauseWorkView<'_>],
    scan_outcomes: &[ConvClauseScanOutcome],
    document: &[u8],
    effective_len: usize,
    step_seed: u64,
    gates: FeedbackGates,
    label_is_positive: bool,
) -> Result<(), GranmoModelError> {
    for view in views.iter_mut() {
        let clause = view.view_clause_index();
        let outcome = *scan_outcomes
            .get(clause)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let positive_polarity = clause % 2 == 0;

        // Which feedback family, and which gate, this clause receives.
        let (gate, receives_type_i) = if label_is_positive {
            (gates.gate_when_target, positive_polarity)
        } else {
            (gates.gate_when_other, !positive_polarity)
        };

        let mut clause_rng = FastRng::seed(derive_clause_stream_seed(
            step_seed,
            clause as u64,
            RNG_PURPOSE_CLAUSE_FEEDBACK,
        ));
        let draw = clause_rng.gen_index(gates.two_t)? as i32;
        if draw >= gate {
            continue; // gated out this step
        }

        if receives_type_i {
            if outcome.fired {
                view.view_apply_type_ia_feedback(
                    document,
                    effective_len,
                    outcome.sampled_window_start,
                    &mut clause_rng,
                )?;
            } else {
                view.view_apply_type_ib_feedback(&mut clause_rng)?;
            }
        } else if outcome.fired {
            view.view_apply_type_ii_feedback(
                document,
                effective_len,
                outcome.sampled_window_start,
            )?;
        }
    }
    Ok(())
}

impl ByteConvTM {
    /// Builds the probability LUT matched to THIS engine's clause count and
    /// vote threshold. Routing construction through the enforced newtypes
    /// re-validates both values, so a corrupted engine cannot silently
    /// produce a mis-sized table (value-integrity rule).
    pub fn conv_build_probability_lut(&self) -> Result<ProbabilityLut, GranmoModelError> {
        let clause_count =
            u16::try_from(self.n_clauses).map_err(|_| GranmoModelError::BctIndexOutOfRange)?;
        let threshold =
            i16::try_from(self.vote_threshold).map_err(|_| GranmoModelError::BctIndexOutOfRange)?;
        ProbabilityLut::build(
            ClauseCount::new(clause_count)?,
            VoteThreshold::new(threshold)?,
        )
    }

    /// Constructs a fresh engine: every automaton at the exclude boundary
    /// (state == N), hence zero includes, all-permissive masks, zero counts —
    /// a consistent starting invariant (a fresh clause fires on every window).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        patch_size: PatchSize,
        stride: StrideLen,
        n_clauses: ClauseCount,
        vote_threshold: VoteThreshold,
        states_per_action: StatesPerAction,
        specificity: SpecificityThresholds,
        max_scan_bytes: MaxScanBytes,
        guarded_include: bool,
    ) -> Result<Self, GranmoModelError> {
        let patch = usize::from(patch_size.get()?);
        let stride_len = usize::from(stride.get()?);
        let clause_total = usize::from(n_clauses.get()?);
        let threshold = i32::from(vote_threshold.get()?);
        let depth_n = states_per_action.get()?;
        let (forget, reinforce) = specificity.get()?;
        let scan_cap = max_scan_bytes.get()? as usize;

        if stride_len > patch {
            #[cfg(debug_assertions)]
            eprintln!("BCT-400: stride {} > patch_size {}", stride_len, patch);
            return Err(GranmoModelError::BctStrideExceedsPatchSize);
        }

        let literals_per_clause = patch
            .checked_mul(BYTE_ALPHABET_SIZE)
            .and_then(|v| v.checked_mul(2))
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        let total_states = clause_total
            .checked_mul(literals_per_clause)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        let total_slots = clause_total
            .checked_mul(patch)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;

        Ok(Self {
            patch_size: patch,
            stride: stride_len,
            n_clauses: clause_total,
            vote_threshold: threshold,
            states_per_action: depth_n,
            forget_threshold_u16: forget,
            reinforce_threshold_u16: reinforce,
            max_scan_bytes: scan_cap,
            guarded_include,
            ta_states: vec![depth_n; total_states],
            // Fresh state: no includes anywhere => every byte allowed.
            allowed_masks: vec![[u64::MAX; MASK_WORDS]; total_slots],
            positive_include_counts: vec![0u16; total_slots],
        })
    }

    // --- Layout helpers ---------------------------------------------------

    #[inline(always)]
    fn literals_per_clause(&self) -> usize {
        2 * self.patch_size * BYTE_ALPHABET_SIZE
    }

    #[inline(always)]
    fn positive_local_index(&self, slot: usize, byte_value: usize) -> usize {
        slot * BYTE_ALPHABET_SIZE + byte_value
    }

    #[inline(always)]
    fn negated_local_index(&self, slot: usize, byte_value: usize) -> usize {
        self.patch_size * BYTE_ALPHABET_SIZE + slot * BYTE_ALPHABET_SIZE + byte_value
    }

    /// Global state index for (clause, local literal), bounds-checked.
    fn global_state_index(
        &self,
        clause: usize,
        local_literal: usize,
    ) -> Result<usize, GranmoModelError> {
        let base = clause
            .checked_mul(self.literals_per_clause())
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let idx = base
            .checked_add(local_literal)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        if idx >= self.ta_states.len() {
            return Err(GranmoModelError::BctIndexOutOfRange);
        }
        Ok(idx)
    }

    /// Global slot index for (clause, slot) into masks/counts, bounds-checked.
    fn global_slot_index(&self, clause: usize, slot: usize) -> Result<usize, GranmoModelError> {
        let base = clause
            .checked_mul(self.patch_size)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let idx = base
            .checked_add(slot)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        if idx >= self.allowed_masks.len() {
            return Err(GranmoModelError::BctIndexOutOfRange);
        }
        Ok(idx)
    }

    // --- Per-clause P4-property accessors (M-Hetero accommodation) ---------
    //
    // Design record (Sessions 1–3): heterogeneous team composition may
    // legitimately vary the P4-TIER tuning properties (automaton depth N,
    // specificity thresholds, GuardedInclude flag) PER CLAUSE, and must
    // never vary P1/P2/P5 (automaton structure). These four accessors are
    // the single seam through which every hot-path read of a P4 property
    // flows. Today they return the engine-level scalar (homogeneous team —
    // the recorded baseline). When the TeamCompositionPalette lands
    // (M-Hetero), ONLY these bodies, the constructor, and the artifact
    // format grow — no feedback, mask, or validation logic changes. The
    // `_clause` parameter is therefore deliberate, not dead weight.

    /// Automaton depth N for this clause's team (states live in [1, 2N]).
    #[inline(always)]
    fn depth_for_clause(&self, _clause: usize) -> i16 {
        self.states_per_action
    }

    /// P(forget) coin threshold (≈ 65536 × 1/s) for this clause.
    #[inline(always)]
    fn forget_threshold_for_clause(&self, _clause: usize) -> u16 {
        self.forget_threshold_u16
    }

    /// P(reinforce) coin threshold (≈ 65536 × (s−1)/s) for this clause.
    #[inline(always)]
    fn reinforce_threshold_for_clause(&self, _clause: usize) -> u16 {
        self.reinforce_threshold_u16
    }

    /// GuardedInclude flag for this clause (ablation arm; default false).
    #[inline(always)]
    fn guard_for_clause(&self, _clause: usize) -> bool {
        self.guarded_include
    }

    // --- Window geometry ----------------------------------------------------

    /// Effective scan length (cap applied) and the short-document flag.
    /// A document shorter than K yields exactly ONE right-PAD-padded window
    /// (locked decision §10.4); otherwise windows start at 0, S, 2S, ... with
    /// the whole window inside the capped length.
    #[inline(always)]
    fn scan_plan(&self, document: &[u8]) -> (usize, bool) {
        let effective_len = document.len().min(self.max_scan_bytes);
        (effective_len, effective_len < self.patch_size)
    }

    /// Byte at window offset k, PAD past the effective end. `unwrap_or` is
    /// `Option::unwrap_or` (a total function), not a panic path.
    #[inline(always)]
    fn window_byte(document: &[u8], effective_len: usize, start: usize, k: usize) -> u8 {
        match start.checked_add(k) {
            Some(pos) if pos < effective_len => document.get(pos).copied().unwrap_or(PAD_BYTE),
            _ => PAD_BYTE,
        }
    }

    // --- Clause evaluation (integer-only hot path) --------------------------

    /// Does `clause` match the window at `start`? K bit-tests on the
    /// allowed-bytes masks. The inner slice indexing is safe by construction:
    /// the slice is range-checked to exactly `patch_size` masks, `k` iterates
    /// `0..patch_size`, and the word index is `byte >> 6 <= 3 < MASK_WORDS`.
    fn clause_fires_at(
        &self,
        clause: usize,
        document: &[u8],
        effective_len: usize,
        start: usize,
    ) -> Result<bool, GranmoModelError> {
        let mask_base = self.global_slot_index(clause, 0)?;
        let mask_end = mask_base
            .checked_add(self.patch_size)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let clause_masks = self
            .allowed_masks
            .get(mask_base..mask_end)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;

        for k in 0..self.patch_size {
            let byte_value = Self::window_byte(document, effective_len, start, k);
            let word = clause_masks[k][usize::from(byte_value >> 6)];
            if word & (1u64 << (byte_value & 63)) == 0 {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// OR-pooled clause output with early exit on the first matching window
    /// (canonical pooling, locked decision §10.2; inference path).
    fn clause_fires_any(&self, clause: usize, document: &[u8]) -> Result<bool, GranmoModelError> {
        let (effective_len, single_padded) = self.scan_plan(document);
        if single_padded {
            return self.clause_fires_at(clause, document, effective_len, 0);
        }
        let mut start = 0usize;
        while start
            .checked_add(self.patch_size)
            .map_or(false, |end| end <= effective_len)
        {
            if self.clause_fires_at(clause, document, effective_len, start)? {
                return Ok(true);
            }
            match start.checked_add(self.stride) {
                Some(next) => start = next,
                None => break,
            }
        }
        Ok(false)
    }

    /// Training-path scan: counts fired windows and reservoir-samples ONE
    /// uniformly (keep the current window with probability 1/count) in a
    /// single pass — no per-clause Vec of fired positions is ever built
    /// (the recorded improvement over Phase 1's `fired: Vec<usize>`).
    /// Returns `(fired_count, sampled_window_start)`.
    fn scan_clause_reservoir(
        &self,
        clause: usize,
        document: &[u8],
        rng: &mut FastRng,
    ) -> Result<(u32, usize), GranmoModelError> {
        let (effective_len, single_padded) = self.scan_plan(document);
        let mut fired_count: u32 = 0;
        let mut sampled_start: usize = 0;

        if single_padded {
            if self.clause_fires_at(clause, document, effective_len, 0)? {
                fired_count = 1;
            }
            return Ok((fired_count, sampled_start));
        }

        let mut start = 0usize;
        while start
            .checked_add(self.patch_size)
            .map_or(false, |end| end <= effective_len)
        {
            if self.clause_fires_at(clause, document, effective_len, start)? {
                fired_count = fired_count
                    .checked_add(1)
                    .ok_or(GranmoModelError::BctArithmeticOverflow)?;
                if rng.gen_index(fired_count as usize)? == 0 {
                    sampled_start = start;
                }
            }
            match start.checked_add(self.stride) {
                Some(next) => start = next,
                None => break,
            }
        }
        Ok((fired_count, sampled_start))
    }

    // --- Public inference surface -------------------------------------------

    /// Signed vote sum V = Σ fired(+) − Σ fired(−) over OR-pooled clauses.
    pub fn conv_vote_sum(&self, document: &[u8]) -> Result<i32, GranmoModelError> {
        let mut vote: i32 = 0;
        for clause in 0..self.n_clauses {
            if self.clause_fires_any(clause, document)? {
                vote = if clause % 2 == 0 {
                    vote.checked_add(1)
                } else {
                    vote.checked_sub(1)
                }
                .ok_or(GranmoModelError::BctArithmeticOverflow)?;
            }
        }
        Ok(vote)
    }

    /// Binary prediction: label 1 iff `V > decision_threshold` (default 0;
    /// the threshold is a free sweep axis for imbalance handling, §8).
    pub fn conv_predict(
        &self,
        document: &[u8],
        decision_threshold: i32,
    ) -> Result<bool, GranmoModelError> {
        Ok(self.conv_vote_sum(document)? > decision_threshold)
    }

    /// The fired-clause bitset: the free-byproduct binary document embedding
    /// (§7.2). Bit `c` set iff clause `c` fired; Hamming distance between
    /// bitsets is a learned document similarity.
    pub fn conv_fired_clause_bits(&self, document: &[u8]) -> Result<Vec<u64>, GranmoModelError> {
        let word_count = self
            .n_clauses
            .checked_add(63)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?
            / 64;
        let mut bits = vec![0u64; word_count];
        for clause in 0..self.n_clauses {
            if self.clause_fires_any(clause, document)? {
                let word = bits
                    .get_mut(clause >> 6)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)?;
                *word |= 1u64 << (clause & 63);
            }
        }
        Ok(bits)
    }

    /// All window start positions (byte offsets) where `clause` fires —
    /// the explainability source-span primitive (§7.3). Research/explain
    /// path: heap allocation is in the data path, which is permitted.
    pub fn conv_fired_window_positions(
        &self,
        clause: usize,
        document: &[u8],
    ) -> Result<Vec<usize>, GranmoModelError> {
        if clause >= self.n_clauses {
            return Err(GranmoModelError::BctIndexOutOfRange);
        }
        let (effective_len, single_padded) = self.scan_plan(document);
        let mut positions = Vec::new();
        if single_padded {
            if self.clause_fires_at(clause, document, effective_len, 0)? {
                positions.push(0);
            }
            return Ok(positions);
        }
        let mut start = 0usize;
        while start
            .checked_add(self.patch_size)
            .map_or(false, |end| end <= effective_len)
        {
            if self.clause_fires_at(clause, document, effective_len, start)? {
                positions.push(start);
            }
            match start.checked_add(self.stride) {
                Some(next) => start = next,
                None => break,
            }
        }
        Ok(positions)
    }

    /// Decodes one clause into a human-readable byte pattern, e.g.
    /// `k0='n' ∧ k1='o' ∧ k2='t' ∧ k4≠' '`. Non-printable bytes render as
    /// hex (`0x00`). Rendering is capped so heavily-negated clauses stay
    /// readable. Research/explain data path: heap permitted here; this is
    /// the primary debugging instrument per the plan (M-Explain built
    /// alongside M-Conv-Core, not after).
    pub fn conv_describe_clause(
        &self,
        clause: usize,
        max_rendered_literals: usize,
    ) -> Result<String, GranmoModelError> {
        if clause >= self.n_clauses {
            return Err(GranmoModelError::BctIndexOutOfRange);
        }
        let depth_n = self.depth_for_clause(clause);
        let mut parts: Vec<String> = Vec::new();
        let mut omitted: usize = 0;

        /// Renders a byte as a quoted char if printable ASCII, else hex.
        fn render_byte(byte_value: u8) -> String {
            if (0x20..=0x7E).contains(&byte_value) {
                format!("'{}'", byte_value as char)
            } else {
                format!("0x{:02X}", byte_value)
            }
        }

        for slot in 0..self.patch_size {
            for byte_value in 0..BYTE_ALPHABET_SIZE {
                let pos_idx =
                    self.global_state_index(clause, self.positive_local_index(slot, byte_value))?;
                let neg_idx =
                    self.global_state_index(clause, self.negated_local_index(slot, byte_value))?;
                let pos_state = *self
                    .ta_states
                    .get(pos_idx)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)?;
                let neg_state = *self
                    .ta_states
                    .get(neg_idx)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)?;

                if pos_state > depth_n {
                    if parts.len() < max_rendered_literals {
                        parts.push(format!("k{}={}", slot, render_byte(byte_value as u8)));
                    } else {
                        omitted = omitted.saturating_add(1);
                    }
                }
                if neg_state > depth_n {
                    if parts.len() < max_rendered_literals {
                        parts.push(format!("k{}≠{}", slot, render_byte(byte_value as u8)));
                    } else {
                        omitted = omitted.saturating_add(1);
                    }
                }
            }
        }

        let mut rendered = parts.join(" ∧ ");
        if omitted > 0 {
            rendered.push_str(&format!(" … (+{} more literals)", omitted));
        }
        Ok(rendered)
    }

    // --- Mask derivation and cache maintenance ------------------------------

    /// Builds an exclusive mutable view of ONE clause (sequential paths:
    /// cache rebuild, test helpers).
    fn conv_build_clause_view(
        &mut self,
        clause: usize,
    ) -> Result<ConvClauseWorkView<'_>, GranmoModelError> {
        if clause >= self.n_clauses {
            return Err(GranmoModelError::BctIndexOutOfRange);
        }
        // P4 properties are read through the per-clause accessors BEFORE
        // any mutable borrow is taken (the M-Hetero seam).
        let properties = ConvClauseProperties {
            depth_n: self.depth_for_clause(clause),
            forget_threshold: self.forget_threshold_for_clause(clause),
            reinforce_threshold: self.reinforce_threshold_for_clause(clause),
            guarded_include: self.guard_for_clause(clause),
        };
        let literals_per_clause = self.literals_per_clause();
        let patch = self.patch_size;

        let state_start = clause
            .checked_mul(literals_per_clause)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let state_end = state_start
            .checked_add(literals_per_clause)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let slot_start = clause
            .checked_mul(patch)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let slot_end = slot_start
            .checked_add(patch)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;

        Ok(ConvClauseWorkView {
            clause_index: clause,
            patch_size: patch,
            depth_n: properties.depth_n,
            forget_threshold: properties.forget_threshold,
            reinforce_threshold: properties.reinforce_threshold,
            guarded_include: properties.guarded_include,
            states: self
                .ta_states
                .get_mut(state_start..state_end)
                .ok_or(GranmoModelError::ParClauseViewGeometryFault)?,
            masks: self
                .allowed_masks
                .get_mut(slot_start..slot_end)
                .ok_or(GranmoModelError::ParClauseViewGeometryFault)?,
            positive_counts: self
                .positive_include_counts
                .get_mut(slot_start..slot_end)
                .ok_or(GranmoModelError::ParClauseViewGeometryFault)?,
        })
    }

    /// Builds exclusive mutable views of EVERY clause at once, by zipping
    /// `chunks_mut` over the three storage vectors. The borrow checker
    /// proves the ranges are disjoint, which is exactly what permits
    /// handing contiguous view ranges to different threads with no
    /// `unsafe`, no locks, and no atomics.
    fn conv_build_all_clause_views(
        &mut self,
    ) -> Result<Vec<ConvClauseWorkView<'_>>, GranmoModelError> {
        let clause_total = self.n_clauses;
        let literals_per_clause = self.literals_per_clause();
        let patch = self.patch_size;

        // Snapshot P4 properties through the accessors BEFORE the mutable
        // borrows below (the accessors need `&self`).
        let mut clause_properties: Vec<ConvClauseProperties> = Vec::with_capacity(clause_total);
        for clause in 0..clause_total {
            clause_properties.push(ConvClauseProperties {
                depth_n: self.depth_for_clause(clause),
                forget_threshold: self.forget_threshold_for_clause(clause),
                reinforce_threshold: self.reinforce_threshold_for_clause(clause),
                guarded_include: self.guard_for_clause(clause),
            });
        }

        let mut views: Vec<ConvClauseWorkView<'_>> = Vec::with_capacity(clause_total);
        let state_chunks = self.ta_states.chunks_mut(literals_per_clause);
        let mask_chunks = self.allowed_masks.chunks_mut(patch);
        let count_chunks = self.positive_include_counts.chunks_mut(patch);

        for (clause_index, ((state_chunk, mask_chunk), count_chunk)) in
            state_chunks.zip(mask_chunks).zip(count_chunks).enumerate()
        {
            let properties = *clause_properties
                .get(clause_index)
                .ok_or(GranmoModelError::ParClauseViewGeometryFault)?;
            if state_chunk.len() != literals_per_clause
                || mask_chunk.len() != patch
                || count_chunk.len() != patch
            {
                return Err(GranmoModelError::ParClauseViewGeometryFault);
            }
            views.push(ConvClauseWorkView {
                clause_index,
                patch_size: patch,
                depth_n: properties.depth_n,
                forget_threshold: properties.forget_threshold,
                reinforce_threshold: properties.reinforce_threshold,
                guarded_include: properties.guarded_include,
                states: state_chunk,
                masks: mask_chunk,
                positive_counts: count_chunk,
            });
        }
        if views.len() != clause_total {
            return Err(GranmoModelError::ParClauseViewGeometryFault);
        }
        Ok(views)
    }

    /// Recomputes and stores the mask for (clause, slot) through the clause
    /// view, so live training and the artifact-load rebuild share one
    /// implementation.
    fn recompute_mask(&mut self, clause: usize, slot: usize) -> Result<(), GranmoModelError> {
        let mut clause_view = self.conv_build_clause_view(clause)?;
        clause_view.view_recompute_mask(slot)
    }

    // --- Automaton transitions (ALL state changes route through these) ------

    /// Recomputes the allowed-bytes mask for (clause, slot) purely from raw
    /// automaton states, via the shared `derive_allowed_mask_from_states`
    /// helper (the single source of truth for mask semantics). Read-only
    /// path: used by validation and by the artifact-load cache rebuild.
    fn compute_mask_from_states(
        &self,
        clause: usize,
        slot: usize,
    ) -> Result<[u64; MASK_WORDS], GranmoModelError> {
        let depth_n = self.depth_for_clause(clause);
        let positive_start = self.global_state_index(clause, self.positive_local_index(slot, 0))?;
        let negated_start = self.global_state_index(clause, self.negated_local_index(slot, 0))?;
        let positive_end = positive_start
            .checked_add(BYTE_ALPHABET_SIZE)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let negated_end = negated_start
            .checked_add(BYTE_ALPHABET_SIZE)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let positive_states = self
            .ta_states
            .get(positive_start..positive_end)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let negated_states = self
            .ta_states
            .get(negated_start..negated_end)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        derive_allowed_mask_from_states(positive_states, negated_states, depth_n)
    }

    // --- Training ------------------------------------------------------------

    /// One stochastic training update for one document, single-threaded.
    ///
    /// Byte-identical to `conv_train_step_with_workers(..., WorkerCount 1)`
    /// — and, by the per-clause RNG contract, byte-identical to ANY worker
    /// count. Retained as the convenience entry point for tests and callers
    /// that do not thread a worker count through.
    pub fn conv_train_step(
        &mut self,
        document: &[u8],
        label_is_positive: bool,
        rng: &mut FastRng,
    ) -> Result<(), GranmoModelError> {
        self.conv_train_step_with_workers(document, label_is_positive, rng, WorkerCount::new(1)?)
    }

    /// Pass 1 of one training step: reservoir-scans every clause (fork-join
    /// over the clause bank) and reduces the signed vote sum V as an EXACT
    /// integer sum (order-independent). Read-only against the engine.
    ///
    /// Extracted from `conv_train_step_with_workers` so `HybridTM` can insert
    /// the COMBINED vote between the two passes; the standalone step and the
    /// hybrid step therefore run byte-identical scan code. Returns
    /// `(per-clause scan outcomes, vote sum)`.
    fn conv_scan_pass_with_workers(
        &self,
        document: &[u8],
        step_seed: u64,
        worker_count: WorkerCount,
    ) -> Result<(Vec<ConvClauseScanOutcome>, i32), GranmoModelError> {
        let clause_total = self.n_clauses;
        let worker_total = resolve_effective_worker_count(worker_count, clause_total)?;
        let chunk_size = resolve_work_chunk_size(clause_total, worker_total)?;

        let mut scan_outcomes: Vec<ConvClauseScanOutcome> = vec![
            ConvClauseScanOutcome {
                fired: false,
                sampled_window_start: 0,
            };
            clause_total
        ];

        if worker_total == 1 {
            conv_scan_clause_range(self, document, step_seed, 0, &mut scan_outcomes)?;
        } else {
            let engine_shared: &ByteConvTM = self;
            let scan_results: Vec<Result<(), GranmoModelError>> =
                std::thread::scope(|scan_scope| {
                    let mut worker_handles = Vec::with_capacity(worker_total);
                    for (chunk_index, outcome_chunk) in
                        scan_outcomes.chunks_mut(chunk_size).enumerate()
                    {
                        worker_handles.push(scan_scope.spawn(move || {
                            let first_clause = chunk_index
                                .checked_mul(chunk_size)
                                .ok_or(GranmoModelError::ParClauseViewGeometryFault)?;
                            conv_scan_clause_range(
                                engine_shared,
                                document,
                                step_seed,
                                first_clause,
                                outcome_chunk,
                            )
                        }));
                    }
                    worker_handles
                        .into_iter()
                        .map(|handle| match handle.join() {
                            Ok(worker_result) => worker_result,
                            // join() errs only on a worker panic. Production
                            // code never panics: report, never re-raise.
                            Err(_panic_payload) => Err(GranmoModelError::ParWorkerJoinFailed),
                        })
                        .collect()
                });
            for worker_result in scan_results {
                worker_result?;
            }
        }

        // Reduce: vote sum (exact integer, order-independent).
        let mut vote: i32 = 0;
        for (clause, outcome) in scan_outcomes.iter().enumerate() {
            if outcome.fired {
                vote = if clause % 2 == 0 {
                    vote.checked_add(1)
                } else {
                    vote.checked_sub(1)
                }
                .ok_or(GranmoModelError::BctArithmeticOverflow)?;
            }
        }
        Ok((scan_outcomes, vote))
    }

    /// Pass 2 of one training step: applies feedback under the given gates
    /// to every clause through disjoint mutable clause views (fork-join).
    /// The gates may come from this engine's own vote (standalone) or from a
    /// combined vote (`HybridTM`) — this function does not care, which is
    /// exactly the seam the hybrid needs.
    fn conv_feedback_pass_with_workers(
        &mut self,
        document: &[u8],
        scan_outcomes: &[ConvClauseScanOutcome],
        step_seed: u64,
        gates: FeedbackGates,
        label_is_positive: bool,
        worker_count: WorkerCount,
    ) -> Result<(), GranmoModelError> {
        if scan_outcomes.len() != self.n_clauses {
            return Err(GranmoModelError::ParClauseViewGeometryFault);
        }
        let (effective_len, _single_padded) = self.scan_plan(document);
        let clause_total = self.n_clauses;
        let worker_total = resolve_effective_worker_count(worker_count, clause_total)?;
        let chunk_size = resolve_work_chunk_size(clause_total, worker_total)?;

        let mut clause_views = self.conv_build_all_clause_views()?;

        if worker_total == 1 {
            conv_apply_feedback_to_view_range(
                &mut clause_views,
                scan_outcomes,
                document,
                effective_len,
                step_seed,
                gates,
                label_is_positive,
            )?;
        } else {
            let feedback_results: Vec<Result<(), GranmoModelError>> =
                std::thread::scope(|feedback_scope| {
                    let mut worker_handles = Vec::with_capacity(worker_total);
                    for view_chunk in clause_views.chunks_mut(chunk_size) {
                        worker_handles.push(feedback_scope.spawn(move || {
                            conv_apply_feedback_to_view_range(
                                view_chunk,
                                scan_outcomes,
                                document,
                                effective_len,
                                step_seed,
                                gates,
                                label_is_positive,
                            )
                        }));
                    }
                    worker_handles
                        .into_iter()
                        .map(|handle| match handle.join() {
                            Ok(worker_result) => worker_result,
                            Err(_panic_payload) => Err(GranmoModelError::ParWorkerJoinFailed),
                        })
                        .collect()
                });
            for worker_result in feedback_results {
                worker_result?;
            }
        }
        Ok(())
    }

    /// One stochastic training update for one document, fork-join over the
    /// clause bank.
    ///
    /// ## Structure
    /// Pass 1 (read-only, parallel): each worker scans its clause range,
    /// reservoir-sampling one fired window per clause. Reduce: the signed
    /// vote sum V (an EXACT integer sum, hence order-independent — a float
    /// reduction would not be). Gates: the shared `resolve_feedback_gates`.
    /// Pass 2 (mutating, parallel): each worker applies feedback to its own
    /// clause views only, which the borrow checker proves are disjoint.
    ///
    /// ## Determinism
    /// Every clause draws from its own stream derived from (step seed,
    /// clause, purpose), so the result does not depend on the worker count
    /// or on the thread schedule. The master `rng` advances by exactly one
    /// `next_u64()` per step at every worker count.
    ///
    /// ## Scheduling note (recorded, not hidden)
    /// This spawns one scope per pass per document. If profiling shows
    /// spawn cost dominating per-document work, the next iteration is a
    /// persistent worker pool with two `std::sync::Barrier`s held across an
    /// epoch — a pure scheduling change with no effect on results.
    pub fn conv_train_step_with_workers(
        &mut self,
        document: &[u8],
        label_is_positive: bool,
        rng: &mut FastRng,
        worker_count: WorkerCount,
    ) -> Result<(), GranmoModelError> {
        // EXACTLY one master draw per step, at every worker count.
        let step_seed = rng.next_u64();
        let (scan_outcomes, vote) =
            self.conv_scan_pass_with_workers(document, step_seed, worker_count)?;
        let gates = resolve_feedback_gates(vote, self.vote_threshold)?;
        self.conv_feedback_pass_with_workers(
            document,
            &scan_outcomes,
            step_seed,
            gates,
            label_is_positive,
            worker_count,
        )
    }

    // --- Invariant validation --------------------------------------------------

    /// Re-derives every mask and every positive-include count from raw
    /// automaton states and compares against the caches; also checks every
    /// state lies in its clause's legal band [1, 2N]. Structured PER CLAUSE
    /// (not one flat pass) because depth N is read through
    /// `depth_for_clause` — under a future TeamCompositionPalette each
    /// clause may carry its own band, and this loop is already shaped for
    /// that. Call after any artifact load (same pattern as Phase 1) and
    /// after training in tests.
    pub fn conv_validate_internal_consistency(&self) -> Result<(), GranmoModelError> {
        // Storage-geometry gate: the state vector must be exactly
        // clause_count × literals_per_clause, or the per-clause slicing
        // below could silently skip trailing states.
        let expected_total = self
            .n_clauses
            .checked_mul(self.literals_per_clause())
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        if self.ta_states.len() != expected_total {
            return Err(GranmoModelError::BctIndexOutOfRange);
        }

        for clause in 0..self.n_clauses {
            let depth_n = self.depth_for_clause(clause);
            let twice_n = depth_n
                .checked_mul(2)
                .ok_or(GranmoModelError::BctArithmeticOverflow)?;

            // Band check over exactly this clause's states.
            let clause_base = clause
                .checked_mul(self.literals_per_clause())
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            let clause_end = clause_base
                .checked_add(self.literals_per_clause())
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            let clause_states = self
                .ta_states
                .get(clause_base..clause_end)
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            for &state in clause_states {
                if state < 1 || state > twice_n {
                    return Err(GranmoModelError::BctStateValueOutOfRange);
                }
            }

            for slot in 0..self.patch_size {
                let slot_idx = self.global_slot_index(clause, slot)?;

                // Count cache check.
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
                let cached_count = *self
                    .positive_include_counts
                    .get(slot_idx)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)?;
                if recomputed_count != cached_count {
                    return Err(GranmoModelError::BctCountCacheInconsistent);
                }

                // Mask cache check.
                let recomputed_mask = self.compute_mask_from_states(clause, slot)?;
                let cached_mask = *self
                    .allowed_masks
                    .get(slot_idx)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)?;
                if recomputed_mask != cached_mask {
                    return Err(GranmoModelError::BctMaskCacheInconsistent);
                }
            }
        }
        Ok(())
    }

    /// Read accessor for the clause count (research harness / reporting).
    pub fn conv_clause_count(&self) -> usize {
        self.n_clauses
    }

    /// Per-clause total of INCLUDED literals (positive + negated),
    /// counted from raw automaton states — the conv-side counterpart of
    /// `bag_clause_include_totals` (same reading guide: 0 = vacuous
    /// bootstrap clause, fires on every window by construction).
    /// Reporting tier: read-only, O(clauses × literals).
    pub fn conv_clause_include_totals(&self) -> Result<Vec<u32>, GranmoModelError> {
        let literals_per_clause = self.literals_per_clause();
        let mut totals: Vec<u32> = Vec::with_capacity(self.n_clauses);
        for clause in 0..self.n_clauses {
            let depth_n = self.depth_for_clause(clause);
            let clause_base = clause
                .checked_mul(literals_per_clause)
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            let clause_end = clause_base
                .checked_add(literals_per_clause)
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            let clause_states = self
                .ta_states
                .get(clause_base..clause_end)
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            let included = clause_states.iter().filter(|&&s| s > depth_n).count();
            totals.push(
                u32::try_from(included).map_err(|_| GranmoModelError::BctArithmeticOverflow)?,
            );
        }
        Ok(totals)
    }

    /// Test-only helper: forces a literal fully into the include region via
    /// the cache-maintaining view transition path, so tests can
    /// hand-construct exact clauses without breaking the mask/count
    /// invariants. Terminates when the state stops changing (saturated at
    /// 2N, or refused by the GuardedInclude clamp).
    #[cfg(test)]
    fn conv_test_force_include(&mut self, clause: usize, local_literal: usize) {
        loop {
            let mut clause_view = self.conv_build_clause_view(clause).unwrap();
            let before = clause_view.states[local_literal];
            clause_view.view_increment_state(local_literal).unwrap();
            let after = clause_view.states[local_literal];
            if after == before {
                break;
            }
        }
    }
}

// ===========================================================================
// SECTION 5B: ClassifierEngine — engine-agnostic dispatch (R1 refactor)
// ===========================================================================

/// The polymorphic model unit: every engine this crate can train, evaluate,
/// persist, or predict with is exactly one variant of this enum.
///
/// ## Why an enum and not a trait object
/// The coding rules exclude `dyn` dispatch ("no fancy pointer use") and
/// favor exhaustive `match`: adding a variant makes every dispatch site
/// below fail to compile until the new engine is wired — no silent
/// fall-through, no default-method surprises. This mirrors the Phase 1
/// `ClassifierModel` enum pattern recorded in the hand-off as "good pattern."
///
/// ## Scientific role
/// The §8 comparison discipline requires competing engines to share ONE
/// harness, ONE splitter, ONE sweep, ONE artifact framing — differing in
/// exactly the variable under test. This enum is the seam that makes that
/// structural: `run_single_experiment`, the CLI handlers, and
/// `ModelArtifact` are written against it, once.
#[derive(Debug, Clone)]
pub enum ClassifierEngine {
    /// Byte-level convolutional Granmo Model (specification §4 of record;
    /// Section 5). Positional: learns byte patterns at window offsets.
    ByteConv(ByteConvTM),
    /// Flat bag-of-byte-n-grams Granmo Model (Section 12B) — the §8
    /// scientific control. Differs from ByteConv in exactly one variable:
    /// positional windowing (the bag has none).
    ByteBag(ByteBagTM),
    /// Joint two-bank co-training engine (Section 12C): a sequence-aware
    /// conv bank ("seq") and a presence bag bank ("freq"), both receiving
    /// feedback gated by their COMBINED vote against one shared T.
    SeqFreqHybrid(HybridTM),
}

impl ClassifierEngine {
    /// Stable engine identifier for reports, run labels, and the
    /// `--engine` CLI value.
    pub fn engine_name(&self) -> &'static str {
        match self {
            Self::ByteConv(_) => "byte-conv",
            Self::ByteBag(_) => "byte-bag",
            Self::SeqFreqHybrid(_) => "seq-freq-hybrid",
        }
    }

    /// The artifact kind byte this engine persists as (Section 8 format).
    pub fn artifact_kind(&self) -> u8 {
        match self {
            Self::ByteConv(_) => ARTIFACT_KIND_BYTECONV_FULL_TRAINING,
            Self::ByteBag(_) => ARTIFACT_KIND_BYTEBAG_FULL_TRAINING,
            Self::SeqFreqHybrid(_) => ARTIFACT_KIND_SEQ_FREQ_HYBRID_FULL_TRAINING,
        }
    }

    /// Signed vote sum V (semantics documented per engine).
    pub fn engine_vote_sum(&self, document: &[u8]) -> Result<i32, GranmoModelError> {
        match self {
            Self::ByteConv(conv_engine) => conv_engine.conv_vote_sum(document),
            Self::ByteBag(bag_engine) => bag_engine.bag_vote_sum(document),
            Self::SeqFreqHybrid(hybrid_engine) => hybrid_engine.hyb_vote_sum(document),
        }
    }

    /// Binary prediction: label 1 iff `V > decision_threshold`.
    pub fn engine_predict(
        &self,
        document: &[u8],
        decision_threshold: i32,
    ) -> Result<bool, GranmoModelError> {
        match self {
            Self::ByteConv(conv_engine) => conv_engine.conv_predict(document, decision_threshold),
            Self::ByteBag(bag_engine) => bag_engine.bag_predict(document, decision_threshold),
            Self::SeqFreqHybrid(hybrid_engine) => {
                hybrid_engine.hyb_predict(document, decision_threshold)
            }
        }
    }

    /// One stochastic training update for one document (dispatch),
    /// single-threaded. Equivalent to `engine_train_step_with_workers`
    /// with `WorkerCount::new(1)` at every worker count.
    pub fn engine_train_step(
        &mut self,
        document: &[u8],
        label_is_positive: bool,
        rng: &mut FastRng,
    ) -> Result<(), GranmoModelError> {
        self.engine_train_step_with_workers(document, label_is_positive, rng, WorkerCount::new(1)?)
    }

    /// One stochastic training update for one document (dispatch), with
    /// clause-level fork-join across `worker_count` threads. PERFORMANCE-
    /// ONLY: results are byte-identical at every worker count.
    pub fn engine_train_step_with_workers(
        &mut self,
        document: &[u8],
        label_is_positive: bool,
        rng: &mut FastRng,
        worker_count: WorkerCount,
    ) -> Result<(), GranmoModelError> {
        match self {
            Self::ByteConv(conv_engine) => conv_engine.conv_train_step_with_workers(
                document,
                label_is_positive,
                rng,
                worker_count,
            ),
            Self::ByteBag(bag_engine) => bag_engine.bag_train_step_with_workers(
                document,
                label_is_positive,
                rng,
                worker_count,
            ),
            Self::SeqFreqHybrid(hybrid_engine) => hybrid_engine.hyb_train_step_with_workers(
                document,
                label_is_positive,
                rng,
                worker_count,
            ),
        }
    }

    /// Fired-clause bitset (embedding deliverable §7.2; fire-rate input).
    pub fn engine_fired_clause_bits(&self, document: &[u8]) -> Result<Vec<u64>, GranmoModelError> {
        match self {
            Self::ByteConv(conv_engine) => conv_engine.conv_fired_clause_bits(document),
            Self::ByteBag(bag_engine) => bag_engine.bag_fired_clause_bits(document),
            Self::SeqFreqHybrid(hybrid_engine) => hybrid_engine.hyb_fired_clause_bits(document),
        }
    }

    /// Human-readable clause decode (explainability §7.3). Engine-specific
    /// SPAN reporting (e.g. conv window offsets) is deliberately NOT here:
    /// spans are positional concepts that not every engine has, so callers
    /// needing them match the variant (see `handle_predict`).
    pub fn engine_describe_clause(
        &self,
        clause: usize,
        max_rendered_literals: usize,
    ) -> Result<String, GranmoModelError> {
        match self {
            Self::ByteConv(conv_engine) => {
                conv_engine.conv_describe_clause(clause, max_rendered_literals)
            }
            Self::ByteBag(bag_engine) => {
                bag_engine.bag_describe_clause(clause, max_rendered_literals)
            }
            Self::SeqFreqHybrid(hybrid_engine) => {
                hybrid_engine.hyb_describe_clause(clause, max_rendered_literals)
            }
        }
    }

    /// Full derived-cache and state-band validation (artifact load gate 4).
    pub fn engine_validate_internal_consistency(&self) -> Result<(), GranmoModelError> {
        match self {
            Self::ByteConv(conv_engine) => conv_engine.conv_validate_internal_consistency(),
            Self::ByteBag(bag_engine) => bag_engine.bag_validate_internal_consistency(),
            Self::SeqFreqHybrid(hybrid_engine) => hybrid_engine.hyb_validate_internal_consistency(),
        }
    }

    /// Clause count (reporting; LUT sizing).
    pub fn engine_clause_count(&self) -> usize {
        match self {
            Self::ByteConv(conv_engine) => conv_engine.conv_clause_count(),
            Self::ByteBag(bag_engine) => bag_engine.bag_clause_count(),
            Self::SeqFreqHybrid(hybrid_engine) => hybrid_engine.hyb_clause_count(),
        }
    }

    /// Total training-time fire-guard resets (telemetry; Drop 4.1).
    /// The conv engine has no guard yet and reports 0 — the identical
    /// pattern ports to `ConvClauseWorkView` later (recorded backlog:
    /// conv vacuity test = zero includes across all slots, readable from
    /// `positive_include_counts` plus the negated-literal states).
    /// The hybrid reports its bag bank's resets.
    pub fn engine_fire_guard_reset_total(&self) -> u64 {
        match self {
            Self::ByteConv(_conv_engine) => 0,
            Self::ByteBag(bag_engine) => bag_engine.bag_fire_guard_reset_total(),
            Self::SeqFreqHybrid(hybrid_engine) => hybrid_engine.hyb_fire_guard_reset_total(),
        }
    }

    /// Per-clause included-literal totals (specialization/vacuity
    /// diagnostic; see the engine methods for the reading guide).
    pub fn engine_clause_include_totals(&self) -> Result<Vec<u32>, GranmoModelError> {
        match self {
            Self::ByteConv(conv_engine) => conv_engine.conv_clause_include_totals(),
            Self::ByteBag(bag_engine) => bag_engine.bag_clause_include_totals(),
            Self::SeqFreqHybrid(hybrid_engine) => hybrid_engine.hyb_clause_include_totals(),
        }
    }

    /// Probability LUT matched to this engine's clause count and T.
    pub fn engine_build_probability_lut(&self) -> Result<ProbabilityLut, GranmoModelError> {
        match self {
            Self::ByteConv(conv_engine) => conv_engine.conv_build_probability_lut(),
            Self::ByteBag(bag_engine) => bag_engine.bag_build_probability_lut(),
            Self::SeqFreqHybrid(hybrid_engine) => hybrid_engine.hyb_build_probability_lut(),
        }
    }
}

// ===========================================================================
// SECTION 5C: Shared Engine Training Math (Trn* codes, 1000-block)
// ===========================================================================

/// Integer feedback gates for one training step, precomputed once per
/// document from the vote sum V (clamped to ±T) and the vote target T.
///
/// Semantics — the (T∓V)/2T resource-allocation rule of the specification,
/// in exact integer form, identical for every engine:
/// - a clause receiving TARGET-consistent feedback is selected with
///   probability (T − V_clamped) / 2T: apply iff draw < `gate_when_target`;
/// - a clause receiving the OTHER feedback family is selected with
///   probability (T + V_clamped) / 2T: apply iff draw < `gate_when_other`;
/// - the draw is one integer r ∈ [0, 2T) from `FastRng::gen_index`.
///
/// Extracted from `ByteConvTM::train_step` (Drop 3.0b) so that ByteBagTM
/// (Drop 2.2b) uses the byte-identical gate implementation: the §8
/// comparison discipline requires the two engines to differ ONLY in
/// feature structure, never in shared training mechanics.
#[derive(Debug, Clone, Copy)]
struct FeedbackGates {
    /// Gate for the clause polarity aligned with the document's label.
    gate_when_target: i32,
    /// Gate for the opposite polarity.
    gate_when_other: i32,
    /// Draw range 2T (always ≥ 2, since `VoteThreshold` ≥ 1).
    two_t: usize,
}

/// Computes the gates. Overflow is mathematically unreachable here
/// (`VoteThreshold` ≤ 10000 bounds every intermediate at ≤ 20000), but the
/// arithmetic is checked regardless, per the coding rules.
fn resolve_feedback_gates(
    vote_sum: i32,
    vote_threshold: i32,
) -> Result<FeedbackGates, GranmoModelError> {
    let clamped_v = vote_sum.clamp(-vote_threshold, vote_threshold);
    let gate_when_target = vote_threshold
        .checked_sub(clamped_v)
        .ok_or(GranmoModelError::TrnGateArithmeticOverflow)?;
    let gate_when_other = vote_threshold
        .checked_add(clamped_v)
        .ok_or(GranmoModelError::TrnGateArithmeticOverflow)?;
    let two_t = usize::try_from(vote_threshold)
        .ok()
        .and_then(|threshold| threshold.checked_mul(2))
        .ok_or(GranmoModelError::TrnGateArithmeticOverflow)?;
    Ok(FeedbackGates {
        gate_when_target,
        gate_when_other,
        two_t,
    })
}

// ===========================================================================
// SECTION 6: Self-check entry point
// ===========================================================================
/// Minimal self-check exercised when the binary is run directly. Verifies
/// the P0 and P2 preprocessing pipelines on fixed inputs, then a ByteConvTM
/// construction + fresh-model invariant smoke test. On failure the process
/// exits with the numeric error code (production behavior: code out, no
/// strings); debug builds additionally print diagnostics.
fn run_self_check() -> Result<(), GranmoModelError> {
    let mut p0 = BytePreprocessor::new(PreprocessProfile::preset_p0())?;
    let out_p0 = p0.process_document(b"  HeLLo\n\nW0rld! ")?;
    if out_p0 != b"hello w0rld! " {
        return Err(GranmoModelError::PpProfileRecheckCorrupt);
    }

    let mut p2 = BytePreprocessor::new(PreprocessProfile::preset_p2())?;
    let out_p2 = p2.process_document(b"y0u $uck!!")?;
    if out_p2 != b"you suckii" {
        return Err(GranmoModelError::PpProfileRecheckCorrupt);
    }

    // Fresh-engine smoke: all clauses fire on any input (all-permissive
    // masks), so with balanced polarities the vote sum must be exactly 0.
    let engine = ByteConvTM::new(
        PatchSize::new(5)?,
        StrideLen::new(2)?,
        ClauseCount::new(20)?,
        VoteThreshold::new(15)?,
        StatesPerAction::new(100)?,
        SpecificityThresholds::from_specificity(3.0)?,
        MaxScanBytes::new(256)?,
        false,
    )?;
    if engine.conv_vote_sum(b"hello world")? != 0 {
        return Err(GranmoModelError::BctMaskCacheInconsistent);
    }
    engine.conv_validate_internal_consistency()?;

    #[cfg(debug_assertions)]
    eprintln!("self-check passed: preprocess pipelines and fresh ByteConvTM verified");
    Ok(())
}

// ===========================================================================
// SECTION 7: M-Prob — Probability LUT and Threshold-Sweep Reporting
// ===========================================================================

/// Precomputed vote-sum → probability lookup table (specification §6/§7.1).
///
/// ## Why a LUT
/// The vote sum V of a C-clause bank lies in `[-C/2, +C/2]` (each clause
/// contributes at most one vote under OR-pooling). Mapping V to a calibrated
/// probability therefore needs only `C + 1` precomputed entries. The table
/// is filled ONCE at model-save/config time — the only place float math is
/// permitted — with a fixed-point sigmoid; the runtime path is a single
/// array index returning a `u16` (probability × 65535). This keeps the
/// production inference path float-free end to end.
///
/// ## Calibration
/// `p(V) = sigmoid(V / tau)` with `tau = T/2` per the specification of
/// record. Platt-style recalibration is deferred (backlog §12) and would
/// only change how this table is FILLED — the runtime lookup is untouched,
/// which is exactly why the LUT indirection was chosen.
///
/// ## Value integrity
/// The table is monotone non-decreasing by construction; `.probability_u16()`
/// re-checks cheap structural facts (size, endpoint ordering) on every call,
/// and `.validity_recheck()` verifies full monotonicity, so post-construction
/// corruption surfaces as an error code rather than a silently wrong
/// probability.
#[derive(Debug, Clone)]
pub struct ProbabilityLut {
    /// `entries[V + half_range]` = probability of class 1 in units of 1/65535.
    entries: Vec<u16>,
    /// `C/2`: offset converting a signed vote into a table index.
    half_range: usize,
}

impl ProbabilityLut {
    /// Builds the table for a given clause count and vote threshold T.
    /// Offline/config-time float math is permitted here (and ONLY here on
    /// the probability path); see struct docs for the calibration formula.
    pub fn build(
        n_clauses: ClauseCount,
        vote_threshold: VoteThreshold,
    ) -> Result<Self, GranmoModelError> {
        let clause_total = usize::from(n_clauses.get()?);
        let threshold_t = f64::from(vote_threshold.get()?);
        let half_range = clause_total / 2;
        let table_len = clause_total
            .checked_add(1)
            .ok_or(GranmoModelError::PrbLutSizeInvalid)?;

        // tau = T/2; VoteThreshold guarantees T >= 1, so tau >= 0.5 and the
        // sigmoid argument is always finite.
        let tau = threshold_t / 2.0;

        let mut entries = Vec::with_capacity(table_len);
        for index in 0..table_len {
            // Signed vote this index represents: index - half_range.
            let vote_value = index as f64 - half_range as f64;
            let sigmoid = 1.0 / (1.0 + (-vote_value / tau).exp());
            // Fixed-point encode: 0.0 -> 0, 1.0 -> 65535, round-to-nearest.
            let fixed_point = (sigmoid * 65535.0).round().clamp(0.0, 65535.0) as u16;
            entries.push(fixed_point);
        }

        if entries.is_empty() {
            return Err(GranmoModelError::PrbLutSizeInvalid);
        }
        Ok(Self {
            entries,
            half_range,
        })
    }

    /// Runtime probability lookup: one bounds check + one array index,
    /// integer-only. Returns probability of class 1 in units of 1/65535.
    /// A vote outside `[-C/2, +C/2]` means the caller paired this LUT with
    /// a different-sized model — a wiring error, reported not clamped.
    pub fn probability_u16(&self, vote_sum: i32) -> Result<u16, GranmoModelError> {
        // Cheap structural recheck on every call (value-integrity rule):
        // a sigmoid table must be non-empty with first entry <= last entry.
        let first = *self
            .entries
            .first()
            .ok_or(GranmoModelError::PrbLutRecheckCorrupt)?;
        let last = *self
            .entries
            .last()
            .ok_or(GranmoModelError::PrbLutRecheckCorrupt)?;
        if first > last {
            return Err(GranmoModelError::PrbLutRecheckCorrupt);
        }

        let index = vote_sum
            .checked_add(self.half_range as i32)
            .ok_or(GranmoModelError::PrbVoteOutOfRange)?;
        if index < 0 {
            return Err(GranmoModelError::PrbVoteOutOfRange);
        }
        self.entries
            .get(index as usize)
            .copied()
            .ok_or(GranmoModelError::PrbVoteOutOfRange)
    }

    /// Full-table monotonicity recheck (value-integrity rule): call after
    /// artifact load or on demand in long-running processes. O(C), so it is
    /// deliberately NOT run per prediction — `probability_u16` performs the
    /// cheap endpoint check instead.
    pub fn lut_validity_recheck(&self) -> Result<(), GranmoModelError> {
        if self.entries.is_empty() {
            return Err(GranmoModelError::PrbLutRecheckCorrupt);
        }
        for pair in self.entries.windows(2) {
            if pair[0] > pair[1] {
                return Err(GranmoModelError::PrbLutRecheckCorrupt);
            }
        }
        Ok(())
    }

    /// Reporting convenience: fixed-point probability as f64 in [0, 1].
    /// Offline evaluation/reporting path ONLY (floats permitted there, §6).
    pub fn probability_for_report(&self, vote_sum: i32) -> Result<f64, GranmoModelError> {
        Ok(f64::from(self.probability_u16(vote_sum)?) / 65535.0)
    }
}

/// One row of a decision-threshold sweep: full confusion counts plus derived
/// metrics at `predicted_positive := (V > decision_threshold)`.
///
/// The sweep exists because the dataset is expected to be class-imbalanced
/// (§8): the integer decision boundary is a FREE post-training knob (no
/// retraining needed), and this table is how M-Ablate reports pick their
/// operating point. Floats appear only in the derived metric fields —
/// offline reporting, permitted by the compute policy.
#[derive(Debug, Clone, PartialEq)]
pub struct ThresholdSweepRow {
    pub decision_threshold: i32,
    pub true_positives: usize,
    pub false_positives: usize,
    pub true_negatives: usize,
    pub false_negatives: usize,
    pub precision: f64,
    pub recall: f64,
    pub f1: f64,
}

/// Sweeps every meaningful integer decision threshold over a set of scored
/// documents. Thresholds run from `min(votes) - 1` (everything predicted
/// positive) through `max(votes)` (nothing predicted positive), so the full
/// precision/recall trade-off curve is covered with no gaps and no waste.
///
/// Inputs are parallel slices: `vote_sums[i]` is the engine's vote for the
/// document whose ground-truth label is `labels[i]` (true = class 1).
pub fn sweep_decision_thresholds(
    vote_sums: &[i32],
    labels: &[bool],
) -> Result<Vec<ThresholdSweepRow>, GranmoModelError> {
    if vote_sums.is_empty() {
        return Err(GranmoModelError::PrbSweepEmptyInput);
    }
    if vote_sums.len() != labels.len() {
        return Err(GranmoModelError::PrbSweepLengthMismatch);
    }

    // min/max over a non-empty slice: fold avoids unwrap on Option.
    let min_vote = vote_sums.iter().fold(i32::MAX, |acc, &v| acc.min(v));
    let max_vote = vote_sums.iter().fold(i32::MIN, |acc, &v| acc.max(v));
    let sweep_start = min_vote.saturating_sub(1);

    let mut rows = Vec::new();
    let mut candidate = sweep_start;
    while candidate <= max_vote {
        let mut true_positives = 0usize;
        let mut false_positives = 0usize;
        let mut true_negatives = 0usize;
        let mut false_negatives = 0usize;

        for (&vote, &is_positive) in vote_sums.iter().zip(labels.iter()) {
            let predicted_positive = vote > candidate;
            match (predicted_positive, is_positive) {
                (true, true) => true_positives += 1,
                (true, false) => false_positives += 1,
                (false, false) => true_negatives += 1,
                (false, true) => false_negatives += 1,
            }
        }

        // Metric derivations with explicit zero-denominator guards.
        let precision_denominator = true_positives + false_positives;
        let precision = if precision_denominator > 0 {
            true_positives as f64 / precision_denominator as f64
        } else {
            0.0
        };
        let recall_denominator = true_positives + false_negatives;
        let recall = if recall_denominator > 0 {
            true_positives as f64 / recall_denominator as f64
        } else {
            0.0
        };
        let f1 = if precision + recall > 0.0 {
            2.0 * precision * recall / (precision + recall)
        } else {
            0.0
        };

        rows.push(ThresholdSweepRow {
            decision_threshold: candidate,
            true_positives,
            false_positives,
            true_negatives,
            false_negatives,
            precision,
            recall,
            f1,
        });

        candidate = match candidate.checked_add(1) {
            Some(next) => next,
            None => break,
        };
    }
    Ok(rows)
}

// ===========================================================================
// SECTION 8: Artifact I/O — Kind-Dispatched Binary Artifacts (Art* codes)
// ===========================================================================
//
// Binary format, version 2, little-endian throughout.
//
// COMMON PRELUDE (all kinds — the loader dispatches on `kind` and never
// needs any engine's header layout to do so):
//
//   offset  size  field
//   0       8     magic  b"GRANMOB1"
//   8       2     format version (u16) = 2
//   10      1     artifact kind (u8):
//                   1 = ByteConv full training (raw automaton states)
//                   2 = compact inference        (RESERVED, M-Prod-Pass)
//                   3 = ByteBag full training    (vocabulary + raw states)
//   11      1     reserved, must be 0
//   12      2     preprocess profile bits (u16)              [locked §10.9]
//
// KIND-1 BODY (ByteConv full training; engine header is 18 bytes, so the
// state payload begins at offset 32):
//
//   14      1     patch size K (u8)
//   15      1     stride S (u8)
//   16      2     clause count (u16)
//   18      2     vote threshold T (i16)
//   20      2     states per action N (i16)
//   22      2     forget threshold (u16)
//   24      2     reinforce threshold (u16)
//   26      4     max scan bytes (u32)
//   30      1     guarded-include flag (u8: 0/1)
//   31      1     reserved, must be 0
//   32      2*L   automaton states (i16 each), L = clauses * 2 * K * 256
//
// KIND-3 BODY (ByteBag full training; engine header is also 18 bytes, so
// the variable-length sections begin at offset 32):
//
//   14      1     n-gram length n (u8)
//   15      1     reserved, must be 0
//   16      2     vocabulary count M (u16, >= 1): the ACTUAL learned
//                 vocabulary size, which may be smaller than the requested
//                 VocabSize cap — that is a recorded property, not an error
//   18      2     clause count (u16)
//   20      2     vote threshold T (i16)
//   22      2     states per action N (i16)
//   24      2     forget threshold (u16)
//   26      2     reinforce threshold (u16)
//   28      4     max scan bytes (u32)
//   32      M*n   vocabulary flat bytes: rank-ordered, fixed width, no
//                 length prefixes (rank r occupies [r*n, (r+1)*n))
//   32+M*n  2*L   automaton states (i16 each), L = clauses * 2 * M
//
// TRAILER (all kinds):
//   end     8     FNV-1a-64 checksum over ALL preceding bytes (u64)
//
//   10      1     artifact kind (u8):
//                   ...
//                   4 = Hybrid full training     (shared T + kind-1 body +
//                                                 kind-3 body, verbatim)
//
// KIND-4 BODY (seq-freq hybrid full training):
//
//   14      2     shared vote target T (i16)
//   16      ...   kind-1 body, verbatim (18-byte engine header at 16..34,
//                 conv automaton states from 34)
//   ...     ...   kind-3 body, verbatim (18-byte header, vocabulary flat
//                 bytes, bag automaton states)
//
// Format version stays 2: adding a kind does not change the prelude, and
// older builds reject kind 4 through the existing ArtKindUnsupported gate.
//
//
// Design notes:
// - v1 -> v2: v1 placed the kind byte inside the ByteConv header, which
//   could not generalize to a second engine; v2 hoists kind into the
//   common prelude. v1 files are rejected by the version gate; artifacts
//   are regenerable experiment outputs, no migration path by design.
// - Derived data is NEVER stored: conv masks/counts, bag include masks,
//   and the bag vocabulary's byte-sorted lookup order are all rebuilt from
//   raw persisted data at load and then cross-checked by full consistency
//   validation (load gate 4).
// - Specificity round-trips as the exact integer thresholds (never as the
//   float `s`), so a loaded model is bit-identical in behavior.
// - The probability LUT is not stored in FULL artifacts (rebuilt from the
//   header by the harness). The compact inference artifact (kind 2,
//   M-Prod-Pass) WILL embed the LUT verbatim to keep production load
//   float-free.

/// File magic identifying a Granmo Model artifact, format family B.
const ARTIFACT_MAGIC: [u8; 8] = *b"GRANMOB1";
/// Current binary format version (v2: kind-dispatched common prelude).
const ARTIFACT_FORMAT_VERSION: u16 = 2;
/// Artifact kind byte: ByteConv full-training artifact (raw states).
const ARTIFACT_KIND_BYTECONV_FULL_TRAINING: u8 = 1;
/// Artifact kind byte: ByteBag full-training artifact (vocabulary flat
/// bytes + raw automaton states). Kind 2 remains reserved for the compact
/// inference artifact (M-Prod-Pass).
const ARTIFACT_KIND_BYTEBAG_FULL_TRAINING: u8 = 3;

/// Artifact kind byte: seq-freq hybrid full-training artifact (shared T,
/// then the kind-1 body and the kind-3 body verbatim).
const ARTIFACT_KIND_SEQ_FREQ_HYBRID_FULL_TRAINING: u8 = 4;

/// FNV-1a 64-bit hash. `wrapping_mul` is the DEFINED behavior of this hash
/// (modular arithmetic), not an unchecked-arithmetic violation: wrapping is
/// explicit, intentional, and cannot panic.
fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}

/// Bounds-checked sequential reader over a loaded artifact buffer. Every
/// read returns `ArtTruncated` instead of panicking if the file is short —
/// truncated/corrupt files are an EXPECTED production input, not a bug.
struct ByteCursor<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> ByteCursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    /// Reads exactly `count` bytes, advancing the cursor.
    fn take(&mut self, count: usize) -> Result<&'a [u8], GranmoModelError> {
        let end = self
            .position
            .checked_add(count)
            .ok_or(GranmoModelError::ArtTruncated)?;
        let slice = self
            .data
            .get(self.position..end)
            .ok_or(GranmoModelError::ArtTruncated)?;
        self.position = end;
        Ok(slice)
    }

    fn read_u8(&mut self) -> Result<u8, GranmoModelError> {
        Ok(self.take(1)?[0])
    }

    fn read_u16_le(&mut self) -> Result<u16, GranmoModelError> {
        let bytes = self.take(2)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn read_i16_le(&mut self) -> Result<i16, GranmoModelError> {
        let bytes = self.take(2)?;
        Ok(i16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn read_u32_le(&mut self) -> Result<u32, GranmoModelError> {
        let bytes = self.take(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}
/// Writes the kind-1 (ByteConv full-training) body: 18-byte engine header +
/// raw i16 states. The engine's usize fields were validated into these
/// ranges at construction, so the narrowing conversions cannot truncate —
/// still guarded via `try_from`/range checks as defence-in-depth.
fn write_byte_conv_full_body(
    engine: &ByteConvTM,
    buffer: &mut Vec<u8>,
) -> Result<(), GranmoModelError> {
    if engine.patch_size > usize::from(u8::MAX)
        || engine.stride > usize::from(u8::MAX)
        || engine.n_clauses > usize::from(u16::MAX)
        || engine.max_scan_bytes > u32::MAX as usize
    {
        return Err(GranmoModelError::BctIndexOutOfRange);
    }
    let vote_threshold_i16 =
        i16::try_from(engine.vote_threshold).map_err(|_| GranmoModelError::BctIndexOutOfRange)?;

    buffer.reserve(engine.ta_states.len().saturating_mul(2).saturating_add(18));
    buffer.push(engine.patch_size as u8);
    buffer.push(engine.stride as u8);
    buffer.extend_from_slice(&(engine.n_clauses as u16).to_le_bytes());
    buffer.extend_from_slice(&vote_threshold_i16.to_le_bytes());
    buffer.extend_from_slice(&engine.states_per_action.to_le_bytes());
    buffer.extend_from_slice(&engine.forget_threshold_u16.to_le_bytes());
    buffer.extend_from_slice(&engine.reinforce_threshold_u16.to_le_bytes());
    buffer.extend_from_slice(&(engine.max_scan_bytes as u32).to_le_bytes());
    buffer.push(u8::from(engine.guarded_include));
    buffer.push(0u8); // reserved
    for &state in &engine.ta_states {
        buffer.extend_from_slice(&state.to_le_bytes());
    }
    Ok(())
}

/// Parses the kind-1 (ByteConv full-training) body: every header value
/// passes through its enforced-type constructor (load gate 2), raw states
/// are read, derived caches rebuilt (gate 3). Gate 4 — full consistency
/// validation — runs in the COMMON load path via the engine enum, because
/// every kind must pass it.
fn read_byte_conv_full_body(cursor: &mut ByteCursor<'_>) -> Result<ByteConvTM, GranmoModelError> {
    let patch_size = PatchSize::new(cursor.read_u8()?)?;
    let stride = StrideLen::new(cursor.read_u8()?)?;
    let n_clauses = ClauseCount::new(cursor.read_u16_le()?)?;
    let vote_threshold = VoteThreshold::new(cursor.read_i16_le()?)?;
    let states_per_action = StatesPerAction::new(cursor.read_i16_le()?)?;
    let forget_threshold = cursor.read_u16_le()?;
    let reinforce_threshold = cursor.read_u16_le()?;
    let specificity =
        SpecificityThresholds::from_raw_thresholds(forget_threshold, reinforce_threshold)?;
    let max_scan_bytes = MaxScanBytes::new(cursor.read_u32_le()?)?;
    let guarded_include = match cursor.read_u8()? {
        0 => false,
        1 => true,
        _ => return Err(GranmoModelError::ArtHeaderFieldInvalid),
    };
    if cursor.read_u8()? != 0 {
        return Err(GranmoModelError::ArtHeaderFieldInvalid);
    }

    let mut engine = ByteConvTM::new(
        patch_size,
        stride,
        n_clauses,
        vote_threshold,
        states_per_action,
        specificity,
        max_scan_bytes,
        guarded_include,
    )?;
    let expected_state_count = engine.ta_states.len();
    let state_bytes = cursor.take(
        expected_state_count
            .checked_mul(2)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?,
    )?;
    for (state_slot, chunk) in engine.ta_states.iter_mut().zip(state_bytes.chunks_exact(2)) {
        *state_slot = i16::from_le_bytes([chunk[0], chunk[1]]);
    }
    engine.rebuild_caches_from_states()?;
    Ok(engine)
}

/// Writes the kind-3 (ByteBag full-training) body: 18-byte engine header +
/// vocabulary flat bytes + raw i16 states (format table in the section
/// banner). Every narrowing conversion is guarded via `try_from` even
/// though construction-time validation makes truncation unreachable
/// (defence-in-depth, same posture as the kind-1 writer).
fn write_byte_bag_full_body(
    bag_engine: &ByteBagTM,
    buffer: &mut Vec<u8>,
) -> Result<(), GranmoModelError> {
    let ngram_len_u8 = u8::try_from(bag_engine.bag_vocabulary.ngram_length())
        .map_err(|_| GranmoModelError::BbgEngineIndexOutOfRange)?;
    let vocab_count_u16 = u16::try_from(bag_engine.bag_vocabulary.vocabulary_len())
        .map_err(|_| GranmoModelError::BbgEngineIndexOutOfRange)?;
    let clause_count_u16 = u16::try_from(bag_engine.bag_clause_total)
        .map_err(|_| GranmoModelError::BbgEngineIndexOutOfRange)?;
    let vote_target_i16 = i16::try_from(bag_engine.bag_vote_target)
        .map_err(|_| GranmoModelError::BbgEngineIndexOutOfRange)?;
    let scan_cap_u32 = u32::try_from(bag_engine.bag_scan_cap_bytes)
        .map_err(|_| GranmoModelError::BbgEngineIndexOutOfRange)?;

    // Same-module private-field access, same pattern as the kind-1 writer's
    // access to conv engine internals: the writer IS part of the engine's
    // persistence contract.
    let vocabulary_flat = &bag_engine.bag_vocabulary.ngram_flat_bytes;

    buffer.reserve(
        18usize
            .saturating_add(vocabulary_flat.len())
            .saturating_add(bag_engine.bag_ta_states.len().saturating_mul(2)),
    );
    buffer.push(ngram_len_u8);
    buffer.push(0u8); // reserved
    buffer.extend_from_slice(&vocab_count_u16.to_le_bytes());
    buffer.extend_from_slice(&clause_count_u16.to_le_bytes());
    buffer.extend_from_slice(&vote_target_i16.to_le_bytes());
    buffer.extend_from_slice(&bag_engine.bag_automaton_depth.to_le_bytes());
    buffer.extend_from_slice(&bag_engine.bag_forget_coin_threshold.to_le_bytes());
    buffer.extend_from_slice(&bag_engine.bag_reinforce_coin_threshold.to_le_bytes());
    buffer.extend_from_slice(&scan_cap_u32.to_le_bytes());
    buffer.extend_from_slice(vocabulary_flat);
    for &state in &bag_engine.bag_ta_states {
        buffer.extend_from_slice(&state.to_le_bytes());
    }
    Ok(())
}

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
    // Fire guard: DISABLED by design — guard limit, streaks, and reset
    // counters are ephemeral training-session state and are deliberately
    // not part of artifact kind 3 (Drop 4.1 decision of record). A loaded
    // model resumes training guard-off unless the caller reconstructs it
    // with an active limit.
    let mut bag_engine = ByteBagTM::new_with_vocabulary(
        restored_vocabulary,
        clause_count,
        vote_target,
        automaton_depth,
        specificity,
        scan_cap,
        FireGuardStreakLimit::new(FireGuardStreakLimit::DISABLED)?,
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

/// Writes the kind-4 (seq-freq hybrid full-training) body by composing the
/// two existing body writers behind the 2-byte shared vote target. No new
/// field semantics are introduced: every sub-engine field round-trips
/// through exactly the writer that persists it standalone.
fn write_seq_freq_hybrid_full_body(
    hybrid_engine: &HybridTM,
    buffer: &mut Vec<u8>,
) -> Result<(), GranmoModelError> {
    let shared_target_i16 = i16::try_from(hybrid_engine.hyb_vote_target)
        .map_err(|_| GranmoModelError::HybIndexOutOfRange)?;
    buffer.extend_from_slice(&shared_target_i16.to_le_bytes());
    write_byte_conv_full_body(hybrid_engine.hyb_conv_engine_ref(), buffer)?;
    write_byte_bag_full_body(hybrid_engine.hyb_bag_engine_ref(), buffer)
}

/// Parses the kind-4 body: shared T through `VoteThreshold` (gate 2), then
/// each sub-engine through its own body reader (gates 2 + 3 per bank), then
/// composition through the constructor, which re-validates both banks.
/// Gate 4 runs again in the common load path via the engine enum.
fn read_seq_freq_hybrid_full_body(
    cursor: &mut ByteCursor<'_>,
) -> Result<HybridTM, GranmoModelError> {
    let shared_target = VoteThreshold::new(cursor.read_i16_le()?)?;
    let conv_engine = read_byte_conv_full_body(cursor)?;
    let bag_engine = read_byte_bag_full_body(cursor)?;
    HybridTM::new_from_sub_engines(conv_engine, bag_engine, shared_target)
}

/// The persisted unit: engine + the preprocessing profile it was trained
/// with, coupled in ONE artifact so inference can never accidentally replay
/// the wrong preprocessing (locked decision §10.9 made structural). The
/// engine is the polymorphic `ClassifierEngine`: ONE save/load path serves
/// every engine kind — the common prelude carries the kind byte, and each
/// kind supplies only its private body writer/reader.
#[derive(Debug, Clone)]
pub struct ModelArtifact {
    pub preprocess_profile: PreprocessProfile,
    pub engine: ClassifierEngine,
}

impl ModelArtifact {
    /// Serializes per the Section 8 format and writes to `absolute_path`.
    /// The path must be absolute (crate policy); filesystem failure detail
    /// is dropped (no-PII policy) and reported as the retryable
    /// `ArtFileWriteFailed` — callers may Tier-1 retry.
    pub fn save_to_file(&self, absolute_path: &std::path::Path) -> Result<(), GranmoModelError> {
        if !absolute_path.is_absolute() {
            #[cfg(debug_assertions)]
            eprintln!("ART-500: path not absolute: {}", absolute_path.display());
            return Err(GranmoModelError::ArtPathNotAbsolute);
        }
        let profile_bits = self.preprocess_profile.get_bits()?;

        let mut buffer: Vec<u8> = Vec::with_capacity(64);
        buffer.extend_from_slice(&ARTIFACT_MAGIC);
        buffer.extend_from_slice(&ARTIFACT_FORMAT_VERSION.to_le_bytes());
        buffer.push(self.engine.artifact_kind());
        buffer.push(0u8); // reserved
        buffer.extend_from_slice(&profile_bits.to_le_bytes());

        match &self.engine {
            ClassifierEngine::ByteConv(conv_engine) => {
                write_byte_conv_full_body(conv_engine, &mut buffer)?
            }
            ClassifierEngine::ByteBag(bag_engine) => {
                write_byte_bag_full_body(bag_engine, &mut buffer)?
            }
            ClassifierEngine::SeqFreqHybrid(hybrid_engine) => {
                write_seq_freq_hybrid_full_body(hybrid_engine, &mut buffer)?
            }
        }

        let checksum = fnv1a_64(&buffer);
        buffer.extend_from_slice(&checksum.to_le_bytes());

        match std::fs::write(absolute_path, &buffer) {
            Ok(()) => Ok(()),
            Err(_dropped_io_detail) => {
                #[cfg(debug_assertions)]
                eprintln!("ART-501: write failed: {}", _dropped_io_detail);
                Err(GranmoModelError::ArtFileWriteFailed)
            }
        }
    }

    /// Loads and fully validates an artifact, four gates in order:
    /// 1. checksum over the raw bytes (bit-rot/truncation caught first);
    /// 2. common prelude + kind dispatch, then the kind's body parse with
    ///    every config value re-bounded through its enforced type;
    /// 3. derived-cache rebuild from raw persisted data (conv masks/counts,
    ///    bag include masks, bag vocabulary lookup order — never trusted
    ///    from disk);
    /// 4. `validate_internal_consistency` via the engine enum — every kind
    ///    must pass it, so it runs in the common path.
    /// A model that survives all four is behaviorally identical to the one
    /// that was saved.
    pub fn load_from_file(absolute_path: &std::path::Path) -> Result<Self, GranmoModelError> {
        if !absolute_path.is_absolute() {
            #[cfg(debug_assertions)]
            eprintln!("ART-500: path not absolute: {}", absolute_path.display());
            return Err(GranmoModelError::ArtPathNotAbsolute);
        }

        let raw = match std::fs::read(absolute_path) {
            Ok(bytes) => bytes,
            Err(_dropped_io_detail) => {
                #[cfg(debug_assertions)]
                eprintln!("ART-502: read failed: {}", _dropped_io_detail);
                return Err(GranmoModelError::ArtFileReadFailed);
            }
        };

        // Gate 1: checksum. Last 8 bytes = stored FNV-1a-64 of the rest.
        if raw.len() < 8 {
            return Err(GranmoModelError::ArtTruncated);
        }
        let payload_len = raw.len() - 8;
        let payload = raw
            .get(..payload_len)
            .ok_or(GranmoModelError::ArtTruncated)?;
        let stored_checksum_bytes = raw
            .get(payload_len..)
            .ok_or(GranmoModelError::ArtTruncated)?;
        let mut checksum_array = [0u8; 8];
        checksum_array.copy_from_slice(stored_checksum_bytes);
        if fnv1a_64(payload) != u64::from_le_bytes(checksum_array) {
            return Err(GranmoModelError::ArtChecksumMismatch);
        }

        // Gate 2: common prelude, then kind dispatch into the body parser.
        let mut cursor = ByteCursor::new(payload);
        if cursor.take(8)? != ARTIFACT_MAGIC {
            return Err(GranmoModelError::ArtMagicMismatch);
        }
        if cursor.read_u16_le()? != ARTIFACT_FORMAT_VERSION {
            return Err(GranmoModelError::ArtVersionUnsupported);
        }
        let kind = cursor.read_u8()?;
        if cursor.read_u8()? != 0 {
            return Err(GranmoModelError::ArtHeaderFieldInvalid);
        }
        let profile = PreprocessProfile::from_bits(cursor.read_u16_le()?)?;

        // Gates 2 (body) + 3 (cache rebuild) happen inside the kind parser.
        let engine = match kind {
            ARTIFACT_KIND_BYTECONV_FULL_TRAINING => {
                ClassifierEngine::ByteConv(read_byte_conv_full_body(&mut cursor)?)
            }
            ARTIFACT_KIND_BYTEBAG_FULL_TRAINING => {
                ClassifierEngine::ByteBag(read_byte_bag_full_body(&mut cursor)?)
            }
            ARTIFACT_KIND_SEQ_FREQ_HYBRID_FULL_TRAINING => {
                ClassifierEngine::SeqFreqHybrid(read_seq_freq_hybrid_full_body(&mut cursor)?)
            }
            _unsupported => {
                #[cfg(debug_assertions)]
                eprintln!("ART-507: unsupported artifact kind {}", _unsupported);
                return Err(GranmoModelError::ArtKindUnsupported);
            }
        };

        // Common structural gate: the body must consume the payload EXACTLY;
        // trailing bytes mean header and body disagree about size.
        if cursor.position != payload.len() {
            return Err(GranmoModelError::ArtStateCountMismatch);
        }

        // Gate 4: full consistency validation, engine-agnostic.
        engine.engine_validate_internal_consistency()?;

        Ok(Self {
            preprocess_profile: profile,
            engine,
        })
    }
}

impl ByteConvTM {
    /// Rebuilds the positive-include counts and allowed-bytes masks purely
    /// from raw automaton states. Artifact-load path: derived caches are
    /// never trusted from disk (they are not even stored). After this,
    /// `validate_internal_consistency` must pass by construction — it is
    /// still run as the final load gate (defence-in-depth against a bug in
    /// THIS function). Depth is read per clause through `depth_for_clause`
    /// (the M-Hetero seam), matching the validation loop's structure.
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
                let slot_idx = self.global_slot_index(clause, slot)?;
                *self
                    .positive_include_counts
                    .get_mut(slot_idx)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)? = recomputed_count;

                // Recompute the mask from states.
                self.recompute_mask(clause, slot)?;
            }
        }
        Ok(())
    }
}

impl ByteBagTM {
    /// Rebuilds both include masks purely from raw automaton states
    /// (artifact-load path, gate 3). Masks are zeroed and re-derived
    /// through the SAME view transition path used by live training, so
    /// there is exactly one implementation of include-bit semantics.
    /// After this, `bag_validate_internal_consistency` must pass by
    /// construction — it is still run as the final load gate
    /// (defence-in-depth, mirroring `rebuild_caches_from_states`).
    fn bag_rebuild_masks_from_states(&mut self) -> Result<(), GranmoModelError> {
        for word in self.bag_positive_include_masks.iter_mut() {
            *word = 0;
        }
        for word in self.bag_negated_include_masks.iter_mut() {
            *word = 0;
        }
        let mut clause_views = self.bag_build_all_clause_views()?;
        for clause_view in clause_views.iter_mut() {
            clause_view.view_bag_rebuild_include_bits()?;
        }
        Ok(())
    }
}

// ===========================================================================
// SECTION 9: Dataset Ingestion (research-harness tier)
// ===========================================================================
//
// Tier note: this section is research-harness code. Data-path heap use and
// whole-file reads are its purpose; ERROR paths remain code-only/no-PII like
// everything else. Documents are kept as raw BYTES end to end — the model is
// byte-level, so no UTF-8 decode is performed (or needed) on document text.

/// One labeled example: raw document bytes + binary label + source line.
///
/// `line_index` is the 1-based line number in the source JSONL file. It is
/// the row's stable identity across every split, fold, and seed — the join
/// key for the per-row prediction records and the row audit. Positional,
/// not PII. It travels WITH the document through shuffles and splits, so
/// no side table is ever needed to recover which row a prediction was for.
#[derive(Debug, Clone)]
pub struct LabeledDocument {
    pub line_index: usize,
    pub text: Vec<u8>,
    pub label_is_positive: bool,
}

/// Seeded in-place Fisher–Yates shuffle (high index downward), generic over
/// element type. This is the SINGLE shuffle implementation in the crate,
/// used by `split_dataset` (shuffling documents) and by the per-epoch
/// training-order shuffle in `run_single_experiment` (shuffling indices).
/// One implementation means the two sites cannot drift apart — and the
/// draw sequence is part of the crate's determinism contract: identical
/// seeds must give identical splits and identical training order.
fn shuffle_in_place<T>(items: &mut [T], rng: &mut FastRng) -> Result<(), GranmoModelError> {
    let mut i = items.len();
    while i > 1 {
        i -= 1;
        // i <= len - 1 here, so i + 1 <= len: this addition cannot overflow.
        let j = rng.gen_index(i + 1)?;
        items.swap(i, j);
    }
    Ok(())
}

/// Seeded Fisher–Yates shuffle + split. `train_percent` is an INTEGER
/// percentage (1..=99) — no float enters split geometry, so identical
/// seeds give byte-identical splits across machines (the §8 requirement
/// that every comparison-matrix run share one split).
pub fn split_dataset(
    documents: &[LabeledDocument],
    train_percent: u8,
    rng: &mut FastRng,
) -> Result<(Vec<LabeledDocument>, Vec<LabeledDocument>), GranmoModelError> {
    if !(1..=99).contains(&train_percent) {
        return Err(GranmoModelError::DsSplitRatioInvalid);
    }
    let mut shuffled: Vec<LabeledDocument> = documents.to_vec();
    shuffle_in_place(&mut shuffled, rng)?;
    let train_count = shuffled
        .len()
        .checked_mul(usize::from(train_percent))
        .ok_or(GranmoModelError::BctArithmeticOverflow)?
        / 100;
    if train_count == 0 || train_count == shuffled.len() {
        return Err(GranmoModelError::DsSplitEmptySide);
    }
    let test_side = shuffled.split_off(train_count);
    Ok((shuffled, test_side))
}

/// Supported k-fold range. Below 2 is not a fold; above 50 is a typo for
/// any dataset this crate targets (fold test sides become too small to
/// carry a threshold sweep).
pub const KFOLD_MIN: u8 = 2;
pub const KFOLD_MAX: u8 = 50;

/// Seeded k-fold partition. ONE shuffle under `rng`, then fold `i` is the
/// contiguous slice `[i*n/k, (i+1)*n/k)` of the shuffled order; its train
/// side is everything else. Every row is on exactly one test side, so
/// out-of-fold predictions are evenly sampled — the property the per-row
/// audit depends on and that repeated random splits cannot provide.
///
/// Returns k `(train, test)` pairs in fold order. Owned copies (harness
/// tier, data path): k × dataset memory, acceptable at this crate's
/// corpus sizes; an index-range form is the optimization if that changes.
pub fn split_dataset_kfold(
    documents: &[LabeledDocument],
    fold_count: u8,
    rng: &mut FastRng,
) -> Result<Vec<(Vec<LabeledDocument>, Vec<LabeledDocument>)>, GranmoModelError> {
    if fold_count < KFOLD_MIN || fold_count > KFOLD_MAX {
        #[cfg(debug_assertions)]
        eprintln!(
            "DS-713: fold count {} outside {}..={}",
            fold_count, KFOLD_MIN, KFOLD_MAX
        );
        return Err(GranmoModelError::DsFoldCountInvalid);
    }
    let k = usize::from(fold_count);
    let total = documents.len();
    if total < k {
        return Err(GranmoModelError::DsFoldGeometryFault);
    }

    let mut shuffled: Vec<LabeledDocument> = documents.to_vec();
    shuffle_in_place(&mut shuffled, rng)?;

    let mut folds = Vec::with_capacity(k);
    for fold_index in 0..k {
        let start = fold_index
            .checked_mul(total)
            .ok_or(GranmoModelError::DsFoldGeometryFault)?
            / k;
        let end = fold_index
            .checked_add(1)
            .and_then(|next| next.checked_mul(total))
            .ok_or(GranmoModelError::DsFoldGeometryFault)?
            / k;
        if start >= end || end > total {
            return Err(GranmoModelError::DsFoldGeometryFault);
        }
        let test_side = shuffled
            .get(start..end)
            .ok_or(GranmoModelError::DsFoldGeometryFault)?
            .to_vec();
        let mut train_side: Vec<LabeledDocument> = Vec::with_capacity(total - (end - start));
        train_side.extend_from_slice(
            shuffled
                .get(..start)
                .ok_or(GranmoModelError::DsFoldGeometryFault)?,
        );
        train_side.extend_from_slice(
            shuffled
                .get(end..)
                .ok_or(GranmoModelError::DsFoldGeometryFault)?,
        );
        if train_side.is_empty() {
            return Err(GranmoModelError::DsFoldGeometryFault);
        }
        folds.push((train_side, test_side));
    }
    Ok(folds)
}

// ---------------------------------------------------------------------------
// SECTION 9B: JSONL Ingestion (research-harness tier, zero dependencies)
// ---------------------------------------------------------------------------
//
// Scope note (design decision of record): this is NOT a general JSON parser.
// It is a JSONL *record extractor* for the fixed schema of this project —
// one object per line carrying a "text" string and a "label" value — with
// exactly enough JSON machinery to be robust against real exported data:
//   - unknown keys are tolerated and skipped (exports carry extra fields);
//   - string escapes are fully decoded, INCLUDING \uXXXX surrogate pairs,
//     because social-media exports escape emoji that way and the model is
//     byte-level (an undecoded escape would corrupt the byte stream);
//   - raw UTF-8 (unescaped emoji/scripts) passes through untouched,
//     consistent with locked decision §10.5.
//
// Skip-vs-error policy:
//   SKIP  (record dropped, run proceeds): missing "text"/"label" key,
//         empty text, non-string text value, JSON null label, blank line.
//   ERROR (whole load rejected): structurally malformed JSON on any line.
//         Malformed structure means the file is not what the operator
//         thinks it is — proceeding would silently train on garbage.
//
// Label semantics: the label value's decoded token bytes are compared
// against `positive_label` byte-for-byte. `"label": 1` and `"label": "1"`
// are therefore identical. Numeric tokens are NOT parsed as numbers
// ("1.0" != "1" by design — the loader stays float-free and exact).
// Duplicate keys: last occurrence wins (documented, tested behavior).

/// Sequential cursor over one JSONL line. Bounds-safe by construction:
/// every access goes through `peek`/`advance`, which cannot overrun.
struct JsonLineCursor<'a> {
    line: &'a [u8],
    position: usize,
}

impl<'a> JsonLineCursor<'a> {
    fn new(line: &'a [u8]) -> Self {
        Self { line, position: 0 }
    }

    /// Current byte without consuming it; `None` at end of line.
    #[inline(always)]
    fn peek(&self) -> Option<u8> {
        self.line.get(self.position).copied()
    }

    /// Consumes one byte. Saturating: advancing past the end is harmless
    /// (subsequent `peek` returns `None`), never a panic.
    #[inline(always)]
    fn advance(&mut self) {
        self.position = self.position.saturating_add(1);
    }

    /// Skips JSON insignificant whitespace (space, tab, CR; LF cannot
    /// appear — lines were split on it before the cursor ever sees them).
    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ') | Some(b'\t') | Some(b'\r')) {
            self.advance();
        }
    }
}

/// Parses exactly four hex digits at the cursor into a u32 (the XXXX of a
/// \uXXXX escape). Short input or a non-hex digit is a bad-unicode-escape
/// error, never a panic.
fn parse_four_hex_digits(cursor: &mut JsonLineCursor<'_>) -> Result<u32, GranmoModelError> {
    let mut value: u32 = 0;
    for _digit_index in 0..4 {
        let byte = match cursor.peek() {
            Some(b) => b,
            None => return Err(GranmoModelError::DsJsonBadUnicodeEscape),
        };
        let nibble: u32 = match byte {
            b'0'..=b'9' => u32::from(byte - b'0'),
            b'a'..=b'f' => u32::from(byte - b'a') + 10,
            b'A'..=b'F' => u32::from(byte - b'A') + 10,
            _ => return Err(GranmoModelError::DsJsonBadUnicodeEscape),
        };
        // Shift-and-or over 4 nibbles cannot exceed 16 bits — no overflow
        // is possible; checked arithmetic is still used per the rules.
        value = value
            .checked_mul(16)
            .and_then(|v| v.checked_add(nibble))
            .ok_or(GranmoModelError::DsJsonBadUnicodeEscape)?;
        cursor.advance();
    }
    Ok(value)
}

/// Encodes one Unicode scalar value as UTF-8 bytes onto `output`. The model
/// consumes BYTES, so escaped text must land in the exact same byte form an
/// unescaped file would carry — otherwise "\u00e9" and a raw 'é' would be
/// different patterns to the engine, silently splitting the training signal.
fn push_utf8_encoded_codepoint(
    codepoint: u32,
    output: &mut Vec<u8>,
) -> Result<(), GranmoModelError> {
    // Surrogate halves and out-of-range values are not scalar values.
    if (0xD800..=0xDFFF).contains(&codepoint) || codepoint > 0x10_FFFF {
        return Err(GranmoModelError::DsJsonBadUnicodeEscape);
    }
    if codepoint < 0x80 {
        output.push(codepoint as u8);
    } else if codepoint < 0x800 {
        output.push(0xC0 | (codepoint >> 6) as u8);
        output.push(0x80 | (codepoint & 0x3F) as u8);
    } else if codepoint < 0x10000 {
        output.push(0xE0 | (codepoint >> 12) as u8);
        output.push(0x80 | ((codepoint >> 6) & 0x3F) as u8);
        output.push(0x80 | (codepoint & 0x3F) as u8);
    } else {
        output.push(0xF0 | (codepoint >> 18) as u8);
        output.push(0x80 | ((codepoint >> 12) & 0x3F) as u8);
        output.push(0x80 | ((codepoint >> 6) & 0x3F) as u8);
        output.push(0x80 | (codepoint & 0x3F) as u8);
    }
    Ok(())
}

/// Decodes a JSON string at the cursor (cursor must be ON the opening
/// quote) into raw bytes, consuming through the closing quote.
///
/// Leniency note (documented): raw bytes below 0x20 inside the string are
/// passed through rather than rejected. Strict JSON forbids them, but
/// scraped datasets contain them, and the preprocessing stages (§5) are the
/// jurisdiction for control-byte policy — the loader's job is fidelity.
fn decode_json_string(cursor: &mut JsonLineCursor<'_>) -> Result<Vec<u8>, GranmoModelError> {
    if cursor.peek() != Some(b'"') {
        return Err(GranmoModelError::DsJsonMalformedStructure);
    }
    cursor.advance(); // consume opening quote

    let mut decoded: Vec<u8> = Vec::new();
    loop {
        let byte = match cursor.peek() {
            Some(b) => b,
            None => return Err(GranmoModelError::DsJsonUnterminatedString),
        };
        match byte {
            b'"' => {
                cursor.advance(); // consume closing quote
                return Ok(decoded);
            }
            b'\\' => {
                cursor.advance(); // consume backslash
                let escape_kind = match cursor.peek() {
                    Some(b) => b,
                    None => return Err(GranmoModelError::DsJsonUnterminatedString),
                };
                cursor.advance(); // consume escape designator
                match escape_kind {
                    b'"' => decoded.push(b'"'),
                    b'\\' => decoded.push(b'\\'),
                    b'/' => decoded.push(b'/'),
                    b'b' => decoded.push(0x08),
                    b'f' => decoded.push(0x0C),
                    b'n' => decoded.push(b'\n'),
                    b'r' => decoded.push(b'\r'),
                    b't' => decoded.push(b'\t'),
                    b'u' => {
                        let first_unit = parse_four_hex_digits(cursor)?;
                        if (0xDC00..=0xDFFF).contains(&first_unit) {
                            // A LOW surrogate first is orderless — broken pair.
                            return Err(GranmoModelError::DsJsonBadUnicodeEscape);
                        }
                        if (0xD800..=0xDBFF).contains(&first_unit) {
                            // High surrogate: a low surrogate MUST follow
                            // immediately as another \uXXXX escape.
                            if cursor.peek() != Some(b'\\') {
                                return Err(GranmoModelError::DsJsonBadUnicodeEscape);
                            }
                            cursor.advance();
                            if cursor.peek() != Some(b'u') {
                                return Err(GranmoModelError::DsJsonBadUnicodeEscape);
                            }
                            cursor.advance();
                            let second_unit = parse_four_hex_digits(cursor)?;
                            if !(0xDC00..=0xDFFF).contains(&second_unit) {
                                return Err(GranmoModelError::DsJsonBadUnicodeEscape);
                            }
                            // Standard surrogate-pair combination. All
                            // operands are bounded 16-bit values; checked
                            // arithmetic per the rules regardless.
                            let combined = 0x10000u32
                                .checked_add(
                                    (first_unit - 0xD800)
                                        .checked_mul(0x400)
                                        .ok_or(GranmoModelError::DsJsonBadUnicodeEscape)?,
                                )
                                .and_then(|v| v.checked_add(second_unit - 0xDC00))
                                .ok_or(GranmoModelError::DsJsonBadUnicodeEscape)?;
                            push_utf8_encoded_codepoint(combined, &mut decoded)?;
                        } else {
                            push_utf8_encoded_codepoint(first_unit, &mut decoded)?;
                        }
                    }
                    _other => return Err(GranmoModelError::DsJsonBadEscape),
                }
            }
            other => {
                // Raw byte (including UTF-8 continuation bytes and, per the
                // leniency note above, raw control bytes): pass through.
                decoded.push(other);
                cursor.advance();
            }
        }
    }
}

/// Skips one complete JSON value of ANY type at the cursor — the machinery
/// that makes unknown keys tolerable. Permissive by design: it validates
/// only enough structure to find the value's end (e.g. bracket DEPTH is
/// tracked but `{...]` mismatches are not caught here — a later structural
/// check on the record will reject such lines).
fn skip_json_value(cursor: &mut JsonLineCursor<'_>) -> Result<(), GranmoModelError> {
    cursor.skip_whitespace();
    match cursor.peek() {
        None => Err(GranmoModelError::DsJsonMalformedStructure),
        Some(b'"') => {
            // Escape-aware string skip: after '\', the next byte is consumed
            // blind, which is sufficient — no escape's SECOND byte can be
            // the terminating quote of the string.
            cursor.advance();
            loop {
                match cursor.peek() {
                    None => return Err(GranmoModelError::DsJsonUnterminatedString),
                    Some(b'"') => {
                        cursor.advance();
                        return Ok(());
                    }
                    Some(b'\\') => {
                        cursor.advance();
                        if cursor.peek().is_none() {
                            return Err(GranmoModelError::DsJsonUnterminatedString);
                        }
                        cursor.advance();
                    }
                    Some(_) => cursor.advance(),
                }
            }
        }
        Some(b'{') | Some(b'[') => {
            // Iterative depth tracking (no recursion: no stack-depth risk
            // from adversarial deeply-nested lines).
            let mut depth: usize = 0;
            loop {
                match cursor.peek() {
                    None => return Err(GranmoModelError::DsJsonMalformedStructure),
                    Some(b'"') => skip_json_value(cursor)?, // one level: string
                    Some(b'{') | Some(b'[') => {
                        depth = depth
                            .checked_add(1)
                            .ok_or(GranmoModelError::DsJsonMalformedStructure)?;
                        cursor.advance();
                    }
                    Some(b'}') | Some(b']') => {
                        depth = depth
                            .checked_sub(1)
                            .ok_or(GranmoModelError::DsJsonMalformedStructure)?;
                        cursor.advance();
                        if depth == 0 {
                            return Ok(());
                        }
                    }
                    Some(_) => cursor.advance(),
                }
            }
        }
        Some(_) => {
            // Bare token: number / true / false / null. Consume to the next
            // delimiter; content is not numerically validated (permissive —
            // the token is either compared as label bytes or discarded).
            let mut consumed_any = false;
            while let Some(byte) = cursor.peek() {
                if matches!(byte, b',' | b'}' | b']' | b' ' | b'\t' | b'\r') {
                    break;
                }
                consumed_any = true;
                cursor.advance();
            }
            if consumed_any {
                Ok(())
            } else {
                Err(GranmoModelError::DsJsonMalformedStructure)
            }
        }
    }
}

/// Captures a bare (unquoted) value token's raw bytes — the label path for
/// `"label": 1`, `"label": true`, etc. Same delimiter rules as the skipper.
fn capture_bare_token(cursor: &mut JsonLineCursor<'_>) -> Result<Vec<u8>, GranmoModelError> {
    let mut token: Vec<u8> = Vec::new();
    while let Some(byte) = cursor.peek() {
        if matches!(byte, b',' | b'}' | b']' | b' ' | b'\t' | b'\r') {
            break;
        }
        token.push(byte);
        cursor.advance();
    }
    if token.is_empty() {
        return Err(GranmoModelError::DsJsonMalformedStructure);
    }
    Ok(token)
}

/// Parses ONE JSONL line into `(text_bytes, label_token_bytes)`.
///
/// Returns `Ok(None)` for the SKIP cases (blank line; missing text/label;
/// empty text; non-string text; null label) and `Err` for structural
/// malformation — the policy table at the top of this section.
fn parse_jsonl_record(
    line: &[u8],
    text_key: &str,
    label_key: &str,
) -> Result<Option<(Vec<u8>, Vec<u8>)>, GranmoModelError> {
    let mut cursor = JsonLineCursor::new(line);
    cursor.skip_whitespace();

    match cursor.peek() {
        None => return Ok(None), // blank/whitespace-only line: skip
        Some(b'{') => cursor.advance(),
        Some(_) => return Err(GranmoModelError::DsJsonLineNotObject),
    }

    let mut text_value: Option<Vec<u8>> = None;
    let mut label_token: Option<Vec<u8>> = None;

    loop {
        cursor.skip_whitespace();
        match cursor.peek() {
            Some(b'}') => {
                cursor.advance();
                break; // empty object `{}` — both fields absent, record skips
            }
            Some(b'"') => {}
            _ => return Err(GranmoModelError::DsJsonMalformedStructure),
        }

        // Keys are decoded with the full string decoder so escaped key names
        // still compare correctly (rare, but free to support).
        let key = decode_json_string(&mut cursor)?;
        cursor.skip_whitespace();
        match cursor.peek() {
            Some(b':') => cursor.advance(),
            _ => return Err(GranmoModelError::DsJsonMalformedStructure),
        }
        cursor.skip_whitespace();

        if key == text_key.as_bytes() {
            if cursor.peek() == Some(b'"') {
                text_value = Some(decode_json_string(&mut cursor)?);
            } else {
                // Non-string text (e.g. null): schema-tolerant SKIP path —
                // consume the value and leave text unset, so the record is
                // dropped below rather than erroring the whole load.
                skip_json_value(&mut cursor)?;
            }
        } else if key == label_key.as_bytes() {
            if cursor.peek() == Some(b'"') {
                label_token = Some(decode_json_string(&mut cursor)?);
            } else {
                label_token = Some(capture_bare_token(&mut cursor)?);
            }
        } else {
            skip_json_value(&mut cursor)?;
        }

        cursor.skip_whitespace();
        match cursor.peek() {
            Some(b',') => cursor.advance(),
            Some(b'}') => {
                cursor.advance();
                break;
            }
            _ => return Err(GranmoModelError::DsJsonMalformedStructure),
        }
    }

    // Exactly one object per line: trailing non-whitespace is malformation.
    cursor.skip_whitespace();
    if cursor.peek().is_some() {
        return Err(GranmoModelError::DsJsonMalformedStructure);
    }

    // Skip-policy resolution.
    match (text_value, label_token) {
        (Some(text), Some(label)) => {
            if text.is_empty() || label.is_empty() || label == b"null" {
                Ok(None)
            } else {
                Ok(Some((text, label)))
            }
        }
        _ => Ok(None),
    }
}

/// Loads a labeled binary-classification dataset from a JSONL file
/// (one JSON object per line; schema and policies per the section header).
///.
pub fn load_labeled_jsonl(
    absolute_path: &std::path::Path,
    text_key: &str,
    label_key: &str,
    positive_label: &str,
) -> Result<Vec<LabeledDocument>, GranmoModelError> {
    if !absolute_path.is_absolute() {
        #[cfg(debug_assertions)]
        eprintln!("DS-700: path not absolute: {}", absolute_path.display());
        return Err(GranmoModelError::DsPathNotAbsolute);
    }
    let raw = match std::fs::read(absolute_path) {
        Ok(bytes) => bytes,
        Err(_dropped_io_detail) => {
            #[cfg(debug_assertions)]
            eprintln!("DS-701: read failed: {}", _dropped_io_detail);
            return Err(GranmoModelError::DsFileReadFailed);
        }
    };

    let mut documents: Vec<LabeledDocument> = Vec::new();
    for (line_index, line) in raw.split(|&b| b == b'\n').enumerate() {
        // Explicit match rather than '?': the line NUMBER is the diagnostic
        // that makes a malformed-file report actionable, and it exists only
        // at this site. (Line numbers are positional, not PII; line CONTENT
        // is never printed.)
        match parse_jsonl_record(line, text_key, label_key) {
            Ok(Some((text, label))) => documents.push(LabeledDocument {
                line_index: line_index.saturating_add(1),
                label_is_positive: label == positive_label.as_bytes(),
                text,
            }),
            Ok(None) => {}
            Err(parse_error) => {
                #[cfg(debug_assertions)]
                eprintln!(
                    "DS-{}: JSONL parse error at line {}",
                    parse_error.code(),
                    line_index.saturating_add(1)
                );
                return Err(parse_error);
            }
        }
    }

    if documents.is_empty() {
        return Err(GranmoModelError::DsNoUsableRecords);
    }
    Ok(documents)
}

// ===========================================================================
// SECTION 10: Experiment Runner (research-harness tier)
// ===========================================================================

/// Pre-construction engine choice for one experiment run: the
/// config-surface counterpart of `ClassifierEngine` (which holds a LIVE
/// engine; this selects which one to build). A fieldless enum rather than
/// a string so an invalid engine name is rejected exactly once, at CLI
/// parse time (fail-fast policy), and everything downstream matches
/// exhaustively — adding an engine later makes the compiler flag every
/// selection site, same checklist mechanism as `ClassifierEngine`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineSelection {
    /// Byte-level convolutional engine (`ByteConvTM`, Section 5).
    ByteConv,
    /// Flat bag-of-byte-n-grams control engine (`ByteBagTM`, Section 12B).
    ByteBag,
    /// Joint two-bank co-training engine (`HybridTM`, Section 12C). Uses
    /// BOTH the conv fields (`patch_size`, `stride`, `guarded_include`) and
    /// the bag fields (`bag_ngram_len`, `bag_vocab_size`,
    /// `fire_guard_streak_limit`) of the run config; `n_clauses` sizes EACH
    /// bank and `vote_threshold` is the shared T.
    SeqFreqHybrid,
}

/// All resolved parameters for one experiment run. Raw primitive fields are
/// deliberate HERE (and only here): this struct is the CLI-facing surface,
/// and every value is pushed through its enforced newtype constructor inside
/// `run_single_experiment` — so validation happens exactly once, at the
/// boundary between user input and engine.
///
/// Engine-specific fields: `patch_size`, `stride`, and `guarded_include`
/// apply ONLY to the ByteConv engine; `bag_ngram_len` and `bag_vocab_size`
/// apply ONLY to the ByteBag engine. The non-selected engine's fields are
/// carried but ignored — deliberately, so ONE config struct describes any
/// run and batch mode can flip `engine_selection` on an otherwise identical
/// config (the §8 single-variable comparison discipline).
#[derive(Debug, Clone)]
pub struct HarnessRunConfig {
    pub profile: PreprocessProfile,
    pub engine_selection: EngineSelection,
    pub patch_size: u8,
    pub stride: u8,
    pub bag_ngram_len: u8,
    pub bag_vocab_size: u16,
    pub n_clauses: u16,
    pub vote_threshold: i16,
    pub states_per_action: i16,
    pub specificity: f64,
    pub max_scan_bytes: u32,
    pub guarded_include: bool,
    /// Training-time always-fire guard limit (Drop 4.1): 0 = disabled
    /// (recorded-baseline default); active values reset a byte-bag clause
    /// that fires on this many CONSECUTIVE shuffled training documents
    /// while non-vacuous. Applies ONLY to the ByteBag engine; carried but
    /// ignored for ByteConv (same one-config-describes-any-run posture as
    /// `patch_size` for the bag). Validated through
    /// `FireGuardStreakLimit` at the engine-construction boundary.
    pub fire_guard_streak_limit: u32,
    pub epochs: u32,
    pub seed: u64,
    /// Worker threads for clause-parallel training and document-parallel
    /// evaluation. PERFORMANCE-ONLY: every value in `WorkerCount` range
    /// produces byte-identical results, so this field never affects a
    /// comparison-matrix row's science, only its wall-clock.
    pub worker_count: u16,
    /// Also score the TRAINING side after training and record those
    /// predictions (opt-in: one extra evaluation pass). Off by default;
    /// has no effect on the model or on any test-side number.
    pub score_train_side: bool,
}

/// Everything a comparison-matrix row needs from one run.
#[derive(Debug, Clone)]
pub struct ExperimentReport {
    /// Which engine produced this row (taken from the live engine, so it
    /// can never disagree with what actually ran).
    pub engine_name_reported: &'static str,
    pub train_count: usize,
    pub test_count: usize,
    /// Accuracy at the default decision threshold V > 0.
    pub accuracy_at_zero: f64,
    /// The sweep row maximizing F1 (ties -> lowest threshold).
    pub best_f1_row: ThresholdSweepRow,
    pub train_seconds: f64,
    /// Per-clause count of test documents on which the clause fired — the
    /// S2-8 fire-rate diagnostic. Reading guide: clauses near 0 are dead
    /// weight (never contribute a vote); clauses near `test_count` fire
    /// indiscriminately (vote offset, not evidence); a healthy bank sits
    /// between. This histogram is the recorded instrument for the M-Hetero
    /// vote-imbalance risk and for deciding whether the CoTM clause-weight
    /// arm gets pulled forward.
    pub clause_fire_counts: Vec<u32>,
    /// Total fire-guard resets performed during training (Drop 4.1) —
    /// the guard's ACTIVITY instrument. Read TOGETHER with the fire-rate
    /// report's "always" count (the OUTCOME instrument): many resets with
    /// an unchanged always-count means clauses re-converge to the same
    /// ubiquitous patterns, which argues for a vocabulary-side fix
    /// (frequency capping) rather than more guarding. Always 0 for the
    /// conv engine and whenever the guard is disabled.
    pub fire_guard_reset_total: u64,
    /// Per-clause included-literal totals at end of training (paired
    /// index-for-index with `clause_fire_counts`). Powers the
    /// vacuous-vs-specialized breakdown of the always-fire count and the
    /// includes-per-clause histogram: a large vacuous population means
    /// the depth/epoch budget (N vs. epochs) is the lever, not the guard.
    pub clause_include_totals: Vec<u32>,
    /// The fire-guard limit this run actually trained with (0 = off, and
    /// always 0 for the conv engine). Recorded so the report can print
    /// guard activity UNCONDITIONALLY when the guard was armed —
    /// "limit 500, resets 0" is itself a finding, and was previously
    /// indistinguishable from "guard off" in the output.
    pub fire_guard_limit_used: u32,
    /// Test documents misclassified at the default threshold V > 0,
    /// captured for the misprediction inspection log. Pairs raw and
    /// preprocessed text so label errors AND preprocessing artifacts are
    /// both auditable.
    pub mispredictions: Vec<MispredictionRecord>,
    /// Every test-side (out-of-fold) prediction, one record per row.
    pub test_row_predictions: Vec<RowPredictionRecord>,
    /// Training-side predictions when `score_train_side` was set; empty
    /// otherwise.
    pub train_row_predictions: Vec<RowPredictionRecord>,
    /// Signed vote sum per test document, in test-split order. Retained so
    /// batch mode can evaluate a late-fusion ensemble (Section 10B) by
    /// summing two runs' votes document-for-document WITHOUT re-scoring.
    pub test_vote_sums: Vec<i32>,
    /// Ground-truth label per test document, paired index-for-index with
    /// `test_vote_sums`. Late fusion cross-checks the two reports' label
    /// vectors for EQUALITY as proof they were scored on one split.
    pub test_labels: Vec<bool>,
}

/// Builds a fresh ByteConvTM for one harness run from the validated config
/// values. Shared by the ByteConv and SeqFreqHybrid arms so the two cannot
/// drift apart on construction.
fn harness_build_conv_engine(config: &HarnessRunConfig) -> Result<ByteConvTM, GranmoModelError> {
    ByteConvTM::new(
        PatchSize::new(config.patch_size)?,
        StrideLen::new(config.stride)?,
        ClauseCount::new(config.n_clauses)?,
        VoteThreshold::new(config.vote_threshold)?,
        StatesPerAction::new(config.states_per_action)?,
        SpecificityThresholds::from_specificity(config.specificity)?,
        MaxScanBytes::new(config.max_scan_bytes)?,
        config.guarded_include,
    )
}

/// Builds a fresh ByteBagTM for one harness run, with its vocabulary learned
/// from the PREPROCESSED TRAINING split ONLY — the leakage guard of record
/// (Session 2 §6), kept in ONE place so the ByteBag and SeqFreqHybrid arms
/// cannot drift apart on it.
fn harness_build_bag_engine(
    config: &HarnessRunConfig,
    train_prepared: &[(Vec<u8>, bool)],
) -> Result<ByteBagTM, GranmoModelError> {
    let training_document_views: Vec<&[u8]> = train_prepared
        .iter()
        .map(|(document, _label)| document.as_slice())
        .collect();
    let training_vocabulary = ByteBagVocabulary::build_from_documents(
        NgramLength::new(config.bag_ngram_len)?,
        VocabSize::new(config.bag_vocab_size)?,
        &training_document_views,
    )?;
    ByteBagTM::new_with_vocabulary(
        training_vocabulary,
        ClauseCount::new(config.n_clauses)?,
        VoteThreshold::new(config.vote_threshold)?,
        StatesPerAction::new(config.states_per_action)?,
        SpecificityThresholds::from_specificity(config.specificity)?,
        MaxScanBytes::new(config.max_scan_bytes)?,
        FireGuardStreakLimit::new(config.fire_guard_streak_limit)?,
    )
}

/// Preprocesses every document ONCE through the given profile — the
/// established performance pattern (no per-epoch re-tokenization), and the
/// artifact-integrity pattern (the same profile object that trains is the
/// one persisted).
fn preprocess_documents(
    profile: PreprocessProfile,
    documents: &[LabeledDocument],
) -> Result<Vec<(Vec<u8>, bool)>, GranmoModelError> {
    let mut preprocessor = BytePreprocessor::new(profile)?;
    let mut prepared = Vec::with_capacity(documents.len());
    for document in documents {
        let processed = preprocessor.process_document(&document.text)?;
        prepared.push((processed, document.label_is_positive));
    }
    Ok(prepared)
}

/// Derives the signed vote sum from a fired-clause bitset by applying the
/// crate-wide polarity rule: even clause index votes +1, odd votes −1.
/// Definitionally identical to each engine's own vote computation, because
/// both operate on the same per-clause firing results under the same
/// polarity rule — this is what lets the harness evaluate each test
/// document with ONE `fired_clause_bits` call instead of scanning twice
/// (once for the vote, once for fire rates).
fn vote_from_fired_words(
    fired_words: &[u64],
    clause_total: usize,
) -> Result<i32, GranmoModelError> {
    let mut vote: i32 = 0;
    for clause in 0..clause_total {
        let fired_word = fired_words
            .get(clause >> 6)
            .copied()
            .ok_or(GranmoModelError::CliFireRateReportInternalFault)?;
        if fired_word & (1u64 << (clause & 63)) != 0 {
            vote = if clause % 2 == 0 {
                vote.checked_add(1)
            } else {
                vote.checked_sub(1)
            }
            .ok_or(GranmoModelError::CliFireRateReportInternalFault)?;
        }
    }
    Ok(vote)
}

// ---------------------------------------------------------------------------
// Misprediction inspection log (research-harness tier)
// ---------------------------------------------------------------------------
//
// Purpose: after evaluation, every test document the model got wrong at the
// default decision threshold (V > 0) is captured so an operator can inspect
// whether the DATA is wrong (mislabeled "golden" records are common in
// scraped corpora) or the MODEL is wrong. Records carry BOTH the raw bytes
// (what the dataset says) and the preprocessed bytes (what the engine
// actually scored), because preprocessing differences are themselves a
// frequent cause of mispredictions and the two views diverge under most
// presets.
//
// Policies (matching crate law):
// - Absolute paths only. The DEFAULT log path is resolved against the
//   executable's parent directory (the crate's logging rule of thumb), so
//   it never depends on the caller's working directory.
// - Logging is BEST-EFFORT at call sites: a failed log write must never
//   abort a training run whose report has already been produced. Callers
//   receive the Result and decide; the provided CLI hooks warn and proceed.
// - Error paths allocate nothing and carry no PII (codes only); document
//   text goes to the log FILE, never to stderr.

/// One captured misprediction: a test document the engine classified
/// incorrectly at the default decision threshold (V > 0).
#[derive(Debug, Clone)]
pub struct MispredictionRecord {
    /// Raw un-preprocessed document bytes, exactly as loaded from the
    /// dataset (for auditing the LABEL against the original text).
    pub raw_text_bytes: Vec<u8>,
    /// Preprocessed document bytes, exactly as the engine scored them
    /// (for auditing the MODEL's view of the text).
    pub preprocessed_text_bytes: Vec<u8>,
    /// Ground-truth label from the dataset (true = positive / 1).
    pub actual_label_is_positive: bool,
    /// Predicted label at the default threshold V > 0.
    pub predicted_label_is_positive: bool,
    /// The signed vote sum V that produced the prediction. A small |V|
    /// means a near-miss; a large wrong-signed |V| means the model is
    /// confidently wrong — the most interesting records to inspect.
    pub vote_sum_at_prediction: i32,
}

/// Resolves the default misprediction-log path: `<executable_dir>/logs/
/// misprediction_log.txt`. Anchoring to the executable's parent directory
/// (rather than the working directory) keeps the location deterministic
/// across shells, cron jobs, and IDE runners — the absolute-path policy's
/// intent. Failure to locate the executable is reported, never guessed
/// around.
pub fn resolve_default_misprediction_log_path() -> Result<std::path::PathBuf, GranmoModelError> {
    let executable_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(_dropped_io_detail) => {
            #[cfg(debug_assertions)]
            eprintln!(
                "CLI-808: could not resolve executable path: {}",
                _dropped_io_detail
            );
            return Err(GranmoModelError::CliLogPathNotAbsolute);
        }
    };
    let executable_dir = match executable_path.parent() {
        Some(dir) => dir,
        None => return Err(GranmoModelError::CliLogPathNotAbsolute),
    };
    let resolved = executable_dir.join("logs").join("misprediction_log.txt");
    if !resolved.is_absolute() {
        return Err(GranmoModelError::CliLogPathNotAbsolute);
    }
    Ok(resolved)
}

/// Appends misprediction records to a persistent inspection log.
///
/// One record per line, tab-separated fields:
/// `data_path`, `run_label` (preset), `engine`, `actual`, `pred`, `vote`,
/// `raw_text`, `prep_text`. Interior newlines/carriage returns and tabs in
/// document text are replaced with spaces so the file stays strictly
/// line-per-record and field-per-tab (greppable / spreadsheet-importable).
/// Text is rendered with lossy UTF-8 so arbitrary byte streams (this is a
/// byte-level model) remain printable without ever panicking.
///
/// Behavior contract:
/// - `log_path` must be absolute (crate-wide policy) -> `CliLogPathNotAbsolute`.
/// - Empty `mispredictions` is a successful no-op: the file and directory
///   are NOT created for a clean run.
/// - Append mode: prior runs' records are preserved (this is the point —
///   accumulating a cross-run inspection corpus).
/// - No `?` on I/O: each failure site maps to its specific code explicitly.
pub fn append_mispredictions_to_log(
    log_path: &std::path::Path,
    data_source_path: &std::path::Path,
    run_label: &str,
    engine_name: &str,
    mispredictions: &[MispredictionRecord],
) -> Result<(), GranmoModelError> {
    if mispredictions.is_empty() {
        return Ok(()); // clean run: deliberately no file/dir creation
    }
    if !log_path.is_absolute() {
        #[cfg(debug_assertions)]
        eprintln!("CLI-808: log path not absolute: {}", log_path.display());
        return Err(GranmoModelError::CliLogPathNotAbsolute);
    }

    if let Some(parent_dir) = log_path.parent() {
        if !parent_dir.as_os_str().is_empty() {
            if let Err(_dropped_io_detail) = std::fs::create_dir_all(parent_dir) {
                #[cfg(debug_assertions)]
                eprintln!("CLI-809: log dir create failed: {}", _dropped_io_detail);
                return Err(GranmoModelError::CliLogDirCreateFailed);
            }
        }
    }

    let opened_file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
    {
        Ok(file) => file,
        Err(_dropped_io_detail) => {
            #[cfg(debug_assertions)]
            eprintln!("CLI-810: log open failed: {}", _dropped_io_detail);
            return Err(GranmoModelError::CliLogWriteFailed);
        }
    };
    let mut log_writer = std::io::BufWriter::new(opened_file);
    use std::io::Write;

    /// Renders document bytes as one log field: lossy UTF-8, with the
    /// line/field separator characters folded to spaces.
    fn render_log_text_field(text_bytes: &[u8]) -> String {
        String::from_utf8_lossy(text_bytes)
            .chars()
            .map(|c| {
                if matches!(c, '\n' | '\r' | '\t') {
                    ' '
                } else {
                    c
                }
            })
            .collect()
    }

    for record in mispredictions {
        let write_outcome = writeln!(
            log_writer,
            "data_path={}\trun={}\tengine={}\tactual={}\tpred={}\tvote={}\traw_text=\"{}\"\tprep_text=\"{}\"",
            data_source_path.display(),
            run_label,
            engine_name,
            if record.actual_label_is_positive {
                "1"
            } else {
                "0"
            },
            if record.predicted_label_is_positive {
                "1"
            } else {
                "0"
            },
            record.vote_sum_at_prediction,
            render_log_text_field(&record.raw_text_bytes),
            render_log_text_field(&record.preprocessed_text_bytes),
        );
        if let Err(_dropped_io_detail) = write_outcome {
            #[cfg(debug_assertions)]
            eprintln!("CLI-810: log line write failed: {}", _dropped_io_detail);
            return Err(GranmoModelError::CliLogWriteFailed);
        }
    }
    if let Err(_dropped_io_detail) = log_writer.flush() {
        #[cfg(debug_assertions)]
        eprintln!("CLI-810: log flush failed: {}", _dropped_io_detail);
        return Err(GranmoModelError::CliLogWriteFailed);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Per-row prediction records (research-harness tier)
// ---------------------------------------------------------------------------
//
// One record per (row, run, side). Generalizes the misprediction log: EVERY
// scored row is recorded, right or wrong, so the row audit can compute
// consistency across folds and seeds. Test-side records are out-of-fold
// predictions; train-side records (opt-in) are the model's view of rows it
// trained on, which separates "memorized but does not generalize" from
// "wrong even when trained on" (the strongest mislabel signal a model can
// give without a human).
//
// Format: one line per record, tab-separated `key=value` fields. Keyed
// (not positional) so the audit reader tolerates added context columns.
// Document text is deliberately NOT recorded here; the line index joins
// back to the source file.

/// One scored row from one run.
#[derive(Debug, Clone, Copy)]
pub struct RowPredictionRecord {
    /// 1-based source line (join key; see `LabeledDocument::line_index`).
    pub line_index: usize,
    /// true = the row was on this run's TRAINING side when scored.
    pub side_is_train: bool,
    pub label_is_positive: bool,
    pub vote_sum: i32,
}

/// Run-level context written into every record of one run.
#[derive(Debug, Clone, Copy)]
pub struct RowPredictionContext<'a> {
    pub split_seed: u64,
    pub train_seed: u64,
    /// 0 for a single split; the fold number under k-fold.
    pub fold_index: u16,
    pub engine_name: &'a str,
    pub run_label: &'a str,
}

/// Appends prediction records. Same contract as `append_mispredictions_to_log`:
/// absolute path only; empty input is a successful no-op; append mode; each
/// I/O failure site maps to its own code (no `?` on I/O).
pub fn append_row_prediction_records(
    records_path: &std::path::Path,
    context: RowPredictionContext<'_>,
    records: &[RowPredictionRecord],
) -> Result<(), GranmoModelError> {
    if records.is_empty() {
        return Ok(());
    }
    if !records_path.is_absolute() {
        #[cfg(debug_assertions)]
        eprintln!(
            "CLI-811: records path not absolute: {}",
            records_path.display()
        );
        return Err(GranmoModelError::CliPredictionRecordPathNotAbsolute);
    }
    if let Some(parent_dir) = records_path.parent() {
        if !parent_dir.as_os_str().is_empty() {
            if let Err(_dropped_io_detail) = std::fs::create_dir_all(parent_dir) {
                #[cfg(debug_assertions)]
                eprintln!("CLI-812: records dir create failed: {}", _dropped_io_detail);
                return Err(GranmoModelError::CliPredictionRecordWriteFailed);
            }
        }
    }
    let opened_file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(records_path)
    {
        Ok(file) => file,
        Err(_dropped_io_detail) => {
            #[cfg(debug_assertions)]
            eprintln!("CLI-812: records open failed: {}", _dropped_io_detail);
            return Err(GranmoModelError::CliPredictionRecordWriteFailed);
        }
    };
    let mut writer = std::io::BufWriter::new(opened_file);
    use std::io::Write;

    for record in records {
        let outcome = writeln!(
            writer,
            "line={}\tsplit_seed={}\ttrain_seed={}\tfold={}\tside={}\tengine={}\trun={}\tlabel={}\tvote={}",
            record.line_index,
            context.split_seed,
            context.train_seed,
            context.fold_index,
            if record.side_is_train {
                "train"
            } else {
                "test"
            },
            context.engine_name,
            context.run_label,
            if record.label_is_positive { "1" } else { "0" },
            record.vote_sum,
        );
        if let Err(_dropped_io_detail) = outcome {
            #[cfg(debug_assertions)]
            eprintln!("CLI-812: record write failed: {}", _dropped_io_detail);
            return Err(GranmoModelError::CliPredictionRecordWriteFailed);
        }
    }
    if let Err(_dropped_io_detail) = writer.flush() {
        #[cfg(debug_assertions)]
        eprintln!("CLI-812: record flush failed: {}", _dropped_io_detail);
        return Err(GranmoModelError::CliPredictionRecordWriteFailed);
    }
    Ok(())
}

/// Pairs one evaluation's vote sums with the documents that produced them
/// (index-for-index — the same pairing invariant `evaluate_test_chunk`
/// enforces for mispredictions) into records for one side of one run.
fn build_row_prediction_records(
    documents: &[LabeledDocument],
    vote_sums: &[i32],
    side_is_train: bool,
) -> Result<Vec<RowPredictionRecord>, GranmoModelError> {
    if documents.len() != vote_sums.len() {
        return Err(GranmoModelError::ParEvalMergeGeometryFault);
    }
    let mut records = Vec::with_capacity(documents.len());
    for (document, &vote_sum) in documents.iter().zip(vote_sums.iter()) {
        records.push(RowPredictionRecord {
            line_index: document.line_index,
            side_is_train,
            label_is_positive: document.label_is_positive,
            vote_sum,
        });
    }
    Ok(records)
}

// ---------------------------------------------------------------------------
// Parallel test-set evaluation (research-harness tier)
// ---------------------------------------------------------------------------
//
// Evaluation is read-only against a trained engine and consumes ZERO
// randomness, so it parallelizes with no contract change at all. Bit-
// identity at any worker count rests on two facts:
//   1. `ClassifierEngine` has no interior mutability, so `&engine` is
//      shareable across threads (auto `Sync`) with no lock and no clone;
//   2. partials are merged strictly in CHUNK ORDER, which reconstructs the
//      original document order exactly — so the vote/label vectors, the
//      threshold sweep derived from them, and the misprediction-log record
//      order are all unchanged from the sequential implementation.
// Fire counts merge by exact integer addition (order-independent).

/// One worker's partial evaluation results over a contiguous chunk of the
/// test split.
struct EvalChunkPartial {
    correct_at_zero: usize,
    clause_fire_counts: Vec<u32>,
    vote_sums: Vec<i32>,
    labels: Vec<bool>,
    mispredictions: Vec<MispredictionRecord>,
}

/// Evaluates one contiguous chunk of test documents. Byte-identical logic
/// to the previous inline evaluation loop; callable from one thread or many.
fn evaluate_test_chunk(
    engine: &ClassifierEngine,
    prepared_chunk: &[(Vec<u8>, bool)],
    raw_chunk: &[LabeledDocument],
) -> Result<EvalChunkPartial, GranmoModelError> {
    if prepared_chunk.len() != raw_chunk.len() {
        return Err(GranmoModelError::ParEvalMergeGeometryFault);
    }
    let engine_clause_total = engine.engine_clause_count();
    let mut partial = EvalChunkPartial {
        correct_at_zero: 0,
        clause_fire_counts: vec![0u32; engine_clause_total],
        vote_sums: Vec::with_capacity(prepared_chunk.len()),
        labels: Vec::with_capacity(prepared_chunk.len()),
        mispredictions: Vec::new(),
    };

    for (doc_index, (document, label)) in prepared_chunk.iter().enumerate() {
        let fired_words = engine.engine_fired_clause_bits(document)?;
        let vote = vote_from_fired_words(&fired_words, engine_clause_total)?;
        for clause in 0..engine_clause_total {
            let fired_word = fired_words
                .get(clause >> 6)
                .copied()
                .ok_or(GranmoModelError::CliFireRateReportInternalFault)?;
            if fired_word & (1u64 << (clause & 63)) != 0 {
                let count_slot = partial
                    .clause_fire_counts
                    .get_mut(clause)
                    .ok_or(GranmoModelError::CliFireRateReportInternalFault)?;
                *count_slot = count_slot
                    .checked_add(1)
                    .ok_or(GranmoModelError::CliFireRateReportInternalFault)?;
            }
        }

        let predicted_positive = vote > 0;
        if predicted_positive == *label {
            partial.correct_at_zero += 1;
        } else {
            let raw_original = raw_chunk
                .get(doc_index)
                .ok_or(GranmoModelError::ParEvalMergeGeometryFault)?;
            partial.mispredictions.push(MispredictionRecord {
                raw_text_bytes: raw_original.text.clone(),
                preprocessed_text_bytes: document.clone(),
                actual_label_is_positive: *label,
                predicted_label_is_positive: predicted_positive,
                vote_sum_at_prediction: vote,
            });
        }
        partial.vote_sums.push(vote);
        partial.labels.push(*label);
    }
    Ok(partial)
}

/// Evaluates the whole test split, splitting documents across workers and
/// merging partials in chunk order. Value-identical to sequential
/// evaluation for EVERY output field, at ANY worker count.
fn evaluate_test_split_with_workers(
    engine: &ClassifierEngine,
    test_prepared: &[(Vec<u8>, bool)],
    test_documents: &[LabeledDocument],
    worker_count: WorkerCount,
) -> Result<EvalChunkPartial, GranmoModelError> {
    if test_prepared.len() != test_documents.len() {
        return Err(GranmoModelError::ParEvalMergeGeometryFault);
    }
    let worker_total = resolve_effective_worker_count(worker_count, test_prepared.len())?;

    if worker_total == 1 {
        return evaluate_test_chunk(engine, test_prepared, test_documents);
    }

    let chunk_size = resolve_work_chunk_size(test_prepared.len(), worker_total)?;
    let chunk_outcomes: Vec<Result<EvalChunkPartial, GranmoModelError>> =
        std::thread::scope(|eval_scope| {
            let mut worker_handles = Vec::with_capacity(worker_total);
            for (prepared_chunk, raw_chunk) in test_prepared
                .chunks(chunk_size)
                .zip(test_documents.chunks(chunk_size))
            {
                worker_handles.push(
                    eval_scope
                        .spawn(move || evaluate_test_chunk(engine, prepared_chunk, raw_chunk)),
                );
            }
            worker_handles
                .into_iter()
                .map(|handle| match handle.join() {
                    Ok(worker_result) => worker_result,
                    Err(_panic_payload) => Err(GranmoModelError::ParWorkerJoinFailed),
                })
                .collect()
        });

    // Ordered merge: chunk order == original document order.
    let engine_clause_total = engine.engine_clause_count();
    let mut merged = EvalChunkPartial {
        correct_at_zero: 0,
        clause_fire_counts: vec![0u32; engine_clause_total],
        vote_sums: Vec::with_capacity(test_prepared.len()),
        labels: Vec::with_capacity(test_prepared.len()),
        mispredictions: Vec::new(),
    };
    for chunk_outcome in chunk_outcomes {
        let partial = chunk_outcome?;
        merged.correct_at_zero += partial.correct_at_zero;
        merged.vote_sums.extend(partial.vote_sums);
        merged.labels.extend(partial.labels);
        merged.mispredictions.extend(partial.mispredictions);
        for (total_slot, chunk_count) in merged
            .clause_fire_counts
            .iter_mut()
            .zip(partial.clause_fire_counts.iter())
        {
            *total_slot = total_slot
                .checked_add(*chunk_count)
                .ok_or(GranmoModelError::ParEvalMergeGeometryFault)?;
        }
    }
    if merged.vote_sums.len() != test_prepared.len() {
        return Err(GranmoModelError::ParEvalMergeGeometryFault);
    }
    Ok(merged)
}

/// Trains one engine and evaluates it on the held-out set.
///
/// Engine construction dispatches on `config.engine_selection`. For the
/// ByteBag engine, the vocabulary is built HERE, from the PREPROCESSED
/// TRAINING split ONLY — never from test documents. This placement is the
/// leakage guard of record (Session 2 §6): `ByteBagTM::new_with_vocabulary`
/// takes a finished vocabulary and cannot see corpora, and this function
/// hands the builder only the training side.
///
/// Training order is shuffled EVERY epoch (canonical fit practice: sample
/// order decorrelation), deterministically under the run seed — one RNG
/// stream drives shuffling and feedback, so a run is reproducible from
/// (dataset, split seed, run config) alone.
///
/// Evaluation computes ONE fired-clause bitset per test document and
/// derives from it BOTH the vote sum (hence accuracy and the threshold
/// sweep) AND the per-clause fire-rate counts (S2-8 diagnostic) — a single
/// evaluation pass, no double scanning. See `vote_from_fired_words` for the
/// equivalence guarantee with the engines' own vote computation.
pub fn run_single_experiment(
    train_documents: &[LabeledDocument],
    test_documents: &[LabeledDocument],
    config: &HarnessRunConfig,
) -> Result<(ClassifierEngine, ExperimentReport), GranmoModelError> {
    if train_documents.is_empty() || test_documents.is_empty() {
        return Err(GranmoModelError::DsNoUsableRecords);
    }

    let train_prepared = preprocess_documents(config.profile, train_documents)?;
    let test_prepared = preprocess_documents(config.profile, test_documents)?;

    let mut engine = match config.engine_selection {
        EngineSelection::ByteConv => ClassifierEngine::ByteConv(harness_build_conv_engine(config)?),
        EngineSelection::ByteBag => {
            ClassifierEngine::ByteBag(harness_build_bag_engine(config, &train_prepared)?)
        }
        EngineSelection::SeqFreqHybrid => {
            ClassifierEngine::SeqFreqHybrid(HybridTM::new_from_sub_engines(
                harness_build_conv_engine(config)?,
                harness_build_bag_engine(config, &train_prepared)?,
                VoteThreshold::new(config.vote_threshold)?,
            )?)
        }
    };

    // parallel version
    let mut rng = FastRng::seed(config.seed);
    let mut order: Vec<usize> = (0..train_prepared.len()).collect();
    let resolved_worker_count = WorkerCount::new(config.worker_count)?;

    let start = std::time::Instant::now();
    for _epoch in 0..config.epochs {
        // Per-epoch shuffle of the visitation order (documents themselves
        // are never moved — only the index order).
        shuffle_in_place(&mut order, &mut rng)?;
        for &doc_index in &order {
            let (document, label) = train_prepared
                .get(doc_index)
                .map(|(d, l)| (d, *l))
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            engine.engine_train_step_with_workers(
                document,
                label,
                &mut rng,
                resolved_worker_count,
            )?;
        }
    }
    let train_seconds = start.elapsed().as_secs_f64();

    // parallel
    // INVARIANT (misprediction capture): `test_prepared` was produced by
    // `preprocess_documents`, which emits exactly one output per input in
    // input order — so index i pairs each prepared document with its raw
    // original in `test_documents`. If preprocessing ever filters or
    // reorders, this pairing breaks; the length check inside the evaluator
    // catches that in every mode, not just debug.
    let evaluation = evaluate_test_split_with_workers(
        &engine,
        &test_prepared,
        test_documents,
        resolved_worker_count,
    )?;
    let clause_fire_counts = evaluation.clause_fire_counts;
    let vote_sums = evaluation.vote_sums;
    let labels = evaluation.labels;
    let mispredictions = evaluation.mispredictions;
    let accuracy_at_zero = evaluation.correct_at_zero as f64 / test_prepared.len() as f64;

    let test_row_predictions = build_row_prediction_records(test_documents, &vote_sums, false)?;
    let train_row_predictions = if config.score_train_side {
        let train_evaluation = evaluate_test_split_with_workers(
            &engine,
            &train_prepared,
            train_documents,
            resolved_worker_count,
        )?;
        build_row_prediction_records(train_documents, &train_evaluation.vote_sums, true)?
    } else {
        Vec::new()
    };

    let sweep_rows = sweep_decision_thresholds(&vote_sums, &labels)?;
    let mut best_f1_row = sweep_rows
        .first()
        .cloned()
        .ok_or(GranmoModelError::PrbSweepEmptyInput)?;
    for row in &sweep_rows {
        if row.f1 > best_f1_row.f1 {
            best_f1_row = row.clone();
        }
    }

    // Post-training specialization snapshot (vacuity/includes diagnostic).
    let clause_include_totals = engine.engine_clause_include_totals()?;

    let report = ExperimentReport {
        engine_name_reported: engine.engine_name(),
        train_count: train_documents.len(),
        test_count: test_documents.len(),
        accuracy_at_zero,
        best_f1_row,
        train_seconds,
        clause_fire_counts,
        fire_guard_reset_total: engine.engine_fire_guard_reset_total(),
        clause_include_totals,
        // The guard is armed for the bag engine and for the hybrid's bag
        // bank; the conv engine has no guard (recorded backlog).
        fire_guard_limit_used: match config.engine_selection {
            EngineSelection::ByteBag | EngineSelection::SeqFreqHybrid => {
                config.fire_guard_streak_limit
            }
            EngineSelection::ByteConv => FireGuardStreakLimit::DISABLED,
        },
        mispredictions,
        test_row_predictions,
        train_row_predictions,
        test_vote_sums: vote_sums,
        test_labels: labels,
    };
    Ok((engine, report))
}

// ===========================================================================
// SECTION 10B: Late-Fusion Ensemble Evaluation (research-harness tier)
// ===========================================================================
//
// Option 2 of the hybrid experiment: two INDEPENDENTLY trained engines,
// votes summed post hoc per test document, V_ens = V_conv + V_bag. No
// engine code is involved; this consumes two `ExperimentReport`s produced
// on the same split and re-derives every report field for the ensemble.
// Reporting tier: floats and heap permitted in the data path.

/// Sums two runs' TRAIN-side prediction records document-for-document.
///
/// Returns an empty vector when either input is empty (train-side scoring
/// is opt-in and a run may not have produced it). When both are present
/// they must be the same length and agree on `line_index` and label at
/// every position — proof the two runs scored the same training split in
/// the same order; any disagreement is `EnsVoteVectorMismatch`, never a
/// silent partial sum.
fn fuse_train_side_row_predictions(
    conv_train_records: &[RowPredictionRecord],
    bag_train_records: &[RowPredictionRecord],
) -> Result<Vec<RowPredictionRecord>, GranmoModelError> {
    if conv_train_records.is_empty() || bag_train_records.is_empty() {
        return Ok(Vec::new());
    }
    if conv_train_records.len() != bag_train_records.len() {
        #[cfg(debug_assertions)]
        eprintln!(
            "ENS-1202: train-side record count mismatch conv {} bag {}",
            conv_train_records.len(),
            bag_train_records.len()
        );
        return Err(GranmoModelError::EnsVoteVectorMismatch);
    }
    let mut fused = Vec::with_capacity(conv_train_records.len());
    for (conv_record, bag_record) in conv_train_records.iter().zip(bag_train_records.iter()) {
        if conv_record.line_index != bag_record.line_index
            || conv_record.label_is_positive != bag_record.label_is_positive
            || !conv_record.side_is_train
            || !bag_record.side_is_train
        {
            return Err(GranmoModelError::EnsVoteVectorMismatch);
        }
        fused.push(RowPredictionRecord {
            line_index: conv_record.line_index,
            side_is_train: true,
            label_is_positive: conv_record.label_is_positive,
            vote_sum: conv_record
                .vote_sum
                .checked_add(bag_record.vote_sum)
                .ok_or(GranmoModelError::HybArithmeticOverflow)?,
        });
    }
    Ok(fused)
}

/// Evaluates a late-fusion ensemble from two already-scored runs.
///
/// Preconditions (all checked, all reported as `Ens*` codes): the two
/// reports carry vote vectors of equal length, their label vectors are
/// EQUAL (proof of one split, one order), and `test_documents` /
/// `test_prepared` pair index-for-index with them (for misprediction
/// records). `test_prepared` must be produced with the same profile the
/// two runs used.
pub fn evaluate_late_fusion_ensemble(
    conv_report: &ExperimentReport,
    bag_report: &ExperimentReport,
    test_documents: &[LabeledDocument],
    test_prepared: &[(Vec<u8>, bool)],
) -> Result<ExperimentReport, GranmoModelError> {
    let conv_votes = &conv_report.test_vote_sums;
    let bag_votes = &bag_report.test_vote_sums;
    let labels = &conv_report.test_labels;
    let document_total = conv_votes.len();

    if bag_votes.len() != document_total
        || labels.len() != document_total
        || bag_report.test_labels.len() != document_total
        || test_documents.len() != document_total
        || test_prepared.len() != document_total
    {
        #[cfg(debug_assertions)]
        eprintln!(
            "ENS-1202: length mismatch conv {} bag {} labels {} docs {} prepared {}",
            conv_votes.len(),
            bag_votes.len(),
            labels.len(),
            test_documents.len(),
            test_prepared.len()
        );
        return Err(GranmoModelError::EnsVoteVectorMismatch);
    }
    if *labels != bag_report.test_labels {
        #[cfg(debug_assertions)]
        eprintln!("ENS-1204: the two reports were not scored on the same test split/order");
        return Err(GranmoModelError::EnsLabelVectorMismatch);
    }
    if document_total == 0 {
        return Err(GranmoModelError::PrbSweepEmptyInput);
    }

    let mut ensemble_votes: Vec<i32> = Vec::with_capacity(document_total);
    let mut correct_at_zero: usize = 0;
    let mut mispredictions: Vec<MispredictionRecord> = Vec::new();

    for index in 0..document_total {
        let conv_vote = *conv_votes
            .get(index)
            .ok_or(GranmoModelError::EnsVoteVectorMismatch)?;
        let bag_vote = *bag_votes
            .get(index)
            .ok_or(GranmoModelError::EnsVoteVectorMismatch)?;
        let label = *labels
            .get(index)
            .ok_or(GranmoModelError::EnsVoteVectorMismatch)?;
        let ensemble_vote = conv_vote
            .checked_add(bag_vote)
            .ok_or(GranmoModelError::HybArithmeticOverflow)?;
        ensemble_votes.push(ensemble_vote);

        let predicted_positive = ensemble_vote > 0;
        if predicted_positive == label {
            correct_at_zero = correct_at_zero
                .checked_add(1)
                .ok_or(GranmoModelError::HybArithmeticOverflow)?;
        } else {
            let raw_original = test_documents
                .get(index)
                .ok_or(GranmoModelError::EnsVoteVectorMismatch)?;
            let (prepared_text, _prepared_label) = test_prepared
                .get(index)
                .ok_or(GranmoModelError::EnsVoteVectorMismatch)?;
            mispredictions.push(MispredictionRecord {
                raw_text_bytes: raw_original.text.clone(),
                preprocessed_text_bytes: prepared_text.clone(),
                actual_label_is_positive: label,
                predicted_label_is_positive: predicted_positive,
                vote_sum_at_prediction: ensemble_vote,
            });
        }
    }

    let sweep_rows = sweep_decision_thresholds(&ensemble_votes, labels)?;
    let mut best_f1_row = sweep_rows
        .first()
        .cloned()
        .ok_or(GranmoModelError::PrbSweepEmptyInput)?;
    for row in &sweep_rows {
        if row.f1 > best_f1_row.f1 {
            best_f1_row = row.clone();
        }
    }

    // Diagnostics are concatenated conv-first, bag-second — the same order
    // as HybridTM's combined clause space, so the two rows read alike.
    let mut combined_fire_counts = conv_report.clause_fire_counts.clone();
    combined_fire_counts.extend_from_slice(&bag_report.clause_fire_counts);
    let mut combined_include_totals = conv_report.clause_include_totals.clone();
    combined_include_totals.extend_from_slice(&bag_report.clause_include_totals);

    // Per-row records for the ensemble. Test side: the summed vote per
    // document, paired index-for-index with `test_documents` (the same
    // pairing the length gates above already proved). Train side: the
    // fusion has no engine to re-score with, so train-side records exist
    // ONLY as the sum of the two component reports' train-side records,
    // and only when both runs recorded them (`--score-train-side`).
    let test_row_predictions =
        build_row_prediction_records(test_documents, &ensemble_votes, false)?;
    let train_row_predictions = fuse_train_side_row_predictions(
        &conv_report.train_row_predictions,
        &bag_report.train_row_predictions,
    )?;

    Ok(ExperimentReport {
        engine_name_reported: "seq-freq-late-fusion",
        train_count: conv_report.train_count,
        test_count: document_total,
        accuracy_at_zero: correct_at_zero as f64 / document_total as f64,
        best_f1_row,
        // Sum of the two independent training times (the ensemble's cost).
        train_seconds: conv_report.train_seconds + bag_report.train_seconds,
        clause_fire_counts: combined_fire_counts,
        fire_guard_reset_total: bag_report.fire_guard_reset_total,
        clause_include_totals: combined_include_totals,
        fire_guard_limit_used: bag_report.fire_guard_limit_used,
        mispredictions,
        test_row_predictions,
        train_row_predictions,
        test_vote_sums: ensemble_votes,
        test_labels: labels.clone(),
    })
}

// ===========================================================================
// SECTION 11: CLI (fail-fast parsing; train / predict / batch modes)
// ===========================================================================

/// Parsed CLI arguments. Unknown flags and malformed values are hard errors
/// (fail-fast policy) — a typo must never silently run with defaults,
/// because recorded benchmark runs are reconstructed from command lines.
#[derive(Debug, Clone)]
struct CliArgs {
    mode: String,
    data_path: Option<std::path::PathBuf>,
    text_key: String,
    label_key: String,
    positive_label: String,
    preset_name: String,
    engine_selection: EngineSelection,
    patch_size: u8,
    stride: u8,
    bag_ngram_len: u8,
    bag_vocab_size: u16,
    n_clauses: u16,
    vote_threshold: i16,
    states_per_action: i16,
    specificity: f64,
    max_scan_bytes: u32,
    guarded_include: bool,
    /// `--fire-guard` value: 0 = disabled (default). Validated through
    /// `FireGuardStreakLimit` in `to_run_config` (fail-fast at the CLI
    /// boundary, per crate policy).
    fire_guard_streak_limit: u32,
    epochs: u32,
    seed: u64,
    train_percent: u8,
    model_out: Option<std::path::PathBuf>,
    model_in: Option<std::path::PathBuf>,
    predict_text: Option<String>,
    log_out: Option<std::path::PathBuf>,
    /// `None` = resolve automatically from available parallelism.
    worker_count: Option<u16>,
    /// `--guard-limits` sweep for batch-guard mode (comma-separated;
    /// 0 = guard-off baseline row). Each value validated through
    /// `FireGuardStreakLimit` at parse time (fail-fast).
    guard_limits: Vec<u32>,
    /// `--test-cap`: truncate the test split to at most N documents
    /// (0 = no cap). BATCH-GUARD ONLY — deliberately not applied in
    /// train/batch so recorded baseline rows stay comparable. Truncation
    /// happens AFTER the seeded shuffle-split, so it is deterministic.
    test_cap: usize,
    /// `--split-seed`: seeds the shuffle/split (and k-fold) independently
    /// of the training seed. `None` = same as `--seed` (prior behavior).
    split_seed: Option<u64>,
    /// `--folds k`: 1 = single split (default); 2..=50 = k-fold.
    fold_count: u8,
    /// `--records-out`: per-row prediction-record file (opt-in).
    records_out: Option<std::path::PathBuf>,
    /// `--records-in`: input for `--mode row-audit`.
    records_in: Option<std::path::PathBuf>,
    /// `--score-train-side`: also record training-side predictions.
    score_train_side: bool,
    /// `--audit-top N`: rows printed by row-audit (default 50).
    audit_top: usize,
}

/// Maps a preset name to its profile (§5 presets of record).
fn preset_from_name(name: &str) -> Result<PreprocessProfile, GranmoModelError> {
    match name {
        "raw" => Ok(PreprocessProfile::preset_raw()),
        "p0" => Ok(PreprocessProfile::preset_p0()),
        "p1" => Ok(PreprocessProfile::preset_p1()),
        "p2" => Ok(PreprocessProfile::preset_p2()),
        "p3" => Ok(PreprocessProfile::preset_p3()),
        "p4" => Ok(PreprocessProfile::preset_p4()),
        "p5" => Ok(PreprocessProfile::preset_p5()),
        _other => {
            #[cfg(debug_assertions)]
            eprintln!("CLI-805: unknown preset '{}'", _other);
            Err(GranmoModelError::CliUnknownPreset)
        }
    }
}

/// Maps an `--engine` value to its selection (fail-fast: rejected at parse
/// time, never downstream). Names match `ClassifierEngine::engine_name`
/// exactly, so CLI input, run labels, and report rows use one spelling.
fn engine_selection_from_name(name: &str) -> Result<EngineSelection, GranmoModelError> {
    match name {
        "byte-conv" => Ok(EngineSelection::ByteConv),
        "byte-bag" => Ok(EngineSelection::ByteBag),
        "seq-freq-hybrid" => Ok(EngineSelection::SeqFreqHybrid),
        _other => {
            #[cfg(debug_assertions)]
            eprintln!("CLI-806: unknown engine '{}'", _other);
            Err(GranmoModelError::CliUnknownEngine)
        }
    }
}

/// Parses one numeric flag value; malformed input is a hard error attributed
/// to the flag (debug builds name the flag and value; production returns
/// only the code).
fn parse_flag_number<T: std::str::FromStr>(_flag: &str, raw: &str) -> Result<T, GranmoModelError> {
    match raw.parse::<T>() {
        Ok(value) => Ok(value),
        Err(_) => {
            #[cfg(debug_assertions)]
            eprintln!("CLI-802: flag '{}' has invalid value '{}'", _flag, raw);
            Err(GranmoModelError::CliInvalidValue)
        }
    }
}

impl CliArgs {
    /// Defaults mirror the specification of record: engine byte-conv, K=5,
    /// S=2, bag n=5, bag M=4000, 200 clauses, T=50, N=100, s=5.0, preset
    /// p0, 25 epochs, seed 42, 80/20 split.
    fn parse_cliargs(args: &[String]) -> Result<Self, GranmoModelError> {
        let mut parsed = Self {
            mode: String::new(),
            data_path: None,
            text_key: "text".to_string(),
            label_key: "label".to_string(),
            positive_label: "1".to_string(),
            preset_name: "p0".to_string(),
            engine_selection: EngineSelection::ByteConv,
            patch_size: 5,
            stride: 2,
            bag_ngram_len: 5,
            bag_vocab_size: 100, // default for quick test: moderate/normal: 4000
            n_clauses: 50,       // default for quick test: moderate/normal: 100-200
            vote_threshold: 50,
            states_per_action: 10, // default for quick test: moderate/normal: 100
            specificity: 5.0,
            max_scan_bytes: 1024,
            guarded_include: false,
            fire_guard_streak_limit: 0,
            epochs: 5, // default for quick test: moderate/normal: 25
            seed: 42,
            train_percent: 80,
            model_out: None,
            model_in: None,
            predict_text: None,
            log_out: None,
            worker_count: None,
            guard_limits: vec![0, 500, 2000, 8000],
            test_cap: 0,
            split_seed: None,
            fold_count: 1,
            records_out: None,
            records_in: None,
            score_train_side: false,
            audit_top: 50,
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
        while i < args.len() {
            let flag = args[i].as_str();
            match flag {
                "--mode" => parsed.mode = take_value(args, &mut i, flag)?.to_string(),
                "--data" => {
                    parsed.data_path =
                        Some(std::path::PathBuf::from(take_value(args, &mut i, flag)?))
                }
                "--text-key" => parsed.text_key = take_value(args, &mut i, flag)?.to_string(),
                "--label-key" => parsed.label_key = take_value(args, &mut i, flag)?.to_string(),
                "--positive-label" => {
                    parsed.positive_label = take_value(args, &mut i, flag)?.to_string()
                }
                "--preset" => parsed.preset_name = take_value(args, &mut i, flag)?.to_string(),
                "--engine" => {
                    parsed.engine_selection =
                        engine_selection_from_name(take_value(args, &mut i, flag)?)?
                }
                "--patch" => {
                    parsed.patch_size = parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--stride" => {
                    parsed.stride = parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--ngram-len" => {
                    parsed.bag_ngram_len = parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--vocab-size" => {
                    parsed.bag_vocab_size =
                        parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--clauses" => {
                    parsed.n_clauses = parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--vote-threshold" => {
                    parsed.vote_threshold =
                        parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--states" => {
                    parsed.states_per_action =
                        parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--specificity" => {
                    parsed.specificity = parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--max-scan" => {
                    parsed.max_scan_bytes =
                        parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--guarded" => parsed.guarded_include = true,
                "--fire-guard" => {
                    parsed.fire_guard_streak_limit =
                        parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--epochs" => {
                    parsed.epochs = parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--seed" => parsed.seed = parse_flag_number(flag, take_value(args, &mut i, flag)?)?,
                "--train-percent" => {
                    parsed.train_percent = parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--model-out" => {
                    parsed.model_out =
                        Some(std::path::PathBuf::from(take_value(args, &mut i, flag)?))
                }
                "--model-in" => {
                    parsed.model_in =
                        Some(std::path::PathBuf::from(take_value(args, &mut i, flag)?))
                }
                "--text" => parsed.predict_text = Some(take_value(args, &mut i, flag)?.to_string()),
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                "--log-out" => {
                    parsed.log_out = Some(std::path::PathBuf::from(take_value(args, &mut i, flag)?))
                }
                "--workers" => {
                    let raw_value = take_value(args, &mut i, flag)?;
                    parsed.worker_count = if raw_value == "auto" {
                        Some(WorkerCount::resolve_automatic().get()?)
                    } else {
                        Some(parse_flag_number::<u16>(flag, raw_value)?)
                    };
                }
                "--guard-limits" => {
                    let raw_list = take_value(args, &mut i, flag)?;
                    let mut parsed_limits: Vec<u32> = Vec::new();
                    for piece in raw_list.split(',') {
                        let trimmed = piece.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        let limit_value: u32 = parse_flag_number(flag, trimmed)?;
                        // Fail-fast at the CLI boundary: 0 or an active
                        // in-range limit; 1..=15 rejected as typos.
                        let _ = FireGuardStreakLimit::new(limit_value)?;
                        parsed_limits.push(limit_value);
                    }
                    if parsed_limits.is_empty() {
                        #[cfg(debug_assertions)]
                        eprintln!("CLI-802: --guard-limits produced an empty list");
                        return Err(GranmoModelError::CliInvalidValue);
                    }
                    parsed.guard_limits = parsed_limits;
                }
                "--test-cap" => {
                    parsed.test_cap = parse_flag_number(flag, take_value(args, &mut i, flag)?)?;
                }
                "--split-seed" => {
                    parsed.split_seed =
                        Some(parse_flag_number(flag, take_value(args, &mut i, flag)?)?)
                }
                "--folds" => {
                    parsed.fold_count = parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--records-out" => {
                    parsed.records_out =
                        Some(std::path::PathBuf::from(take_value(args, &mut i, flag)?))
                }
                "--records-in" => {
                    parsed.records_in =
                        Some(std::path::PathBuf::from(take_value(args, &mut i, flag)?))
                }
                "--score-train-side" => parsed.score_train_side = true,
                "--audit-top" => {
                    parsed.audit_top = parse_flag_number(flag, take_value(args, &mut i, flag)?)?
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

    /// The seed driving shuffle/split/k-fold: `--split-seed` if given,
    /// else `--seed` (the pre-fold behavior, preserved exactly).
    fn resolved_split_seed(&self) -> u64 {
        self.split_seed.unwrap_or(self.seed)
    }

    /// Assembles the resolved run config from parsed flags.
    fn to_run_config(&self) -> Result<HarnessRunConfig, GranmoModelError> {
        Ok(HarnessRunConfig {
            profile: preset_from_name(&self.preset_name)?,
            engine_selection: self.engine_selection,
            patch_size: self.patch_size,
            stride: self.stride,
            bag_ngram_len: self.bag_ngram_len,
            bag_vocab_size: self.bag_vocab_size,
            n_clauses: self.n_clauses,
            vote_threshold: self.vote_threshold,
            states_per_action: self.states_per_action,
            specificity: self.specificity,
            max_scan_bytes: self.max_scan_bytes,
            guarded_include: self.guarded_include,
            // Validate through the enforced newtype so an out-of-range
            // --fire-guard fails fast at the CLI boundary, then carry the
            // validated raw value (revalidated again at engine wiring).
            fire_guard_streak_limit: FireGuardStreakLimit::new(self.fire_guard_streak_limit)?
                .get()?,
            epochs: self.epochs,
            seed: self.seed,
            worker_count: {
                // Resolve, then validate through the enforced newtype so an
                // out-of-range --workers fails fast at the CLI boundary.
                let resolved = match self.worker_count {
                    Some(explicit) => explicit,
                    None => WorkerCount::resolve_automatic().get()?,
                };
                WorkerCount::new(resolved)?.get()?
            },
            score_train_side: self.score_train_side,
        })
    }
}

fn print_help() {
    println!("Byte-Convolution Granmo Model — experiment harness");
    println!("===================================================");
    println!("Dataset format: JSONL only — one JSON object per line with");
    println!("  a \"text\" string field and a \"label\" field (string or number).");
    println!("TRAIN:   --mode train --data /abs/path.jsonl [options]");
    println!("BATCH:   --mode batch --data /abs/path.jsonl [options]");
    println!(
        "         (comparison matrix: presets raw,p0,p1,p2 x [byte-conv, byte-bag, late-fusion, seq-freq-hybrid]"
    );
    println!("          byte-conv,byte-bag on ONE identical split and seed)");
    println!("BATCH-GUARD: --mode batch-guard --data /abs/path.jsonl [options]");
    println!("         (fire-guard ladder: byte-bag only, ONE preset, one row");
    println!("          per --guard-limits value; 0 = guard-off baseline)");
    println!("  --guard-limits 0,500,2000,8000  (comma-separated sweep)");
    println!("  --test-cap 0    (batch-guard only: cap eval docs; 0 = off)");
    println!("PREDICT: --mode predict --model-in /abs/model.gmb --text \"...\"");
    println!("Options (defaults) NOTE: '|' is a separator NOT 'or')");
    println!("  --text-key text | --label-key label | --positive-label 1");
    println!("  --preset p0 (raw|p0..p5)");
    println!("  --engine byte-conv (byte-conv|byte-bag|seq-freq-hybrid)");
    println!("  --patch 5 | --stride 2 | --guarded          (byte-conv only)");
    println!("  --ngram-len 5 | --vocab-size 4000           (byte-bag only)");
    println!("  --fire-guard 0  (byte-bag only; 0=off; active 16..=16777216:");
    println!("                   reset any clause that fires on N consecutive");
    println!("                   shuffled training docs while holding a learned");
    println!("                   pattern — always-fire pruning, Drop 4.1)");
    println!("  --clauses 200 | --vote-threshold 50 | --states 100");
    println!("  --specificity 5.0 | --max-scan 1024 | --epochs 25 | --seed 42");
    println!("  --train-percent 80 | --model-out /abs/path.gmb");
    println!("  --log-out /abs/path.txt  (misprediction inspection log;");
    println!("                   default: <exe_dir>/logs/))");
    println!("  --workers auto  (thread count; PERFORMANCE-ONLY — results are");
    println!("                   byte-identical at every worker count)");
    println!("===================================================");
    println!("  --preset -> optional cumulative levels of how much text preprocessing");
    println!("p0. None");
    println!("  --split-seed N   (shuffle/split seed; default: same as --seed)");
    println!("  --folds k        (1 = single split; 2..=50 = k-fold cross-validation)");
    println!("  --records-out /abs/path.tsv  (per-row prediction records, appended)");
    println!("  --score-train-side           (also record training-side predictions)");
    println!("ROW-AUDIT: --mode row-audit --records-in /abs/path.tsv [--audit-top 50]");
    println!(
        "p1. **`WhitespaceFold`** (Stage 1): Converts all newline (`\n`), carriage return (`\r`), and tab (`\t`) bytes into spaces (`' '`)."
    );
    println!(
        "p2. **`SpaceDedupe`** (Stage 2): Collapses consecutive runs of spaces down to a single space."
    );
    println!(
        "p3. **`LeadingTrim`** (Stage 3): Drops any leading spaces that occur before the first non-space byte."
    );
    println!(
        "p4. **`AsciiLowercase`** (Stage 4): Converts uppercase ASCII alphabetic bytes (`65` to `90`, or `'A'` through `'Z'`) to lowercase by adding `32`."
    );
}

/// Formats the S2-8 fire-rate diagnostic, now with the vacuous-vs-
/// specialized breakdown of the always-firing population (Drop 4.2):
/// a VACUOUS always-firer (zero includes) is bootstrap state — guard-
/// exempt by design, cured by depth/epoch budget; a SPECIALIZED
/// always-firer (a learned ubiquitous pattern) is the guard's actual
/// target. Reporting tier: floats and heap permitted.
fn summarize_fire_counts(
    clause_fire_counts: &[u32],
    clause_include_totals: &[u32],
    test_count: usize,
) -> String {
    if clause_fire_counts.is_empty() || test_count == 0 {
        return "fire-rate: (no data)".to_string();
    }
    let clause_total = clause_fire_counts.len();
    let never_fired = clause_fire_counts.iter().filter(|&&c| c == 0).count();
    let always_fired = clause_fire_counts
        .iter()
        .filter(|&&c| c as usize == test_count)
        .count();

    // Breakdown only when the include totals align (defensive: a length
    // mismatch means report wiring changed — degrade to the old line
    // rather than misattribute counts).
    let always_breakdown = if clause_include_totals.len() == clause_total {
        let mut always_vacuous = 0usize;
        let mut always_specialized = 0usize;
        for (&fired_count, &include_total) in
            clause_fire_counts.iter().zip(clause_include_totals.iter())
        {
            if fired_count as usize == test_count {
                if include_total == 0 {
                    always_vacuous += 1;
                } else {
                    always_specialized += 1;
                }
            }
        }
        format!(
            " ({} vacuous, {} specialized)",
            always_vacuous, always_specialized
        )
    } else {
        String::new()
    };

    let mut sorted_counts: Vec<u32> = clause_fire_counts.to_vec();
    sorted_counts.sort_unstable();
    let percentile_rate = |numerator: usize, denominator: usize| -> f64 {
        let index = (sorted_counts.len().saturating_sub(1)) * numerator / denominator;
        let count_at_index = sorted_counts.get(index).copied().unwrap_or(0);
        100.0 * f64::from(count_at_index) / test_count as f64
    };
    format!(
        "fire-rate over {} test docs: never {}/{}  always {}/{}{}  p25 {:.1}%  median {:.1}%  p75 {:.1}%",
        test_count,
        never_fired,
        clause_total,
        always_fired,
        clause_total,
        always_breakdown,
        percentile_rate(1, 4),
        percentile_rate(1, 2),
        percentile_rate(3, 4)
    )
}

/// Formats the includes-per-clause histogram (specialization summary).
/// Reading guide: a median near 0 after many epochs means automata are
/// not crossing the include boundary at the configured depth N within
/// the epoch budget — lower `--states` or raise `--epochs` before
/// reaching for the guard. Reporting tier.
fn summarize_include_totals(clause_include_totals: &[u32]) -> String {
    if clause_include_totals.is_empty() {
        return "includes/clause: (no data)".to_string();
    }
    let mut sorted_totals: Vec<u32> = clause_include_totals.to_vec();
    sorted_totals.sort_unstable();
    let vacuous_total = sorted_totals.iter().filter(|&&c| c == 0).count();
    let value_at = |numerator: usize, denominator: usize| -> u32 {
        let index = (sorted_totals.len().saturating_sub(1)) * numerator / denominator;
        sorted_totals.get(index).copied().unwrap_or(0)
    };
    format!(
        "includes/clause: min {}  p25 {}  median {}  p75 {}  max {}  ({} clauses vacuous)",
        sorted_totals.first().copied().unwrap_or(0),
        value_at(1, 4),
        value_at(1, 2),
        value_at(3, 4),
        sorted_totals.last().copied().unwrap_or(0),
        vacuous_total
    )
}

/// Net constant vote offset contributed by VACUOUS clauses. A vacuous
/// clause fires on EVERY document, so it adds a constant +1 (even index,
/// positive polarity) or −1 (odd index) to every vote sum. Reported so
/// the operator can decompose the optimal decision threshold into
/// structural offset vs. genuinely learned class asymmetry. Reporting
/// tier; i64 arithmetic cannot overflow (clause count <= 65534).
fn summarize_vacuous_vote_offset(clause_include_totals: &[u32]) -> String {
    let mut vacuous_positive_polarity: i64 = 0;
    let mut vacuous_negative_polarity: i64 = 0;
    for (clause, &include_total) in clause_include_totals.iter().enumerate() {
        if include_total == 0 {
            if clause % 2 == 0 {
                vacuous_positive_polarity += 1;
            } else {
                vacuous_negative_polarity += 1;
            }
        }
    }
    let net_offset = vacuous_positive_polarity - vacuous_negative_polarity;
    format!(
        "vacuous vote offset: {:+}  ({} positive-polarity, {} negative-polarity vacuous)",
        net_offset, vacuous_positive_polarity, vacuous_negative_polarity
    )
}

/// Prints a formatted classification evaluation report with metrics,
/// a 2x2 confusion matrix grid, and clause fire-rate diagnostics.
fn print_experiment_report(run_label: &str, report: &ExperimentReport) {
    let best = &report.best_f1_row;
    // Training wall-clock as h:m:s. `as u64` on a finite non-negative f64
    // saturates and cannot panic.
    let total_secs = report.train_seconds as u64;

    println!("\n============================================================");
    println!("               Classification Evaluation Report             ");
    println!("============================================================");
    println!(
        "  Run Preset:        {:<12} (Engine: {})",
        run_label, report.engine_name_reported
    );
    println!(
        "  Train/Test Split:  {}/{} samples",
        report.train_count, report.test_count
    );
    println!(
        "Training Time Duration (h:m:s): {:02}:{:02}:{:02}",
        total_secs / 3600,
        (total_secs % 3600) / 60,
        total_secs % 60
    );
    println!("------------------------------------------------------------");
    println!(
        "  Accuracy (@ V > 0): {:.2}%",
        report.accuracy_at_zero * 100.0
    );
    println!("  Best-F1 Threshold:  V > {}", best.decision_threshold);
    println!("  Precision:          {:.4}", best.precision);
    println!("  Recall:             {:.4}", best.recall);
    println!("  F1-Score:           {:.4}", best.f1);
    println!("------------------------------------------------------------");
    println!("Confusion Matrix (at optimal threshold):");
    println!("{:<18}{:<12}{:<12}", "", "Pred Neg (0)", "Pred Pos (1)");
    println!(
        "{:<18}{:<12}{:<12}",
        "Actual Neg (0)", best.true_negatives, best.false_positives
    );
    println!(
        "{:<18}{:<12}{:<12}",
        "Actual Pos (1)", best.false_negatives, best.true_positives
    );
    println!("------------------------------------------------------------");
    println!("Clause Dynamics:");
    println!(
        "  {}",
        summarize_fire_counts(
            &report.clause_fire_counts,
            &report.clause_include_totals,
            report.test_count
        )
    );
    println!(
        "  {}",
        summarize_include_totals(&report.clause_include_totals)
    );
    println!(
        "  {}",
        summarize_vacuous_vote_offset(&report.clause_include_totals)
    );
    // Guard activity is printed UNCONDITIONALLY whenever the guard was
    // armed: "resets 0" is a finding (guard ran, found nothing to prune),
    // previously indistinguishable from "guard off".
    if report.fire_guard_limit_used != FireGuardStreakLimit::DISABLED {
        println!(
            "  fire-guard: limit {}, resets {}",
            report.fire_guard_limit_used, report.fire_guard_reset_total
        );
    }

    println!("============================================================\n");
}

/*
Alt version draft
fn handle_train(args: &CliArgs) -> Result<(), GranmoModelError> {
    let data_path = args
        .data_path
        .as_ref()
        .ok_or(GranmoModelError::CliMissingRequiredFlag)?;

    println!("[1/3] Loading dataset from: {}", data_path.display());
    let documents = load_labeled_jsonl(
        data_path,
        &args.text_key,
        &args.label_key,
        &args.positive_label,
    )?;
    println!("  Total valid records loaded: {}", documents.len());

    let mut split_rng = FastRng::seed(args.seed);
    let (train_side, test_side) = split_dataset(&documents, args.train_percent, &mut split_rng)?;
    println!(
        "  Split dataset: {} train rows ({:.0}%), {} test rows ({:.0}%)",
        train_side.len(),
        args.train_percent,
        test_side.len(),
        100 - args.train_percent
    );

    let run_config = args.to_run_config()?;
    println!("[2/3] Initializing {} engine with preset '{}'...",
        args.engine_selection_from_name(&args.preset_name).map(|_| args.engine_selection).unwrap_or(EngineSelection::ByteConv).name(),
        args.preset_name
    );

    println!(
        "[3/3] Training {} ({} clauses, T={}, epochs={})...",
        match run_config.engine_selection {
            EngineSelection::ByteConv => "ByteConvTM",
            EngineSelection::ByteBag => "ByteBagTM",
        },
        run_config.n_clauses,
        run_config.vote_threshold,
        run_config.epochs
    );

    let (engine, report) = run_single_experiment(&train_side, &test_side, &run_config)?;
    print_experiment_report(&args.preset_name, &report);

    if let Some(model_path) = &args.model_out {
        let artifact = ModelArtifact {
            preprocess_profile: run_config.profile,
            engine,
        };
        artifact.save_to_file(model_path)?;
        println!("Successfully saved trained model artifact to: {}", model_path.display());
    }
    Ok(())
}
*/

/// Best-effort prediction-record writing (same non-fatal posture as the
/// misprediction log). No-op when `--records-out` was not given.
fn write_row_predictions_best_effort(
    records_path: Option<&std::path::Path>,
    context: RowPredictionContext<'_>,
    report: &ExperimentReport,
) {
    let Some(path) = records_path else {
        return;
    };
    let total = report
        .test_row_predictions
        .len()
        .saturating_add(report.train_row_predictions.len());
    let test_outcome = append_row_prediction_records(path, context, &report.test_row_predictions);
    let train_outcome = test_outcome
        .and_then(|()| append_row_prediction_records(path, context, &report.train_row_predictions));
    match train_outcome {
        Ok(()) => println!(
            "prediction records: appended {} records to {}",
            total,
            path.display()
        ),
        Err(write_error) => println!(
            "prediction records FAILED (code {}, retryable: {}) — run results above are unaffected",
            write_error.code(),
            write_error.is_retryable()
        ),
    }
}

/// Cross-fold summary line: mean and range of the two headline metrics.
/// Reporting tier (floats permitted).
fn print_fold_summary(fold_accuracies: &[f64], fold_best_f1s: &[f64]) {
    fn mean_min_max(values: &[f64]) -> (f64, f64, f64) {
        if values.is_empty() {
            return (0.0, 0.0, 0.0);
        }
        let sum: f64 = values.iter().sum();
        let min = values.iter().fold(f64::INFINITY, |acc, &v| acc.min(v));
        let max = values.iter().fold(f64::NEG_INFINITY, |acc, &v| acc.max(v));
        (sum / values.len() as f64, min, max)
    }
    let (acc_mean, acc_min, acc_max) = mean_min_max(fold_accuracies);
    let (f1_mean, f1_min, f1_max) = mean_min_max(fold_best_f1s);
    println!("============================================================");
    println!(
        "k-fold summary over {} folds: accuracy@0 mean {:.2}% (min {:.2}%, max {:.2}%)  best-F1 mean {:.4} (min {:.4}, max {:.4})",
        fold_accuracies.len(),
        acc_mean * 100.0,
        acc_min * 100.0,
        acc_max * 100.0,
        f1_mean,
        f1_min,
        f1_max
    );
    println!("============================================================\n");
}

/// Best-effort misprediction logging for the CLI handlers. Resolves the
/// destination (`--log-out` override, else the executable-anchored
/// default), attempts the append, and reports the outcome to stdout.
/// DELIBERATELY NON-FATAL: a training run whose report has already printed
/// must not exit nonzero because telemetry writing failed — the error code
/// is surfaced for the operator instead of propagated.
fn log_mispredictions_best_effort(
    explicit_log_path: Option<&std::path::Path>,
    data_source_path: &std::path::Path,
    run_label: &str,
    report: &ExperimentReport,
) {
    if report.mispredictions.is_empty() {
        println!("misprediction log: no mispredictions this run (nothing appended)");
        return;
    }
    let resolved_path = match explicit_log_path {
        Some(path) => path.to_path_buf(),
        None => match resolve_default_misprediction_log_path() {
            Ok(path) => path,
            Err(resolution_error) => {
                println!(
                    "misprediction log SKIPPED: path resolution failed (code {})",
                    resolution_error.code()
                );
                return;
            }
        },
    };
    match append_mispredictions_to_log(
        &resolved_path,
        data_source_path,
        run_label,
        report.engine_name_reported,
        &report.mispredictions,
    ) {
        Ok(()) => println!(
            "misprediction log: appended {} records to {}",
            report.mispredictions.len(),
            resolved_path.display()
        ),
        Err(append_error) => println!(
            "misprediction log FAILED (code {}, retryable: {}) — run results above are unaffected",
            append_error.code(),
            append_error.is_retryable()
        ),
    }
}

fn handle_train(args: &CliArgs) -> Result<(), GranmoModelError> {
    let data_path = args
        .data_path
        .as_ref()
        .ok_or(GranmoModelError::CliMissingRequiredFlag)?;
    let documents = load_labeled_jsonl(
        data_path,
        &args.text_key,
        &args.label_key,
        &args.positive_label,
    )?;
    println!("loaded {} labeled documents", documents.len());

    let run_config = args.to_run_config()?;
    let split_seed = args.resolved_split_seed();
    println!("resolved config: {:?}", run_config);
    println!("split seed {}, training seed {}", split_seed, args.seed);

    if run_config.fire_guard_streak_limit != FireGuardStreakLimit::DISABLED
        && run_config.engine_selection == EngineSelection::ByteConv
    {
        println!(
            "note: --fire-guard applies to the byte-bag engine only; \
             it is ignored for byte-conv this run (conv port is recorded backlog)"
        );
    }

    // --- Single split (fold_count <= 1): the pre-fold path, unchanged ---
    if args.fold_count <= 1 {
        let mut split_rng = FastRng::seed(split_seed);
        let (train_side, test_side) =
            split_dataset(&documents, args.train_percent, &mut split_rng)?;
        let (engine, report) = run_single_experiment(&train_side, &test_side, &run_config)?;
        print_experiment_report(&args.preset_name, &report);
        log_mispredictions_best_effort(
            args.log_out.as_deref(),
            data_path,
            &args.preset_name,
            &report,
        );
        write_row_predictions_best_effort(
            args.records_out.as_deref(),
            RowPredictionContext {
                split_seed,
                train_seed: args.seed,
                fold_index: 0,
                engine_name: report.engine_name_reported,
                run_label: &args.preset_name,
            },
            &report,
        );
        if let Some(model_path) = &args.model_out {
            let artifact = ModelArtifact {
                preprocess_profile: run_config.profile,
                engine,
            };
            artifact.save_to_file(model_path)?;
            println!("saved model artifact to {}", model_path.display());
        }
        return Ok(());
    }

    // --- k-fold: one run per fold, records per fold, summary at the end ---
    // `--model-out` is ambiguous under k-fold (k models, one path):
    // rejected fail-fast rather than silently saving the last fold.
    if args.model_out.is_some() {
        #[cfg(debug_assertions)]
        eprintln!("CLI-802: --model-out is not valid with --folds > 1");
        return Err(GranmoModelError::CliInvalidValue);
    }

    let mut split_rng = FastRng::seed(split_seed);
    let folds = split_dataset_kfold(&documents, args.fold_count, &mut split_rng)?;
    let mut fold_accuracies: Vec<f64> = Vec::with_capacity(folds.len());
    let mut fold_best_f1s: Vec<f64> = Vec::with_capacity(folds.len());
    let kfold_start = std::time::Instant::now();

    for (fold_index, (train_side, test_side)) in folds.iter().enumerate() {
        let fold_index_u16 =
            u16::try_from(fold_index).map_err(|_| GranmoModelError::DsFoldGeometryFault)?;
        let run_label = format!("{}+fold{}", args.preset_name, fold_index);
        let (_engine, report) = run_single_experiment(train_side, test_side, &run_config)?;
        print_experiment_report(&run_label, &report);
        log_mispredictions_best_effort(args.log_out.as_deref(), data_path, &run_label, &report);
        write_row_predictions_best_effort(
            args.records_out.as_deref(),
            RowPredictionContext {
                split_seed,
                train_seed: args.seed,
                fold_index: fold_index_u16,
                engine_name: report.engine_name_reported,
                run_label: &run_label,
            },
            &report,
        );
        fold_accuracies.push(report.accuracy_at_zero);
        fold_best_f1s.push(report.best_f1_row.f1);
    }

    print_fold_summary(&fold_accuracies, &fold_best_f1s);
    let total_secs = kfold_start.elapsed().as_secs();
    println!(
        "k-fold total duration (h:m:s): {:02}:{:02}:{:02}",
        total_secs / 3600,
        (total_secs % 3600) / 60,
        total_secs % 60
    );
    Ok(())
}

/// Batch mode 2.0 — the §8 comparison-matrix command, now in two blocks:
///
/// MATRIX 1 (recorded baseline, unchanged from batch 1.0): the priority
/// presets (raw, P0, P1, P2) × BOTH engines, on ONE identical split with
/// ONE identical training seed, with the fire guard FORCED OFF for every
/// row — even when `--fire-guard` was supplied. This keeps every baseline
/// row byte-comparable with all previously recorded batch results.
///
/// MATRIX 2 (Drop 4.1 guard arms): runs ONLY when `--fire-guard` is
/// active. One additional byte-bag row per preset with the guard ON,
/// labeled `<preset>+fireguard`. Each guard row differs from its Matrix-1
/// byte-bag counterpart in EXACTLY one variable (the guard), on the same
/// split and seed — the single-variable comparison the guard experiment
/// is judged on. Reading guide: compare (a) the fire-rate "always" count
/// (outcome), (b) the fire-guard reset total (activity), and (c) best-F1,
/// guard row vs. baseline row. Guard rows deliberately run the BAG only:
/// the conv engine has no guard yet, and a conv "guard row" would be an
/// unlabeled duplicate of its baseline row.
fn handle_batch(args: &CliArgs) -> Result<(), GranmoModelError> {
    let data_path = args
        .data_path
        .as_ref()
        .ok_or(GranmoModelError::CliMissingRequiredFlag)?;
    let documents = load_labeled_jsonl(
        data_path,
        &args.text_key,
        &args.label_key,
        &args.positive_label,
    )?;
    let mut split_rng = FastRng::seed(args.resolved_split_seed());
    let (train_side, test_side) = split_dataset(&documents, args.train_percent, &mut split_rng)?;
    println!(
        "batch over {} train / {} test documents, seed {}",
        train_side.len(),
        test_side.len(),
        args.seed
    );

    let batch_start = std::time::Instant::now();

    if args.fire_guard_streak_limit != FireGuardStreakLimit::DISABLED {
        println!(
            "fire-guard arms enabled (limit {}): baseline matrix runs guard-OFF; \
             byte-bag guard-ON rows follow as '<preset>+fireguard'",
            args.fire_guard_streak_limit
        );
    }

    // --- MATRIX 1: baseline (guard unconditionally OFF), 4 rows per preset:
    //     byte-conv, byte-bag, late-fusion (post-hoc sum of the two
    //     preceding rows' votes), hybrid (joint co-training). Rows 3 and 4
    //     carry 2x the clause budget of rows 1 and 2 (recorded confound);
    //     row 3 vs. row 4 is the single-variable comparison of record. ---
    for preset_name in ["raw", "p0", "p1", "p2"] {
        let preset_profile = preset_from_name(preset_name)?;

        let mut conv_config = args.to_run_config()?;
        conv_config.profile = preset_profile;
        conv_config.engine_selection = EngineSelection::ByteConv;
        conv_config.fire_guard_streak_limit = FireGuardStreakLimit::DISABLED;
        let (_conv_engine, conv_report) =
            run_single_experiment(&train_side, &test_side, &conv_config)?;
        print_experiment_report(preset_name, &conv_report);

        let mut bag_config = args.to_run_config()?;
        bag_config.profile = preset_profile;
        bag_config.engine_selection = EngineSelection::ByteBag;
        bag_config.fire_guard_streak_limit = FireGuardStreakLimit::DISABLED;
        let (_bag_engine, bag_report) =
            run_single_experiment(&train_side, &test_side, &bag_config)?;
        print_experiment_report(preset_name, &bag_report);

        // Row 3: late fusion of rows 1 and 2 (no training; re-preprocess
        // the test side once for misprediction records).
        let test_prepared_for_preset = preprocess_documents(preset_profile, &test_side)?;
        let fusion_report = evaluate_late_fusion_ensemble(
            &conv_report,
            &bag_report,
            &test_side,
            &test_prepared_for_preset,
        )?;
        let fusion_label = format!("{preset_name}+late-fusion");
        print_experiment_report(&fusion_label, &fusion_report);

        // Row 4: joint co-training.
        let mut hybrid_config = args.to_run_config()?;
        hybrid_config.profile = preset_profile;
        hybrid_config.engine_selection = EngineSelection::SeqFreqHybrid;
        hybrid_config.fire_guard_streak_limit = FireGuardStreakLimit::DISABLED;
        let (_hybrid_engine, hybrid_report) =
            run_single_experiment(&train_side, &test_side, &hybrid_config)?;
        print_experiment_report(preset_name, &hybrid_report);
    }

    // --- MATRIX 2: byte-bag guard arms (only when --fire-guard active) ---
    if args.fire_guard_streak_limit != FireGuardStreakLimit::DISABLED {
        for preset_name in ["raw", "p0", "p1", "p2"] {
            let mut run_config = args.to_run_config()?;
            run_config.profile = preset_from_name(preset_name)?;
            run_config.engine_selection = EngineSelection::ByteBag;
            // to_run_config already validated and carried the active limit.
            let (_trained_engine, report) =
                run_single_experiment(&train_side, &test_side, &run_config)?;
            let guard_run_label = format!("{preset_name}+fireguard");
            print_experiment_report(&guard_run_label, &report);
        }
    }

    // duration hr:min:sec
    let total_secs = batch_start.elapsed().as_secs();
    println!(
        "batch total duration (h:m:s): {:02}:{:02}:{:02}",
        total_secs / 3600,
        (total_secs % 3600) / 60,
        total_secs % 60
    );

    Ok(())
}

/*
Fast iteration recipe
```bash
cargo run --release -- --mode batch-guard \
  --data /abs/path/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl \
  --preset p0 --clauses 200 --vote-threshold 80 \
  --states 50 --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 4 --seed 42 --workers auto \
  --guard-limits 0,200,1000 --test-cap 3000
```
Lower --states 50 is deliberate: it forces fast specialization,
so if pathological (specialized) always-firers can exist at all,
they'll appear here — and the resets line plus the (V vacuous,
S specialized) breakdown will tell you in minutes whether the
guard fires on them. If your full-scale run's 44–49 "always"
clauses turn out all-vacuous, the recorded conclusion is: guard
correct, nothing to prune at N=200/12 epochs; the depth/epoch
budget (or vocabulary frequency capping) is the next lever.

One compile note: the summarize_fire_counts signature change
has exactly one call site (inside print_experiment_report),
which Edit 6 replaces — the compiler will confirm nothing
else references it.
*/

/// batch-guard — the FOCUSED fire-guard iteration mode (Drop 4.2).
///
/// Byte-bag only, ONE preset (`--preset`, default p0), one identical
/// split and seed, one row per `--guard-limits` value (0 = guard-off
/// baseline). This is the single-variable ladder for tuning the guard
/// without paying for the full §8 matrix: at production settings the
/// full batch takes hours; this mode with `--test-cap` and reduced
/// `--states`/`--epochs` takes minutes.
///
/// Reading guide per row: (a) the always-fire breakdown — if the always
/// population is (mostly) VACUOUS, the guard is working as designed and
/// has nothing to prune (the lever is depth/epochs, not the guard);
/// (b) the fire-guard resets line — nonzero resets with a shrinking
/// SPECIALIZED always-count is the guard doing its job; nonzero resets
/// with an UNCHANGED specialized count means clauses re-converge to
/// ubiquitous patterns (argues for vocabulary frequency capping);
/// (c) best-F1 vs. the guard-off row — the cost/benefit verdict.
///
/// CAVEAT (recorded): guard-limit semantics scale with train-split size —
/// a limit of 2000 against a subsampled train side is proportionally
/// stricter than against the full corpus. Sweep limits proportionally
/// when using `--train-percent` to shrink the training side.
fn handle_batch_guard(args: &CliArgs) -> Result<(), GranmoModelError> {
    let data_path = args
        .data_path
        .as_ref()
        .ok_or(GranmoModelError::CliMissingRequiredFlag)?;
    let documents = load_labeled_jsonl(
        data_path,
        &args.text_key,
        &args.label_key,
        &args.positive_label,
    )?;
    let mut split_rng = FastRng::seed(args.resolved_split_seed());
    let (train_side, mut test_side) =
        split_dataset(&documents, args.train_percent, &mut split_rng)?;

    // Optional eval cap for fast iteration. Applied AFTER the seeded
    // shuffle-split: deterministic, and the retained prefix is an
    // unbiased sample because the split already shuffled.
    if args.test_cap > 0 && test_side.len() > args.test_cap {
        test_side.truncate(args.test_cap);
    }

    println!(
        "batch-guard over {} train / {} test documents (test-cap {}), seed {}, preset {}, limits {:?}",
        train_side.len(),
        test_side.len(),
        args.test_cap,
        args.seed,
        args.preset_name,
        args.guard_limits
    );

    // Time Clock
    let batch_start = std::time::Instant::now();

    for &guard_limit in &args.guard_limits {
        let mut run_config = args.to_run_config()?;
        run_config.profile = preset_from_name(&args.preset_name)?;
        run_config.engine_selection = EngineSelection::ByteBag;
        run_config.fire_guard_streak_limit = FireGuardStreakLimit::new(guard_limit)?.get()?;
        let (_trained_engine, report) =
            run_single_experiment(&train_side, &test_side, &run_config)?;
        let guard_run_label = if guard_limit == FireGuardStreakLimit::DISABLED {
            format!("{}+guard-off", args.preset_name)
        } else {
            format!("{}+guard-{}", args.preset_name, guard_limit)
        };
        print_experiment_report(&guard_run_label, &report);
    }

    // duration hr:min:sec
    let total_secs = batch_start.elapsed().as_secs();
    println!(
        "batch total duration (h:m:s): {:02}:{:02}:{:02}",
        total_secs / 3600,
        (total_secs % 3600) / 60,
        total_secs % 60
    );
    Ok(())
}

fn handle_predict(args: &CliArgs) -> Result<(), GranmoModelError> {
    let model_path = args
        .model_in
        .as_ref()
        .ok_or(GranmoModelError::CliMissingRequiredFlag)?;
    let text = args
        .predict_text
        .as_ref()
        .ok_or(GranmoModelError::CliMissingRequiredFlag)?;

    let artifact = ModelArtifact::load_from_file(model_path)?;
    // Replay EXACTLY the persisted preprocessing (locked decision §10.9).
    let mut preprocessor = BytePreprocessor::new(artifact.preprocess_profile)?;
    let processed = preprocessor.process_document(text.as_bytes())?;

    // Engine-agnostic surface: vote, probability, label.
    let vote = artifact.engine.engine_vote_sum(&processed)?;
    let lut = artifact.engine.engine_build_probability_lut()?;
    let probability = lut.probability_for_report(vote)?;
    let label_is_positive = vote > 0;

    println!("\n============================================================");
    println!("                    Single Text Inference                   ");
    println!("============================================================");
    println!("Input Text:        \"{}\"", text);
    println!("Engine Model:      {}", artifact.engine.engine_name());
    println!(
        "Predicted Label:   {}",
        if label_is_positive {
            "POSITIVE (1)"
        } else {
            "NEGATIVE (0)"
        }
    );
    println!("Signed Vote Sum:   {:+4} votes", vote);
    println!("Calibrated Prob:   {:.2}%", probability * 100.0);
    println!("------------------------------------------------------------");
    println!("Triggered Logic Rules (Explainability Trace):");

    // Explainability trace — engine-specific by nature: the conv engine
    // reports byte-offset window spans (positional evidence); the bag has
    // no positional structure, so it reports fired clauses decoded as
    // shingle patterns only. This asymmetry is the models' actual
    // difference, surfaced honestly rather than papered over.
    match &artifact.engine {
        ClassifierEngine::ByteConv(conv_engine) => {
            print_conv_explain_trace(conv_engine, &processed, 10, "")?
        }
        ClassifierEngine::ByteBag(bag_engine) => {
            print_bag_explain_trace(bag_engine, &processed, 10, "")?
        }
        ClassifierEngine::SeqFreqHybrid(hybrid_engine) => {
            print_conv_explain_trace(hybrid_engine.hyb_conv_engine_ref(), &processed, 10, "conv ")?;
            print_bag_explain_trace(hybrid_engine.hyb_bag_engine_ref(), &processed, 10, "bag ")?;
        }
    }

    // match &artifact.engine {
    //     ClassifierEngine::ByteConv(conv_engine) => {
    //         let mut rules_printed = 0usize;
    //         for clause in 0..conv_engine.conv_clause_count() {
    //             if rules_printed >= 10 {
    //                 break;
    //             }
    //             let positions = conv_engine.conv_fired_window_positions(clause, &processed)?;
    //             if positions.is_empty() {
    //                 continue;
    //             }
    //             let pattern = conv_engine.conv_describe_clause(clause, 12)?;
    //             if pattern.is_empty() {
    //                 continue; // empty clause fires everywhere; not a meaningful rule
    //             }
    //             let polarity = if clause % 2 == 0 { "+" } else { "-" };
    //             println!("  [clause {clause} ({polarity})] {pattern} @ byte offsets {positions:?}");
    //             rules_printed += 1;
    //         }
    //     }
    //     ClassifierEngine::ByteBag(bag_engine) => {
    //         let fired_words = bag_engine.bag_fired_clause_bits(&processed)?;
    //         let mut rules_printed = 0usize;
    //         for clause in 0..bag_engine.bag_clause_count() {
    //             if rules_printed >= 10 {
    //                 break;
    //             }
    //             let fired_word = fired_words
    //                 .get(clause >> 6)
    //                 .copied()
    //                 .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
    //             if fired_word & (1u64 << (clause & 63)) == 0 {
    //                 continue;
    //             }
    //             let pattern = bag_engine.bag_describe_clause(clause, 12)?;
    //             if pattern.is_empty() {
    //                 continue; // empty clause fires everywhere; not a meaningful rule
    //             }
    //             let polarity = if clause % 2 == 0 { "+" } else { "-" };
    //             println!("  [clause {clause} ({polarity})] {pattern}");
    //             rules_printed += 1;
    //         }
    //     }
    // }

    Ok(())
}

/// Prints up to `max_rules` fired conv clauses with window offsets.
/// `bank_tag` labels the bank in hybrid traces ("" for standalone).
fn print_conv_explain_trace(
    conv_engine: &ByteConvTM,
    processed: &[u8],
    max_rules: usize,
    bank_tag: &str,
) -> Result<(), GranmoModelError> {
    let mut rules_printed = 0usize;
    for clause in 0..conv_engine.conv_clause_count() {
        if rules_printed >= max_rules {
            break;
        }
        let positions = conv_engine.conv_fired_window_positions(clause, processed)?;
        if positions.is_empty() {
            continue;
        }
        let pattern = conv_engine.conv_describe_clause(clause, 12)?;
        if pattern.is_empty() {
            continue; // vacuous clause fires everywhere; not a meaningful rule
        }
        let polarity = if clause % 2 == 0 { "+" } else { "-" };
        println!(
            "  [{bank_tag}clause {clause} ({polarity})] {pattern} @ byte offsets {positions:?}"
        );
        rules_printed += 1;
    }
    Ok(())
}

/// Prints up to `max_rules` fired bag clauses (no positional spans exist).
fn print_bag_explain_trace(
    bag_engine: &ByteBagTM,
    processed: &[u8],
    max_rules: usize,
    bank_tag: &str,
) -> Result<(), GranmoModelError> {
    let fired_words = bag_engine.bag_fired_clause_bits(processed)?;
    let mut rules_printed = 0usize;
    for clause in 0..bag_engine.bag_clause_count() {
        if rules_printed >= max_rules {
            break;
        }
        let fired_word = fired_words
            .get(clause >> 6)
            .copied()
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        if fired_word & (1u64 << (clause & 63)) == 0 {
            continue;
        }
        let pattern = bag_engine.bag_describe_clause(clause, 12)?;
        if pattern.is_empty() {
            continue;
        }
        let polarity = if clause % 2 == 0 { "+" } else { "-" };
        println!("  [{bank_tag}clause {clause} ({polarity})] {pattern}");
        rules_printed += 1;
    }
    Ok(())
}

/// Compile-time ceiling for shingle length, tied to `NgramLength::MAX`;
/// sized so a padded shingle fits a fixed stack buffer (no heap in the
/// per-window path of feature extraction).
const BAG_NGRAM_MAX_LEN: usize = 16;

/// A learned byte-n-gram vocabulary.
///
/// ## Storage layout
/// - `ngram_flat_bytes`: rank-ordered concatenation, `rank r` occupies
///   `[r*n, (r+1)*n)`. Rank IS the feature index the engine will use, and
///   this flat form is exactly what artifact kind 3 will persist (fixed
///   width — no length prefixes, no escaping).
/// - `lookup_order`: ranks sorted by shingle BYTES, enabling dependency-free
///   binary-search lookup. Derived data — rebuilt on load, never persisted,
///   never trusted (same pattern as ByteConvTM's masks).
#[derive(Debug, Clone)]
pub struct ByteBagVocabulary {
    /// Shingle length n in bytes.
    ngram_len: usize,
    /// Rank-ordered shingle bytes: `vocabulary_len * ngram_len` bytes.
    ngram_flat_bytes: Vec<u8>,
    /// Ranks sorted by shingle bytes (derived; for binary-search lookup).
    lookup_order: Vec<u32>,
}

impl ByteBagVocabulary {
    /// Builds the vocabulary from preprocessed training documents.
    ///
    /// Counting uses `std::collections::HashMap` (heap use in the research
    /// harness DATA path — permitted); determinism does NOT depend on map
    /// iteration order because entries are fully sorted afterwards by
    /// `(count desc, bytes asc)` before truncation to the top M.
    /// The actual vocabulary may be SMALLER than M if the corpus has fewer
    /// distinct shingles — that is recorded, not an error.
    pub fn build_from_documents(
        ngram_len: NgramLength,
        max_vocab_size: VocabSize,
        documents: &[&[u8]],
    ) -> Result<Self, GranmoModelError> {
        let n = usize::from(ngram_len.get()?);
        let cap = usize::from(max_vocab_size.get()?);

        let mut shingle_counts: std::collections::HashMap<Vec<u8>, u64> =
            std::collections::HashMap::new();

        for document in documents {
            if document.len() < n {
                // Short document: ONE right-padded shingle (PAD rule §10.4).
                let mut padded = [PAD_BYTE; BAG_NGRAM_MAX_LEN];
                for (target, &source) in padded.iter_mut().zip(document.iter()) {
                    *target = source;
                }
                let shingle = padded
                    .get(..n)
                    .ok_or(GranmoModelError::BbgVocabIndexOutOfRange)?;
                let counter = shingle_counts.entry(shingle.to_vec()).or_insert(0u64);
                *counter = counter
                    .checked_add(1)
                    .ok_or(GranmoModelError::BbgArithmeticOverflow)?;
            } else {
                // Overlapping stride-1 shingles over the full document.
                let last_start = document.len() - n; // len >= n: cannot underflow
                for start in 0..=last_start {
                    let end = start
                        .checked_add(n)
                        .ok_or(GranmoModelError::BbgArithmeticOverflow)?;
                    let shingle = document
                        .get(start..end)
                        .ok_or(GranmoModelError::BbgVocabIndexOutOfRange)?;
                    let counter = shingle_counts.entry(shingle.to_vec()).or_insert(0u64);
                    *counter = counter
                        .checked_add(1)
                        .ok_or(GranmoModelError::BbgArithmeticOverflow)?;
                }
            }
        }

        if shingle_counts.is_empty() {
            return Err(GranmoModelError::BbgVocabEmptyCorpus);
        }

        // Deterministic ranking: count descending, bytes ascending tiebreak.
        let mut ranked: Vec<(Vec<u8>, u64)> = shingle_counts.into_iter().collect();
        ranked.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        ranked.truncate(cap);

        let mut ngram_flat_bytes: Vec<u8> = Vec::with_capacity(
            ranked
                .len()
                .checked_mul(n)
                .ok_or(GranmoModelError::BbgArithmeticOverflow)?,
        );
        for (shingle, _count) in &ranked {
            ngram_flat_bytes.extend_from_slice(shingle);
        }

        let mut vocabulary = Self {
            ngram_len: n,
            ngram_flat_bytes,
            lookup_order: Vec::new(),
        };
        vocabulary.rebuild_lookup_order()?;
        Ok(vocabulary)
    }

    /// Reconstructs a vocabulary from its persisted rank-ordered flat bytes
    /// (artifact kind-3 load path). The flat storage is the ONLY persisted
    /// vocabulary data; the byte-sorted lookup order is rebuilt here and
    /// never trusted from disk (derived-data rule). Structural validation —
    /// whole-shingle divisibility, nonzero count, strict byte ordering
    /// (which doubles as duplicate detection) — runs inside
    /// `rebuild_lookup_order` via `validity_recheck`, so a tampered or
    /// corrupt vocabulary payload is rejected at reconstruction.
    pub fn from_flat_bytes(
        ngram_len: NgramLength,
        flat_bytes: Vec<u8>,
    ) -> Result<Self, GranmoModelError> {
        let n = usize::from(ngram_len.get()?);
        let mut restored = Self {
            ngram_len: n,
            ngram_flat_bytes: flat_bytes,
            lookup_order: Vec::new(),
        };
        restored.rebuild_lookup_order()?;
        Ok(restored)
    }

    /// Number of shingles in the vocabulary (may be < requested M).
    pub fn vocabulary_len(&self) -> usize {
        // Integer division is total; a corrupt flat length is caught by
        // `validity_recheck`, which callers run at load/wire time.
        if self.ngram_len == 0 {
            0
        } else {
            self.ngram_flat_bytes.len() / self.ngram_len
        }
    }

    /// Shingle length n.
    pub fn ngram_length(&self) -> usize {
        self.ngram_len
    }

    /// The shingle bytes at a given rank — the explainability primitive
    /// (decodes an engine literal index back to a human-readable byte
    /// pattern, mirroring `describe_clause` on the conv side).
    pub fn ngram_at_rank(&self, rank: usize) -> Result<&[u8], GranmoModelError> {
        let start = rank
            .checked_mul(self.ngram_len)
            .ok_or(GranmoModelError::BbgVocabIndexOutOfRange)?;
        let end = start
            .checked_add(self.ngram_len)
            .ok_or(GranmoModelError::BbgVocabIndexOutOfRange)?;
        self.ngram_flat_bytes
            .get(start..end)
            .ok_or(GranmoModelError::BbgVocabIndexOutOfRange)
    }

    /// Rebuilds the byte-sorted lookup order from the flat storage.
    /// Derived-data rule: called at construction and (later) artifact load;
    /// never persisted.
    fn rebuild_lookup_order(&mut self) -> Result<(), GranmoModelError> {
        let count = self.vocabulary_len();
        let mut order: Vec<u32> = Vec::with_capacity(count);
        for rank in 0..count {
            order.push(u32::try_from(rank).map_err(|_| GranmoModelError::BbgArithmeticOverflow)?);
        }
        // Sort ranks by their shingle bytes. The comparator must be total
        // and panic-free: ranks in `order` were just generated in-bounds,
        // but a defensive fetch failure sorts as Equal (harmless ordering)
        // and is then caught by validity_recheck's strict-ascending check.
        order.sort_unstable_by(|&rank_a, &rank_b| {
            match (
                self.ngram_at_rank(rank_a as usize),
                self.ngram_at_rank(rank_b as usize),
            ) {
                (Ok(bytes_a), Ok(bytes_b)) => bytes_a.cmp(bytes_b),
                _ => core::cmp::Ordering::Equal,
            }
        });
        self.lookup_order = order;
        self.vocab_validity_recheck()
    }

    /// Binary-search lookup: shingle bytes -> rank (feature index).
    /// `Ok(None)` = not in vocabulary (absent feature, by design not an
    /// error). A wrong-LENGTH shingle is a caller wiring bug -> error code.
    pub fn lookup(&self, shingle: &[u8]) -> Result<Option<usize>, GranmoModelError> {
        if shingle.len() != self.ngram_len {
            return Err(GranmoModelError::BbgVocabIndexOutOfRange);
        }
        let mut low = 0usize;
        let mut high = self.lookup_order.len();
        while low < high {
            let mid = low + (high - low) / 2;
            let rank = *self
                .lookup_order
                .get(mid)
                .ok_or(GranmoModelError::BbgVocabIndexOutOfRange)? as usize;
            let candidate = self.ngram_at_rank(rank)?;
            match candidate.cmp(shingle) {
                core::cmp::Ordering::Less => low = mid + 1,
                core::cmp::Ordering::Greater => high = mid,
                core::cmp::Ordering::Equal => return Ok(Some(rank)),
            }
        }
        Ok(None)
    }

    /// Extracts the presence bitset for one document: bit `r` set iff
    /// vocabulary shingle `r` occurs in the (scan-capped) document.
    /// Presence, not counts — matching the flat-BOW semantics of the
    /// Phase 1 control this baseline replicates at byte granularity.
    pub fn extract_presence_bits(
        &self,
        document: &[u8],
        max_scan_bytes: usize,
    ) -> Result<Vec<u64>, GranmoModelError> {
        let vocab_count = self.vocabulary_len();
        let word_count = vocab_count
            .checked_add(63)
            .ok_or(GranmoModelError::BbgArithmeticOverflow)?
            / 64;
        let mut bits = vec![0u64; word_count];

        let mut set_bit = |rank: usize| -> Result<(), GranmoModelError> {
            let word = bits
                .get_mut(rank >> 6)
                .ok_or(GranmoModelError::BbgVocabIndexOutOfRange)?;
            *word |= 1u64 << (rank & 63);
            Ok(())
        };

        let effective_len = document.len().min(max_scan_bytes);
        if effective_len < self.ngram_len {
            // Short document: one right-padded shingle (PAD rule §10.4).
            let mut padded = [PAD_BYTE; BAG_NGRAM_MAX_LEN];
            for (target, &source) in padded.iter_mut().zip(document.iter().take(effective_len)) {
                *target = source;
            }
            let shingle = padded
                .get(..self.ngram_len)
                .ok_or(GranmoModelError::BbgVocabIndexOutOfRange)?;
            if let Some(rank) = self.lookup(shingle)? {
                set_bit(rank)?;
            }
        } else {
            let last_start = effective_len - self.ngram_len; // no underflow: checked above
            for start in 0..=last_start {
                let end = start
                    .checked_add(self.ngram_len)
                    .ok_or(GranmoModelError::BbgArithmeticOverflow)?;
                let shingle = document
                    .get(start..end)
                    .ok_or(GranmoModelError::BbgVocabIndexOutOfRange)?;
                if let Some(rank) = self.lookup(shingle)? {
                    set_bit(rank)?;
                }
            }
        }
        Ok(bits)
    }

    /// Full structural revalidation (value-integrity rule): flat storage is
    /// a whole number of shingles; the lookup order is exactly one entry per
    /// rank and STRICTLY ascending by shingle bytes (strictness also proves
    /// no duplicate shingles — impossible from construction, so a duplicate
    /// means corruption). Call at construction, artifact load, or on demand.
    pub fn vocab_validity_recheck(&self) -> Result<(), GranmoModelError> {
        if self.ngram_len < usize::from(NgramLength::MIN)
            || self.ngram_len > usize::from(NgramLength::MAX)
        {
            return Err(GranmoModelError::BbgVocabRecheckCorrupt);
        }
        if self.ngram_flat_bytes.len() % self.ngram_len != 0 {
            return Err(GranmoModelError::BbgVocabRecheckCorrupt);
        }
        let count = self.vocabulary_len();
        if self.lookup_order.len() != count || count == 0 {
            return Err(GranmoModelError::BbgVocabRecheckCorrupt);
        }
        // Permutation check (each rank exactly once) + strict byte ordering.
        let mut seen = vec![false; count];
        let mut previous: Option<&[u8]> = None;
        for &rank_u32 in &self.lookup_order {
            let rank = rank_u32 as usize;
            match seen.get_mut(rank) {
                Some(flag) if !*flag => *flag = true,
                _ => return Err(GranmoModelError::BbgVocabRecheckCorrupt),
            }
            let bytes = self.ngram_at_rank(rank)?;
            if let Some(previous_bytes) = previous {
                if previous_bytes >= bytes {
                    return Err(GranmoModelError::BbgVocabRecheckCorrupt);
                }
            }
            previous = Some(bytes);
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Row audit: per-row consistency across recorded runs (research-harness tier)
// ---------------------------------------------------------------------------

/// Per-row aggregate over all prediction records for one line index.
#[derive(Debug, Clone, Copy)]
struct RowAuditAccumulator {
    label_is_positive: bool,
    tested_total: u32,
    tested_correct: u32,
    /// Sum of signed votes over test-side records (for the mean).
    tested_vote_total: i64,
    trained_total: u32,
    trained_correct: u32,
}

impl RowAuditAccumulator {
    /// Mean test-side vote in the WRONG direction for this row's label:
    /// positive = the model leans against the label. 0.0 when untested.
    fn wrongness(&self) -> f64 {
        if self.tested_total == 0 {
            return 0.0;
        }
        let mean_vote = self.tested_vote_total as f64 / f64::from(self.tested_total);
        if self.label_is_positive {
            -mean_vote
        } else {
            mean_vote
        }
    }
}

/// Parses one record line into `(line_index, side_is_train, label, vote)`.
/// Blank lines -> `Ok(None)`. Context keys are ignored; the four audit
/// keys are mandatory.
fn parse_row_prediction_record_line(
    line: &str,
) -> Result<Option<(usize, bool, bool, i32)>, GranmoModelError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let mut line_index: Option<usize> = None;
    let mut side_is_train: Option<bool> = None;
    let mut label_is_positive: Option<bool> = None;
    let mut vote_sum: Option<i32> = None;
    for field in trimmed.split('\t') {
        let (key, value) = match field.split_once('=') {
            Some(pair) => pair,
            None => return Err(GranmoModelError::CliPredictionRecordMalformed),
        };
        match key {
            "line" => {
                line_index = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| GranmoModelError::CliPredictionRecordMalformed)?,
                )
            }
            "side" => {
                side_is_train = Some(match value {
                    "train" => true,
                    "test" => false,
                    _ => return Err(GranmoModelError::CliPredictionRecordMalformed),
                })
            }
            "label" => {
                label_is_positive = Some(match value {
                    "1" => true,
                    "0" => false,
                    _ => return Err(GranmoModelError::CliPredictionRecordMalformed),
                })
            }
            "vote" => {
                vote_sum = Some(
                    value
                        .parse::<i32>()
                        .map_err(|_| GranmoModelError::CliPredictionRecordMalformed)?,
                )
            }
            _context_key => {} // split_seed, train_seed, fold, engine, run: not audited
        }
    }
    match (line_index, side_is_train, label_is_positive, vote_sum) {
        (Some(index), Some(side), Some(label), Some(vote)) => Ok(Some((index, side, label, vote))),
        _ => Err(GranmoModelError::CliPredictionRecordMalformed),
    }
}

/// `row-audit` mode: aggregates a prediction-record file by line index and
/// prints the review queue — rows never predicted correctly on any test
/// side first, then rows wrong even when on the training side, ordered by
/// how strongly the model leans against the label. Predictions use the
/// default decision V > 0.
///
/// Output is a review QUEUE, not a deletion list: "never correct" is the
/// union of mislabeled, genuinely ambiguous, and under-represented rows,
/// and no statistic here separates them. Quarantine, review, then measure.
fn handle_row_audit(args: &CliArgs) -> Result<(), GranmoModelError> {
    let records_path = args
        .records_in
        .as_ref()
        .ok_or(GranmoModelError::CliMissingRequiredFlag)?;
    if !records_path.is_absolute() {
        return Err(GranmoModelError::CliPredictionRecordPathNotAbsolute);
    }
    let raw = match std::fs::read(records_path) {
        Ok(bytes) => bytes,
        Err(_dropped_io_detail) => {
            #[cfg(debug_assertions)]
            eprintln!("CLI-813: records read failed: {}", _dropped_io_detail);
            return Err(GranmoModelError::CliPredictionRecordReadFailed);
        }
    };
    let text = String::from_utf8_lossy(&raw);

    // BTreeMap: deterministic iteration order for the tie-break by line.
    let mut rows: std::collections::BTreeMap<usize, RowAuditAccumulator> =
        std::collections::BTreeMap::new();
    let mut record_total: usize = 0;
    for line in text.lines() {
        let Some((line_index, side_is_train, label_is_positive, vote_sum)) =
            parse_row_prediction_record_line(line)?
        else {
            continue;
        };
        record_total = record_total.saturating_add(1);
        let predicted_positive = vote_sum > 0;
        let correct = predicted_positive == label_is_positive;
        let entry = rows.entry(line_index).or_insert(RowAuditAccumulator {
            label_is_positive,
            tested_total: 0,
            tested_correct: 0,
            tested_vote_total: 0,
            trained_total: 0,
            trained_correct: 0,
        });
        if entry.label_is_positive != label_is_positive {
            // Same line index, different labels: records from different
            // datasets were mixed. Every statistic would be wrong.
            return Err(GranmoModelError::CliPredictionRecordMalformed);
        }
        if side_is_train {
            entry.trained_total = entry.trained_total.saturating_add(1);
            if correct {
                entry.trained_correct = entry.trained_correct.saturating_add(1);
            }
        } else {
            entry.tested_total = entry.tested_total.saturating_add(1);
            entry.tested_vote_total = entry.tested_vote_total.saturating_add(i64::from(vote_sum));
            if correct {
                entry.tested_correct = entry.tested_correct.saturating_add(1);
            }
        }
    }

    // Only rows with at least one out-of-fold prediction are auditable.
    let mut ranked: Vec<(usize, RowAuditAccumulator)> = rows
        .into_iter()
        .filter(|(_, acc)| acc.tested_total > 0)
        .collect();
    let never_correct_total = ranked
        .iter()
        .filter(|(_, acc)| acc.tested_correct == 0)
        .count();
    let wrong_in_train_total = ranked
        .iter()
        .filter(|(_, acc)| acc.trained_total > 0 && acc.trained_correct < acc.trained_total)
        .count();

    // Sort key, descending priority: never correct; wrong even in training;
    // strength of the lean against the label. Line index breaks ties
    // (BTreeMap order is preserved by the stable sort).
    ranked.sort_by(|(_, a), (_, b)| {
        let a_never = a.tested_correct == 0;
        let b_never = b.tested_correct == 0;
        let a_wrong_train = a.trained_total > 0 && a.trained_correct < a.trained_total;
        let b_wrong_train = b.trained_total > 0 && b.trained_correct < b.trained_total;
        b_never
            .cmp(&a_never)
            .then(b_wrong_train.cmp(&a_wrong_train))
            .then(b.wrongness().total_cmp(&a.wrongness()))
    });

    println!("\n============================================================");
    println!("                      Row Audit Report                      ");
    println!("============================================================");
    println!(
        "records {}  rows tested {}  never-correct {}  wrong-even-in-train {}",
        record_total,
        ranked.len(),
        never_correct_total,
        wrong_in_train_total
    );
    println!("------------------------------------------------------------");
    println!(
        "{:>8} {:>5} {:>6} {:>7} {:>10} {:>7} {:>8}",
        "line", "label", "tested", "correct", "mean_vote", "trained", "tr_corr"
    );
    for (line_index, acc) in ranked.iter().take(args.audit_top) {
        let mean_vote = acc.tested_vote_total as f64 / f64::from(acc.tested_total);
        println!(
            "{:>8} {:>5} {:>6} {:>7} {:>+10.2} {:>7} {:>8}",
            line_index,
            if acc.label_is_positive { "1" } else { "0" },
            acc.tested_total,
            acc.tested_correct,
            mean_vote,
            acc.trained_total,
            acc.trained_correct
        );
    }
    println!("============================================================\n");
    Ok(())
}

// ===========================================================================
// SECTION 12B: ByteBagTM — Flat Bag-of-Byte-N-Grams Granmo Model (M-ByteBag)
// ===========================================================================
//
// Scientific role (§8): THE control engine. It shares the preprocessor,
// dataset loader, splitter, RNG, threshold sweep, and the shared
// `resolve_feedback_gates` helper with ByteConvTM, so the two engines
// differ in exactly one variable: positional windowing (the bag has none).
//
// NAMING CONVENTION (project rule: globally unique names): every method,
// helper, field, and associated function of this engine carries the `bag_`
// prefix (constructor: `new_with_vocabulary`). No name in this section
// exists anywhere else in the crate.
//
// Structure of record (Session 2 §3, locked decisions):
// - Single flat clause bank; even clause index = +1 polarity, odd = −1
//   (same polarity scheme as the conv engine).
// - Per clause, 2 × M automata over presence literals, where M is the
//   ACTUAL vocabulary size: local literal index r in [0, M) is the
//   positive literal "vocabulary shingle r IS present in the document";
//   local index M + r is its negated twin "shingle r is ABSENT".
//   States are i16 in [1, 2N]; a literal is included iff state > N; ALL
//   state changes route through `bag_increment_automaton` /
//   `bag_decrement_automaton` so the mask caches cannot drift (the same
//   cache-integrity pattern as the conv engine's transitions).
// - Derived evaluation caches (design commitment): per clause, two M-bit
//   include masks (positive and negated), maintained by exact bit set/clear
//   on include/exclude boundary crossings. A clause fires on a document iff
//     (positive_mask AND NOT presence_bits) == 0   [every required shingle
//                                                   is present]  AND
//     (negated_mask  AND     presence_bits) == 0   [no forbidden shingle
//                                                   is present]
//   evaluated word-parallel over u64 words — integer-only, allocation-free
//   given the document's presence bitset.
// - Feedback: Phase-1 FLAT semantics — no window sampling, no reservoir
//   (no positional structure exists). Type Ia: true literals reinforce at
//   the reinforce coin, false literals decay at the forget coin. Type Ib:
//   input-independent 1/s decay over all literals. Type II: increment
//   every FALSE literal (deterministic). The (T∓V)/2T vote gate is the
//   SHARED `resolve_feedback_gates` (Section 5C) — byte-identical training
//   mechanics to the conv engine, per the §8 control discipline.
// - GuardedInclude does NOT apply and is deliberately absent: a bag has no
//   per-slot categorical exclusivity, hence no structurally dead joint
//   states — that absence is itself the S2-7 three-way diagnostic control.
// - P4-tier properties (depth, specificity coins) are read through
//   per-clause accessors (`bag_depth_for_clause` etc.), scalar-backed
//   today — the same M-Hetero seam as the conv engine (S2-9 requirement).
//
// Tier notes: `bag_train_step` allocates one presence bitset and one
// fired-flag Vec per step; `bag_vote_sum` allocates one presence bitset
// per document (feature extraction). Documented harness-tier allocations —
// a production inference path may later accept a caller-provided reusable
// buffer (backlog; not needed for the §8 experiments).

/// The flat bag-of-byte-n-grams Granmo Model — the §8 scientific control.
/// See the section banner above for the full structure of record.
#[derive(Debug, Clone)]
pub struct ByteBagTM {
    /// The learned vocabulary (built from the preprocessed TRAINING split
    /// ONLY — the leakage guard is the harness's responsibility). Owned:
    /// the vocabulary is part of the model and travels with it.
    bag_vocabulary: ByteBagVocabulary,
    /// Total clauses; even = positive polarity, odd = negative.
    bag_clause_total: usize,
    /// T, as i32 for clamp/gate arithmetic (shared gate helper input).
    bag_vote_target: i32,
    /// N: automaton depth per action; states live in [1, 2N].
    bag_automaton_depth: i16,
    /// P(forget) coin threshold ≈ 65536 × 1/s.
    bag_forget_coin_threshold: u16,
    /// P(reinforce) coin threshold ≈ 65536 × (s−1)/s.
    bag_reinforce_coin_threshold: u16,
    /// Document scan cap in bytes (applied inside feature extraction).
    bag_scan_cap_bytes: usize,
    /// Raw automaton states: `clause * (2M) + local_literal`.
    bag_ta_states: Vec<i16>,
    /// Positive-include masks: `clause * words_per_clause + word`; bit r
    /// set iff positive literal r is included. Derived data — rebuilt or
    /// validated from raw states, never independently trusted.
    bag_positive_include_masks: Vec<u64>,
    /// Negated-include masks, same layout: bit r set iff negated literal
    /// (M + r) is included.
    bag_negated_include_masks: Vec<u64>,
    /// Training-time always-fire guard: consecutive-fire streak limit,
    /// 0 = disabled (the recorded-baseline default). EPHEMERAL training
    /// configuration — deliberately NOT persisted in artifact kind 3;
    /// the kind-3 reader constructs loaded engines with the guard
    /// disabled (recorded design decision, Drop 4.1).
    bag_fire_guard_limit: u32,
    /// Per-clause consecutive-fire streaks (guard state). Incremented
    /// only while a clause is both firing AND non-vacuous; reset to zero
    /// on any non-fire, on vacuity, and on a guard reset. EPHEMERAL —
    /// zeroed at construction and at artifact load, never persisted.
    bag_fire_streaks: Vec<u32>,
    /// Per-clause count of guard resets performed during this training
    /// session (run-report telemetry: the guard's ACTIVITY instrument,
    /// complementing the eval-time fire-rate report's OUTCOME
    /// instrument). EPHEMERAL — never persisted.
    bag_fire_guard_reset_counts: Vec<u32>,
}

/// Exclusive mutable access to ONE ByteBag clause's automaton states,
/// its two include-mask word ranges, and (Drop 4.1) its fire-guard
/// streak and reset-count slots. Same disjointness argument as
/// `ConvClauseWorkView`: clause c owns `bag_ta_states[c*2M..(c+1)*2M]`,
/// mask words `[c*W..(c+1)*W]` in both mask arrays, and exactly one slot
/// in each guard vector — and nothing else. The borrow checker proves
/// these ranges disjoint via the zipped-`chunks_mut`/`iter_mut`
/// construction, which is what permits handing contiguous view ranges to
/// different threads with no `unsafe`, no locks, and no atomics.
/// Indices are LOCAL to the clause (`0..2M`).
pub struct BagClauseWorkView<'engine> {
    clause_index: usize,
    feature_count: usize,
    depth_n: i16,
    forget_threshold: u16,
    reinforce_threshold: u16,
    /// Always-fire guard limit (0 = disabled). Engine-level scalar today;
    /// under a future TeamCompositionPalette this becomes a per-clause
    /// property through the same snapshot seam as depth and specificity.
    fire_guard_limit: u32,
    states: &'engine mut [i16],
    positive_mask_words: &'engine mut [u64],
    negated_mask_words: &'engine mut [u64],
    /// This clause's consecutive-fire streak (guard state; Drop 4.1).
    fire_streak: &'engine mut u32,
    /// This clause's guard-reset telemetry counter (Drop 4.1).
    fire_guard_reset_count: &'engine mut u32,
}

impl<'engine> BagClauseWorkView<'engine> {
    #[inline(always)]
    fn view_bag_clause_index(&self) -> usize {
        self.clause_index
    }

    /// Sets or clears one literal's include-mask bit. Called ONLY from the
    /// two transition methods on boundary crossings (and from the
    /// artifact-load rebuild) — a bag crossing maps to a single bit, so
    /// set/clear is exact.
    fn view_bag_set_include_bit(
        &mut self,
        local_literal: usize,
        included: bool,
    ) -> Result<(), GranmoModelError> {
        let (rank, is_positive_literal) = if local_literal < self.feature_count {
            (local_literal, true)
        } else {
            (local_literal - self.feature_count, false)
        };
        if rank >= self.feature_count {
            return Err(GranmoModelError::BbgEngineIndexOutOfRange);
        }
        let word_index = rank >> 6;
        let target_words = if is_positive_literal {
            &mut *self.positive_mask_words
        } else {
            &mut *self.negated_mask_words
        };
        let word = target_words
            .get_mut(word_index)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        if included {
            *word |= 1u64 << (rank & 63);
        } else {
            *word &= !(1u64 << (rank & 63));
        }
        Ok(())
    }

    /// Increments one automaton state (saturating at 2N), setting the
    /// include bit on an exclude→include crossing.
    fn view_bag_increment(&mut self, local_literal: usize) -> Result<(), GranmoModelError> {
        let saturation_ceiling = self
            .depth_n
            .checked_mul(2)
            .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
        let current_state = *self
            .states
            .get(local_literal)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        if current_state >= saturation_ceiling {
            return Ok(());
        }
        let updated_state = current_state
            .checked_add(1)
            .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
        *self
            .states
            .get_mut(local_literal)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)? = updated_state;
        if current_state == self.depth_n {
            self.view_bag_set_include_bit(local_literal, true)?;
        }
        Ok(())
    }

    /// Decrements one automaton state (floor at 1), clearing the include
    /// bit on an include→exclude crossing.
    fn view_bag_decrement(&mut self, local_literal: usize) -> Result<(), GranmoModelError> {
        let boundary_plus_one = self
            .depth_n
            .checked_add(1)
            .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
        let current_state = *self
            .states
            .get(local_literal)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        if current_state <= 1 {
            return Ok(());
        }
        let updated_state = current_state
            .checked_sub(1)
            .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
        *self
            .states
            .get_mut(local_literal)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)? = updated_state;
        if current_state == boundary_plus_one {
            self.view_bag_set_include_bit(local_literal, false)?;
        }
        Ok(())
    }

    /// Rebuilds this clause's include bits from its raw states
    /// (artifact-load path; the caller zeroes the mask arrays first).
    fn view_bag_rebuild_include_bits(&mut self) -> Result<(), GranmoModelError> {
        let literals_total = self.states.len();
        for local_literal in 0..literals_total {
            let state = *self
                .states
                .get(local_literal)
                .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
            if state > self.depth_n {
                self.view_bag_set_include_bit(local_literal, true)?;
            }
        }
        Ok(())
    }

    /// True iff this clause has at least one included literal in EITHER
    /// include mask — the non-vacuous test for the fire guard.
    ///
    /// A vacuous clause (zero includes) fires everywhere BY CONSTRUCTION:
    /// that is the bootstrap state every clause starts in and passes
    /// through while Type Ia feedback specializes it. Vacuous clauses are
    /// therefore EXEMPT from the guard — resetting one would be a no-op
    /// cycle, and counting its fires would recycle every fresh clause at
    /// exactly `limit` steps. Cost: an OR-scan over ~ceil(M/64) words with
    /// early exit (typically the first word is nonzero for any clause
    /// with includes) — negligible against the feedback pass's O(2M).
    fn view_bag_has_any_include(&self) -> bool {
        self.positive_mask_words.iter().any(|&word| word != 0)
            || self.negated_mask_words.iter().any(|&word| word != 0)
    }

    /// Resets this clause to FRESH state: every automaton at the exclude
    /// boundary (state == N), both include masks zeroed — byte-identical
    /// to the constructor's fresh invariant, so
    /// `bag_validate_internal_consistency` passes by construction after a
    /// reset. Consumes ZERO RNG (part of the determinism contract: guard
    /// activity never shifts any stochastic stream).
    ///
    /// This is the "prune-and-respawn" action of record (Drop 4.1):
    /// the clause slot is reclaimed for re-specialization rather than
    /// removed (removal would change bank geometry, polarity balance,
    /// LUT sizing, and the artifact format) or frozen (which would waste
    /// the slot permanently and complicate voting).
    fn view_bag_reset_clause_to_fresh(&mut self) -> Result<(), GranmoModelError> {
        for state_slot in self.states.iter_mut() {
            *state_slot = self.depth_n;
        }
        for word in self.positive_mask_words.iter_mut() {
            *word = 0;
        }
        for word in self.negated_mask_words.iter_mut() {
            *word = 0;
        }
        Ok(())
    }

    /// The always-fire guard (Drop 4.1): updates this clause's
    /// consecutive-fire streak from this step's pass-1 fired flag and, if
    /// the active limit is reached, resets the clause to fresh state.
    /// Returns `Ok(true)` iff a reset occurred — the caller skips
    /// feedback for this clause this step (the recycled clause starts its
    /// next step clean, receiving normal Type Ia bootstrap from then on).
    ///
    /// ## Streak rule (specification of record)
    /// - increments ONLY when `fired == true` AND the clause is
    ///   non-vacuous (see `view_bag_has_any_include`);
    /// - resets to 0 on any non-fire OR on vacuity.
    /// Because the harness re-shuffles training order every epoch,
    /// `limit` consecutive fires is a representative corpus sample: the
    /// guard condemns exactly the pathological case — a clause whose
    /// LEARNED pattern is ubiquitous (vote offset / dead capacity, per
    /// the recorded batch findings of 23–48 always-firing clauses per 600
    /// persisting through 12 epochs).
    ///
    /// ## Contract notes
    /// - Consumes zero RNG; with `fire_guard_limit == DISABLED` this
    ///   method short-circuits before touching ANY state, so the
    ///   guard-disabled build is byte-identical to the pre-guard build.
    /// - Runs exactly once per clause per training step inside the
    ///   disjoint clause views, so worker-count invariance holds by the
    ///   same argument as all other clause state.
    fn view_bag_apply_fire_guard(&mut self, fired: bool) -> Result<bool, GranmoModelError> {
        if self.fire_guard_limit == FireGuardStreakLimit::DISABLED {
            return Ok(false); // guard off: structurally zero behavioral delta
        }
        if fired && self.view_bag_has_any_include() {
            *self.fire_streak = self
                .fire_streak
                .checked_add(1)
                .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
        } else {
            *self.fire_streak = 0;
        }
        if *self.fire_streak >= self.fire_guard_limit {
            self.view_bag_reset_clause_to_fresh()?;
            *self.fire_streak = 0;
            // Telemetry: saturating by design — a counter that has hit
            // u32::MAX has long since made its diagnostic point, and
            // saturation cannot panic or corrupt (documented posture).
            *self.fire_guard_reset_count = self.fire_guard_reset_count.saturating_add(1);
            return Ok(true);
        }
        Ok(false)
    }

    /// Type Ia: flat semantics — true literals reinforce, false literals
    /// decay. Draw ORDER unchanged from the pre-parallel implementation.
    fn view_bag_apply_type_ia(
        &mut self,
        presence_bits: &[u64],
        rng: &mut FastRng,
    ) -> Result<(), GranmoModelError> {
        let reinforce_coin = self.reinforce_threshold;
        let forget_coin = self.forget_threshold;
        let feature_count = self.feature_count;
        for rank in 0..feature_count {
            let shingle_present = ByteBagTM::bag_presence_bit(presence_bits, rank)?;
            let positive_local = rank;
            let negated_local = feature_count + rank;
            if shingle_present {
                if rng.coin(reinforce_coin) {
                    self.view_bag_increment(positive_local)?;
                }
                if rng.coin(forget_coin) {
                    self.view_bag_decrement(negated_local)?;
                }
            } else {
                if rng.coin(forget_coin) {
                    self.view_bag_decrement(positive_local)?;
                }
                if rng.coin(reinforce_coin) {
                    self.view_bag_increment(negated_local)?;
                }
            }
        }
        Ok(())
    }

    /// Type Ib: input-independent uniform decay over all 2M literals.
    fn view_bag_apply_type_ib(&mut self, rng: &mut FastRng) -> Result<(), GranmoModelError> {
        let forget_coin = self.forget_threshold;
        let literals_total = self.states.len();
        for local_literal in 0..literals_total {
            if rng.coin(forget_coin) {
                self.view_bag_decrement(local_literal)?;
            }
        }
        Ok(())
    }

    /// Type II: deterministically increment every FALSE literal.
    fn view_bag_apply_type_ii(&mut self, presence_bits: &[u64]) -> Result<(), GranmoModelError> {
        let feature_count = self.feature_count;
        for rank in 0..feature_count {
            let shingle_present = ByteBagTM::bag_presence_bit(presence_bits, rank)?;
            if shingle_present {
                // Negated literal (M + rank) is false here.
                self.view_bag_increment(feature_count + rank)?;
            } else {
                // Positive literal (rank) is false here.
                self.view_bag_increment(rank)?;
            }
        }
        Ok(())
    }
}

/// One ByteBag clause's P4-tier properties, snapshotted through the
/// per-clause accessors BEFORE mutable borrows are taken. Same M-Hetero
/// seam as `ConvClauseProperties`; no guard field, because GuardedInclude
/// does not apply to a bag (recorded S2-7 control).
#[derive(Debug, Clone, Copy)]
struct BagClauseProperties {
    depth_n: i16,
    forget_threshold: u16,
    reinforce_threshold: u16,
}

/// Pass 2 body for a contiguous range of ByteBag clause views. Used by
/// BOTH the single-threaded path and each worker thread, so the two
/// cannot diverge.
fn bag_apply_feedback_to_view_range(
    views: &mut [BagClauseWorkView<'_>],
    fired_flags: &[bool],
    presence_bits: &[u64],
    step_seed: u64,
    gates: FeedbackGates,
    label_is_positive: bool,
) -> Result<(), GranmoModelError> {
    for view in views.iter_mut() {
        let clause = view.view_bag_clause_index();
        let fired = *fired_flags
            .get(clause)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;

        /*
        Placement rationale (recorded): the fired flag was computed
        in pass 1 against the pre-reset masks — that observation
        is precisely what condemns the clause. Skipping the gate
        draw for a reset clause is deterministic (the per-clause
        stream is re-derived from (step, clause, purpose) next step,
        so there is no cross-step stream drift).
        */
        // Always-fire guard: checked BEFORE the gate draw. A
        // reset consumes no RNG and skips feedback this step, so with the
        // guard disabled this call is a structural no-op and the build is
        // byte-identical to the pre-guard build. Runs exactly once per
        // clause per step regardless of worker count (disjoint views).
        if view.view_bag_apply_fire_guard(fired)? {
            continue;
        }

        let positive_polarity = clause % 2 == 0;

        let (gate, receives_type_i) = if label_is_positive {
            (gates.gate_when_target, positive_polarity)
        } else {
            (gates.gate_when_other, !positive_polarity)
        };

        let mut clause_rng = FastRng::seed(derive_clause_stream_seed(
            step_seed,
            clause as u64,
            RNG_PURPOSE_CLAUSE_FEEDBACK,
        ));
        let draw = clause_rng.gen_index(gates.two_t)? as i32;
        if draw >= gate {
            continue; // gated out this step
        }

        if receives_type_i {
            if fired {
                view.view_bag_apply_type_ia(presence_bits, &mut clause_rng)?;
            } else {
                view.view_bag_apply_type_ib(&mut clause_rng)?;
            }
        } else if fired {
            view.view_bag_apply_type_ii(presence_bits)?;
        }
    }
    Ok(())
}

impl ByteBagTM {
    /// Constructs a fresh engine around an already-built vocabulary.
    ///
    /// The constructor TAKES the vocabulary (rather than building it) to
    /// make the train-split-only rule structural: the harness builds the
    /// vocabulary from preprocessed TRAINING documents and hands it over;
    /// this engine never sees raw corpora. The vocabulary is structurally
    /// revalidated here (value-integrity rule) so a corrupt vocabulary
    /// cannot silently configure a mis-sized engine.
    ///
    /// `fire_guard_limit` (Drop 4.1) configures the training-time
    /// always-fire guard; `FireGuardStreakLimit::DISABLED` preserves the
    /// pre-guard behavior byte-for-byte (the guard consumes no RNG and
    /// short-circuits before touching any state). Guard state is
    /// ephemeral and never persisted; the artifact reader passes DISABLED.
    ///
    /// Fresh state: every automaton at the exclude boundary (state == N),
    /// hence zero includes, all-zero masks, and every clause fires on every
    /// document (vacuous conjunction) — with balanced polarities the fresh
    /// vote sum is exactly 0, mirroring the conv engine's fresh invariant.
    pub fn new_with_vocabulary(
        vocabulary_for_engine: ByteBagVocabulary,
        clauses: ClauseCount,
        vote_target: VoteThreshold,
        automaton_depth: StatesPerAction,
        specificity: SpecificityThresholds,
        scan_cap: MaxScanBytes,
        fire_guard_limit: FireGuardStreakLimit,
    ) -> Result<Self, GranmoModelError> {
        vocabulary_for_engine.vocab_validity_recheck()?;

        let resolved_clause_count = usize::from(clauses.get()?);
        let resolved_vote_target = i32::from(vote_target.get()?);
        let resolved_depth = automaton_depth.get()?;
        let (resolved_forget, resolved_reinforce) = specificity.get()?;
        let resolved_scan_cap = scan_cap.get()? as usize;
        let resolved_fire_guard_limit = fire_guard_limit.get()?;

        // Sizing. validity_recheck guarantees vocabulary_len() >= 1.
        let feature_count = vocabulary_for_engine.vocabulary_len();
        let literal_count_per_clause = feature_count
            .checked_mul(2)
            .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
        let total_state_count = resolved_clause_count
            .checked_mul(literal_count_per_clause)
            .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
        let mask_words_for_features = feature_count
            .checked_add(63)
            .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?
            / 64;
        let total_mask_words = resolved_clause_count
            .checked_mul(mask_words_for_features)
            .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;

        Ok(Self {
            bag_vocabulary: vocabulary_for_engine,
            bag_clause_total: resolved_clause_count,
            bag_vote_target: resolved_vote_target,
            bag_automaton_depth: resolved_depth,
            bag_forget_coin_threshold: resolved_forget,
            bag_reinforce_coin_threshold: resolved_reinforce,
            bag_scan_cap_bytes: resolved_scan_cap,
            bag_ta_states: vec![resolved_depth; total_state_count],
            bag_positive_include_masks: vec![0u64; total_mask_words],
            bag_negated_include_masks: vec![0u64; total_mask_words],
            bag_fire_guard_limit: resolved_fire_guard_limit,
            bag_fire_streaks: vec![0u32; resolved_clause_count],
            bag_fire_guard_reset_counts: vec![0u32; resolved_clause_count],
        })
    }
    // --- Layout helpers -----------------------------------------------------

    /// M: the actual vocabulary size. Derived from the vocabulary on every
    /// call (one integer division) rather than cached — no second source of
    /// truth exists, so no cache-consistency check is needed for it.
    #[inline(always)]
    fn bag_feature_count(&self) -> usize {
        self.bag_vocabulary.vocabulary_len()
    }

    /// 2M literals per clause. Plain arithmetic is safe: M <= 65000
    /// (`VocabSize::MAX`), so 2M <= 130000, far inside usize.
    #[inline(always)]
    fn bag_literals_per_clause(&self) -> usize {
        2 * self.bag_feature_count()
    }

    /// Mask words per clause = ceil(M / 64). Plain arithmetic is safe:
    /// M + 63 <= 65063, far inside usize.
    #[inline(always)]
    fn bag_mask_words_per_clause(&self) -> usize {
        (self.bag_feature_count() + 63) / 64
    }

    /// Global state index for (clause, local literal), fully bounds-checked
    /// including the local-literal range (prevents cross-clause bleed).
    fn bag_state_index(
        &self,
        clause: usize,
        local_literal: usize,
    ) -> Result<usize, GranmoModelError> {
        if clause >= self.bag_clause_total || local_literal >= self.bag_literals_per_clause() {
            return Err(GranmoModelError::BbgEngineIndexOutOfRange);
        }
        let clause_base = clause
            .checked_mul(self.bag_literals_per_clause())
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        let state_index = clause_base
            .checked_add(local_literal)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        if state_index >= self.bag_ta_states.len() {
            return Err(GranmoModelError::BbgEngineIndexOutOfRange);
        }
        Ok(state_index)
    }

    /// The [start, end) word range of one clause's masks (both mask arrays
    /// share this layout), bounds-checked against both arrays.
    fn bag_mask_word_range(&self, clause: usize) -> Result<(usize, usize), GranmoModelError> {
        if clause >= self.bag_clause_total {
            return Err(GranmoModelError::BbgEngineIndexOutOfRange);
        }
        let words = self.bag_mask_words_per_clause();
        let range_start = clause
            .checked_mul(words)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        let range_end = range_start
            .checked_add(words)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        if range_end > self.bag_positive_include_masks.len()
            || range_end > self.bag_negated_include_masks.len()
        {
            return Err(GranmoModelError::BbgEngineIndexOutOfRange);
        }
        Ok((range_start, range_end))
    }

    /// Reads one presence bit from a document's presence bitset.
    #[inline(always)]
    fn bag_presence_bit(presence_bits: &[u64], rank: usize) -> Result<bool, GranmoModelError> {
        let word = presence_bits
            .get(rank >> 6)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        Ok(word & (1u64 << (rank & 63)) != 0)
    }

    // --- Per-clause P4-property accessors (M-Hetero accommodation) ---------
    //
    // Same seam as the conv engine's accessors (S2-9 requirement): today
    // scalar-backed (homogeneous team); under a future TeamCompositionPalette
    // only these bodies, the constructor, and the artifact format grow.
    // No guard accessor exists: GuardedInclude does not apply to a bag.

    /// Automaton depth N for this clause's team (states live in [1, 2N]).
    #[inline(always)]
    fn bag_depth_for_clause(&self, _clause: usize) -> i16 {
        self.bag_automaton_depth
    }

    /// P(forget) coin threshold for this clause.
    #[inline(always)]
    fn bag_forget_threshold_for_clause(&self, _clause: usize) -> u16 {
        self.bag_forget_coin_threshold
    }

    /// P(reinforce) coin threshold for this clause.
    #[inline(always)]
    fn bag_reinforce_threshold_for_clause(&self, _clause: usize) -> u16 {
        self.bag_reinforce_coin_threshold
    }

    // --- Feature extraction ---------------------------------------------------

    /// Extracts the document's presence bitset through the engine's own
    /// vocabulary and scan cap. Called ONCE per document per training step
    /// and once per inference (locked decision, Session 2 §3).
    fn bag_document_presence_bits(&self, document: &[u8]) -> Result<Vec<u64>, GranmoModelError> {
        self.bag_vocabulary
            .extract_presence_bits(document, self.bag_scan_cap_bytes)
    }

    // --- Clause evaluation (integer-only, word-parallel) ----------------------

    /// Does `clause` fire on this presence bitset? Word-parallel test of
    /// both include masks; see the section banner for the firing condition.
    /// A presence bitset of the wrong word count is a wiring error (bits
    /// produced by a different vocabulary), reported not tolerated.
    fn bag_clause_fires(
        &self,
        clause: usize,
        presence_bits: &[u64],
    ) -> Result<bool, GranmoModelError> {
        let (range_start, range_end) = self.bag_mask_word_range(clause)?;
        let positive_words = self
            .bag_positive_include_masks
            .get(range_start..range_end)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        let negated_words = self
            .bag_negated_include_masks
            .get(range_start..range_end)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        if presence_bits.len() != positive_words.len() {
            return Err(GranmoModelError::BbgEngineIndexOutOfRange);
        }
        for ((&positive_word, &negated_word), &document_word) in positive_words
            .iter()
            .zip(negated_words.iter())
            .zip(presence_bits.iter())
        {
            // A required shingle is absent, or a forbidden shingle present.
            if positive_word & !document_word != 0 {
                return Ok(false);
            }
            if negated_word & document_word != 0 {
                return Ok(false);
            }
        }
        Ok(true)
    }

    // --- Public inference surface ----------------------------------------------

    /// Signed vote sum V = Σ fired(+) − Σ fired(−) over the flat bank.
    pub fn bag_vote_sum(&self, document: &[u8]) -> Result<i32, GranmoModelError> {
        let presence_bits = self.bag_document_presence_bits(document)?;
        let mut vote: i32 = 0;
        for clause in 0..self.bag_clause_total {
            if self.bag_clause_fires(clause, &presence_bits)? {
                vote = if clause % 2 == 0 {
                    vote.checked_add(1)
                } else {
                    vote.checked_sub(1)
                }
                .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
            }
        }
        Ok(vote)
    }

    /// Binary prediction: label 1 iff `V > decision_threshold` (same free
    /// sweep axis as the conv engine, §8).
    pub fn bag_predict(
        &self,
        document: &[u8],
        decision_threshold: i32,
    ) -> Result<bool, GranmoModelError> {
        Ok(self.bag_vote_sum(document)? > decision_threshold)
    }

    /// The fired-clause bitset — the same embedding deliverable (§7.2) and
    /// fire-rate-report input (S2-8) as the conv engine's version.
    pub fn bag_fired_clause_bits(&self, document: &[u8]) -> Result<Vec<u64>, GranmoModelError> {
        let presence_bits = self.bag_document_presence_bits(document)?;
        let fired_word_count = self
            .bag_clause_total
            .checked_add(63)
            .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?
            / 64;
        let mut fired_words = vec![0u64; fired_word_count];
        for clause in 0..self.bag_clause_total {
            if self.bag_clause_fires(clause, &presence_bits)? {
                let word = fired_words
                    .get_mut(clause >> 6)
                    .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
                *word |= 1u64 << (clause & 63);
            }
        }
        Ok(fired_words)
    }

    /// Decodes one clause into a human-readable shingle pattern, e.g.
    /// `has "ab" ∧ lacks "ba"`. Printable bytes render as chars, others as
    /// `\xHH` — the same convention as the conv engine's decoder, decoded
    /// through `ngram_at_rank` (the explainability primitive). Rendering is
    /// capped; the overflow count is reported.
    pub fn bag_describe_clause(
        &self,
        clause: usize,
        max_rendered_literals: usize,
    ) -> Result<String, GranmoModelError> {
        if clause >= self.bag_clause_total {
            return Err(GranmoModelError::BbgEngineIndexOutOfRange);
        }
        let clause_depth = self.bag_depth_for_clause(clause);
        let feature_count = self.bag_feature_count();

        /// Renders shingle bytes: printable ASCII as chars, others as \xHH.
        fn render_shingle_bytes(shingle: &[u8]) -> String {
            let mut rendered = String::with_capacity(shingle.len() * 4);
            for &byte_value in shingle {
                if (0x20..=0x7E).contains(&byte_value) {
                    rendered.push(byte_value as char);
                } else {
                    rendered.push_str(&format!("\\x{:02X}", byte_value));
                }
            }
            rendered
        }

        let mut parts: Vec<String> = Vec::new();
        let mut omitted: usize = 0;
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

            if positive_state > clause_depth {
                if parts.len() < max_rendered_literals {
                    let shingle = self.bag_vocabulary.ngram_at_rank(rank)?;
                    parts.push(format!("has \"{}\"", render_shingle_bytes(shingle)));
                } else {
                    omitted = omitted.saturating_add(1);
                }
            }
            if negated_state > clause_depth {
                if parts.len() < max_rendered_literals {
                    let shingle = self.bag_vocabulary.ngram_at_rank(rank)?;
                    parts.push(format!("lacks \"{}\"", render_shingle_bytes(shingle)));
                } else {
                    omitted = omitted.saturating_add(1);
                }
            }
        }

        let mut rendered = parts.join(" ∧ ");
        if omitted > 0 {
            rendered.push_str(&format!(" … (+{} more literals)", omitted));
        }
        Ok(rendered)
    }

    /// Clause count (reporting; LUT sizing).
    pub fn bag_clause_count(&self) -> usize {
        self.bag_clause_total
    }

    /// Total fire-guard resets performed during this training session,
    /// summed over all clauses (run-report telemetry). Saturating fold:
    /// telemetry must never be a panic path.
    pub fn bag_fire_guard_reset_total(&self) -> u64 {
        self.bag_fire_guard_reset_counts
            .iter()
            .fold(0u64, |accumulated, &clause_resets| {
                accumulated.saturating_add(u64::from(clause_resets))
            })
    }

    /// Per-clause total of INCLUDED literals (positive + negated),
    /// popcounted from the two include masks — the specialization
    /// instrument. A total of 0 means the clause is VACUOUS: it fires
    /// everywhere by construction (bootstrap state), is deliberately
    /// exempt from the fire guard, and is a depth/epoch-budget symptom,
    /// not an always-fire pathology. Reporting tier: read-only,
    /// O(clauses × mask words).
    pub fn bag_clause_include_totals(&self) -> Result<Vec<u32>, GranmoModelError> {
        let mut totals: Vec<u32> = Vec::with_capacity(self.bag_clause_total);
        for clause in 0..self.bag_clause_total {
            let (range_start, range_end) = self.bag_mask_word_range(clause)?;
            let positive_words = self
                .bag_positive_include_masks
                .get(range_start..range_end)
                .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
            let negated_words = self
                .bag_negated_include_masks
                .get(range_start..range_end)
                .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
            // Checked fold per the arithmetic rules (mathematically the
            // sum is bounded by 2M <= 130000, far inside u32).
            let mut include_total: u32 = 0;
            for word in positive_words.iter().chain(negated_words.iter()) {
                include_total = include_total
                    .checked_add(word.count_ones())
                    .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
            }
            totals.push(include_total);
        }
        Ok(totals)
    }

    /// Read access to the vocabulary (harness explainability / reporting).
    pub fn bag_vocabulary_ref(&self) -> &ByteBagVocabulary {
        &self.bag_vocabulary
    }

    /// Probability LUT matched to THIS engine's clause count and vote
    /// target, routed through the enforced newtypes exactly as the conv
    /// engine's LUT builder is (value-integrity rule).
    pub fn bag_build_probability_lut(&self) -> Result<ProbabilityLut, GranmoModelError> {
        let clause_count_u16 = u16::try_from(self.bag_clause_total)
            .map_err(|_| GranmoModelError::BbgEngineIndexOutOfRange)?;
        let vote_target_i16 = i16::try_from(self.bag_vote_target)
            .map_err(|_| GranmoModelError::BbgEngineIndexOutOfRange)?;
        ProbabilityLut::build(
            ClauseCount::new(clause_count_u16)?,
            VoteThreshold::new(vote_target_i16)?,
        )
    }

    // --- Mask maintenance -------------------------------------------------------
    /// Builds an exclusive mutable view of ONE clause (sequential paths:
    /// artifact-load mask rebuild via the all-views builder is preferred;
    /// this single-clause form serves test helpers and unit-level guard
    /// tests).
    fn _test_bag_build_clause_view(
        &mut self,
        clause: usize,
    ) -> Result<BagClauseWorkView<'_>, GranmoModelError> {
        if clause >= self.bag_clause_total {
            return Err(GranmoModelError::BbgEngineIndexOutOfRange);
        }
        let properties = BagClauseProperties {
            depth_n: self.bag_depth_for_clause(clause),
            forget_threshold: self.bag_forget_threshold_for_clause(clause),
            reinforce_threshold: self.bag_reinforce_threshold_for_clause(clause),
        };
        let feature_count = self.bag_feature_count();
        let literals_per_clause = self.bag_literals_per_clause();
        let mask_words_per_clause = self.bag_mask_words_per_clause();
        let fire_guard_limit = self.bag_fire_guard_limit;

        let state_start = clause
            .checked_mul(literals_per_clause)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        let state_end = state_start
            .checked_add(literals_per_clause)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        let mask_start = clause
            .checked_mul(mask_words_per_clause)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
        let mask_end = mask_start
            .checked_add(mask_words_per_clause)
            .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;

        Ok(BagClauseWorkView {
            clause_index: clause,
            feature_count,
            depth_n: properties.depth_n,
            forget_threshold: properties.forget_threshold,
            reinforce_threshold: properties.reinforce_threshold,
            fire_guard_limit,
            states: self
                .bag_ta_states
                .get_mut(state_start..state_end)
                .ok_or(GranmoModelError::ParClauseViewGeometryFault)?,
            positive_mask_words: self
                .bag_positive_include_masks
                .get_mut(mask_start..mask_end)
                .ok_or(GranmoModelError::ParClauseViewGeometryFault)?,
            negated_mask_words: self
                .bag_negated_include_masks
                .get_mut(mask_start..mask_end)
                .ok_or(GranmoModelError::ParClauseViewGeometryFault)?,
            fire_streak: self
                .bag_fire_streaks
                .get_mut(clause)
                .ok_or(GranmoModelError::BbgFireGuardIndexOutOfRange)?,
            fire_guard_reset_count: self
                .bag_fire_guard_reset_counts
                .get_mut(clause)
                .ok_or(GranmoModelError::BbgFireGuardIndexOutOfRange)?,
        })
    }

    /// Builds exclusive mutable views of EVERY clause at once, by zipping
    /// `chunks_mut` over the three storage vectors and `iter_mut` over the
    /// two per-clause guard vectors — the same disjointness proof as the
    /// conv engine's builder, which is what permits handing contiguous
    /// view ranges to different threads without `unsafe`.
    ///
    /// Geometry note: if either guard vector were shorter than the clause
    /// count (corruption), the zip would truncate and the final
    /// `views.len() != clause_total` check reports it as a geometry fault
    /// — no silent partial coverage is possible.
    fn bag_build_all_clause_views(
        &mut self,
    ) -> Result<Vec<BagClauseWorkView<'_>>, GranmoModelError> {
        let clause_total = self.bag_clause_total;
        let feature_count = self.bag_feature_count();
        let literals_per_clause = self.bag_literals_per_clause();
        let mask_words_per_clause = self.bag_mask_words_per_clause();
        let fire_guard_limit = self.bag_fire_guard_limit;

        // Snapshot P4 properties through the accessors BEFORE the mutable
        // borrows below (the accessors need `&self`).
        let mut clause_properties: Vec<BagClauseProperties> = Vec::with_capacity(clause_total);
        for clause in 0..clause_total {
            clause_properties.push(BagClauseProperties {
                depth_n: self.bag_depth_for_clause(clause),
                forget_threshold: self.bag_forget_threshold_for_clause(clause),
                reinforce_threshold: self.bag_reinforce_threshold_for_clause(clause),
            });
        }

        let mut views: Vec<BagClauseWorkView<'_>> = Vec::with_capacity(clause_total);
        let state_chunks = self.bag_ta_states.chunks_mut(literals_per_clause);
        let positive_chunks = self
            .bag_positive_include_masks
            .chunks_mut(mask_words_per_clause);
        let negated_chunks = self
            .bag_negated_include_masks
            .chunks_mut(mask_words_per_clause);
        let streak_slots = self.bag_fire_streaks.iter_mut();
        let reset_count_slots = self.bag_fire_guard_reset_counts.iter_mut();

        for (
            clause_index,
            ((((state_chunk, positive_chunk), negated_chunk), streak_slot), reset_count_slot),
        ) in state_chunks
            .zip(positive_chunks)
            .zip(negated_chunks)
            .zip(streak_slots)
            .zip(reset_count_slots)
            .enumerate()
        {
            let properties = *clause_properties
                .get(clause_index)
                .ok_or(GranmoModelError::ParClauseViewGeometryFault)?;
            if state_chunk.len() != literals_per_clause
                || positive_chunk.len() != mask_words_per_clause
                || negated_chunk.len() != mask_words_per_clause
            {
                return Err(GranmoModelError::ParClauseViewGeometryFault);
            }
            views.push(BagClauseWorkView {
                clause_index,
                feature_count,
                depth_n: properties.depth_n,
                forget_threshold: properties.forget_threshold,
                reinforce_threshold: properties.reinforce_threshold,
                fire_guard_limit,
                states: state_chunk,
                positive_mask_words: positive_chunk,
                negated_mask_words: negated_chunk,
                fire_streak: streak_slot,
                fire_guard_reset_count: reset_count_slot,
            });
        }
        if views.len() != clause_total {
            return Err(GranmoModelError::ParClauseViewGeometryFault);
        }
        Ok(views)
    }

    // --- Training -------------------------------------------------------------------
    /// One stochastic training update for one document, single-threaded.
    /// Byte-identical to `bag_train_step_with_workers(..., WorkerCount 1)`
    /// and, by the per-clause RNG contract, to ANY worker count.
    pub fn bag_train_step(
        &mut self,
        document: &[u8],
        label_is_positive: bool,
        rng: &mut FastRng,
    ) -> Result<(), GranmoModelError> {
        self.bag_train_step_with_workers(document, label_is_positive, rng, WorkerCount::new(1)?)
    }

    /// Pass 1 of one training step (sequential, immutable — see the
    /// asymmetry note on `bag_train_step_with_workers`): fired flag per
    /// clause plus the exact integer vote sum. Extracted so `HybridTM` can
    /// insert the combined vote between the passes.
    fn bag_scan_pass(&self, presence_bits: &[u64]) -> Result<(Vec<bool>, i32), GranmoModelError> {
        let clause_total = self.bag_clause_total;
        let mut fired_flags: Vec<bool> = Vec::with_capacity(clause_total);
        let mut vote: i32 = 0;
        for clause in 0..clause_total {
            let fired = self.bag_clause_fires(clause, presence_bits)?;
            if fired {
                vote = if clause % 2 == 0 {
                    vote.checked_add(1)
                } else {
                    vote.checked_sub(1)
                }
                .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
            }
            fired_flags.push(fired);
        }
        Ok((fired_flags, vote))
    }

    /// Pass 2 of one training step: feedback under the given gates through
    /// disjoint clause views (fork-join). Gate provenance (own vote or
    /// hybrid combined vote) is the caller's concern.
    fn bag_feedback_pass_with_workers(
        &mut self,
        fired_flags: &[bool],
        presence_bits: &[u64],
        step_seed: u64,
        gates: FeedbackGates,
        label_is_positive: bool,
        worker_count: WorkerCount,
    ) -> Result<(), GranmoModelError> {
        if fired_flags.len() != self.bag_clause_total {
            return Err(GranmoModelError::ParClauseViewGeometryFault);
        }
        let clause_total = self.bag_clause_total;
        let worker_total = resolve_effective_worker_count(worker_count, clause_total)?;
        let chunk_size = resolve_work_chunk_size(clause_total, worker_total)?;

        let mut clause_views = self.bag_build_all_clause_views()?;

        if worker_total == 1 {
            bag_apply_feedback_to_view_range(
                &mut clause_views,
                fired_flags,
                presence_bits,
                step_seed,
                gates,
                label_is_positive,
            )?;
        } else {
            let feedback_results: Vec<Result<(), GranmoModelError>> =
                std::thread::scope(|feedback_scope| {
                    let mut worker_handles = Vec::with_capacity(worker_total);
                    for view_chunk in clause_views.chunks_mut(chunk_size) {
                        worker_handles.push(feedback_scope.spawn(move || {
                            bag_apply_feedback_to_view_range(
                                view_chunk,
                                fired_flags,
                                presence_bits,
                                step_seed,
                                gates,
                                label_is_positive,
                            )
                        }));
                    }
                    worker_handles
                        .into_iter()
                        .map(|handle| match handle.join() {
                            Ok(worker_result) => worker_result,
                            // join() errs only on a worker panic. Production
                            // code never panics: report, never re-raise.
                            Err(_panic_payload) => Err(GranmoModelError::ParWorkerJoinFailed),
                        })
                        .collect()
                });
            for worker_result in feedback_results {
                worker_result?;
            }
        }
        Ok(())
    }

    /// One stochastic training update for one document, fork-join over the
    /// clause bank — flat semantics, no window sampling.
    ///
    /// ## Asymmetry with the conv engine (deliberate, recorded)
    /// Pass 1 is left SEQUENTIAL here. Bag clause evaluation is a
    /// word-parallel mask test costing ceil(M/64) word operations per
    /// clause (~63 at M=4000) against pass 2's O(2M) = 8000 literal visits
    /// per gated clause — two orders of magnitude apart. Spawning threads
    /// for pass 1 would cost more than it saves. This is a scheduling
    /// choice only: it has no effect on results, and the §8 timing
    /// comparison remains honest because each engine is parallelized where
    /// its own cost actually is.
    pub fn bag_train_step_with_workers(
        &mut self,
        document: &[u8],
        label_is_positive: bool,
        rng: &mut FastRng,
        worker_count: WorkerCount,
    ) -> Result<(), GranmoModelError> {
        let presence_bits = self.bag_document_presence_bits(document)?;
        // EXACTLY one master draw per step, at every worker count.
        let step_seed = rng.next_u64();
        let (fired_flags, vote) = self.bag_scan_pass(&presence_bits)?;
        let gates = resolve_feedback_gates(vote, self.bag_vote_target)?;
        self.bag_feedback_pass_with_workers(
            &fired_flags,
            &presence_bits,
            step_seed,
            gates,
            label_is_positive,
            worker_count,
        )
    }
    /*
    Ensemble note:
    Both refactors preserve draw order exactly
    (presence extraction consumes no RNG; the master draw
    is still the single next_u64() per step),
    so existing worker-invariance tests remain the oracle.
    */

    // --- Invariant validation ----------------------------------------------------

    /// Re-derives BOTH include masks for every clause from raw automaton
    /// states and compares against the stored masks; checks every state
    /// lies in its clause's legal band [1, 2N] (depth read through
    /// `bag_depth_for_clause` — the M-Hetero seam, matching the conv
    /// engine's per-clause validation structure); checks storage geometry;
    /// and structurally revalidates the vocabulary. Call after any future
    /// artifact load (kind 3, Drop 2.2c) and after training in tests.
    pub fn bag_validate_internal_consistency(&self) -> Result<(), GranmoModelError> {
        // Vocabulary structural validity first (value-integrity rule).
        self.bag_vocabulary.vocab_validity_recheck()?;

        // Storage-geometry gates.
        let feature_count = self.bag_feature_count();
        let expected_state_total = self
            .bag_clause_total
            .checked_mul(self.bag_literals_per_clause())
            .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
        if self.bag_ta_states.len() != expected_state_total {
            return Err(GranmoModelError::BbgEngineIndexOutOfRange);
        }
        let expected_mask_total = self
            .bag_clause_total
            .checked_mul(self.bag_mask_words_per_clause())
            .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;
        if self.bag_positive_include_masks.len() != expected_mask_total
            || self.bag_negated_include_masks.len() != expected_mask_total
        {
            return Err(GranmoModelError::BbgEngineIndexOutOfRange);
        }

        // Guard storage geometry (Drop 4.1): one streak slot and one
        // reset-count slot per clause. Streaks carry no cross-invariant
        // against automaton states (they are observations, not derived
        // caches), so geometry is the only checkable property.
        if self.bag_fire_streaks.len() != self.bag_clause_total
            || self.bag_fire_guard_reset_counts.len() != self.bag_clause_total
        {
            return Err(GranmoModelError::BbgFireGuardIndexOutOfRange);
        }

        for clause in 0..self.bag_clause_total {
            let clause_depth = self.bag_depth_for_clause(clause);
            let band_ceiling = clause_depth
                .checked_mul(2)
                .ok_or(GranmoModelError::BbgEngineArithmeticOverflow)?;

            // Recompute both masks from raw states, checking the band
            // for every automaton along the way.
            let words = self.bag_mask_words_per_clause();
            let mut recomputed_positive = vec![0u64; words];
            let mut recomputed_negated = vec![0u64; words];
            for rank in 0..feature_count {
                let positive_state = *self
                    .bag_ta_states
                    .get(self.bag_state_index(clause, rank)?)
                    .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
                let negated_state = *self
                    .bag_ta_states
                    .get(self.bag_state_index(clause, feature_count + rank)?)
                    .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;

                if positive_state < 1
                    || positive_state > band_ceiling
                    || negated_state < 1
                    || negated_state > band_ceiling
                {
                    return Err(GranmoModelError::BbgStateValueOutOfRange);
                }
                if positive_state > clause_depth {
                    let word = recomputed_positive
                        .get_mut(rank >> 6)
                        .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
                    *word |= 1u64 << (rank & 63);
                }
                if negated_state > clause_depth {
                    let word = recomputed_negated
                        .get_mut(rank >> 6)
                        .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
                    *word |= 1u64 << (rank & 63);
                }
            }

            // Compare against the stored (incrementally maintained) masks.
            let (range_start, range_end) = self.bag_mask_word_range(clause)?;
            let stored_positive = self
                .bag_positive_include_masks
                .get(range_start..range_end)
                .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
            let stored_negated = self
                .bag_negated_include_masks
                .get(range_start..range_end)
                .ok_or(GranmoModelError::BbgEngineIndexOutOfRange)?;
            if stored_positive != recomputed_positive.as_slice()
                || stored_negated != recomputed_negated.as_slice()
            {
                return Err(GranmoModelError::BbgIncludeMaskInconsistent);
            }
        }
        Ok(())
    }

    /// Test-only helper: forces a literal fully into the include region via
    /// the mask-maintaining view transition path. Terminates when the state
    /// stops changing (saturated at 2N).
    #[cfg(test)]
    fn bag_test_force_include(&mut self, clause: usize, local_literal: usize) {
        loop {
            let mut clause_view = self._test_bag_build_clause_view(clause).unwrap();
            let before = clause_view.states[local_literal];
            clause_view.view_bag_increment(local_literal).unwrap();
            let after = clause_view.states[local_literal];
            if after == before {
                break;
            }
        }
    }
}

// ===========================================================================
// SECTION 12C: HybridTM — Joint Two-Bank Co-Training Engine (M-Hybrid)
// ===========================================================================
//
// Scientific role: tests whether the resource-allocation gate (T ∓ V)/2T,
// applied to the COMBINED vote V = V_conv + V_bag, drives the two feature
// spaces to specialize complementarily (an emergent residual/boosting
// dynamic) versus independently trained banks summed post hoc (Section
// 10B, late fusion). Hybrid vs. late-fusion is a single-variable
// comparison: same two banks, same sizes, same split — joint gating is
// the only difference.
//
// NOT M-Hetero: M-Hetero varies P4-tier tuning (N, s) per clause over ONE
// feature space through the per-clause accessors. This engine composes two
// banks over DIFFERENT feature spaces. The two are orthogonal and may later
// be combined.
//
// Naming: all methods carry the `hyb_` prefix; constructor is
// `new_from_sub_engines` (same convention as `new_with_vocabulary`).
//
// Concatenated clause index space (design commitment): conv clauses
// occupy [0, C_conv), bag clauses occupy [C_conv, C_conv + C_bag). Because
// C_conv is even (`ClauseCount`), bag clause b keeps its parity at index
// C_conv + b, so the crate-wide parity-polarity rule holds over the
// combined space. This is what lets `vote_from_fired_words` (harness
// single-pass evaluation) reproduce `hyb_vote_sum` exactly.
//
// RNG contract: exactly ONE master `next_u64()` per step (as every engine).
// The conv bank uses that step seed directly; the bag bank uses a seed
// derived from it under `RNG_PURPOSE_HYBRID_BAG_BANK`, so same-index
// clauses in the two banks never share a stream (see Section 2 note).
//
// Vote target: the SHARED T governs training gates and the probability
// LUT. Each sub-engine still carries its own T field (needed by the
// kind-1/kind-3 body writers and for standalone reuse) but it is unused
// for hybrid gating. Recorded open question: with 2× the clauses of a
// standalone bank, whether T should be scaled (e.g., 2T) is an ablation,
// not a decision made here.

/// Joint two-bank Granmo Model: a `ByteConvTM` bank and a `ByteBagTM`
/// bank trained under ONE shared vote target with feedback gates computed
/// from the combined vote. See the section banner for the design record.
#[derive(Debug, Clone)]
pub struct HybridTM {
    /// Positional bank (clause indices [0, C_conv) in the combined space).
    hyb_conv_engine: ByteConvTM,
    /// Presence bank (clause indices [C_conv, C_conv + C_bag)).
    hyb_bag_engine: ByteBagTM,
    /// Shared T, as i32 for the shared gate helper.
    hyb_vote_target: i32,
}

/// Copies set fired-bits from one sub-bank bitset into the combined bitset
/// at `target_offset`. Bounds are checked on every word access; an
/// out-of-range word means the sub-bank's clause count disagrees with its
/// bitset (corrupt wiring), reported as `HybIndexOutOfRange`.
fn hyb_copy_fired_bits_with_offset(
    source_words: &[u64],
    source_clause_total: usize,
    target_offset: usize,
    target_words: &mut [u64],
) -> Result<(), GranmoModelError> {
    for source_clause in 0..source_clause_total {
        let source_word = source_words
            .get(source_clause >> 6)
            .copied()
            .ok_or(GranmoModelError::HybIndexOutOfRange)?;
        if source_word & (1u64 << (source_clause & 63)) != 0 {
            let target_clause = target_offset
                .checked_add(source_clause)
                .ok_or(GranmoModelError::HybArithmeticOverflow)?;
            let target_word = target_words
                .get_mut(target_clause >> 6)
                .ok_or(GranmoModelError::HybIndexOutOfRange)?;
            *target_word |= 1u64 << (target_clause & 63);
        }
    }
    Ok(())
}

impl HybridTM {
    /// Composes a hybrid from two already-constructed sub-engines. Both are
    /// consistency-validated here (value-integrity rule), and the conv bank
    /// must have an even clause count so the concatenated parity-polarity
    /// rule holds (see banner). The leakage guard for the bag vocabulary is
    /// the caller's responsibility, exactly as for `new_with_vocabulary`.
    pub fn new_from_sub_engines(
        conv_engine: ByteConvTM,
        bag_engine: ByteBagTM,
        shared_vote_target: VoteThreshold,
    ) -> Result<Self, GranmoModelError> {
        let resolved_target = i32::from(shared_vote_target.get()?);
        if conv_engine.conv_clause_count() % 2 != 0 {
            #[cfg(debug_assertions)]
            eprintln!(
                "HYB-1203: conv bank clause count {} is odd; parity-polarity rule would break",
                conv_engine.conv_clause_count()
            );
            return Err(GranmoModelError::HybClauseGeometryFault);
        }
        conv_engine.conv_validate_internal_consistency()?;
        bag_engine.bag_validate_internal_consistency()?;
        Ok(Self {
            hyb_conv_engine: conv_engine,
            hyb_bag_engine: bag_engine,
            hyb_vote_target: resolved_target,
        })
    }

    /// Read access to the conv bank (explainability trace, tests).
    pub fn hyb_conv_engine_ref(&self) -> &ByteConvTM {
        &self.hyb_conv_engine
    }

    /// Read access to the bag bank (explainability trace, tests).
    pub fn hyb_bag_engine_ref(&self) -> &ByteBagTM {
        &self.hyb_bag_engine
    }

    /// Clause count of the conv bank = the bag bank's index offset in the
    /// combined clause space.
    pub fn hyb_conv_clause_count(&self) -> usize {
        self.hyb_conv_engine.conv_clause_count()
    }

    /// Combined clause count. Saturating by documented posture: each bank
    /// is <= `ClauseCount::MAX`, so the sum (<= 131,068) cannot saturate on
    /// any supported target; the signature matches the dispatch enum's
    /// plain-`usize` clause-count accessor.
    pub fn hyb_clause_count(&self) -> usize {
        self.hyb_conv_engine
            .conv_clause_count()
            .saturating_add(self.hyb_bag_engine.bag_clause_count())
    }

    /// Combined vote V = V_conv + V_bag (exact integer addition).
    pub fn hyb_vote_sum(&self, document: &[u8]) -> Result<i32, GranmoModelError> {
        let conv_vote = self.hyb_conv_engine.conv_vote_sum(document)?;
        let bag_vote = self.hyb_bag_engine.bag_vote_sum(document)?;
        conv_vote
            .checked_add(bag_vote)
            .ok_or(GranmoModelError::HybArithmeticOverflow)
    }

    /// Binary prediction: label 1 iff `V > decision_threshold`.
    pub fn hyb_predict(
        &self,
        document: &[u8],
        decision_threshold: i32,
    ) -> Result<bool, GranmoModelError> {
        Ok(self.hyb_vote_sum(document)? > decision_threshold)
    }

    /// Combined fired-clause bitset over the concatenated clause space
    /// (conv bits first, bag bits at offset C_conv). Embedding deliverable
    /// and single-pass evaluation input, as for the other engines.
    pub fn hyb_fired_clause_bits(&self, document: &[u8]) -> Result<Vec<u64>, GranmoModelError> {
        let conv_total = self.hyb_conv_engine.conv_clause_count();
        let bag_total = self.hyb_bag_engine.bag_clause_count();
        let combined_total = conv_total
            .checked_add(bag_total)
            .ok_or(GranmoModelError::HybArithmeticOverflow)?;
        let word_count = combined_total
            .checked_add(63)
            .ok_or(GranmoModelError::HybArithmeticOverflow)?
            / 64;
        let mut combined_words = vec![0u64; word_count];

        let conv_words = self.hyb_conv_engine.conv_fired_clause_bits(document)?;
        hyb_copy_fired_bits_with_offset(&conv_words, conv_total, 0, &mut combined_words)?;
        let bag_words = self.hyb_bag_engine.bag_fired_clause_bits(document)?;
        hyb_copy_fired_bits_with_offset(&bag_words, bag_total, conv_total, &mut combined_words)?;
        Ok(combined_words)
    }

    /// One joint training update, single-threaded. Byte-identical to
    /// `hyb_train_step_with_workers(..., WorkerCount 1)` and, by the
    /// per-clause RNG contract, to any worker count.
    pub fn hyb_train_step(
        &mut self,
        document: &[u8],
        label_is_positive: bool,
        rng: &mut FastRng,
    ) -> Result<(), GranmoModelError> {
        self.hyb_train_step_with_workers(document, label_is_positive, rng, WorkerCount::new(1)?)
    }

    /// One joint training update: both banks scan (pass 1), the votes are
    /// summed, ONE set of gates is computed from the combined vote against
    /// the SHARED T, and both banks receive feedback under those gates
    /// (pass 2). Each pass reuses the sub-engines' own phase functions, so
    /// the hybrid runs byte-identical scan/feedback code to the standalone
    /// engines — only the gate provenance differs.
    pub fn hyb_train_step_with_workers(
        &mut self,
        document: &[u8],
        label_is_positive: bool,
        rng: &mut FastRng,
        worker_count: WorkerCount,
    ) -> Result<(), GranmoModelError> {
        // EXACTLY one master draw per step, at every worker count.
        let step_seed = rng.next_u64();
        // Decorrelate the bag bank's clause streams from the conv bank's.
        let bag_step_seed = derive_clause_stream_seed(
            step_seed,
            HYBRID_BAG_BANK_ORDINAL,
            RNG_PURPOSE_HYBRID_BAG_BANK,
        );

        // --- Pass 1: both banks scan (read-only) ---
        let (conv_scan_outcomes, conv_vote) =
            self.hyb_conv_engine
                .conv_scan_pass_with_workers(document, step_seed, worker_count)?;
        let presence_bits = self.hyb_bag_engine.bag_document_presence_bits(document)?;
        let (bag_fired_flags, bag_vote) = self.hyb_bag_engine.bag_scan_pass(&presence_bits)?;

        // --- Reduce: combined vote, SHARED gates ---
        let combined_vote = conv_vote
            .checked_add(bag_vote)
            .ok_or(GranmoModelError::HybArithmeticOverflow)?;
        let shared_gates = resolve_feedback_gates(combined_vote, self.hyb_vote_target)?;

        // --- Pass 2: both banks receive feedback under the shared gates ---
        self.hyb_conv_engine.conv_feedback_pass_with_workers(
            document,
            &conv_scan_outcomes,
            step_seed,
            shared_gates,
            label_is_positive,
            worker_count,
        )?;
        self.hyb_bag_engine.bag_feedback_pass_with_workers(
            &bag_fired_flags,
            &presence_bits,
            bag_step_seed,
            shared_gates,
            label_is_positive,
            worker_count,
        )?;
        Ok(())
    }

    /// Full consistency validation of both banks (artifact load gate 4).
    pub fn hyb_validate_internal_consistency(&self) -> Result<(), GranmoModelError> {
        self.hyb_conv_engine.conv_validate_internal_consistency()?;
        self.hyb_bag_engine.bag_validate_internal_consistency()
    }

    /// Per-clause included-literal totals over the combined clause space
    /// (conv first, then bag — same order as `hyb_fired_clause_bits`).
    pub fn hyb_clause_include_totals(&self) -> Result<Vec<u32>, GranmoModelError> {
        let mut totals = self.hyb_conv_engine.conv_clause_include_totals()?;
        totals.extend_from_slice(&self.hyb_bag_engine.bag_clause_include_totals()?);
        Ok(totals)
    }

    /// Fire-guard resets (bag bank only; the conv bank has no guard yet).
    pub fn hyb_fire_guard_reset_total(&self) -> u64 {
        self.hyb_bag_engine.bag_fire_guard_reset_total()
    }

    /// Decodes a clause of the COMBINED index space: conv decode for
    /// indices below C_conv, bag decode (index minus C_conv) otherwise.
    pub fn hyb_describe_clause(
        &self,
        clause: usize,
        max_rendered_literals: usize,
    ) -> Result<String, GranmoModelError> {
        let conv_total = self.hyb_conv_engine.conv_clause_count();
        if clause < conv_total {
            self.hyb_conv_engine
                .conv_describe_clause(clause, max_rendered_literals)
        } else {
            let bag_clause = clause
                .checked_sub(conv_total)
                .ok_or(GranmoModelError::HybIndexOutOfRange)?;
            // bag_describe_clause rejects bag_clause >= C_bag itself.
            self.hyb_bag_engine
                .bag_describe_clause(bag_clause, max_rendered_literals)
        }
    }

    /// Probability LUT over the combined vote range [-(C_conv+C_bag)/2,
    /// +(C_conv+C_bag)/2] with the SHARED T. Routed through the enforced
    /// newtypes; the combined count must fit `ClauseCount::MAX` (each bank
    /// <= 32,766 clauses), else `HybIndexOutOfRange`.
    pub fn hyb_build_probability_lut(&self) -> Result<ProbabilityLut, GranmoModelError> {
        let combined_count_u16 = u16::try_from(self.hyb_clause_count())
            .map_err(|_| GranmoModelError::HybIndexOutOfRange)?;
        let vote_target_i16 = i16::try_from(self.hyb_vote_target)
            .map_err(|_| GranmoModelError::HybIndexOutOfRange)?;
        ProbabilityLut::build(
            ClauseCount::new(combined_count_u16)?,
            VoteThreshold::new(vote_target_i16)?,
        )
    }
}

// ===========================================================================
//  SECTION le main
// ===========================================================================

/*
# 5-fold, records for both sides, shallow settings
cargo run --release -- --mode train --engine byte-bag --preset p0 \
  --data /abs/data.jsonl --folds 5 --seed 42 --split-seed 7 \
  --score-train-side --records-out /abs/out/predictions.tsv \
  --clauses 200 --states 50 --epochs 4 --workers auto

# repeat with other --split-seed values to accumulate; then
cargo run --release -- --mode row-audit \
  --records-in /abs/out/predictions.tsv --audit-top 100
*/

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();
    let outcome = if raw_args.len() <= 1 {
        run_self_check()
    } else {
        match CliArgs::parse_cliargs(&raw_args) {
            // Main Dispatch, Read all about it!
            Ok(args) => match args.mode.as_str() {
                "train" => handle_train(&args),
                "batch" => handle_batch(&args),
                "predict" => handle_predict(&args),
                "batch-guard" => handle_batch_guard(&args),
                "row-audit" => handle_row_audit(&args),
                _other => {
                    #[cfg(debug_assertions)]
                    eprintln!("CLI-804: unknown mode '{}'", _other);
                    Err(GranmoModelError::CliUnknownMode)
                }
            },
            Err(parse_error) => Err(parse_error),
        }
    };

    if let Err(error_code) = outcome {
        #[cfg(debug_assertions)]
        eprintln!("failed: {}", error_code);
        std::process::exit(i32::from(error_code.code()));
    }
}

// ===========================================================================
// Tests
// ===========================================================================

/*
Test Notes:

Because folding precedes stripping, P3 can never remove !, @, $, 0, 1, 3 — they
always fold first. If an ablation ever needs "strip everything including leet
carriers," that is a different profile (symbol strip without leet fold, i.e.
P0 + stages 6+7), which the bitmask already expresses — no code change required,
just a preset name if you want one (e.g., P3b).
*/

#[cfg(test)]
mod tests {
    use super::*;

    // --- ByteConvTM: construction and geometry ---

    /// Test-harness constructor with sane defaults; parameters that the
    /// individual test varies are exposed.
    fn make_engine(patch: u8, stride: u8, clauses: u16, guarded: bool) -> ByteConvTM {
        ByteConvTM::new(
            PatchSize::new(patch).unwrap(),
            StrideLen::new(stride).unwrap(),
            ClauseCount::new(clauses).unwrap(),
            VoteThreshold::new(15).unwrap(),
            StatesPerAction::new(100).unwrap(),
            SpecificityThresholds::from_specificity(3.0).unwrap(),
            MaxScanBytes::new(256).unwrap(),
            guarded,
        )
        .unwrap()
    }

    #[test]
    fn engine_rejects_stride_exceeding_patch() {
        let result = ByteConvTM::new(
            PatchSize::new(2).unwrap(),
            StrideLen::new(4).unwrap(),
            ClauseCount::new(4).unwrap(),
            VoteThreshold::new(15).unwrap(),
            StatesPerAction::new(100).unwrap(),
            SpecificityThresholds::from_specificity(3.0).unwrap(),
            MaxScanBytes::new(256).unwrap(),
            false,
        );
        assert_eq!(
            result.err(),
            Some(GranmoModelError::BctStrideExceedsPatchSize)
        );
    }

    #[test]
    fn fresh_engine_fires_everywhere_and_votes_zero() {
        let engine = make_engine(5, 2, 8, false);
        // Fresh clauses include no literals => all-permissive masks => every
        // clause fires; balanced polarity => V = 0; consistency must hold.
        assert_eq!(engine.conv_vote_sum(b"any text at all").unwrap(), 0);
        assert!(!engine.conv_predict(b"any text at all", 0).unwrap());
        engine.conv_validate_internal_consistency().unwrap();
        // Embedding bitset: all 8 clause bits set on a fresh model.
        let bits = engine.conv_fired_clause_bits(b"x").unwrap();
        assert_eq!(bits, vec![0b1111_1111u64]);
    }

    // --- ByteConvTM: mask semantics, order sensitivity, stride, padding ---

    #[test]
    fn positive_include_restricts_slot_and_reports_positions() {
        let mut engine = make_engine(2, 1, 2, false);
        // Clause 0: require byte 'a' at slot 0 and 'b' at slot 1.
        engine.conv_test_force_include(0, engine.positive_local_index(0, usize::from(b'a')));
        engine.conv_test_force_include(0, engine.positive_local_index(1, usize::from(b'b')));
        engine.conv_validate_internal_consistency().unwrap();

        // "zab": windows at 0 ("za") and 1 ("ab") => fires only at offset 1.
        assert_eq!(
            engine.conv_fired_window_positions(0, b"zab").unwrap(),
            vec![1]
        );
        // Order sensitivity: same bytes reversed must NOT fire.
        assert!(
            engine
                .conv_fired_window_positions(0, b"zba")
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn stride_two_misses_odd_offset_pattern_stride_one_finds_it() {
        // Documents the known cost of S=2 recorded in the plan: patterns at
        // odd offsets are only visible at odd window starts.
        let mut s2 = make_engine(2, 2, 2, false);
        s2.conv_test_force_include(0, s2.positive_local_index(0, usize::from(b'a')));
        s2.conv_test_force_include(0, s2.positive_local_index(1, usize::from(b'b')));
        assert!(
            s2.conv_fired_window_positions(0, b"zab")
                .unwrap()
                .is_empty()
        );

        let mut s1 = make_engine(2, 1, 2, false);
        s1.conv_test_force_include(0, s1.positive_local_index(0, usize::from(b'a')));
        s1.conv_test_force_include(0, s1.positive_local_index(1, usize::from(b'b')));
        assert_eq!(s1.conv_fired_window_positions(0, b"zab").unwrap(), vec![1]);
    }

    #[test]
    fn short_document_right_pads_with_zero() {
        let mut engine = make_engine(5, 1, 4, false);
        // Clause 0: requires PAD (0x00) at slot 4 — satisfied by a short doc.
        engine.conv_test_force_include(0, engine.positive_local_index(4, 0));
        // Clause 2: forbids PAD at slot 2 (negated 0x00) — blocked by padding.
        engine.conv_test_force_include(2, engine.negated_local_index(2, 0));
        engine.conv_validate_internal_consistency().unwrap();

        // "hi" (2 bytes < K=5) => exactly one window: ['h','i',0,0,0].
        assert_eq!(
            engine.conv_fired_window_positions(0, b"hi").unwrap(),
            vec![0]
        );
        assert!(
            engine
                .conv_fired_window_positions(2, b"hi")
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn guarded_include_refuses_second_positive_at_slot() {
        // Guard ON: second positive include at the same slot is refused;
        // the mask stays single-byte, the clause stays alive.
        let mut guarded = make_engine(2, 1, 2, true);
        guarded.conv_test_force_include(0, guarded.positive_local_index(0, usize::from(b'a')));
        guarded.conv_test_force_include(0, guarded.positive_local_index(0, usize::from(b'b')));
        guarded.conv_validate_internal_consistency().unwrap();
        assert_eq!(
            guarded.conv_fired_window_positions(0, b"ax").unwrap(),
            vec![0]
        );

        // Guard OFF: both includes land; the slot mask is empty; the clause
        // is structurally dead (the defect the guard exists to prevent).
        let mut unguarded = make_engine(2, 1, 2, false);
        unguarded.conv_test_force_include(0, unguarded.positive_local_index(0, usize::from(b'a')));
        unguarded.conv_test_force_include(0, unguarded.positive_local_index(0, usize::from(b'b')));
        unguarded.conv_validate_internal_consistency().unwrap();
        assert!(
            unguarded
                .conv_fired_window_positions(0, b"ax")
                .unwrap()
                .is_empty()
        );
        assert!(
            unguarded
                .conv_fired_window_positions(0, b"bx")
                .unwrap()
                .is_empty()
        );
    }

    // --- ByteConvTM: training invariants and learning ---

    #[test]
    fn consistency_survives_stochastic_training() {
        let mut engine = make_engine(5, 1, 16, false);
        let mut rng = FastRng::seed(7);
        let docs: [(&[u8], bool); 5] = [
            (b"abcdefg", true),
            (b"gfedcba", false),
            (b"hi", true), // short doc: padded-window path
            (b"", false),  // empty doc: all-PAD window must train cleanly
            (b"aaaaaaaaaaaaaaaa", true),
        ];
        for _ in 0..50 {
            for (doc, label) in docs {
                engine.conv_train_step(doc, label, &mut rng).unwrap();
            }
        }
        engine.conv_validate_internal_consistency().unwrap();
    }

    #[test]
    fn learns_byte_level_negation_micro_corpus() {
        // The core Phase 2 learning test, byte-level analogue of Phase 1's
        // negation demo: both classes contain " good"; only local byte
        // context ("very " vs "not g") separates them. Deterministic seed.
        let positives: [&[u8]; 4] = [
            b"very good movie",
            b"very good story",
            b"very good acting",
            b"it was very good",
        ];
        let negatives: [&[u8]; 4] = [
            b"not good movie",
            b"not good story",
            b"not good acting",
            b"it was not good",
        ];

        let mut engine = make_engine(5, 1, 32, false);
        let mut rng = FastRng::seed(42);
        for _ in 0..300 {
            for doc in positives {
                engine.conv_train_step(doc, true, &mut rng).unwrap();
            }
            for doc in negatives {
                engine.conv_train_step(doc, false, &mut rng).unwrap();
            }
        }
        engine.conv_validate_internal_consistency().unwrap();

        for doc in positives {
            assert!(
                engine.conv_predict(doc, 0).unwrap(),
                "positive doc misclassified: {:?}",
                core::str::from_utf8(doc)
            );
        }
        for doc in negatives {
            assert!(
                !engine.conv_predict(doc, 0).unwrap(),
                "negative doc misclassified: {:?}",
                core::str::from_utf8(doc)
            );
        }
    }
    /*
    Test-Note:
    The learning test is the only stochastic-outcome test; it is deterministic under
    seed 42 but its pass depends on hyperparameters I cannot execute here. If it fails
    on your machine, report the failure pattern (which docs misclassify, and the
    vote_sum values) — the first knobs are epochs 300→500 and clauses 32→48, and I will
    tune against your observed votes rather than guessing.
    */

    #[test]
    fn describe_clause_decodes_forced_pattern() {
        let mut engine = make_engine(2, 1, 2, false);
        engine.conv_test_force_include(0, engine.positive_local_index(0, usize::from(b'n')));
        engine.conv_test_force_include(0, engine.negated_local_index(1, 0)); // k1 ≠ 0x00
        let description = engine.conv_describe_clause(0, 12).unwrap();
        assert!(description.contains("k0='n'"), "got: {description}");
        assert!(description.contains("k1≠0x00"), "got: {description}");
    }

    // --- Error system ---

    #[test]
    fn error_codes_are_two_bytes_and_copy() {
        assert_eq!(core::mem::size_of::<GranmoModelError>(), 2);
        let e = GranmoModelError::RngGenIndexEmptyRange;
        let e2 = e; // Copy
        assert_eq!(e, e2);
        assert_eq!(e.code(), 200);
        assert!(!e.is_retryable());
    }

    // --- RNG ---

    #[test]
    fn rng_is_deterministic_and_zero_seed_safe() {
        let mut a = FastRng::seed(42);
        let mut b = FastRng::seed(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
        let mut z = FastRng::seed(0);
        assert_ne!(z.next_u64(), 0, "zero seed must not absorb");
    }

    #[test]
    fn rng_gen_index_bounds_and_empty_range() {
        let mut rng = FastRng::seed(7);
        for len in [1usize, 2, 3, 100, 65537] {
            for _ in 0..200 {
                let idx = rng.gen_index(len).unwrap();
                assert!(idx < len);
            }
        }
        assert_eq!(
            rng.gen_index(0),
            Err(GranmoModelError::RngGenIndexEmptyRange)
        );
    }

    #[test]
    fn rng_coin_threshold_extremes() {
        let mut rng = FastRng::seed(9);
        // threshold 0 => never true; threshold u16::MAX => almost always true.
        for _ in 0..1000 {
            assert!(!rng.coin(0));
        }
        let hits = (0..1000).filter(|_| rng.coin(u16::MAX)).count();
        assert!(hits > 990);
    }

    // --- Custom types ---

    #[test]
    fn patch_size_bounds_and_recheck() {
        assert!(PatchSize::new(1).is_err());
        assert!(PatchSize::new(17).is_err());
        let k = PatchSize::new(5).unwrap();
        assert_eq!(k.get().unwrap(), 5);
    }

    #[test]
    fn clause_count_must_be_even_nonzero() {
        assert!(ClauseCount::new(0).is_err());
        assert!(ClauseCount::new(3).is_err());
        assert_eq!(ClauseCount::new(200).unwrap().get().unwrap(), 200);
    }

    #[test]
    fn specificity_thresholds_integer_exactness() {
        // s = 4.0 => forget = 16384 (=65536/4), reinforce = 49152.
        let t = SpecificityThresholds::from_specificity(4.0).unwrap();
        assert_eq!(t.get().unwrap(), (16384, 49152));
        // Invalid s values rejected.
        assert!(SpecificityThresholds::from_specificity(1.0).is_err());
        assert!(SpecificityThresholds::from_specificity(f64::NAN).is_err());
        assert!(SpecificityThresholds::from_specificity(0.5).is_err());
    }

    // --- Preprocess profiles ---

    #[test]
    fn profile_rejects_reserved_bits() {
        assert!(PreprocessProfile::from_bits(0x0100).is_err());
        assert!(PreprocessProfile::from_bits(0x00FF).is_ok());
    }

    #[test]
    fn presets_have_expected_stage_sets() {
        let p0 = PreprocessProfile::preset_p0();
        assert!(p0.has_stage(PreprocessStage::AsciiLowercase).unwrap());
        assert!(!p0.has_stage(PreprocessStage::LeetFold).unwrap());
        let p1 = PreprocessProfile::preset_p1();
        assert!(!p1.has_stage(PreprocessStage::AsciiLowercase).unwrap());
        assert!(p1.has_stage(PreprocessStage::WhitespaceFold).unwrap());
        let p2 = PreprocessProfile::preset_p2();
        assert!(p2.has_stage(PreprocessStage::LeetFold).unwrap());
        assert_eq!(PreprocessProfile::preset_raw().get_bits().unwrap(), 0);
    }

    // --- Preprocess pipeline semantics ---

    #[test]
    fn raw_profile_is_identity_including_nonascii() {
        let mut pp = BytePreprocessor::new(PreprocessProfile::preset_raw()).unwrap();
        let input: Vec<u8> = vec![0x00, b'A', b' ', b' ', 0xF0, 0x9F, 0x98, 0x80, b'\n'];
        assert_eq!(pp.process_document(&input).unwrap(), input);
    }

    #[test]
    fn p0_folds_dedupes_trims_lowercases() {
        let mut pp = BytePreprocessor::new(PreprocessProfile::preset_p0()).unwrap();
        assert_eq!(
            pp.process_document(b"\t  HeLLo\n\nW0rld! ").unwrap(),
            b"hello w0rld! ".to_vec()
        );
    }

    #[test]
    fn p2_leet_fold_merges_obfuscated_spelling() {
        let mut pp = BytePreprocessor::new(PreprocessProfile::preset_p2()).unwrap();
        assert_eq!(pp.process_document(b"1d10t").unwrap(), b"ldlot".to_vec());
        assert_eq!(
            pp.process_document(b"y0u $uck!!").unwrap(),
            b"you suckii".to_vec()
        );
    }

    #[test]
    fn p4_digit_strip_and_p5_space_strip() {
        let mut p4 = BytePreprocessor::new(PreprocessProfile::preset_p4()).unwrap();
        assert_eq!(p4.process_document(b"ab1c2 3d").unwrap(), b"abc d".to_vec());
        let mut p5 = BytePreprocessor::new(PreprocessProfile::preset_p5()).unwrap();
        assert_eq!(p5.process_document(b"a b  c").unwrap(), b"abc".to_vec());
    }

    #[test]
    fn symbol_strip_excludes_space_and_passes_nonascii() {
        // Symbol strip tested in ISOLATION (P0 + SymbolStrip, no leet fold),
        // so punctuation removal semantics are not confounded by stage 5.
        let profile = PreprocessProfile::from_bits(
            PreprocessProfile::preset_p0().get_bits().unwrap()
                | PreprocessStage::SymbolStrip as u16,
        )
        .unwrap();
        let mut pp = BytePreprocessor::new(profile).unwrap();
        // Punctuation removed (including '!'), spaces kept,
        // non-ASCII (0xC3 0xA9 = 'é') passes through untouched.
        assert_eq!(
            pp.process_document(b"a.b, c\xC3\xA9!").unwrap(),
            b"ab c\xC3\xA9".to_vec()
        );
    }

    #[test]
    fn p3_leet_fold_wins_over_symbol_strip_by_stage_order() {
        // Canonical stage order is load-bearing: in P3, stage 5 (leet fold)
        // runs BEFORE stage 7 (symbol strip), so leet-carrier symbols
        // ('!', '@', '$') are folded into letters and SURVIVE the strip,
        // while non-carrier punctuation ('.', ',') is removed. This is the
        // intended obfuscation-merging behavior, not an accident.
        let mut p3 = BytePreprocessor::new(PreprocessProfile::preset_p3()).unwrap();
        assert_eq!(
            p3.process_document(b"a.b, c\xC3\xA9!").unwrap(),
            b"ab c\xC3\xA9i".to_vec()
        );
        // Carrier vs non-carrier side by side: '$' -> 's' survives; '.' dies.
        assert_eq!(p3.process_document(b"a$.b").unwrap(), b"asb".to_vec());
    }

    #[test]
    fn streaming_equals_batch_processing() {
        // The core streaming guarantee: byte-at-a-time output must be
        // byte-identical to batch output for the same document.
        let profile = PreprocessProfile::preset_p2();
        let input = b"  Y0u  ArE\ta 1d10t!!  \n".to_vec();

        let mut batch = BytePreprocessor::new(profile).unwrap();
        let batch_out = batch.process_document(&input).unwrap();

        let mut streaming = BytePreprocessor::new(profile).unwrap();
        streaming.reset();
        let mut stream_out = Vec::new();
        for &b in &input {
            if let Some(e) = streaming.process_byte(b).unwrap() {
                stream_out.push(e);
            }
        }
        assert_eq!(batch_out, stream_out);
    }

    #[test]
    fn reset_isolates_documents() {
        let mut pp = BytePreprocessor::new(PreprocessProfile::preset_p0()).unwrap();
        let first = pp.process_document(b"  a").unwrap();
        let second = pp.process_document(b"  b").unwrap();
        // Leading trim must re-apply on the second document.
        assert_eq!(first, b"a".to_vec());
        assert_eq!(second, b"b".to_vec());
    }

    // --- Probability LUT ---

    /// The LUT must be a monotone sigmoid with its midpoint at V = 0:
    /// p(0) = 0.5 exactly (32768/65535 after rounding), extremes near 0/1,
    /// and symmetric tails. These are the properties the M-Ablate reports
    /// will rely on when comparing operating points across models.
    #[test]
    fn probability_lut_is_monotone_sigmoid_centered_at_zero() {
        let lut = ProbabilityLut::build(
            ClauseCount::new(20).unwrap(),
            VoteThreshold::new(15).unwrap(),
        )
        .unwrap();
        lut.lut_validity_recheck().unwrap();

        let p_neg = lut.probability_u16(-10).unwrap();
        let p_zero = lut.probability_u16(0).unwrap();
        let p_pos = lut.probability_u16(10).unwrap();
        assert!(p_neg < p_zero && p_zero < p_pos);
        // sigmoid(0) = 0.5 -> round(0.5 * 65535) = 32768.
        assert_eq!(p_zero, 32768);
        // Symmetry: p(-v) + p(+v) ≈ 65535 (within rounding of 1 unit).
        let tail_sum = u32::from(p_neg) + u32::from(p_pos);
        assert!((65534..=65536).contains(&tail_sum), "tail sum {tail_sum}");
    }

    /// Votes outside [-C/2, +C/2] indicate a LUT paired with the wrong
    /// model — must be reported as an error, never clamped silently.
    #[test]
    fn probability_lut_rejects_out_of_range_votes() {
        let lut = ProbabilityLut::build(
            ClauseCount::new(20).unwrap(),
            VoteThreshold::new(15).unwrap(),
        )
        .unwrap();
        assert!(lut.probability_u16(-10).is_ok());
        assert!(lut.probability_u16(10).is_ok());
        assert_eq!(
            lut.probability_u16(11),
            Err(GranmoModelError::PrbVoteOutOfRange)
        );
        assert_eq!(
            lut.probability_u16(-11),
            Err(GranmoModelError::PrbVoteOutOfRange)
        );
    }

    // --- Threshold sweep ---

    /// Hand-computed sweep over four documents. Votes/labels chosen so each
    /// threshold row has a distinct, easily verified confusion matrix:
    ///   votes  [-2, -1, 1, 3], labels [neg, pos, pos, pos].
    #[test]
    fn threshold_sweep_matches_hand_computation() {
        let votes = [-2, -1, 1, 3];
        let labels = [false, true, true, true];
        let rows = sweep_decision_thresholds(&votes, &labels).unwrap();

        // Thresholds swept: -3 (all positive) .. 3 (none positive) = 7 rows.
        assert_eq!(rows.len(), 7);
        assert_eq!(rows[0].decision_threshold, -3);
        assert_eq!(rows[6].decision_threshold, 3);

        // At c = -3: everything predicted positive -> TP=3, FP=1.
        assert_eq!((rows[0].true_positives, rows[0].false_positives), (3, 1));
        assert!((rows[0].recall - 1.0).abs() < 1e-12);

        // At c = 0: predictions positive for votes {1, 3} -> TP=2, FP=0,
        // FN=1, TN=1: precision 1.0, recall 2/3.
        let row_zero = rows.iter().find(|r| r.decision_threshold == 0).unwrap();
        assert_eq!(row_zero.true_positives, 2);
        assert_eq!(row_zero.false_positives, 0);
        assert_eq!(row_zero.false_negatives, 1);
        assert_eq!(row_zero.true_negatives, 1);
        assert!((row_zero.precision - 1.0).abs() < 1e-12);
        assert!((row_zero.recall - 2.0 / 3.0).abs() < 1e-12);

        // At c = 3: nothing predicted positive -> precision defined as 0.
        assert_eq!(rows[6].true_positives, 0);
        assert!((rows[6].precision - 0.0).abs() < 1e-12);
    }

    #[test]
    fn threshold_sweep_rejects_bad_inputs() {
        assert_eq!(
            sweep_decision_thresholds(&[], &[]),
            Err(GranmoModelError::PrbSweepEmptyInput)
        );
        assert_eq!(
            sweep_decision_thresholds(&[1], &[true, false]),
            Err(GranmoModelError::PrbSweepLengthMismatch)
        );
    }

    // --- Artifact round-trip ---

    /// Unique-per-test temp path (tests run in parallel; a shared filename
    /// would race). `std::env::temp_dir()` is absolute, satisfying policy.
    fn temp_artifact_path(file_name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(file_name)
    }

    /// The central artifact guarantee: a trained model, saved and reloaded,
    /// must (a) pass full invariant validation, (b) carry its preprocessing
    /// profile, and (c) produce IDENTICAL vote sums on every probe document
    /// — behavioral bit-equivalence, not approximate agreement.
    #[test]
    fn artifact_round_trip_preserves_behavior_exactly() {
        let mut conv_engine = make_engine(5, 1, 16, false);
        let mut rng = FastRng::seed(11);
        let training_docs: [(&[u8], bool); 4] = [
            (b"very good movie", true),
            (b"not good movie", false),
            (b"it was very good", true),
            (b"it was not good", false),
        ];
        for _ in 0..50 {
            for (doc, label) in training_docs {
                conv_engine.conv_train_step(doc, label, &mut rng).unwrap();
            }
        }

        let artifact = ModelArtifact {
            preprocess_profile: PreprocessProfile::preset_p2(),
            engine: ClassifierEngine::ByteConv(conv_engine),
        };
        let path = temp_artifact_path("granmo_roundtrip_test.gmb");
        artifact.save_to_file(&path).unwrap();
        let loaded = ModelArtifact::load_from_file(&path).unwrap();

        assert_eq!(
            loaded.preprocess_profile.get_bits().unwrap(),
            PreprocessProfile::preset_p2().get_bits().unwrap()
        );
        loaded
            .engine
            .engine_validate_internal_consistency()
            .unwrap();

        let probe_docs: [&[u8]; 5] = [
            b"very good movie",
            b"not good movie",
            b"something unrelated",
            b"hi",
            b"",
        ];
        for doc in probe_docs {
            assert_eq!(
                artifact.engine.engine_vote_sum(doc).unwrap(),
                loaded.engine.engine_vote_sum(doc).unwrap(),
                "vote divergence after round-trip on {:?}",
                core::str::from_utf8(doc)
            );
        }
    }

    /// Corruption detection: flipping ONE payload byte must fail the
    /// checksum gate — the artifact never reaches the parser.
    /// (Kind-1 v2 layout keeps the state payload starting at offset 32,
    /// so byte 40 remains inside the states for this engine size.)
    #[test]
    fn artifact_detects_single_byte_corruption() {
        let artifact = ModelArtifact {
            preprocess_profile: PreprocessProfile::preset_raw(),
            engine: ClassifierEngine::ByteConv(make_engine(2, 1, 4, false)),
        };
        let path = temp_artifact_path("granmo_corruption_test.gmb");
        artifact.save_to_file(&path).unwrap();

        let mut bytes = std::fs::read(&path).unwrap();
        bytes[40] ^= 0x01; // flip one bit inside the state payload
        std::fs::write(&path, &bytes).unwrap();

        assert_eq!(
            ModelArtifact::load_from_file(&path).err(),
            Some(GranmoModelError::ArtChecksumMismatch)
        );
    }

    /// Wrong-magic and relative-path rejections: the two cheapest gates,
    /// checked before any parsing work is done.
    #[test]
    fn artifact_rejects_foreign_files_and_relative_paths() {
        let path = temp_artifact_path("granmo_not_an_artifact.gmb");
        // A file with valid checksum framing but wrong magic: build a body
        // of zeros plus a correct FNV trailer, so ONLY the magic gate trips.
        let body = vec![0u8; 64];
        let mut framed = body.clone();
        framed.extend_from_slice(&fnv1a_64(&body).to_le_bytes());
        std::fs::write(&path, &framed).unwrap();
        assert_eq!(
            ModelArtifact::load_from_file(&path).err(),
            Some(GranmoModelError::ArtMagicMismatch)
        );

        assert_eq!(
            ModelArtifact::load_from_file(std::path::Path::new("relative/path.gmb")).err(),
            Some(GranmoModelError::ArtPathNotAbsolute)
        );
    }

    /// Raw-threshold reconstruction must enforce the fixed-point invariant
    /// (sum == 65536, forget >= 1) exactly as `.get()` does — a tampered
    /// artifact header cannot smuggle in an invalid specificity.
    #[test]
    fn specificity_from_raw_thresholds_enforces_invariant() {
        assert!(SpecificityThresholds::from_raw_thresholds(16384, 49152).is_ok());
        assert_eq!(
            SpecificityThresholds::from_raw_thresholds(16384, 49151).err(),
            Some(GranmoModelError::CfgSpecificityRawThresholdsInvalid)
        );
        assert_eq!(
            SpecificityThresholds::from_raw_thresholds(0, 65535).err(),
            Some(GranmoModelError::CfgSpecificityRawThresholdsInvalid)
        );
    }

    /*
    Test-Note
    artifact_detects_single_byte_corruption flips byte 40, which lies
    in the state payload for the smallest test engine (header
    is 32 bytes; 4 clauses × 2×2×256 states = 4096 states = 8192 bytes of payload).
    If you ever shrink that test engine further,
    keep the flipped offset below 32 + 2 × state_count.
    */

    /// Splits must be deterministic under a seed (the §8 identical-split
    /// requirement) and reject degenerate geometries.
    #[test]
    fn test_split_dataset_is_seeded_deterministic_and_validated() {
        let documents: Vec<LabeledDocument> = (0..10)
            .map(|i| LabeledDocument {
                line_index: 0,
                text: vec![b'a' + i as u8],
                label_is_positive: i % 2 == 0,
            })
            .collect();

        let mut rng_a = FastRng::seed(5);
        let mut rng_b = FastRng::seed(5);
        let (train_a, test_a) = split_dataset(&documents, 80, &mut rng_a).unwrap();
        let (train_b, test_b) = split_dataset(&documents, 80, &mut rng_b).unwrap();
        assert_eq!(train_a.len(), 8);
        assert_eq!(test_a.len(), 2);
        let texts = |side: &[LabeledDocument]| -> Vec<Vec<u8>> {
            side.iter().map(|d| d.text.clone()).collect()
        };
        assert_eq!(texts(&train_a), texts(&train_b));
        assert_eq!(texts(&test_a), texts(&test_b));

        let mut rng_c = FastRng::seed(5);
        assert_eq!(
            split_dataset(&documents, 0, &mut rng_c).err(),
            Some(GranmoModelError::DsSplitRatioInvalid)
        );
        // 5% of 10 docs floors to 0 training docs -> empty side.
        assert_eq!(
            split_dataset(&documents, 5, &mut rng_c).err(),
            Some(GranmoModelError::DsSplitEmptySide)
        );
    }

    // --- End-to-end harness run ---

    /// Full-pipeline micro-run on the negation corpus: preprocess (P0) ->
    /// train -> evaluate -> sweep. Self-evaluation on the training docs is
    /// intentional here — this test verifies PLUMBING correctness, not
    /// generalization (the engine-level learning test covers learning).
    #[test]
    fn harness_end_to_end_on_negation_corpus() {
        let make_doc = |text: &[u8], positive: bool| LabeledDocument {
            line_index: 0,
            text: text.to_vec(),
            label_is_positive: positive,
        };
        let documents = vec![
            make_doc(b"very good movie", true),
            make_doc(b"very good story", true),
            make_doc(b"very good acting", true),
            make_doc(b"it was very good", true),
            make_doc(b"not good movie", false),
            make_doc(b"not good story", false),
            make_doc(b"not good acting", false),
            make_doc(b"it was not good", false),
        ];
        let config = HarnessRunConfig {
            profile: PreprocessProfile::preset_p0(),
            engine_selection: EngineSelection::ByteConv,
            patch_size: 5,
            stride: 1,
            bag_ngram_len: 5,
            bag_vocab_size: 4000,
            n_clauses: 32,
            vote_threshold: 15,
            states_per_action: 100,
            specificity: 3.0,
            max_scan_bytes: 256,
            guarded_include: false,
            fire_guard_streak_limit: 0,
            epochs: 400,
            seed: 42,
            worker_count: 2,
            score_train_side: false,
        };
        let (engine, report) = run_single_experiment(&documents, &documents, &config).unwrap();
        engine.engine_validate_internal_consistency().unwrap();
        assert_eq!(report.engine_name_reported, "byte-conv");
        assert_eq!(report.test_count, 8);
        assert_eq!(report.clause_fire_counts.len(), 32);
        assert!(
            report.accuracy_at_zero >= 0.875,
            "accuracy {} below tolerance",
            report.accuracy_at_zero
        );
        assert!(
            report.best_f1_row.f1 >= 0.85,
            "best F1 {}",
            report.best_f1_row.f1
        );
    }

    // --- CLI parsing ---

    fn cli(args: &[&str]) -> Result<CliArgs, GranmoModelError> {
        let mut full: Vec<String> = vec!["binary_name".to_string()];
        full.extend(args.iter().map(|s| s.to_string()));
        CliArgs::parse_cliargs(&full)
    }

    #[test]
    fn cli_parses_flags_and_fails_fast_on_bad_input() {
        let parsed = cli(&[
            "--mode",
            "train",
            "--clauses",
            "64",
            "--guarded",
            "--preset",
            "p2",
        ])
        .unwrap();
        assert_eq!(parsed.mode, "train");
        assert_eq!(parsed.n_clauses, 64);
        assert!(parsed.guarded_include);
        assert_eq!(parsed.preset_name, "p2");

        assert_eq!(
            cli(&["--nonsense"]).err(),
            Some(GranmoModelError::CliUnknownFlag)
        );
        assert_eq!(
            cli(&["--clauses"]).err(),
            Some(GranmoModelError::CliFlagMissingValue)
        );
        assert_eq!(
            cli(&["--clauses", "not_a_number"]).err(),
            Some(GranmoModelError::CliInvalidValue)
        );
        assert_eq!(
            preset_from_name("p9").err(),
            Some(GranmoModelError::CliUnknownPreset)
        );
    }
    /*
    Test Note
     (1) harness learning test uses a 0.875/0.85 tolerance
     rather than exactness because per-epoch shuffling consumes
     the RNG stream differently from the engine-level learning
     test — if it fails outright... tune it;
     (2) default --epochs 25 --clauses 200 on a
     real dataset will be slow in debug builds — always
     benchmark with cargo run --release.
     */
    // --- JSONL record extraction ---

    /// Convenience wrapper: parse one line with the fixed schema keys.
    fn jsonl_line(line: &str) -> Result<Option<(Vec<u8>, Vec<u8>)>, GranmoModelError> {
        parse_jsonl_record(line.as_bytes(), "text", "label")
    }

    #[test]
    fn jsonl_parses_basic_records_and_label_forms() {
        // Unquoted numeric label and quoted label must be byte-identical.
        let (text, label) = jsonl_line(r#"{"text":"hello","label":1}"#)
            .unwrap()
            .unwrap();
        assert_eq!(text, b"hello".to_vec());
        assert_eq!(label, b"1".to_vec());

        // Key order must not matter; extra whitespace tolerated.
        let (text, label) = jsonl_line(r#" { "label" : "1" , "text" : "x" } "#)
            .unwrap()
            .unwrap();
        assert_eq!(text, b"x".to_vec());
        assert_eq!(label, b"1".to_vec());
    }

    #[test]
    fn jsonl_decodes_simple_escapes() {
        let (text, _label) = jsonl_line(r#"{"text":"a\nb\t\"q\"\\/","label":1}"#)
            .unwrap()
            .unwrap();
        assert_eq!(text, b"a\nb\t\"q\"\\/".to_vec());
    }

    #[test]
    fn jsonl_decodes_unicode_escapes_and_surrogate_pairs() {
        // \u00e9 -> 'é' (2 UTF-8 bytes); the engine must see the SAME bytes
        // as an unescaped file would carry.
        let (text, _) = jsonl_line(r#"{"text":"caf\u00e9","label":1}"#)
            .unwrap()
            .unwrap();
        assert_eq!(text, "café".as_bytes().to_vec());

        // Surrogate pair: U+1F600 emoji -> F0 9F 98 80.
        let (text, _) = jsonl_line(r#"{"text":"\uD83D\uDE00","label":1}"#)
            .unwrap()
            .unwrap();
        assert_eq!(text, vec![0xF0, 0x9F, 0x98, 0x80]);

        // Raw (unescaped) UTF-8 passes through untouched.
        let (text, _) = jsonl_line("{\"text\":\"\u{1F600}\",\"label\":1}")
            .unwrap()
            .unwrap();
        assert_eq!(text, vec![0xF0, 0x9F, 0x98, 0x80]);
    }

    #[test]
    fn jsonl_rejects_bad_escapes_and_broken_surrogates() {
        assert_eq!(
            jsonl_line(r#"{"text":"a\xb","label":1}"#).err(),
            Some(GranmoModelError::DsJsonBadEscape)
        );
        // Lone high surrogate; low-first surrogate; non-hex digits.
        assert_eq!(
            jsonl_line(r#"{"text":"\uD83D","label":1}"#).err(),
            Some(GranmoModelError::DsJsonBadUnicodeEscape)
        );
        assert_eq!(
            jsonl_line(r#"{"text":"\uDE00\uD83D","label":1}"#).err(),
            Some(GranmoModelError::DsJsonBadUnicodeEscape)
        );
        assert_eq!(
            jsonl_line(r#"{"text":"\uZZZZ","label":1}"#).err(),
            Some(GranmoModelError::DsJsonBadUnicodeEscape)
        );
    }

    #[test]
    fn jsonl_skips_unknown_keys_including_nested_structures() {
        // Unknown fields with braces/brackets INSIDE strings are the trap
        // a naive skipper falls into; this line exercises exactly that.
        let line = r#"{"id":123,"meta":{"a":[1,2,{"b":"}"}],"c":"]}"},"text":"t","label":"1","flag":true}"#;
        let (text, label) = jsonl_line(line).unwrap().unwrap();
        assert_eq!(text, b"t".to_vec());
        assert_eq!(label, b"1".to_vec());
    }

    #[test]
    fn jsonl_skip_policy_drops_incomplete_records_without_error() {
        assert_eq!(jsonl_line(r#"{"label":1}"#).unwrap(), None); // no text
        assert_eq!(jsonl_line(r#"{"text":"x"}"#).unwrap(), None); // no label
        assert_eq!(jsonl_line(r#"{"text":"","label":1}"#).unwrap(), None); // empty text
        assert_eq!(jsonl_line(r#"{"text":null,"label":1}"#).unwrap(), None); // non-string text
        assert_eq!(jsonl_line(r#"{"text":"x","label":null}"#).unwrap(), None); // null label
        assert_eq!(jsonl_line("").unwrap(), None); // blank line
        assert_eq!(jsonl_line("   ").unwrap(), None); // whitespace-only line
        assert_eq!(jsonl_line("{}").unwrap(), None); // empty object
    }

    #[test]
    fn jsonl_rejects_structural_malformation() {
        assert_eq!(
            jsonl_line(r#"["text","label"]"#).err(),
            Some(GranmoModelError::DsJsonLineNotObject)
        );
        // A GENUINELY unterminated string: no closing quote before
        // end-of-line. (Note the trap this test previously fell into:
        // `"unterminated,"label"` is a TERMINATED string — the quote
        // before `label` closes it — and correctly yields the structural
        // code 712 below, not 709.)
        assert_eq!(
            jsonl_line(r#"{"text":"never closes"#).err(),
            Some(GranmoModelError::DsJsonUnterminatedString)
        );
        // Terminated string followed by garbage where ','/'}' is required:
        // structural malformation (the case that exposed the old test bug).
        assert_eq!(
            jsonl_line(r#"{"text":"unterminated,"label":1}"#).err(),
            Some(GranmoModelError::DsJsonMalformedStructure)
        );
        assert_eq!(
            jsonl_line(r#"{"text" "x","label":1}"#).err(),
            Some(GranmoModelError::DsJsonMalformedStructure)
        );
        assert_eq!(
            jsonl_line(r#"{"text":"x","label":1} extra"#).err(),
            Some(GranmoModelError::DsJsonMalformedStructure)
        );
    }

    #[test]
    fn jsonl_duplicate_keys_last_wins() {
        let (text, label) = jsonl_line(r#"{"text":"first","label":0,"text":"second","label":1}"#)
            .unwrap()
            .unwrap();
        assert_eq!(text, b"second".to_vec());
        assert_eq!(label, b"1".to_vec());
    }

    // --- JSONL file loading and format dispatch ---

    #[test]
    fn load_labeled_jsonl_end_to_end_with_skips() {
        let path = temp_artifact_path("granmo_dataset_test.jsonl");
        let content = concat!(
            r#"{"text":"you are great","label":0}"#,
            "\n",
            "\n", // blank line: skipped
            r#"{"text":"you suck","label":"1","source":"site_a"}"#,
            "\n",
            r#"{"label":1}"#,
            "\n", // no text: skipped
        );
        std::fs::write(&path, content).unwrap();

        let documents = load_labeled_jsonl(&path, "text", "label", "1").unwrap();
        assert_eq!(documents.len(), 2);
        assert!(!documents[0].label_is_positive);
        assert!(documents[1].label_is_positive);
        assert_eq!(documents[1].text, b"you suck".to_vec());

        // A structurally malformed line rejects the whole load.
        let bad_path = temp_artifact_path("granmo_dataset_bad.jsonl");
        std::fs::write(&bad_path, "{\"text\":\"x\",\"label\":1}\nnot json\n").unwrap();
        assert_eq!(
            load_labeled_jsonl(&bad_path, "text", "label", "1").err(),
            Some(GranmoModelError::DsJsonLineNotObject)
        );

        assert_eq!(
            load_labeled_jsonl(std::path::Path::new("relative.jsonl"), "text", "label", "1").err(),
            Some(GranmoModelError::DsPathNotAbsolute)
        );
    }

    // --- ByteBag vocabulary (Drop 2.2a) ---

    /// Builds a small vocabulary from string docs with n and M.
    fn build_vocab(n: u8, m: u16, docs: &[&[u8]]) -> ByteBagVocabulary {
        ByteBagVocabulary::build_from_documents(
            NgramLength::new(n).unwrap(),
            VocabSize::new(m).unwrap(),
            docs,
        )
        .unwrap()
    }

    #[test]
    fn vocabulary_ranking_is_count_desc_then_bytes_asc() {
        // "ababab" -> ab:3, ba:2; "ba" (short? no, len 2 = n) -> ba:3 total;
        // "aa" -> aa:1. Tie ab/ba at 3 breaks bytes-ascending: ab < ba.
        let vocab = build_vocab(2, 100, &[b"ababab", b"ba", b"aa"]);
        assert_eq!(vocab.vocabulary_len(), 3);
        assert_eq!(vocab.ngram_at_rank(0).unwrap(), b"ab");
        assert_eq!(vocab.ngram_at_rank(1).unwrap(), b"ba");
        assert_eq!(vocab.ngram_at_rank(2).unwrap(), b"aa");
        vocab.vocab_validity_recheck().unwrap();
    }

    #[test]
    fn vocabulary_truncates_to_top_m_and_misses_return_none() {
        let vocab = build_vocab(2, 2, &[b"ababab", b"ba", b"aa"]);
        assert_eq!(vocab.vocabulary_len(), 2);
        assert_eq!(vocab.lookup(b"ab").unwrap(), Some(0));
        assert_eq!(vocab.lookup(b"ba").unwrap(), Some(1));
        assert_eq!(vocab.lookup(b"aa").unwrap(), None); // truncated out
        assert_eq!(vocab.lookup(b"zz").unwrap(), None); // never seen
        // Wrong-length shingle is a wiring error, not a miss.
        assert_eq!(
            vocab.lookup(b"abc").err(),
            Some(GranmoModelError::BbgVocabIndexOutOfRange)
        );
    }

    #[test]
    fn vocabulary_construction_is_deterministic() {
        let docs: &[&[u8]] = &[b"the cat sat", b"the bat sat", b"a cat"];
        let vocab_a = build_vocab(3, 50, docs);
        let vocab_b = build_vocab(3, 50, docs);
        assert_eq!(vocab_a.ngram_flat_bytes, vocab_b.ngram_flat_bytes);
        assert_eq!(vocab_a.lookup_order, vocab_b.lookup_order);
    }

    #[test]
    fn short_documents_yield_one_padded_shingle() {
        // n=3, doc "hi" -> single shingle ['h','i',0x00] (PAD rule §10.4).
        let vocab = build_vocab(3, 10, &[b"hi"]);
        assert_eq!(vocab.vocabulary_len(), 1);
        assert_eq!(vocab.ngram_at_rank(0).unwrap(), &[b'h', b'i', 0x00]);
        // Extraction of the same short doc must hit that shingle.
        let bits = vocab.extract_presence_bits(b"hi", 1024).unwrap();
        assert_eq!(bits, vec![0b1u64]);
    }

    #[test]
    fn presence_bits_mark_exactly_occurring_shingles() {
        let vocab = build_vocab(2, 100, &[b"ababab", b"ba", b"aa"]);
        // "aab" contains shingles "aa" (rank 2) and "ab" (rank 0), not "ba".
        let bits = vocab.extract_presence_bits(b"aab", 1024).unwrap();
        assert_eq!(bits, vec![0b101u64]);
        // Scan cap: capping "aab" to 2 bytes leaves only "aa".
        let capped = vocab.extract_presence_bits(b"aab", 2).unwrap();
        assert_eq!(capped, vec![0b100u64]);
    }

    #[test]
    fn presence_is_order_free_the_bag_contrast_property() {
        // THE scientific contrast with the conv engine's order-sensitivity
        // test: documents that are byte-reversals produce IDENTICAL feature
        // sets whenever their shingle multisets coincide as sets.
        let vocab = build_vocab(2, 100, &[b"abba"]);
        // "abba" shingles: ab, bb, ba. "baab": ba, aa, ab — differs (bb vs aa),
        // so use the classic pair: "abab" (ab,ba,ab) vs "baba" (ba,ab,ba).
        let bits_forward = vocab.extract_presence_bits(b"abab", 1024).unwrap();
        let bits_reverse = vocab.extract_presence_bits(b"baba", 1024).unwrap();
        assert_eq!(bits_forward, bits_reverse);
    }

    #[test]
    fn empty_corpus_is_rejected() {
        let result = ByteBagVocabulary::build_from_documents(
            NgramLength::new(3).unwrap(),
            VocabSize::new(10).unwrap(),
            &[],
        );
        assert_eq!(result.err(), Some(GranmoModelError::BbgVocabEmptyCorpus));
    }

    #[test]
    fn ngram_length_and_vocab_size_bounds() {
        assert!(NgramLength::new(1).is_err());
        assert!(NgramLength::new(17).is_err());
        assert_eq!(NgramLength::new(5).unwrap().get().unwrap(), 5);
        assert!(VocabSize::new(1).is_err());
        assert!(VocabSize::new(65001).is_err());
        assert_eq!(VocabSize::new(4000).unwrap().get().unwrap(), 4000);
    }

    /// Hand-computed feedback gates, including both clamp directions.
    /// These values define the (T∓V)/2T rule in integer form for BOTH
    /// engines; ByteBagTM (Drop 2.2b) trains through this same helper.
    #[test]
    fn feedback_gates_match_hand_computation_including_clamp() {
        // T = 15, V = 5: target gate = 10, other gate = 20, draw range 30.
        let gates = resolve_feedback_gates(5, 15).unwrap();
        assert_eq!(gates.gate_when_target, 10);
        assert_eq!(gates.gate_when_other, 20);
        assert_eq!(gates.two_t, 30);

        // V far beyond +T clamps to +T: the label-consistent side is
        // saturated (gate 0 — no further pushing), the correcting side
        // gets maximal pressure (gate 2T).
        let gates = resolve_feedback_gates(100, 15).unwrap();
        assert_eq!(gates.gate_when_target, 0);
        assert_eq!(gates.gate_when_other, 30);

        // Symmetric clamp on the negative side.
        let gates = resolve_feedback_gates(-100, 15).unwrap();
        assert_eq!(gates.gate_when_target, 30);
        assert_eq!(gates.gate_when_other, 0);
    }

    /// v2 prelude gates: wrong version and unknown kind must be rejected by
    /// their specific codes, with valid checksum framing so ONLY the gate
    /// under test trips.
    #[test]
    fn artifact_rejects_unsupported_version_and_kind() {
        // Version gate.
        let version_path = temp_artifact_path("granmo_bad_version.gmb");
        let mut body = Vec::new();
        body.extend_from_slice(&ARTIFACT_MAGIC);
        body.extend_from_slice(&99u16.to_le_bytes());
        body.extend_from_slice(&[0u8; 4]); // kind/reserved/profile filler
        let mut framed = body.clone();
        framed.extend_from_slice(&fnv1a_64(&body).to_le_bytes());
        std::fs::write(&version_path, &framed).unwrap();
        assert_eq!(
            ModelArtifact::load_from_file(&version_path).err(),
            Some(GranmoModelError::ArtVersionUnsupported)
        );

        // Kind gate.
        let kind_path = temp_artifact_path("granmo_bad_kind.gmb");
        let mut body = Vec::new();
        body.extend_from_slice(&ARTIFACT_MAGIC);
        body.extend_from_slice(&ARTIFACT_FORMAT_VERSION.to_le_bytes());
        body.push(200u8); // no such kind
        body.push(0u8); // reserved
        body.extend_from_slice(&0u16.to_le_bytes()); // profile bits: raw
        let mut framed = body.clone();
        framed.extend_from_slice(&fnv1a_64(&body).to_le_bytes());
        std::fs::write(&kind_path, &framed).unwrap();
        assert_eq!(
            ModelArtifact::load_from_file(&kind_path).err(),
            Some(GranmoModelError::ArtKindUnsupported)
        );
    }

    /// Enum dispatch identity: the variant reports the right name and
    /// artifact kind, and delegates to the wrapped engine.
    #[test]
    fn classifier_engine_dispatch_identity() {
        let wrapped = ClassifierEngine::ByteConv(make_engine(5, 2, 8, false));
        assert_eq!(wrapped.engine_name(), "byte-conv");
        assert_eq!(
            wrapped.artifact_kind(),
            ARTIFACT_KIND_BYTECONV_FULL_TRAINING
        );
        assert_eq!(wrapped.engine_clause_count(), 8);
        // Fresh model: all clauses fire, balanced polarity => V = 0.
        assert_eq!(wrapped.engine_vote_sum(b"any text").unwrap(), 0);
        wrapped.engine_validate_internal_consistency().unwrap();
    }

    // --- ByteBagTM engine (Drop 2.2b) ---

    /// Builds a ByteBagTM over a vocabulary constructed from `corpus`.
    /// Fixed harness defaults (T=15, N=100, s=3.0, scan cap 256) mirror
    /// `make_engine` on the conv side; fire guard DISABLED (the recorded
    /// baseline). Unique name per project rule.
    fn make_bag_engine(
        ngram_len_value: u8,
        vocab_cap: u16,
        corpus: &[&[u8]],
        clause_count_value: u16,
    ) -> ByteBagTM {
        make_bag_engine_with_guard(
            ngram_len_value,
            vocab_cap,
            corpus,
            clause_count_value,
            FireGuardStreakLimit::DISABLED,
        )
    }

    /// Guard-configurable sibling of `make_bag_engine` (Drop 4.1 tests).
    fn make_bag_engine_with_guard(
        ngram_len_value: u8,
        vocab_cap: u16,
        corpus: &[&[u8]],
        clause_count_value: u16,
        fire_guard_limit_value: u32,
    ) -> ByteBagTM {
        let built_vocabulary = ByteBagVocabulary::build_from_documents(
            NgramLength::new(ngram_len_value).unwrap(),
            VocabSize::new(vocab_cap).unwrap(),
            corpus,
        )
        .unwrap();
        ByteBagTM::new_with_vocabulary(
            built_vocabulary,
            ClauseCount::new(clause_count_value).unwrap(),
            VoteThreshold::new(15).unwrap(),
            StatesPerAction::new(100).unwrap(),
            SpecificityThresholds::from_specificity(3.0).unwrap(),
            MaxScanBytes::new(256).unwrap(),
            FireGuardStreakLimit::new(fire_guard_limit_value).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn bag_fresh_engine_fires_everywhere_and_votes_zero() {
        // Fresh clauses include no literals => all-zero masks => vacuous
        // conjunction fires on everything; balanced polarity => V = 0.
        // Mirrors the conv engine's fresh invariant exactly.
        let bag_engine = make_bag_engine(2, 100, &[b"ababab", b"aa"], 8);
        assert_eq!(bag_engine.bag_vote_sum(b"anything at all").unwrap(), 0);
        assert!(!bag_engine.bag_predict(b"anything at all", 0).unwrap());
        bag_engine.bag_validate_internal_consistency().unwrap();
        let fired_words = bag_engine.bag_fired_clause_bits(b"x").unwrap();
        assert_eq!(fired_words, vec![0b1111_1111u64]);
    }

    #[test]
    fn bag_positive_literal_fires_regardless_of_byte_order() {
        // THE engine-level scientific contrast with the conv engine's
        // order-sensitivity test (`positive_include_restricts_slot...`,
        // where "zab" fires and "zba" does not). A bag clause requiring
        // shingle "ab" fires on BOTH "abab" and "baba" — position and
        // order are invisible to it — and the full fired bitsets of the
        // two documents are identical.
        // Vocabulary from ["abab", "ba"]: counts ab:2, ba:2; tie breaks
        // bytes-ascending => "ab" = rank 0, "ba" = rank 1 (pinned below).
        let mut bag_engine = make_bag_engine(2, 100, &[b"abab", b"ba"], 2);
        assert_eq!(bag_engine.bag_vocabulary_ref().vocabulary_len(), 2);
        assert_eq!(
            bag_engine.bag_vocabulary_ref().ngram_at_rank(0).unwrap(),
            b"ab"
        );

        // Clause 0: require shingle "ab" present (positive literal rank 0).
        bag_engine.bag_test_force_include(0, 0);
        bag_engine.bag_validate_internal_consistency().unwrap();

        let fired_forward = bag_engine.bag_fired_clause_bits(b"abab").unwrap();
        let fired_reversed = bag_engine.bag_fired_clause_bits(b"baba").unwrap();
        assert_eq!(fired_forward[0] & 0b1, 1, "clause 0 must fire on abab");
        assert_eq!(fired_forward, fired_reversed, "bag must be order-blind");

        // "bb" contains no vocabulary shingle => required "ab" absent =>
        // clause 0 must NOT fire.
        let fired_miss = bag_engine.bag_fired_clause_bits(b"bb").unwrap();
        assert_eq!(fired_miss[0] & 0b1, 0);
    }

    #[test]
    fn bag_negated_literal_blocks_on_presence() {
        // Clause 0: forbid shingle "ba" (negated literal = local index
        // feature_count + rank = 2 + 1 = 3). Fires only when "ba" absent.
        let mut bag_engine = make_bag_engine(2, 100, &[b"abab", b"ba"], 2);
        assert_eq!(bag_engine.bag_vocabulary_ref().vocabulary_len(), 2);
        assert_eq!(
            bag_engine.bag_vocabulary_ref().ngram_at_rank(1).unwrap(),
            b"ba"
        );

        bag_engine.bag_test_force_include(0, 2 + 1);
        bag_engine.bag_validate_internal_consistency().unwrap();

        // "ab": only shingle "ab" => "ba" absent => fires.
        let fired_clean = bag_engine.bag_fired_clause_bits(b"ab").unwrap();
        assert_eq!(fired_clean[0] & 0b1, 1);
        // "aba": shingles "ab" and "ba" => forbidden "ba" present => blocked.
        let fired_blocked = bag_engine.bag_fired_clause_bits(b"aba").unwrap();
        assert_eq!(fired_blocked[0] & 0b1, 0);
    }

    #[test]
    fn bag_short_document_uses_padded_shingle() {
        // n=3, corpus ["hi"] => single vocabulary shingle ['h','i',0x00]
        // (PAD rule §10.4). A clause requiring it fires on the short doc
        // "hi" and not on a long doc with no vocabulary shingles.
        let mut bag_engine = make_bag_engine(3, 10, &[b"hi"], 2);
        assert_eq!(bag_engine.bag_vocabulary_ref().vocabulary_len(), 1);
        bag_engine.bag_test_force_include(0, 0);
        bag_engine.bag_validate_internal_consistency().unwrap();

        let fired_short = bag_engine.bag_fired_clause_bits(b"hi").unwrap();
        assert_eq!(fired_short[0] & 0b1, 1);
        let fired_long = bag_engine.bag_fired_clause_bits(b"hello").unwrap();
        assert_eq!(fired_long[0] & 0b1, 0);
    }

    #[test]
    fn bag_invariants_survive_stochastic_training() {
        let corpus: &[&[u8]] = &[b"abcdefg", b"gfedcba", b"hi", b"aaaaaaaa"];
        let mut bag_engine = make_bag_engine(2, 100, corpus, 16);
        let mut rng = FastRng::seed(7);
        let training_docs: [(&[u8], bool); 5] = [
            (b"abcdefg", true),
            (b"gfedcba", false),
            (b"hi", true), // short doc: padded-shingle path
            (b"", false),  // empty doc: all-PAD shingle must train cleanly
            (b"aaaaaaaa", true),
        ];
        for _ in 0..50 {
            for (doc, label) in training_docs {
                bag_engine.bag_train_step(doc, label, &mut rng).unwrap();
            }
        }
        bag_engine.bag_validate_internal_consistency().unwrap();
    }

    #[test]
    fn bag_learns_negation_micro_corpus() {
        // The SAME negation micro-corpus as the conv engine's learning
        // test. Recorded prediction (Session 2 §3, locked): the bag SHOULD
        // pass — "not g" and "very " are themselves distinguishing
        // shingles. The suites where the bag is predicted to LOSE are the
        // positional/word-boundary minimal pairs (robustness suites, next
        // roadmap item) — not this corpus.
        let positives: [&[u8]; 4] = [
            b"very good movie",
            b"very good story",
            b"very good acting",
            b"it was very good",
        ];
        let negatives: [&[u8]; 4] = [
            b"not good movie",
            b"not good story",
            b"not good acting",
            b"it was not good",
        ];
        let corpus: Vec<&[u8]> = positives.iter().chain(negatives.iter()).copied().collect();
        let mut bag_engine = make_bag_engine(5, 200, &corpus, 32);
        let mut rng = FastRng::seed(42);
        for _ in 0..300 {
            for doc in positives {
                bag_engine.bag_train_step(doc, true, &mut rng).unwrap();
            }
            for doc in negatives {
                bag_engine.bag_train_step(doc, false, &mut rng).unwrap();
            }
        }
        bag_engine.bag_validate_internal_consistency().unwrap();

        for doc in positives {
            assert!(
                bag_engine.bag_predict(doc, 0).unwrap(),
                "positive doc misclassified: {:?}",
                core::str::from_utf8(doc)
            );
        }
        for doc in negatives {
            assert!(
                !bag_engine.bag_predict(doc, 0).unwrap(),
                "negative doc misclassified: {:?}",
                core::str::from_utf8(doc)
            );
        }
    }
    /*
    Test-Note (bag_learns_negation_micro_corpus):
    This is the second stochastic-outcome test in the suite (the first is
    the conv engine's learning test). Deterministic under seed 42, but the
    pass depends on hyperparameters I cannot execute here. If it fails on
    your machine, report which docs misclassify and their bag_vote_sum
    values; first knobs are epochs 300→500 and clauses 32→48 — tune against
    observed votes, do not weaken the invariant tests.
    */

    #[test]
    fn bag_describe_clause_decodes_forced_pattern() {
        let mut bag_engine = make_bag_engine(2, 100, &[b"abab", b"ba"], 2);
        assert_eq!(bag_engine.bag_vocabulary_ref().vocabulary_len(), 2);
        bag_engine.bag_test_force_include(0, 0); // has "ab" (positive rank 0)
        bag_engine.bag_test_force_include(0, 2 + 1); // lacks "ba" (negated rank 1)
        let description = bag_engine.bag_describe_clause(0, 12).unwrap();
        assert!(description.contains("has \"ab\""), "got: {description}");
        assert!(description.contains("lacks \"ba\""), "got: {description}");
    }

    // --- ByteBag integration: enum dispatch + artifact kind 3 (Drop 2.2c-i) ---

    /// Enum dispatch identity for the ByteBag variant: right name, right
    /// artifact kind, and delegation to the `bag_`-prefixed engine surface.
    #[test]
    fn bag_classifier_engine_dispatch_identity() {
        let wrapped_bag =
            ClassifierEngine::ByteBag(make_bag_engine(2, 100, &[b"ababab", b"aa"], 8));
        assert_eq!(wrapped_bag.engine_name(), "byte-bag");
        assert_eq!(
            wrapped_bag.artifact_kind(),
            ARTIFACT_KIND_BYTEBAG_FULL_TRAINING
        );
        assert_eq!(wrapped_bag.engine_clause_count(), 8);
        // Fresh bag: all clauses fire, balanced polarity => V = 0.
        assert_eq!(wrapped_bag.engine_vote_sum(b"any text").unwrap(), 0);
        wrapped_bag.engine_validate_internal_consistency().unwrap();
        let lut_for_bag = wrapped_bag.engine_build_probability_lut().unwrap();
        lut_for_bag.lut_validity_recheck().unwrap();
    }

    /// The kind-3 artifact guarantee, mirroring the kind-1 round-trip test:
    /// vocabulary AND states persist; the loaded model passes all four load
    /// gates and votes IDENTICALLY on every probe document.
    #[test]
    fn bag_artifact_round_trip_preserves_behavior_exactly() {
        let positives: [&[u8]; 2] = [b"very good movie", b"it was very good"];
        let negatives: [&[u8]; 2] = [b"not good movie", b"it was not good"];
        let corpus: Vec<&[u8]> = positives.iter().chain(negatives.iter()).copied().collect();
        let mut trained_bag = make_bag_engine(5, 200, &corpus, 16);
        let mut rng = FastRng::seed(11);
        for _ in 0..50 {
            for doc in positives {
                trained_bag.bag_train_step(doc, true, &mut rng).unwrap();
            }
            for doc in negatives {
                trained_bag.bag_train_step(doc, false, &mut rng).unwrap();
            }
        }

        let artifact = ModelArtifact {
            preprocess_profile: PreprocessProfile::preset_p0(),
            engine: ClassifierEngine::ByteBag(trained_bag),
        };
        let path = temp_artifact_path("granmo_bag_roundtrip_test.gmb");
        artifact.save_to_file(&path).unwrap();
        let loaded = ModelArtifact::load_from_file(&path).unwrap();

        assert_eq!(loaded.engine.engine_name(), "byte-bag");
        assert_eq!(
            loaded.preprocess_profile.get_bits().unwrap(),
            PreprocessProfile::preset_p0().get_bits().unwrap()
        );
        loaded
            .engine
            .engine_validate_internal_consistency()
            .unwrap();

        let probe_docs: [&[u8]; 5] = [
            b"very good movie",
            b"not good movie",
            b"something unrelated",
            b"hi",
            b"",
        ];
        for doc in probe_docs {
            assert_eq!(
                artifact.engine.engine_vote_sum(doc).unwrap(),
                loaded.engine.engine_vote_sum(doc).unwrap(),
                "bag vote divergence after round-trip on {:?}",
                core::str::from_utf8(doc)
            );
        }
    }

    // --- Engine selection, leakage guard, fire-rate (Drop 2.2c-ii) ---

    /// Hand-computed vote derivation from fired bits: bits 0b1011 over 4
    /// clauses = clauses 0 (+), 1 (−), 3 (−) fired => V = 1 − 1 − 1 = −1.
    #[test]
    fn vote_from_fired_words_matches_hand_computation() {
        assert_eq!(vote_from_fired_words(&[0b1011u64], 4).unwrap(), -1);
        assert_eq!(vote_from_fired_words(&[0u64], 4).unwrap(), 0);
        // Missing bitset word storage is an internal fault, never silence.
        assert_eq!(
            vote_from_fired_words(&[], 1).err(),
            Some(GranmoModelError::CliFireRateReportInternalFault)
        );
    }

    /// The leakage guard of record: a shingle occurring ONLY in test
    /// documents must be absent from the ByteBag vocabulary, while a
    /// training-split shingle is present.
    #[test]
    fn harness_vocabulary_built_from_training_split_only() {
        let train_documents = vec![
            LabeledDocument {
                line_index: 0,
                text: b"aaaaa bbbbb".to_vec(),
                label_is_positive: true,
            },
            LabeledDocument {
                line_index: 0,
                text: b"ccccc ddddd".to_vec(),
                label_is_positive: false,
            },
        ];
        let test_documents = vec![LabeledDocument {
            line_index: 0,
            text: b"zzzzz aaaaa".to_vec(),
            label_is_positive: true,
        }];
        let config = HarnessRunConfig {
            profile: PreprocessProfile::preset_raw(),
            engine_selection: EngineSelection::ByteBag,
            patch_size: 5, // ignored by bag
            stride: 2,     // ignored by bag
            bag_ngram_len: 5,
            bag_vocab_size: 100,
            n_clauses: 4,
            vote_threshold: 15,
            states_per_action: 100,
            specificity: 3.0,
            max_scan_bytes: 256,
            guarded_include: false, // ignored by bag
            fire_guard_streak_limit: 0,
            epochs: 1,
            seed: 42,
            worker_count: 2,
            score_train_side: false,
        };
        let (trained_engine, report) =
            run_single_experiment(&train_documents, &test_documents, &config).unwrap();
        assert_eq!(report.engine_name_reported, "byte-bag");
        match trained_engine {
            ClassifierEngine::ByteBag(bag_engine) => {
                let vocabulary_view = bag_engine.bag_vocabulary_ref();
                assert!(vocabulary_view.lookup(b"aaaaa").unwrap().is_some());
                assert!(vocabulary_view.lookup(b"zzzzz").unwrap().is_none());
            }
            ClassifierEngine::ByteConv(_) => panic!("expected the ByteBag engine"),
            ClassifierEngine::SeqFreqHybrid(_) => {
                panic!("expected the ByteBag engine, got SeqFreqHybrid")
            }
        }
    }

    /// Full-pipeline micro-run for the ByteBag path through the SHARED
    /// harness (preprocess -> vocabulary from train side -> train ->
    /// evaluate -> sweep -> fire-rate). Plumbing verification, mirroring
    /// `harness_end_to_end_on_negation_corpus`.
    #[test]
    fn harness_bag_engine_end_to_end_on_negation_corpus() {
        let make_doc = |text: &[u8], positive: bool| LabeledDocument {
            line_index: 0,
            text: text.to_vec(),
            label_is_positive: positive,
        };
        let documents = vec![
            make_doc(b"very good movie", true),
            make_doc(b"very good story", true),
            make_doc(b"very good acting", true),
            make_doc(b"it was very good", true),
            make_doc(b"not good movie", false),
            make_doc(b"not good story", false),
            make_doc(b"not good acting", false),
            make_doc(b"it was not good", false),
        ];
        let config = HarnessRunConfig {
            profile: PreprocessProfile::preset_p0(),
            engine_selection: EngineSelection::ByteBag,
            patch_size: 5, // ignored by bag
            stride: 1,     // ignored by bag
            bag_ngram_len: 5,
            bag_vocab_size: 200,
            n_clauses: 32,
            vote_threshold: 15,
            states_per_action: 100,
            specificity: 3.0,
            max_scan_bytes: 256,
            guarded_include: false, // ignored by bag
            fire_guard_streak_limit: 0,
            epochs: 400,
            seed: 42,
            worker_count: 2,
            score_train_side: false,
        };
        let (trained_engine, report) =
            run_single_experiment(&documents, &documents, &config).unwrap();
        trained_engine
            .engine_validate_internal_consistency()
            .unwrap();
        assert_eq!(report.engine_name_reported, "byte-bag");
        assert_eq!(report.test_count, 8);
        assert_eq!(report.clause_fire_counts.len(), 32);
        assert!(
            report
                .clause_fire_counts
                .iter()
                .all(|&count| count as usize <= report.test_count),
            "a fire count exceeded the test-document count"
        );
        assert!(
            report.accuracy_at_zero >= 0.875,
            "bag accuracy {} below tolerance",
            report.accuracy_at_zero
        );
        assert!(
            report.best_f1_row.f1 >= 0.85,
            "bag best F1 {}",
            report.best_f1_row.f1
        );
    }
    /*
    Test-Note (harness_bag_engine_end_to_end_on_negation_corpus):
    Stochastic-outcome test, deterministic under seed 42 but hyperparameter-
    dependent (the third such test, after the two engine learning tests).
    If it fails on your machine, report accuracy/F1 and which docs
    misclassify; first knobs are epochs 400→600 and clauses 32→48. Tune
    against observed values; do not weaken invariant tests.
    */

    /// CLI: engine selection and bag flags parse; unknown engine fails fast.
    #[test]
    fn cli_parses_engine_and_bag_flags() {
        let parsed = cli(&[
            "--mode",
            "train",
            "--engine",
            "byte-bag",
            "--ngram-len",
            "4",
            "--vocab-size",
            "1000",
        ])
        .unwrap();
        assert_eq!(parsed.engine_selection, EngineSelection::ByteBag);
        assert_eq!(parsed.bag_ngram_len, 4);
        assert_eq!(parsed.bag_vocab_size, 1000);

        // Default engine is byte-conv.
        let defaulted = cli(&["--mode", "train"]).unwrap();
        assert_eq!(defaulted.engine_selection, EngineSelection::ByteConv);

        assert_eq!(
            cli(&["--engine", "nonsense"]).err(),
            Some(GranmoModelError::CliUnknownEngine)
        );
    }

    // --- Misprediction inspection log ---

    /// `run_single_experiment` must populate `mispredictions` with exactly
    /// the wrong-at-V>0 documents, carrying BOTH raw and preprocessed text.
    /// Uses an untrained fresh model (epochs=0-equivalent via 1 epoch on a
    /// single doc) where behavior is fully predictable: a fresh balanced
    /// bank votes V=0 on everything, and V>0 is false, so every
    /// positive-labeled test doc is a guaranteed misprediction.
    #[test]
    fn experiment_report_captures_mispredictions_with_both_text_forms() {
        let train_docs = vec![LabeledDocument {
            line_index: 0,
            text: b"anything".to_vec(),
            label_is_positive: false,
        }];
        let test_docs = vec![
            LabeledDocument {
                line_index: 0,
                text: b"  POSITIVE Doc".to_vec(),
                label_is_positive: true, // fresh model predicts 0 -> misprediction
            },
            LabeledDocument {
                line_index: 0,
                text: b"negative doc".to_vec(),
                label_is_positive: false, // fresh model predicts 0 -> correct
            },
        ];
        let config = HarnessRunConfig {
            profile: PreprocessProfile::preset_p0(),
            engine_selection: EngineSelection::ByteConv,
            patch_size: 5,
            stride: 2,
            bag_ngram_len: 5,
            bag_vocab_size: 100,
            n_clauses: 4,
            vote_threshold: 15,
            states_per_action: 100,
            specificity: 3.0,
            max_scan_bytes: 256,
            guarded_include: false,
            fire_guard_streak_limit: 0,
            epochs: 0, // no training: model stays fresh and vote stays 0
            seed: 42,
            worker_count: 2,
            score_train_side: false,
        };
        let (_engine, report) = run_single_experiment(&train_docs, &test_docs, &config).unwrap();
        assert_eq!(report.mispredictions.len(), 1);
        let record = &report.mispredictions[0];
        assert_eq!(record.raw_text_bytes, b"  POSITIVE Doc".to_vec());
        // P0 = fold + dedupe + trim + lowercase.
        assert_eq!(record.preprocessed_text_bytes, b"positive doc".to_vec());
        assert!(record.actual_label_is_positive);
        assert!(!record.predicted_label_is_positive);
        assert_eq!(record.vote_sum_at_prediction, 0);
    }

    /// Append helper contract: empty input is a no-op (no file created);
    /// non-empty input appends; a second call PRESERVES the first call's
    /// lines (append mode); text separators are sanitized; relative paths
    /// are rejected with the dedicated code.
    #[test]
    fn misprediction_log_append_contract() {
        let log_path = temp_artifact_path("granmo_mispred_log_test.txt");
        let _ = std::fs::remove_file(&log_path); // clean slate; ignore absence
        let data_path = std::path::PathBuf::from("/data/example.jsonl");

        // Empty: no-op, no file.
        append_mispredictions_to_log(&log_path, &data_path, "p0", "byte-conv", &[]).unwrap();
        assert!(!log_path.exists());

        // Relative path: rejected.
        assert_eq!(
            append_mispredictions_to_log(
                std::path::Path::new("relative/log.txt"),
                &data_path,
                "p0",
                "byte-conv",
                &[MispredictionRecord {
                    raw_text_bytes: b"x".to_vec(),
                    preprocessed_text_bytes: b"x".to_vec(),
                    actual_label_is_positive: true,
                    predicted_label_is_positive: false,
                    vote_sum_at_prediction: -1,
                }],
            )
            .err(),
            Some(GranmoModelError::CliLogPathNotAbsolute)
        );

        // Two appends accumulate; embedded newline/tab sanitized to spaces.
        let record = MispredictionRecord {
            raw_text_bytes: b"line one\nline\ttwo".to_vec(),
            preprocessed_text_bytes: b"line one line two".to_vec(),
            actual_label_is_positive: true,
            predicted_label_is_positive: false,
            vote_sum_at_prediction: -3,
        };
        append_mispredictions_to_log(&log_path, &data_path, "p0", "byte-conv", &[record.clone()])
            .unwrap();
        append_mispredictions_to_log(&log_path, &data_path, "p2", "byte-bag", &[record]).unwrap();

        let contents = std::fs::read_to_string(&log_path).unwrap();
        let lines: Vec<&str> = contents.lines().collect();
        assert_eq!(lines.len(), 2, "append mode must preserve prior records");
        assert!(lines[0].contains("run=p0") && lines[0].contains("engine=byte-conv"));
        assert!(lines[1].contains("run=p2") && lines[1].contains("engine=byte-bag"));
        assert!(lines[0].contains("raw_text=\"line one line two\""));
        assert!(lines[0].contains("actual=1") && lines[0].contains("pred=0"));
        assert!(lines[0].contains("vote=-3"));
    }

    /// CLI: `--log-out` parses into `log_out`; absent flag defaults None.
    #[test]
    fn cli_parses_log_out_flag() {
        let parsed = cli(&["--mode", "train", "--log-out", "/tmp/mispred.txt"]).unwrap();
        assert_eq!(
            parsed.log_out,
            Some(std::path::PathBuf::from("/tmp/mispred.txt"))
        );
        assert!(cli(&["--mode", "train"]).unwrap().log_out.is_none());
    }

    /*
    Test Note:
    the `epochs: 0` test relies on `run_single_experiment` accepting
    zero epochs (the loop `for _epoch in 0..config.epochs`
    doesn't execute — it does today). If you ever add a `epochs >= 1`
    validation, switch that test to a hand-forced clause construction
    via `conv_test_force_include` instead.
    */

    /// THE parallelization contract, executable: clause-parallel training
    /// is a PERFORMANCE-ONLY transformation. One and many workers must
    /// produce byte-identical automaton states after identical training.
    #[test]
    fn conv_training_is_worker_count_invariant() {
        let training_docs: [(&[u8], bool); 5] = [
            (b"very good movie", true),
            (b"not good movie", false),
            (b"hi", true),
            (b"", false),
            (b"aaaaaaaaaaaaaaaa", true),
        ];
        let run_training = |workers: u16| -> ByteConvTM {
            let mut engine = make_engine(5, 1, 16, false);
            let mut rng = FastRng::seed(123);
            let worker_count = WorkerCount::new(workers).unwrap();
            for _ in 0..20 {
                for (doc, label) in training_docs {
                    engine
                        .conv_train_step_with_workers(doc, *&label, &mut rng, worker_count)
                        .unwrap();
                }
            }
            engine.conv_validate_internal_consistency().unwrap();
            engine
        };
        let single = run_training(1);
        let many = run_training(8);
        assert_eq!(single.ta_states, many.ta_states, "states diverged");
        assert_eq!(single.allowed_masks, many.allowed_masks, "masks diverged");
        assert_eq!(
            single.positive_include_counts, many.positive_include_counts,
            "counts diverged"
        );
    }

    /// Same contract for the ByteBag control engine.
    #[test]
    fn bag_training_is_worker_count_invariant() {
        let corpus: &[&[u8]] = &[b"very good movie", b"not good movie", b"hi"];
        let training_docs: [(&[u8], bool); 3] = [
            (b"very good movie", true),
            (b"not good movie", false),
            (b"hi", true),
        ];
        let run_training = |workers: u16| -> ByteBagTM {
            let mut engine = make_bag_engine(3, 100, corpus, 16);
            let mut rng = FastRng::seed(123);
            let worker_count = WorkerCount::new(workers).unwrap();
            for _ in 0..20 {
                for (doc, label) in training_docs {
                    engine
                        .bag_train_step_with_workers(doc, *&label, &mut rng, worker_count)
                        .unwrap();
                }
            }
            engine.bag_validate_internal_consistency().unwrap();
            engine
        };
        let single = run_training(1);
        let many = run_training(8);
        assert_eq!(single.bag_ta_states, many.bag_ta_states, "states diverged");
        assert_eq!(
            single.bag_positive_include_masks, many.bag_positive_include_masks,
            "positive masks diverged"
        );
        assert_eq!(
            single.bag_negated_include_masks, many.bag_negated_include_masks,
            "negated masks diverged"
        );
    }

    // --- Fire-streak guard (Drop 4.1) ---

    /// Newtype bounds: 0 (disabled) and the active range are accepted;
    /// sub-minimum active values and over-max values are rejected.
    #[test]
    fn fire_guard_limit_bounds() {
        assert!(FireGuardStreakLimit::new(0).is_ok());
        assert!(FireGuardStreakLimit::new(16).is_ok());
        assert!(FireGuardStreakLimit::new(16_777_216).is_ok());
        assert_eq!(
            FireGuardStreakLimit::new(15).err(),
            Some(GranmoModelError::CfgFireGuardLimitOutOfBounds)
        );
        assert_eq!(
            FireGuardStreakLimit::new(1).err(),
            Some(GranmoModelError::CfgFireGuardLimitOutOfBounds)
        );
        assert_eq!(
            FireGuardStreakLimit::new(16_777_217).err(),
            Some(GranmoModelError::CfgFireGuardLimitOutOfBounds)
        );
    }

    /// The vacuous exemption at unit level: a fresh clause (zero
    /// includes) fed `fired = true` far past the limit must never
    /// accumulate a streak and never reset — the bootstrap state is
    /// exempt by specification.
    #[test]
    fn bag_fire_guard_exempts_vacuous_clauses() {
        let mut bag_engine = make_bag_engine_with_guard(2, 100, &[b"ababab"], 2, 16);
        {
            let mut clause_view = bag_engine._test_bag_build_clause_view(0).unwrap();
            for _ in 0..100 {
                assert!(!clause_view.view_bag_apply_fire_guard(true).unwrap());
            }
            assert_eq!(*clause_view.fire_streak, 0);
            assert_eq!(*clause_view.fire_guard_reset_count, 0);
        }
        bag_engine.bag_validate_internal_consistency().unwrap();
    }

    /// The streak is CONSECUTIVE by specification: one non-fire zeroes
    /// it, and only an unbroken run of `limit` fires (while non-vacuous)
    /// triggers the reset. Also verifies the reset restores the fresh
    /// invariant exactly (all states == N, both masks zero).
    #[test]
    fn bag_fire_guard_streak_breaks_on_any_non_fire_and_resets_at_limit() {
        let mut bag_engine = make_bag_engine_with_guard(2, 100, &[b"ababab"], 2, 16);
        bag_engine.bag_test_force_include(0, 0); // non-vacuous: require "ab"
        {
            let mut clause_view = bag_engine._test_bag_build_clause_view(0).unwrap();
            for _ in 0..15 {
                assert!(!clause_view.view_bag_apply_fire_guard(true).unwrap());
            }
            assert_eq!(*clause_view.fire_streak, 15);
            // One non-fire zeroes the streak.
            assert!(!clause_view.view_bag_apply_fire_guard(false).unwrap());
            assert_eq!(*clause_view.fire_streak, 0);
            // A fresh run of 15 does not reach the limit...
            for _ in 0..15 {
                assert!(!clause_view.view_bag_apply_fire_guard(true).unwrap());
            }
            // ...and the 16th consecutive fire triggers the reset.
            assert!(clause_view.view_bag_apply_fire_guard(true).unwrap());
            assert_eq!(*clause_view.fire_streak, 0);
            assert_eq!(*clause_view.fire_guard_reset_count, 1);
            assert!(clause_view.states.iter().all(|&s| s == 100));
            assert!(clause_view.positive_mask_words.iter().all(|&w| w == 0));
            assert!(clause_view.negated_mask_words.iter().all(|&w| w == 0));
        }
        bag_engine.bag_validate_internal_consistency().unwrap();
    }

    /// Integration: the guard fires through the REAL training path. A
    /// clause forced to require a shingle present in every training
    /// document — the exact pathological case the guard targets — is
    /// reset at step == limit. Deterministic: the guard is independent of
    /// the feedback gates, and within 16 steps neither Ib decay (needs
    /// ~100 gated decrements to drop the forced include) nor Type II
    /// (needs ~100 gated increments to add a blocking literal) can change
    /// clause 0's firing behavior.
    #[test]
    fn bag_fire_guard_resets_pathological_clause_during_training() {
        let mut bag_engine = make_bag_engine_with_guard(2, 100, &[b"ababab"], 2, 16);
        bag_engine.bag_test_force_include(0, 0); // require "ab" — ubiquitous
        let mut rng = FastRng::seed(42);
        for _ in 0..16 {
            bag_engine.bag_train_step(b"abab", true, &mut rng).unwrap();
        }
        // Step 16 reached streak == limit: exactly one reset (clause 1
        // stays vacuous throughout — exempt — so it cannot contribute).
        assert_eq!(bag_engine.bag_fire_guard_reset_total(), 1);
        // Clause 0 is fresh right after the reset step (feedback was
        // skipped that step, and training stopped there).
        let literals = bag_engine.bag_literals_per_clause();
        assert!(
            bag_engine.bag_ta_states[..literals]
                .iter()
                .all(|&s| s == 100)
        );
        bag_engine.bag_validate_internal_consistency().unwrap();
    }

    /// The parallelization contract EXTENDED to the guard: with the guard
    /// ON, one and many workers must produce byte-identical states,
    /// masks, streaks, AND reset telemetry.
    #[test]
    fn bag_fire_guard_training_is_worker_count_invariant() {
        let corpus: &[&[u8]] = &[b"very good movie", b"not good movie", b"hi"];
        let training_docs: [(&[u8], bool); 3] = [
            (b"very good movie", true),
            (b"not good movie", false),
            (b"hi", true),
        ];
        let run_training = |workers: u16| -> ByteBagTM {
            let mut engine = make_bag_engine_with_guard(3, 100, corpus, 16, 16);
            let mut rng = FastRng::seed(123);
            let worker_count = WorkerCount::new(workers).unwrap();
            for _ in 0..40 {
                for (doc, label) in training_docs {
                    engine
                        .bag_train_step_with_workers(doc, *&label, &mut rng, worker_count)
                        .unwrap();
                }
            }
            engine.bag_validate_internal_consistency().unwrap();
            engine
        };
        let single = run_training(1);
        let many = run_training(8);
        assert_eq!(single.bag_ta_states, many.bag_ta_states, "states diverged");
        assert_eq!(
            single.bag_positive_include_masks, many.bag_positive_include_masks,
            "positive masks diverged"
        );
        assert_eq!(
            single.bag_negated_include_masks, many.bag_negated_include_masks,
            "negated masks diverged"
        );
        assert_eq!(
            single.bag_fire_streaks, many.bag_fire_streaks,
            "guard streaks diverged"
        );
        assert_eq!(
            single.bag_fire_guard_reset_counts, many.bag_fire_guard_reset_counts,
            "guard reset telemetry diverged"
        );
    }
    /*
    Regression coverage note (recorded, no new test needed): the guard-disabled
    path is structurally inert (zero RNG consumed, short-circuit before any state
    touch), and the existing stochastic-outcome tests
    — bag_learns_negation_micro_corpus,
    harness_bag_engine_end_to_end_on_negation_corpus,
    bag_training_is_worker_count_invariant — all train with the guard disabled
    through the updated constructor. If the disabled guard changed any behavior,
    those tests would fail. They are the regression oracle.
    */

    /// CLI: `--fire-guard` parses; absent flag defaults to 0 (disabled);
    /// a sub-minimum active value is rejected at config resolution with
    /// the newtype's specific code (fail-fast at the CLI boundary).
    #[test]
    fn cli_parses_fire_guard_flag_and_validates_bounds() {
        let parsed = cli(&["--mode", "train", "--fire-guard", "2000"]).unwrap();
        assert_eq!(parsed.fire_guard_streak_limit, 2000);
        assert!(parsed.to_run_config().is_ok());

        let defaulted = cli(&["--mode", "train"]).unwrap();
        assert_eq!(defaulted.fire_guard_streak_limit, 0);

        let bad = cli(&["--mode", "train", "--fire-guard", "5"]).unwrap();
        assert_eq!(
            bad.to_run_config().err(),
            Some(GranmoModelError::CfgFireGuardLimitOutOfBounds)
        );
    }

    /// Parallel evaluation must be value-identical to sequential, field by
    /// field — including misprediction RECORD ORDER (the ordered-merge
    /// property, which the sweep and the log both depend on).
    #[test]
    fn evaluation_is_worker_count_invariant() {
        let engine = ClassifierEngine::ByteConv(make_engine(5, 2, 16, false));
        let raw_documents: Vec<LabeledDocument> = (0..37)
            .map(|i| LabeledDocument {
                line_index: 0,
                text: format!("document number {i}").into_bytes(),
                label_is_positive: i % 3 == 0,
            })
            .collect();
        let prepared =
            preprocess_documents(PreprocessProfile::preset_p0(), &raw_documents).unwrap();

        let sequential = evaluate_test_split_with_workers(
            &engine,
            &prepared,
            &raw_documents,
            WorkerCount::new(1).unwrap(),
        )
        .unwrap();
        let parallel = evaluate_test_split_with_workers(
            &engine,
            &prepared,
            &raw_documents,
            WorkerCount::new(8).unwrap(),
        )
        .unwrap();

        assert_eq!(sequential.correct_at_zero, parallel.correct_at_zero);
        assert_eq!(sequential.clause_fire_counts, parallel.clause_fire_counts);
        assert_eq!(sequential.vote_sums, parallel.vote_sums);
        assert_eq!(sequential.labels, parallel.labels);
        assert_eq!(
            sequential.mispredictions.len(),
            parallel.mispredictions.len()
        );
        for (left, right) in sequential
            .mispredictions
            .iter()
            .zip(parallel.mispredictions.iter())
        {
            assert_eq!(left.raw_text_bytes, right.raw_text_bytes);
            assert_eq!(left.vote_sum_at_prediction, right.vote_sum_at_prediction);
        }
    }

    #[cfg(test)]
    mod hybrid_and_ensemble_tests {
        use super::*;

        const TEST_CORPUS: [&[u8]; 3] = [b"very good movie", b"not good movie", b"hi"];
        const TEST_TRAINING: [(&[u8], bool); 3] = [
            (b"very good movie", true),
            (b"not good movie", false),
            (b"hi", true),
        ];

        fn build_fresh_test_hybrid() -> HybridTM {
            let conv = ByteConvTM::new(
                PatchSize::new(3).unwrap(),
                StrideLen::new(1).unwrap(),
                ClauseCount::new(16).unwrap(),
                VoteThreshold::new(15).unwrap(),
                StatesPerAction::new(100).unwrap(),
                SpecificityThresholds::from_specificity(3.0).unwrap(),
                MaxScanBytes::new(256).unwrap(),
                false,
            )
            .unwrap();
            let vocabulary = ByteBagVocabulary::build_from_documents(
                NgramLength::new(3).unwrap(),
                VocabSize::new(50).unwrap(),
                &TEST_CORPUS,
            )
            .unwrap();
            let bag = ByteBagTM::new_with_vocabulary(
                vocabulary,
                ClauseCount::new(16).unwrap(),
                VoteThreshold::new(15).unwrap(),
                StatesPerAction::new(100).unwrap(),
                SpecificityThresholds::from_specificity(3.0).unwrap(),
                MaxScanBytes::new(256).unwrap(),
                FireGuardStreakLimit::new(FireGuardStreakLimit::DISABLED).unwrap(),
            )
            .unwrap();
            HybridTM::new_from_sub_engines(conv, bag, VoteThreshold::new(15).unwrap()).unwrap()
        }

        fn train_test_hybrid(workers: u16, epochs: usize) -> HybridTM {
            let mut hybrid = build_fresh_test_hybrid();
            let mut rng = FastRng::seed(42);
            let worker_count = WorkerCount::new(workers).unwrap();
            for _ in 0..epochs {
                for (document, label) in TEST_TRAINING {
                    hybrid
                        .hyb_train_step_with_workers(document, label, &mut rng, worker_count)
                        .unwrap();
                }
            }
            hybrid.hyb_validate_internal_consistency().unwrap();
            hybrid
        }

        #[test]
        fn hybrid_fresh_engine_votes_zero_and_validates() {
            let hybrid = build_fresh_test_hybrid();
            // Both banks vacuous and polarity-balanced: combined vote is 0.
            assert_eq!(hybrid.hyb_vote_sum(b"hello world").unwrap(), 0);
            assert_eq!(hybrid.hyb_clause_count(), 32);
            hybrid.hyb_validate_internal_consistency().unwrap();
        }

        #[test]
        fn hybrid_training_is_worker_count_invariant() {
            let single = train_test_hybrid(1, 20);
            let multi = train_test_hybrid(4, 20);
            assert_eq!(
                single.hyb_conv_engine.ta_states, multi.hyb_conv_engine.ta_states,
                "conv states diverged across worker counts"
            );
            assert_eq!(
                single.hyb_bag_engine.bag_ta_states, multi.hyb_bag_engine.bag_ta_states,
                "bag states diverged across worker counts"
            );
        }

        #[test]
        fn hybrid_training_changes_both_banks() {
            // Guards against a wiring bug where one bank's feedback is skipped.
            let fresh = build_fresh_test_hybrid();
            let trained = train_test_hybrid(1, 20);
            assert_ne!(
                fresh.hyb_conv_engine.ta_states,
                trained.hyb_conv_engine.ta_states
            );
            assert_ne!(
                fresh.hyb_bag_engine.bag_ta_states,
                trained.hyb_bag_engine.bag_ta_states
            );
        }

        #[test]
        fn hybrid_fired_bits_reproduce_vote_under_parity_rule() {
            // The harness's single-pass evaluation relies on this equivalence
            // over the concatenated clause space.
            let hybrid = train_test_hybrid(1, 10);
            for (document, _label) in TEST_TRAINING {
                let fired_words = hybrid.hyb_fired_clause_bits(document).unwrap();
                let vote_from_bits =
                    vote_from_fired_words(&fired_words, hybrid.hyb_clause_count()).unwrap();
                assert_eq!(vote_from_bits, hybrid.hyb_vote_sum(document).unwrap());
            }
        }

        #[test]
        fn hybrid_bag_bank_seed_differs_from_conv_bank_seed() {
            let step_seed = 0x1234_5678_9ABC_DEF0u64;
            let bag_step_seed = derive_clause_stream_seed(
                step_seed,
                HYBRID_BAG_BANK_ORDINAL,
                RNG_PURPOSE_HYBRID_BAG_BANK,
            );
            assert_ne!(step_seed, bag_step_seed);
            // Same-index clauses in the two banks must not share a stream.
            let conv_clause_seed =
                derive_clause_stream_seed(step_seed, 3, RNG_PURPOSE_CLAUSE_FEEDBACK);
            let bag_clause_seed =
                derive_clause_stream_seed(bag_step_seed, 3, RNG_PURPOSE_CLAUSE_FEEDBACK);
            assert_ne!(conv_clause_seed, bag_clause_seed);
        }

        #[test]
        fn hybrid_artifact_kind_4_round_trips() {
            let trained = train_test_hybrid(1, 10);
            let artifact = ModelArtifact {
                preprocess_profile: PreprocessProfile::preset_p2(),
                engine: ClassifierEngine::SeqFreqHybrid(trained.clone()),
            };
            let path = std::env::temp_dir().join("granmo_hybrid_kind4_round_trip_test.gmb");
            artifact.save_to_file(&path).unwrap();
            let loaded = ModelArtifact::load_from_file(&path).unwrap();
            let _ = std::fs::remove_file(&path);

            assert_eq!(loaded.preprocess_profile, PreprocessProfile::preset_p2());
            match loaded.engine {
                ClassifierEngine::SeqFreqHybrid(loaded_hybrid) => {
                    assert_eq!(loaded_hybrid.hyb_vote_target, trained.hyb_vote_target);
                    assert_eq!(
                        loaded_hybrid.hyb_conv_engine.ta_states,
                        trained.hyb_conv_engine.ta_states
                    );
                    assert_eq!(
                        loaded_hybrid.hyb_bag_engine.bag_ta_states,
                        trained.hyb_bag_engine.bag_ta_states
                    );
                    for (document, _label) in TEST_TRAINING {
                        assert_eq!(
                            loaded_hybrid.hyb_vote_sum(document).unwrap(),
                            trained.hyb_vote_sum(document).unwrap()
                        );
                    }
                }
                other => panic!("expected Hybrid, got {}", other.engine_name()),
            }
        }

        fn mock_scored_report(
            name: &'static str,
            votes: Vec<i32>,
            labels: Vec<bool>,
        ) -> ExperimentReport {
            ExperimentReport {
                engine_name_reported: name,
                train_count: 10,
                test_count: votes.len(),
                accuracy_at_zero: 0.0,
                best_f1_row: ThresholdSweepRow {
                    decision_threshold: 0,
                    true_positives: 0,
                    false_positives: 0,
                    true_negatives: 0,
                    false_negatives: 0,
                    precision: 0.0,
                    recall: 0.0,
                    f1: 0.0,
                },
                train_seconds: 1.0,
                clause_fire_counts: vec![1, 1],
                fire_guard_reset_total: 0,
                clause_include_totals: vec![2, 2],
                fire_guard_limit_used: 0,
                mispredictions: Vec::new(),
                test_row_predictions: Vec::new(),
                train_row_predictions: Vec::new(),
                test_vote_sums: votes,
                test_labels: labels,
            }
        }

        #[test]
        fn late_fusion_sums_votes_and_corrects_complementary_errors() {
            let raw_docs = vec![
                LabeledDocument {
                    line_index: 0,
                    text: b"doc1".to_vec(),
                    label_is_positive: true,
                },
                LabeledDocument {
                    line_index: 0,
                    text: b"doc2".to_vec(),
                    label_is_positive: false,
                },
            ];
            let prepared_docs = vec![(b"doc1".to_vec(), true), (b"doc2".to_vec(), false)];
            // conv misses doc2 (+1 on a negative); bag corrects it (-3).
            let conv_report = mock_scored_report("byte-conv", vec![2, 1], vec![true, false]);
            let bag_report = mock_scored_report("byte-bag", vec![1, -3], vec![true, false]);

            let fusion =
                evaluate_late_fusion_ensemble(&conv_report, &bag_report, &raw_docs, &prepared_docs)
                    .unwrap();
            assert_eq!(fusion.test_vote_sums, vec![3, -2]);
            assert_eq!(fusion.accuracy_at_zero, 1.0);
            assert!(fusion.mispredictions.is_empty());
            assert_eq!(fusion.clause_fire_counts.len(), 4); // 2 conv + 2 bag
            assert_eq!(fusion.train_seconds, 2.0);
            assert_eq!(fusion.test_row_predictions.len(), 2);
            assert_eq!(fusion.test_row_predictions[0].vote_sum, 3);
            assert_eq!(fusion.test_row_predictions[1].vote_sum, -2);
            assert!(fusion.test_row_predictions.iter().all(|r| !r.side_is_train));
            assert!(fusion.train_row_predictions.is_empty()); // mocks recorded no train side
        }

        #[test]
        fn late_fusion_rejects_mismatched_inputs() {
            let raw_docs = vec![
                LabeledDocument {
                    line_index: 0,
                    text: b"doc1".to_vec(),
                    label_is_positive: true,
                },
                LabeledDocument {
                    line_index: 0,
                    text: b"doc2".to_vec(),
                    label_is_positive: false,
                },
            ];
            let prepared_docs = vec![(b"doc1".to_vec(), true), (b"doc2".to_vec(), false)];

            // Length mismatch.
            let conv_report = mock_scored_report("byte-conv", vec![2, 1], vec![true, false]);
            let short_bag = mock_scored_report("byte-bag", vec![1], vec![true]);
            assert_eq!(
                evaluate_late_fusion_ensemble(&conv_report, &short_bag, &raw_docs, &prepared_docs)
                    .unwrap_err(),
                GranmoModelError::EnsVoteVectorMismatch
            );

            // Same length, different labels (different split/order).
            let other_split_bag = mock_scored_report("byte-bag", vec![1, -3], vec![false, true]);
            assert_eq!(
                evaluate_late_fusion_ensemble(
                    &conv_report,
                    &other_split_bag,
                    &raw_docs,
                    &prepared_docs
                )
                .unwrap_err(),
                GranmoModelError::EnsLabelVectorMismatch
            );
        }
    }
}
