//! # Byte-Convolution Granmo Model — Phase 2 Crate
//!
//! Binary classification of short social-media text (hate/bullying detection)
//! using a byte-level convolutional Granmo Model (per Granmo 2018, the
//! bandit-driven propositional-logic learning architecture), built to a
//! production profile: integer-only hot paths, no-heap no-panic error
//! handling, streaming-capable preprocessing, tens-of-KB inference artifact.
//!
//! ## This file (first drop) contains:
//! - `GranmoModelError`: the crate-wide fieldless `#[repr(u16)]` error-code
//!   enum (Mode & Case Handling framework: no heap, no PII, 2-byte Copy,
//!   append-only codes, per-module code blocks, `is_retryable()`).
//! - `FastRng`: deterministic xorshift64 with integer-only draw methods
//!   (`next_u16` coin draws against precomputed u16 thresholds; range draws
//!   via widening multiply — see method docs for the bias statement).
//! - Enforced custom types (`PatchSize`, `StrideLen`, `ClauseCount`,
//!   `StatesPerAction`, `SpecificityThresholds`, `PreprocessProfile`):
//!   bounded at construction, revalidated on every `.get()` so post-
//!   construction corruption surfaces as an error code, never a silent
//!   bad value.
//! - M-Preprocess: byte-stream adapter pipeline, stages 1–8 of the
//!   specification of record (stage 9 / stemmer dropped per resolved
//!   decision §11.3), presets raw/P0–P5, all as single-pass map-or-drop
//!   transforms with O(1) state — streaming-capable by construction.
//!
//! ## Not yet in this file (next drops):
//! - M-ByteBag (byte-n-gram view), M-Conv-Core (`ByteConvTM`), artifact I/O.
//!
//! ## Modes (per the Mode & Case Handling framework):
//! - Production-release: never panics, error paths allocate nothing, no
//!   logging inside functions before the code is returned.
//! - Debug: `eprintln!` diagnostics gated `#[cfg(debug_assertions)]`;
//!   `debug_assert!` gated `#[cfg(all(debug_assertions, not(test)))]`.
//! - Test: `#[cfg(test)]` cargo tests use `assert!` freely.

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
    // --- 800–899: CLI / harness ---
    /// An unrecognized flag was passed (fail-fast: never silently ignored).
    CliUnknownFlag = 800,
    /// A flag that requires a value was the last argument.
    CliFlagMissingValue = 801,
    /// A flag value failed to parse as its required type/range.
    CliInvalidValue = 802,
    /// A flag required by the selected mode was absent.
    CliMissingRequiredFlag = 803,
    /// `--mode` value was not one of train/predict/batch.
    CliUnknownMode = 804,
    /// `--preset` value was not one of raw/p0/p1/p2/p3/p4/p5.
    CliUnknownPreset = 805,
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
            Self::ArtFileWriteFailed | Self::ArtFileReadFailed | Self::DsFileReadFailed => true,

            Self::PpProfileReservedBitsSet
            | Self::PpProfileRecheckCorrupt
            | Self::RngGenIndexEmptyRange
            | Self::CfgPatchSizeOutOfBounds
            | Self::CfgPatchSizeRecheckCorrupt
            | Self::CfgStrideOutOfBounds
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

impl ByteConvTM {
    /// Builds the probability LUT matched to THIS engine's clause count and
    /// vote threshold. Routing construction through the enforced newtypes
    /// re-validates both values, so a corrupted engine cannot silently
    /// produce a mis-sized table (value-integrity rule).
    pub fn build_probability_lut(&self) -> Result<ProbabilityLut, GranmoModelError> {
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
    pub fn vote_sum(&self, document: &[u8]) -> Result<i32, GranmoModelError> {
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
    pub fn predict(
        &self,
        document: &[u8],
        decision_threshold: i32,
    ) -> Result<bool, GranmoModelError> {
        Ok(self.vote_sum(document)? > decision_threshold)
    }

    /// The fired-clause bitset: the free-byproduct binary document embedding
    /// (§7.2). Bit `c` set iff clause `c` fired; Hamming distance between
    /// bitsets is a learned document similarity.
    pub fn fired_clause_bits(&self, document: &[u8]) -> Result<Vec<u64>, GranmoModelError> {
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
    pub fn fired_window_positions(
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
    pub fn describe_clause(
        &self,
        clause: usize,
        max_rendered_literals: usize,
    ) -> Result<String, GranmoModelError> {
        if clause >= self.n_clauses {
            return Err(GranmoModelError::BctIndexOutOfRange);
        }
        let depth_n = self.states_per_action;
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

    /// Recomputes the allowed-bytes mask for (clause, slot) purely from raw
    /// automaton states. Semantics (§4 of the specification of record):
    /// - 0 positive includes: byte b allowed iff negated(b) not included;
    /// - 1 positive include b*: only b* allowed, and only if negated(b*)
    ///   is not also included;
    /// - ≥2 positive includes: NOTHING allowed (structurally dead slot —
    ///   the state the GuardedInclude variant refuses to enter).
    fn compute_mask_from_states(
        &self,
        clause: usize,
        slot: usize,
    ) -> Result<[u64; MASK_WORDS], GranmoModelError> {
        let depth_n = self.states_per_action;
        let pos_start = self.global_state_index(clause, self.positive_local_index(slot, 0))?;
        let neg_start = self.global_state_index(clause, self.negated_local_index(slot, 0))?;
        let pos_end = pos_start
            .checked_add(BYTE_ALPHABET_SIZE)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let neg_end = neg_start
            .checked_add(BYTE_ALPHABET_SIZE)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let pos_states = self
            .ta_states
            .get(pos_start..pos_end)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        let neg_states = self
            .ta_states
            .get(neg_start..neg_end)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;

        let included_positive_total = pos_states.iter().filter(|&&s| s > depth_n).count();

        let mut mask = [0u64; MASK_WORDS];
        if included_positive_total <= 1 {
            for byte_value in 0..BYTE_ALPHABET_SIZE {
                let negated_included = neg_states[byte_value] > depth_n;
                let positive_ok = included_positive_total == 0 || pos_states[byte_value] > depth_n;
                if positive_ok && !negated_included {
                    mask[byte_value >> 6] |= 1u64 << (byte_value & 63);
                }
            }
        }
        Ok(mask)
    }

    /// Recomputes and stores the mask for (clause, slot). Called ONLY from
    /// the two transition methods on boundary crossings — the invariant that
    /// keeps evaluation O(K) bit-tests.
    fn recompute_mask(&mut self, clause: usize, slot: usize) -> Result<(), GranmoModelError> {
        let fresh_mask = self.compute_mask_from_states(clause, slot)?;
        let slot_idx = self.global_slot_index(clause, slot)?;
        let stored = self
            .allowed_masks
            .get_mut(slot_idx)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        *stored = fresh_mask;
        Ok(())
    }

    // --- Automaton transitions (ALL state changes route through these) ------

    /// Increments one automaton state (saturating at 2N), maintaining the
    /// positive-include count cache and recomputing the affected slot mask
    /// on an exclude→include boundary crossing.
    ///
    /// GuardedInclude variant (ablation flag): a POSITIVE literal is refused
    /// the crossing while another positive literal at the same slot is
    /// already included — the state clamps at the boundary N. This prunes
    /// only provably-dead joint configurations (see design notes Q4a).
    fn increment_ta_state(
        &mut self,
        clause: usize,
        local_literal: usize,
    ) -> Result<(), GranmoModelError> {
        let depth_n = self.states_per_action;
        let twice_n = depth_n
            .checked_mul(2)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        let global = self.global_state_index(clause, local_literal)?;
        let state = *self
            .ta_states
            .get(global)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;

        if state >= twice_n {
            return Ok(()); // saturated deep-include; no change
        }

        let positive_region_len = self.patch_size * BYTE_ALPHABET_SIZE;
        let is_positive_literal = local_literal < positive_region_len;
        let crossing = state == depth_n;

        if crossing && is_positive_literal && self.guarded_include {
            let slot = local_literal / BYTE_ALPHABET_SIZE;
            let slot_idx = self.global_slot_index(clause, slot)?;
            let current_count = *self
                .positive_include_counts
                .get(slot_idx)
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            if current_count > 0 {
                return Ok(()); // guard refuses the crossing; clamp at boundary
            }
        }

        let new_state = state
            .checked_add(1)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        *self
            .ta_states
            .get_mut(global)
            .ok_or(GranmoModelError::BctIndexOutOfRange)? = new_state;

        if crossing {
            let slot = if is_positive_literal {
                local_literal / BYTE_ALPHABET_SIZE
            } else {
                (local_literal - positive_region_len) / BYTE_ALPHABET_SIZE
            };
            if is_positive_literal {
                let slot_idx = self.global_slot_index(clause, slot)?;
                let count = self
                    .positive_include_counts
                    .get_mut(slot_idx)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)?;
                *count = count
                    .checked_add(1)
                    .ok_or(GranmoModelError::BctArithmeticOverflow)?;
            }
            self.recompute_mask(clause, slot)?;
        }
        Ok(())
    }

    /// Decrements one automaton state (floor at 1), maintaining the count
    /// cache and recomputing the slot mask on an include→exclude crossing.
    fn decrement_ta_state(
        &mut self,
        clause: usize,
        local_literal: usize,
    ) -> Result<(), GranmoModelError> {
        let depth_n = self.states_per_action;
        let boundary_plus_one = depth_n
            .checked_add(1)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        let global = self.global_state_index(clause, local_literal)?;
        let state = *self
            .ta_states
            .get(global)
            .ok_or(GranmoModelError::BctIndexOutOfRange)?;

        if state <= 1 {
            return Ok(()); // floor; no change
        }

        let new_state = state
            .checked_sub(1)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        *self
            .ta_states
            .get_mut(global)
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
                let slot_idx = self.global_slot_index(clause, slot)?;
                let count = self
                    .positive_include_counts
                    .get_mut(slot_idx)
                    .ok_or(GranmoModelError::BctIndexOutOfRange)?;
                // saturating_sub as defence-in-depth; a validated invariant
                // means this can never actually saturate.
                *count = count.saturating_sub(1);
            }
            self.recompute_mask(clause, slot)?;
        }
        Ok(())
    }

    // --- Feedback (Phase 1 semantics preserved; §3 established facts) --------

    /// Type Ia: clause fired and should have — reinforce the literal pattern
    /// of ONE sampled fired window. True literals strengthen with
    /// P ≈ (s−1)/s; false literals decay with P ≈ 1/s. All draws are u16
    /// threshold coins — integer-only. Cost O(K × 512) literal visits.
    fn apply_type_ia_feedback(
        &mut self,
        clause: usize,
        document: &[u8],
        effective_len: usize,
        window_start: usize,
        rng: &mut FastRng,
    ) -> Result<(), GranmoModelError> {
        for slot in 0..self.patch_size {
            let observed = Self::window_byte(document, effective_len, window_start, slot);
            for byte_value in 0..BYTE_ALPHABET_SIZE {
                let pos_local = self.positive_local_index(slot, byte_value);
                let neg_local = self.negated_local_index(slot, byte_value);
                let positive_literal_true = byte_value == usize::from(observed);

                if positive_literal_true {
                    if rng.coin(self.reinforce_threshold_u16) {
                        self.increment_ta_state(clause, pos_local)?;
                    }
                } else if rng.coin(self.forget_threshold_u16) {
                    self.decrement_ta_state(clause, pos_local)?;
                }

                // Negated literal truth is the complement.
                if !positive_literal_true {
                    if rng.coin(self.reinforce_threshold_u16) {
                        self.increment_ta_state(clause, neg_local)?;
                    }
                } else if rng.coin(self.forget_threshold_u16) {
                    self.decrement_ta_state(clause, neg_local)?;
                }
            }
        }
        Ok(())
    }

    /// Type Ib: clause should have fired but did not at ANY window —
    /// input-independent uniform decay with P ≈ 1/s (no window selection is
    /// meaningful here; §3 fact 3), freeing the clause to learn afresh.
    fn apply_type_ib_feedback(
        &mut self,
        clause: usize,
        rng: &mut FastRng,
    ) -> Result<(), GranmoModelError> {
        for local_literal in 0..self.literals_per_clause() {
            if rng.coin(self.forget_threshold_u16) {
                self.decrement_ta_state(clause, local_literal)?;
            }
        }
        Ok(())
    }

    /// Type II: clause fired but should not have — push toward including a
    /// blocking literal by incrementing every literal FALSE in one sampled
    /// fired window. (In a fired window all included literals are true, so
    /// every false literal is currently excluded; incrementing it moves it
    /// toward blocking.) Deterministic — no randomness in Type II.
    fn apply_type_ii_feedback(
        &mut self,
        clause: usize,
        document: &[u8],
        effective_len: usize,
        window_start: usize,
    ) -> Result<(), GranmoModelError> {
        for slot in 0..self.patch_size {
            let observed = usize::from(Self::window_byte(
                document,
                effective_len,
                window_start,
                slot,
            ));
            for byte_value in 0..BYTE_ALPHABET_SIZE {
                if byte_value != observed {
                    // Positive literal (slot, b≠observed) is false here.
                    self.increment_ta_state(clause, self.positive_local_index(slot, byte_value))?;
                }
            }
            // Negated literal (slot, observed) is false here.
            self.increment_ta_state(clause, self.negated_local_index(slot, observed))?;
        }
        Ok(())
    }

    // --- Training ------------------------------------------------------------

    /// One stochastic training update for one document.
    ///
    /// Binary single-bank semantics (§4): label=1 target ⇒ positive-polarity
    /// clauses get Type I and negative-polarity fired clauses get Type II,
    /// gated per clause with P = (T−V)/2T; label=0 mirrors with (T+V)/2T.
    /// The gate is an exact integer comparison: draw r ∈ [0, 2T), apply iff
    /// r < gate. Scanning (immutable) completes for ALL clauses before any
    /// state mutation, so every clause's feedback is attributed against the
    /// pre-update model — the same discipline as Phase 1.
    pub fn train_step(
        &mut self,
        document: &[u8],
        label_is_positive: bool,
        rng: &mut FastRng,
    ) -> Result<(), GranmoModelError> {
        let (effective_len, _single_padded) = self.scan_plan(document);

        // Pass 1 (immutable): per-clause fired flag + reservoir-sampled
        // window + running vote sum. One scratch Vec per step (training is
        // a research-harness path; per-step heap here is accepted and
        // documented — the inference path allocates nothing).
        let mut fired_and_sample: Vec<(bool, usize)> = Vec::with_capacity(self.n_clauses);
        let mut vote: i32 = 0;
        for clause in 0..self.n_clauses {
            let (fired_count, sampled_start) = self.scan_clause_reservoir(clause, document, rng)?;
            let fired = fired_count > 0;
            if fired {
                vote = if clause % 2 == 0 {
                    vote.checked_add(1)
                } else {
                    vote.checked_sub(1)
                }
                .ok_or(GranmoModelError::BctArithmeticOverflow)?;
            }
            fired_and_sample.push((fired, sampled_start));
        }

        let threshold_t = self.vote_threshold;
        let clamped_v = vote.clamp(-threshold_t, threshold_t);
        // Integer gates: (T−V) for target-consistent updates, (T+V) otherwise.
        let gate_when_target = threshold_t
            .checked_sub(clamped_v)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        let gate_when_other = threshold_t
            .checked_add(clamped_v)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;
        let two_t = (threshold_t as usize)
            .checked_mul(2)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;

        // Pass 2 (mutable): per-clause gated feedback.
        for clause in 0..self.n_clauses {
            let (fired, sampled_start) = *fired_and_sample
                .get(clause)
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            let positive_polarity = clause % 2 == 0;

            // Which feedback family, and which gate, this clause receives.
            let (gate, receives_type_i) = if label_is_positive {
                (gate_when_target, positive_polarity)
            } else {
                (gate_when_other, !positive_polarity)
            };

            let draw = rng.gen_index(two_t)? as i32;
            if draw >= gate {
                continue; // gated out this step
            }

            if receives_type_i {
                if fired {
                    self.apply_type_ia_feedback(
                        clause,
                        document,
                        effective_len,
                        sampled_start,
                        rng,
                    )?;
                } else {
                    self.apply_type_ib_feedback(clause, rng)?;
                }
            } else if fired {
                self.apply_type_ii_feedback(clause, document, effective_len, sampled_start)?;
            }
        }
        Ok(())
    }

    // --- Invariant validation --------------------------------------------------

    /// Re-derives every mask and every positive-include count from raw
    /// automaton states and compares against the caches; also checks every
    /// state lies in the legal band [1, 2N]. Call after any future artifact
    /// load (same pattern as Phase 1's `validate_internal_consistency`) and
    /// after training in tests.
    pub fn validate_internal_consistency(&self) -> Result<(), GranmoModelError> {
        let depth_n = self.states_per_action;
        let twice_n = depth_n
            .checked_mul(2)
            .ok_or(GranmoModelError::BctArithmeticOverflow)?;

        for &state in &self.ta_states {
            if state < 1 || state > twice_n {
                return Err(GranmoModelError::BctStateValueOutOfRange);
            }
        }

        for clause in 0..self.n_clauses {
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
    pub fn clause_count(&self) -> usize {
        self.n_clauses
    }

    /// Test-only helper: forces a literal fully into the include region via
    /// the counter-maintaining transition path, so tests can hand-construct
    /// exact clauses without breaking the mask/count invariants. Terminates
    /// when the state stops changing (saturated at 2N, or refused by the
    /// GuardedInclude clamp).
    #[cfg(test)]
    fn test_force_include(&mut self, clause: usize, local_literal: usize) {
        loop {
            let global = self.global_state_index(clause, local_literal).unwrap();
            let before = self.ta_states[global];
            self.increment_ta_state(clause, local_literal).unwrap();
            let after = self.ta_states[global];
            if after == before {
                break;
            }
        }
    }
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
    if engine.vote_sum(b"hello world")? != 0 {
        return Err(GranmoModelError::BctMaskCacheInconsistent);
    }
    engine.validate_internal_consistency()?;

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
    pub fn validity_recheck(&self) -> Result<(), GranmoModelError> {
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
// SECTION 8: Artifact I/O — Full-Training Artifact (Art* codes, 500-block)
// ===========================================================================
//
// Binary format, version 1, little-endian throughout:
//
//   offset  size  field
//   0       8     magic  b"GRANMOB1"
//   8       2     format version (u16) = 1
//   10      2     preprocess profile bits (u16)          [locked §10.9]
//   12      1     patch size K (u8)
//   13      1     stride S (u8)
//   14      2     clause count (u16)
//   16      2     vote threshold T (i16)
//   18      2     states per action N (i16)
//   20      2     forget threshold (u16)
//   22      2     reinforce threshold (u16)
//   24      4     max scan bytes (u32)
//   28      1     guarded-include flag (u8: 0/1)
//   29      1     artifact kind (u8): 1 = FullTraining
//                 (2 = CompactInference reserved for M-Prod-Pass)
//   30      2     reserved, must be 0
//   32      2*L   automaton states (i16 each), L = clauses * 2 * K * 256
//   end     8     FNV-1a-64 checksum over ALL preceding bytes (u64)
//
// Design notes:
// - Masks and count caches are NOT stored: they are derived data, rebuilt
//   from raw states at load and then cross-checked by
//   `validate_internal_consistency` — the same "re-validate on load"
//   pattern Phase 1 used, extended with a checksum so gross corruption is
//   caught before parsing begins.
// - Specificity round-trips as the exact integer thresholds (never as the
//   float `s`), so a loaded model is bit-identical in behavior.
// - The probability LUT is not stored in the FULL artifact (rebuilt from
//   the header by the harness). The compact inference artifact
//   (M-Prod-Pass) WILL embed the LUT verbatim so production load stays
//   float-free.

/// File magic identifying a Granmo Model artifact, format family B, v1 line.
const ARTIFACT_MAGIC: [u8; 8] = *b"GRANMOB1";
/// Current binary format version.
const ARTIFACT_FORMAT_VERSION: u16 = 1;
/// Artifact kind byte: full training artifact (raw automaton states).
const ARTIFACT_KIND_FULL_TRAINING: u8 = 1;

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

/// The persisted unit: engine + the preprocessing profile it was trained
/// with, coupled in ONE artifact so inference can never accidentally replay
/// the wrong preprocessing (locked decision §10.9 made structural).
#[derive(Debug, Clone)]
pub struct ModelArtifact {
    pub preprocess_profile: PreprocessProfile,
    pub engine: ByteConvTM,
}

impl ModelArtifact {
    /// Serializes to the format documented at the top of this section and
    /// writes to `absolute_path`. The path must be absolute (crate policy);
    /// filesystem failure detail is dropped (no-PII policy) and reported as
    /// the retryable `ArtFileWriteFailed` — callers may Tier-1 retry.
    pub fn save_to_file(&self, absolute_path: &std::path::Path) -> Result<(), GranmoModelError> {
        if !absolute_path.is_absolute() {
            #[cfg(debug_assertions)]
            eprintln!("ART-500: path not absolute: {}", absolute_path.display());
            return Err(GranmoModelError::ArtPathNotAbsolute);
        }

        let profile_bits = self.preprocess_profile.get_bits()?;
        let engine = &self.engine;

        // Header assembly. Field widths are fixed by the format table; the
        // engine's usize fields were validated into these ranges at
        // construction, so the narrowing casts below cannot truncate — but
        // each is still range-guarded as defence-in-depth.
        if engine.patch_size > usize::from(u8::MAX)
            || engine.stride > usize::from(u8::MAX)
            || engine.n_clauses > usize::from(u16::MAX)
            || engine.max_scan_bytes > u32::MAX as usize
        {
            return Err(GranmoModelError::BctIndexOutOfRange);
        }

        let payload_capacity = 32usize.saturating_add(engine.ta_states.len().saturating_mul(2));
        let mut buffer: Vec<u8> = Vec::with_capacity(payload_capacity.saturating_add(8));

        buffer.extend_from_slice(&ARTIFACT_MAGIC);
        buffer.extend_from_slice(&ARTIFACT_FORMAT_VERSION.to_le_bytes());
        buffer.extend_from_slice(&profile_bits.to_le_bytes());
        buffer.push(engine.patch_size as u8);
        buffer.push(engine.stride as u8);
        buffer.extend_from_slice(&(engine.n_clauses as u16).to_le_bytes());
        buffer.extend_from_slice(&(engine.vote_threshold as i16).to_le_bytes());
        buffer.extend_from_slice(&engine.states_per_action.to_le_bytes());
        buffer.extend_from_slice(&engine.forget_threshold_u16.to_le_bytes());
        buffer.extend_from_slice(&engine.reinforce_threshold_u16.to_le_bytes());
        buffer.extend_from_slice(&(engine.max_scan_bytes as u32).to_le_bytes());
        buffer.push(u8::from(engine.guarded_include));
        buffer.push(ARTIFACT_KIND_FULL_TRAINING);
        buffer.extend_from_slice(&[0u8, 0u8]); // reserved

        for &state in &engine.ta_states {
            buffer.extend_from_slice(&state.to_le_bytes());
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

    /// Loads and fully validates an artifact:
    /// 1. checksum over the raw bytes (catches bit-rot/truncation early);
    /// 2. header parse through the enforced-type constructors (every config
    ///    value is re-bounded exactly as at original construction);
    /// 3. cache rebuild from raw states (masks + counts are derived, never
    ///    trusted from disk);
    /// 4. `validate_internal_consistency` as the final gate.
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

        // Step 1: checksum. The last 8 bytes are the stored FNV-1a-64 of
        // everything before them.
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

        // Step 2: header parse, every value through its enforced type.
        let mut cursor = ByteCursor::new(payload);
        if cursor.take(8)? != ARTIFACT_MAGIC {
            return Err(GranmoModelError::ArtMagicMismatch);
        }
        if cursor.read_u16_le()? != ARTIFACT_FORMAT_VERSION {
            return Err(GranmoModelError::ArtVersionUnsupported);
        }
        let profile = PreprocessProfile::from_bits(cursor.read_u16_le()?)?;
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
            _ => return Err(GranmoModelError::ArtKindUnsupported),
        };
        if cursor.read_u8()? != ARTIFACT_KIND_FULL_TRAINING {
            return Err(GranmoModelError::ArtKindUnsupported);
        }
        let _reserved = cursor.take(2)?;

        // Step 3: state payload. Expected count is implied by the header;
        // a mismatch means the header and body disagree — reject.
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
        if cursor.position != payload.len() {
            // Trailing junk after the declared payload: header/body mismatch.
            return Err(GranmoModelError::ArtStateCountMismatch);
        }
        for (state_slot, chunk) in engine.ta_states.iter_mut().zip(state_bytes.chunks_exact(2)) {
            *state_slot = i16::from_le_bytes([chunk[0], chunk[1]]);
        }

        // Step 4: rebuild derived caches from raw states, then validate.
        engine.rebuild_caches_from_states()?;
        engine.validate_internal_consistency()?;

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
    /// THIS function).
    fn rebuild_caches_from_states(&mut self) -> Result<(), GranmoModelError> {
        let depth_n = self.states_per_action;
        for clause in 0..self.n_clauses {
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

// ===========================================================================
// SECTION 9: Dataset Ingestion (research-harness tier)
// ===========================================================================
//
// Tier note: this section is research-harness code. Data-path heap use and
// whole-file reads are its purpose; ERROR paths remain code-only/no-PII like
// everything else. Documents are kept as raw BYTES end to end — the model is
// byte-level, so no UTF-8 decode is performed (or needed) on document text.

/// One labeled example: raw document bytes + binary label.
#[derive(Debug, Clone)]
pub struct LabeledDocument {
    pub text: Vec<u8>,
    pub label_is_positive: bool,
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
    // Fisher–Yates, high index downward, seeded RNG.
    let mut i = shuffled.len();
    while i > 1 {
        i -= 1;
        let j = rng.gen_index(i + 1)?;
        shuffled.swap(i, j);
    }
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
                label_is_positive: label == positive_label.as_bytes(),
                text,
            }),
            Ok(None) => {} // skip per policy
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

/// All resolved parameters for one experiment run. Raw primitive fields are
/// deliberate HERE (and only here): this struct is the CLI-facing surface,
/// and every value is pushed through its enforced newtype constructor inside
/// `run_single_experiment` — so validation happens exactly once, at the
/// boundary between user input and engine.
#[derive(Debug, Clone)]
pub struct HarnessRunConfig {
    pub profile: PreprocessProfile,
    pub patch_size: u8,
    pub stride: u8,
    pub n_clauses: u16,
    pub vote_threshold: i16,
    pub states_per_action: i16,
    pub specificity: f64,
    pub max_scan_bytes: u32,
    pub guarded_include: bool,
    pub epochs: u32,
    pub seed: u64,
}

/// Everything a comparison-matrix row needs from one run.
#[derive(Debug, Clone)]
pub struct ExperimentReport {
    pub train_count: usize,
    pub test_count: usize,
    /// Accuracy at the default decision threshold V > 0.
    pub accuracy_at_zero: f64,
    /// The sweep row maximizing F1 (ties -> lowest threshold).
    pub best_f1_row: ThresholdSweepRow,
    pub train_seconds: f64,
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

/// Trains one `ByteConvTM` and evaluates it on the held-out set.
///
/// Training order is shuffled EVERY epoch (canonical fit practice: sample
/// order decorrelation), deterministically under the run seed — one RNG
/// stream drives shuffling and feedback, so a run is reproducible from
/// (dataset, split seed, run config) alone.
pub fn run_single_experiment(
    train_documents: &[LabeledDocument],
    test_documents: &[LabeledDocument],
    config: &HarnessRunConfig,
) -> Result<(ByteConvTM, ExperimentReport), GranmoModelError> {
    if train_documents.is_empty() || test_documents.is_empty() {
        return Err(GranmoModelError::DsNoUsableRecords);
    }

    let train_prepared = preprocess_documents(config.profile, train_documents)?;
    let test_prepared = preprocess_documents(config.profile, test_documents)?;

    let mut engine = ByteConvTM::new(
        PatchSize::new(config.patch_size)?,
        StrideLen::new(config.stride)?,
        ClauseCount::new(config.n_clauses)?,
        VoteThreshold::new(config.vote_threshold)?,
        StatesPerAction::new(config.states_per_action)?,
        SpecificityThresholds::from_specificity(config.specificity)?,
        MaxScanBytes::new(config.max_scan_bytes)?,
        config.guarded_include,
    )?;

    let mut rng = FastRng::seed(config.seed);
    let mut order: Vec<usize> = (0..train_prepared.len()).collect();

    let start = std::time::Instant::now();
    for _epoch in 0..config.epochs {
        // Per-epoch Fisher–Yates over the index vector (documents themselves
        // are never moved — only the visitation order).
        let mut i = order.len();
        while i > 1 {
            i -= 1;
            let j = rng.gen_index(i + 1)?;
            order.swap(i, j);
        }
        // // defensive
        // for &doc_index in &order {
        //     let (document, label) = order
        //         .get(0) // placeholder to satisfy borrow docs; real access below
        //         .and_then(|_| train_prepared.get(doc_index))
        //         .map(|(d, l)| (d, *l))
        //         .ok_or(GranmoModelError::BctIndexOutOfRange)?;
        //     engine.train_step(document, label, &mut rng)?;
        // }
        for &doc_index in &order {
            let (document, label) = train_prepared
                .get(doc_index)
                .map(|(d, l)| (d, *l))
                .ok_or(GranmoModelError::BctIndexOutOfRange)?;
            engine.train_step(document, label, &mut rng)?;
        }
    }
    let train_seconds = start.elapsed().as_secs_f64();

    // Evaluation: vote sums + labels, then accuracy at V > 0 and full sweep.
    let mut vote_sums = Vec::with_capacity(test_prepared.len());
    let mut labels = Vec::with_capacity(test_prepared.len());
    let mut correct_at_zero = 0usize;
    for (document, label) in &test_prepared {
        let vote = engine.vote_sum(document)?;
        if (vote > 0) == *label {
            correct_at_zero += 1;
        }
        vote_sums.push(vote);
        labels.push(*label);
    }
    let accuracy_at_zero = correct_at_zero as f64 / test_prepared.len() as f64;

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

    Ok((
        engine,
        ExperimentReport {
            train_count: train_documents.len(),
            test_count: test_documents.len(),
            accuracy_at_zero,
            best_f1_row,
            train_seconds,
        },
    ))
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
    patch_size: u8,
    stride: u8,
    n_clauses: u16,
    vote_threshold: i16,
    states_per_action: i16,
    specificity: f64,
    max_scan_bytes: u32,
    guarded_include: bool,
    epochs: u32,
    seed: u64,
    train_percent: u8,
    model_out: Option<std::path::PathBuf>,
    model_in: Option<std::path::PathBuf>,
    predict_text: Option<String>,
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
    /// Defaults mirror the specification of record: K=5, S=2, 200 clauses,
    /// T=50, N=100, s=5.0, preset p0, 25 epochs, seed 42, 80/20 split.
    fn parse(args: &[String]) -> Result<Self, GranmoModelError> {
        let mut parsed = Self {
            mode: String::new(),
            data_path: None,
            text_key: "text".to_string(),
            label_key: "label".to_string(),
            positive_label: "1".to_string(),
            preset_name: "p0".to_string(),
            patch_size: 5,
            stride: 2,
            n_clauses: 200,
            vote_threshold: 50,
            states_per_action: 100,
            specificity: 5.0,
            max_scan_bytes: 1024,
            guarded_include: false,
            epochs: 25,
            seed: 42,
            train_percent: 80,
            model_out: None,
            model_in: None,
            predict_text: None,
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
                "--patch" => {
                    parsed.patch_size = parse_flag_number(flag, take_value(args, &mut i, flag)?)?
                }
                "--stride" => {
                    parsed.stride = parse_flag_number(flag, take_value(args, &mut i, flag)?)?
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

    /// Assembles the resolved run config from parsed flags.
    fn to_run_config(&self) -> Result<HarnessRunConfig, GranmoModelError> {
        Ok(HarnessRunConfig {
            profile: preset_from_name(&self.preset_name)?,
            patch_size: self.patch_size,
            stride: self.stride,
            n_clauses: self.n_clauses,
            vote_threshold: self.vote_threshold,
            states_per_action: self.states_per_action,
            specificity: self.specificity,
            max_scan_bytes: self.max_scan_bytes,
            guarded_include: self.guarded_include,
            epochs: self.epochs,
            seed: self.seed,
        })
    }
}

fn print_help() {
    println!("Byte-Convolution Granmo Model — experiment harness");
    println!("===================================================");
    println!("Dataset format: JSONL only — one JSON object per line with");
    println!("  a \"text\" string field and a \"label\" field (string or number).");
    println!("TRAIN:   --mode train --data /abs/path.jsonl [options]");
    println!("BATCH:   --mode batch --data /abs/path.jsonl [options]  (presets raw,p0,p1,p2)");
    println!("PREDICT: --mode predict --model-in /abs/model.gmb --text \"...\"");
    println!("Options (defaults):");
    println!("  --text-key text | --label-key label | --positive-label 1");
    println!("  --preset p0 (raw|p0..p5) | --patch 5 | --stride 2 | --clauses 200");
    println!("  --vote-threshold 50 | --states 100 | --specificity 5.0");
    println!("  --max-scan 1024 | --guarded | --epochs 25 | --seed 42");
    println!("  --train-percent 80 | --model-out /abs/path.gmb");
}

/// Prints one experiment report (reporting tier: println is this code's
/// output channel, not a diagnostic leak).
fn print_experiment_report(run_label: &str, report: &ExperimentReport) {
    println!("--- run: {run_label} ---");
    println!(
        "  train/test: {}/{}   train time: {:.2}s",
        report.train_count, report.test_count, report.train_seconds
    );
    println!("  accuracy @ V>0:   {:.4}", report.accuracy_at_zero);
    let best = &report.best_f1_row;
    println!(
        "  best-F1 threshold {}: P {:.4} R {:.4} F1 {:.4}  (TP {} FP {} TN {} FN {})",
        best.decision_threshold,
        best.precision,
        best.recall,
        best.f1,
        best.true_positives,
        best.false_positives,
        best.true_negatives,
        best.false_negatives
    );
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

    let mut split_rng = FastRng::seed(args.seed);
    let (train_side, test_side) = split_dataset(&documents, args.train_percent, &mut split_rng)?;
    let run_config = args.to_run_config()?;

    // Echo the resolved config (locked decision §10.9: training echoes it).
    println!("resolved config: {:?}", run_config);

    let (engine, report) = run_single_experiment(&train_side, &test_side, &run_config)?;
    print_experiment_report(&args.preset_name, &report);

    if let Some(model_path) = &args.model_out {
        let artifact = ModelArtifact {
            preprocess_profile: run_config.profile,
            engine,
        };
        artifact.save_to_file(model_path)?;
        println!("saved model artifact to {}", model_path.display());
    }
    Ok(())
}

/// Batch mode: the §5 priority presets (raw, P0, P1, P2) on ONE identical
/// split with ONE identical training seed — the controlled-comparison
/// discipline of §8 made executable as a single command.
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
    let mut split_rng = FastRng::seed(args.seed);
    let (train_side, test_side) = split_dataset(&documents, args.train_percent, &mut split_rng)?;
    println!(
        "batch over {} train / {} test documents, seed {}",
        train_side.len(),
        test_side.len(),
        args.seed
    );

    for preset_name in ["raw", "p0", "p1", "p2"] {
        let mut run_config = args.to_run_config()?;
        run_config.profile = preset_from_name(preset_name)?;
        let (_engine, report) = run_single_experiment(&train_side, &test_side, &run_config)?;
        print_experiment_report(preset_name, &report);
    }
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

    let vote = artifact.engine.vote_sum(&processed)?;
    let lut = artifact.engine.build_probability_lut()?;
    let probability = lut.probability_for_report(vote)?;
    let label = vote > 0;

    println!(
        "prediction: {}",
        if label {
            "POSITIVE (1)"
        } else {
            "NEGATIVE (0)"
        }
    );
    println!("vote sum V = {vote}, probability {probability:.4}");

    // Explainability trace: fired clauses with decoded byte patterns and
    // source-span offsets INTO THE PREPROCESSED byte stream (§7.3).
    let mut rules_printed = 0usize;
    for clause in 0..artifact.engine.clause_count() {
        if rules_printed >= 10 {
            break;
        }
        let positions = artifact.engine.fired_window_positions(clause, &processed)?;
        if positions.is_empty() {
            continue;
        }
        let pattern = artifact.engine.describe_clause(clause, 12)?;
        if pattern.is_empty() {
            continue; // empty clause fires everywhere; not a meaningful rule
        }
        let polarity = if clause % 2 == 0 { "+" } else { "-" };
        println!("  [clause {clause} ({polarity})] {pattern} @ byte offsets {positions:?}");
        rules_printed += 1;
    }
    Ok(())
}

// ===========================================================================
// SECTION 12: M-ByteBag — Byte-N-Gram Vocabulary (flat-baseline foundation)
// ===========================================================================
//
// Scientific role (§8): the flat bag-of-byte-n-grams model is THE control
// that isolates "sub-word granularity" from "convolution." It shares the
// preprocessor, dataset loader, splitter, RNG, and sweep with ByteConvTM,
// so the two models differ in exactly one variable: positional windowing.
//
// This section provides the vocabulary layer only; the flat engine
// (ByteBagTM) builds on it in the next section. Design decisions of record:
// - Shingles are OVERLAPPING, STRIDE 1 (stride is a convolution concept;
//   a bag wants full coverage — this is not a tunable).
// - Documents shorter than n yield exactly ONE right-0x00-padded shingle,
//   consistent with the conv model's PAD rule (locked decision §10.4).
// - Vocabulary = the top-M most frequent shingles of the TRAINING split,
//   ordered deterministically: count DESCENDING, then bytes ASCENDING as
//   the tiebreak — identical corpus + config always yields an identical,
//   machine-independent vocabulary (the §8 reproducibility requirement).
// - Shingles outside the vocabulary are simply absent features (no OOV
//   token). The bag's coverage loss vs. the conv model's total coverage is
//   part of what the experiment measures — hiding it would corrupt the
//   comparison.

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
        self.validity_recheck()
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
    pub fn validity_recheck(&self) -> Result<(), GranmoModelError> {
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

// ===========================================================================
//  SECTION le main
// ===========================================================================

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();
    let outcome = if raw_args.len() <= 1 {
        run_self_check()
    } else {
        match CliArgs::parse(&raw_args) {
            Ok(args) => match args.mode.as_str() {
                "train" => handle_train(&args),
                "batch" => handle_batch(&args),
                "predict" => handle_predict(&args),
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
//  Tests
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
        assert_eq!(engine.vote_sum(b"any text at all").unwrap(), 0);
        assert!(!engine.predict(b"any text at all", 0).unwrap());
        engine.validate_internal_consistency().unwrap();
        // Embedding bitset: all 8 clause bits set on a fresh model.
        let bits = engine.fired_clause_bits(b"x").unwrap();
        assert_eq!(bits, vec![0b1111_1111u64]);
    }

    // --- ByteConvTM: mask semantics, order sensitivity, stride, padding ---

    #[test]
    fn positive_include_restricts_slot_and_reports_positions() {
        let mut engine = make_engine(2, 1, 2, false);
        // Clause 0: require byte 'a' at slot 0 and 'b' at slot 1.
        engine.test_force_include(0, engine.positive_local_index(0, usize::from(b'a')));
        engine.test_force_include(0, engine.positive_local_index(1, usize::from(b'b')));
        engine.validate_internal_consistency().unwrap();

        // "zab": windows at 0 ("za") and 1 ("ab") => fires only at offset 1.
        assert_eq!(engine.fired_window_positions(0, b"zab").unwrap(), vec![1]);
        // Order sensitivity: same bytes reversed must NOT fire.
        assert!(engine.fired_window_positions(0, b"zba").unwrap().is_empty());
    }

    #[test]
    fn stride_two_misses_odd_offset_pattern_stride_one_finds_it() {
        // Documents the known cost of S=2 recorded in the plan: patterns at
        // odd offsets are only visible at odd window starts.
        let mut s2 = make_engine(2, 2, 2, false);
        s2.test_force_include(0, s2.positive_local_index(0, usize::from(b'a')));
        s2.test_force_include(0, s2.positive_local_index(1, usize::from(b'b')));
        assert!(s2.fired_window_positions(0, b"zab").unwrap().is_empty());

        let mut s1 = make_engine(2, 1, 2, false);
        s1.test_force_include(0, s1.positive_local_index(0, usize::from(b'a')));
        s1.test_force_include(0, s1.positive_local_index(1, usize::from(b'b')));
        assert_eq!(s1.fired_window_positions(0, b"zab").unwrap(), vec![1]);
    }

    #[test]
    fn short_document_right_pads_with_zero() {
        let mut engine = make_engine(5, 1, 4, false);
        // Clause 0: requires PAD (0x00) at slot 4 — satisfied by a short doc.
        engine.test_force_include(0, engine.positive_local_index(4, 0));
        // Clause 2: forbids PAD at slot 2 (negated 0x00) — blocked by padding.
        engine.test_force_include(2, engine.negated_local_index(2, 0));
        engine.validate_internal_consistency().unwrap();

        // "hi" (2 bytes < K=5) => exactly one window: ['h','i',0,0,0].
        assert_eq!(engine.fired_window_positions(0, b"hi").unwrap(), vec![0]);
        assert!(engine.fired_window_positions(2, b"hi").unwrap().is_empty());
    }

    #[test]
    fn guarded_include_refuses_second_positive_at_slot() {
        // Guard ON: second positive include at the same slot is refused;
        // the mask stays single-byte, the clause stays alive.
        let mut guarded = make_engine(2, 1, 2, true);
        guarded.test_force_include(0, guarded.positive_local_index(0, usize::from(b'a')));
        guarded.test_force_include(0, guarded.positive_local_index(0, usize::from(b'b')));
        guarded.validate_internal_consistency().unwrap();
        assert_eq!(guarded.fired_window_positions(0, b"ax").unwrap(), vec![0]);

        // Guard OFF: both includes land; the slot mask is empty; the clause
        // is structurally dead (the defect the guard exists to prevent).
        let mut unguarded = make_engine(2, 1, 2, false);
        unguarded.test_force_include(0, unguarded.positive_local_index(0, usize::from(b'a')));
        unguarded.test_force_include(0, unguarded.positive_local_index(0, usize::from(b'b')));
        unguarded.validate_internal_consistency().unwrap();
        assert!(
            unguarded
                .fired_window_positions(0, b"ax")
                .unwrap()
                .is_empty()
        );
        assert!(
            unguarded
                .fired_window_positions(0, b"bx")
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
                engine.train_step(doc, label, &mut rng).unwrap();
            }
        }
        engine.validate_internal_consistency().unwrap();
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
                engine.train_step(doc, true, &mut rng).unwrap();
            }
            for doc in negatives {
                engine.train_step(doc, false, &mut rng).unwrap();
            }
        }
        engine.validate_internal_consistency().unwrap();

        for doc in positives {
            assert!(
                engine.predict(doc, 0).unwrap(),
                "positive doc misclassified: {:?}",
                core::str::from_utf8(doc)
            );
        }
        for doc in negatives {
            assert!(
                !engine.predict(doc, 0).unwrap(),
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
        engine.test_force_include(0, engine.positive_local_index(0, usize::from(b'n')));
        engine.test_force_include(0, engine.negated_local_index(1, 0)); // k1 ≠ 0x00
        let description = engine.describe_clause(0, 12).unwrap();
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
        lut.validity_recheck().unwrap();

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
        let mut engine = make_engine(5, 1, 16, false);
        let mut rng = FastRng::seed(11);
        let training_docs: [(&[u8], bool); 4] = [
            (b"very good movie", true),
            (b"not good movie", false),
            (b"it was very good", true),
            (b"it was not good", false),
        ];
        for _ in 0..50 {
            for (doc, label) in training_docs {
                engine.train_step(doc, label, &mut rng).unwrap();
            }
        }

        let artifact = ModelArtifact {
            preprocess_profile: PreprocessProfile::preset_p2(),
            engine,
        };
        let path = temp_artifact_path("granmo_roundtrip_test.gmb");
        artifact.save_to_file(&path).unwrap();
        let loaded = ModelArtifact::load_from_file(&path).unwrap();

        assert_eq!(
            loaded.preprocess_profile.get_bits().unwrap(),
            PreprocessProfile::preset_p2().get_bits().unwrap()
        );
        loaded.engine.validate_internal_consistency().unwrap();

        let probe_docs: [&[u8]; 5] = [
            b"very good movie",
            b"not good movie",
            b"something unrelated",
            b"hi",
            b"",
        ];
        for doc in probe_docs {
            assert_eq!(
                artifact.engine.vote_sum(doc).unwrap(),
                loaded.engine.vote_sum(doc).unwrap(),
                "vote divergence after round-trip on {:?}",
                core::str::from_utf8(doc)
            );
        }
    }

    /// Corruption detection: flipping ONE payload byte must fail the
    /// checksum gate — the artifact never reaches the parser.
    #[test]
    fn artifact_detects_single_byte_corruption() {
        let engine = make_engine(2, 1, 4, false);
        let artifact = ModelArtifact {
            preprocess_profile: PreprocessProfile::preset_raw(),
            engine,
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
    fn split_dataset_is_seeded_deterministic_and_validated() {
        let documents: Vec<LabeledDocument> = (0..10)
            .map(|i| LabeledDocument {
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
            patch_size: 5,
            stride: 1,
            n_clauses: 32,
            vote_threshold: 15,
            states_per_action: 100,
            specificity: 3.0,
            max_scan_bytes: 256,
            guarded_include: false,
            epochs: 400,
            seed: 42,
        };
        let (engine, report) = run_single_experiment(&documents, &documents, &config).unwrap();
        engine.validate_internal_consistency().unwrap();
        assert_eq!(report.test_count, 8);
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
        CliArgs::parse(&full)
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
        vocab.validity_recheck().unwrap();
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
}
