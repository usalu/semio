//! 🌀️ Zero-dependency information theory: entropies, estimators, divergences, mutual
//! information, information dynamics, and streaming state. All internal computation happens in
//! nats; [`LogBase`] conversion is applied once at the public API boundary. Every non-trivial
//! estimate is estimated from finite data (not a closed-form fact about a given distribution)
//! and therefore returns an [`Estimate`] carrying diagnostics rather than a bare `f64`.
//!
//! Moved verbatim from `🧰️framework/🔨️modules/🧮️math/🎲️entropy` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3e: Rust-only compute internals
//! backing `📊️table`'s inferences (no TS twin — this is algorithm, not boundary vocabulary), mirroring
//! `📊️statistics-internals`'s shape. Fully self-contained (zero external crate deps, zero coupling to
//! any other math sibling) — the internal `spatial`/`graph` submodules below are this module's OWN
//! entropy-over-pixels / entropy-over-edge-lists measures, not references to a sibling `spatial` or
//! `graph` math module. See `💡️inferences/🎲entropy` for the one genuine `InferredField` wiring.

// #region 🔖️Errors
/// 🚨️ Every way an entropy/information computation can fail to produce a result. Kept flat (no
/// nested `source()` chain, no external error crate) so callers can match exhaustively.
#[derive(Clone, PartialEq, Debug)]
pub enum EntropyError {
    /// 🚨️ A configuration value failed validation (`field` names the offending knob).
    InvalidConfig { field: &'static str, reason: &'static str },
    /// 🚨️ An input slice/collection required at least one element but had none.
    EmptyInput { what: &'static str },
    /// 🚨️ Two inputs that must have equal length disagreed.
    LengthMismatch { expected: usize, actual: usize },
    /// 🚨️ An input's shape (e.g. `width * height` vs slice length) did not match what was declared.
    ShapeMismatch { what: &'static str, expected: usize, actual: usize },
    /// 🚨️ A `NaN`/`Inf` value was found where [`MissingPolicy::Error`] rejects it.
    NonFinite { what: &'static str, index: usize },
    /// 🚨️ A probability-mass entry was negative beyond floating-point noise.
    InvalidProbability { index: usize, value: f64 },
    /// 🚨️ A probability vector did not sum to 1 within tolerance and auto-renormalization was
    /// declined (see [`Tolerances::renormalize_sum`]).
    NotNormalized { sum: f64 },
    /// 🚨️ Too few samples remained to satisfy a method's minimum requirement.
    InsufficientData { what: &'static str, needed: usize, actual: usize },
    /// 🚨️ The requested quantity is mathematically undefined for the given inputs (e.g. Rényi at
    /// `alpha == 1` requested without taking the Shannon limit).
    UndefinedResult { reason: &'static str },
    /// 🚨️ An input that must vary (non-constant) was constant, making the method inapplicable.
    DegenerateInput { what: &'static str },
    /// 🚨️ An iterative numerical method did not converge within its iteration budget.
    NotConverged { what: &'static str, iterations: usize },
    /// 🚨️ A [`FeatureRegistry`] lookup referenced a name that was never registered.
    UnknownFeature { name: String },
}

impl core::fmt::Display for EntropyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidConfig { field, reason } => write!(f, "invalid config field `{field}`: {reason}"),
            Self::EmptyInput { what } => write!(f, "empty input: {what}"),
            Self::LengthMismatch { expected, actual } => {
                write!(f, "length mismatch: expected {expected}, found {actual}")
            }
            Self::ShapeMismatch { what, expected, actual } => {
                write!(f, "shape mismatch for {what}: expected {expected}, found {actual}")
            }
            Self::NonFinite { what, index } => write!(f, "non-finite value in {what} at index {index}"),
            Self::InvalidProbability { index, value } => {
                write!(f, "invalid probability at index {index}: {value}")
            }
            Self::NotNormalized { sum } => write!(f, "probabilities sum to {sum}, expected 1"),
            Self::InsufficientData { what, needed, actual } => {
                write!(f, "insufficient data for {what}: needed at least {needed}, found {actual}")
            }
            Self::UndefinedResult { reason } => write!(f, "undefined result: {reason}"),
            Self::DegenerateInput { what } => write!(f, "degenerate input: {what}"),
            Self::NotConverged { what, iterations } => {
                write!(f, "{what} did not converge after {iterations} iterations")
            }
            Self::UnknownFeature { name } => write!(f, "unknown feature: {name}"),
        }
    }
}

impl std::error::Error for EntropyError {}
// #endregion 🔖️Errors

// #region 🔖️Units
/// 📏️ Unit of information a value/[`Estimate`] is expressed in. Internal math is always in nats;
/// conversion to/from a chosen base happens only at the public API boundary.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LogBase {
    /// 📏️ Natural log, base `e` (the internal computation unit).
    Nats,
    /// 📏️ Base 2 — the conventional "bits" of Shannon's original paper.
    Bits,
    /// 📏️ Base 10.
    Hartleys,
    /// 📏️ An arbitrary positive base other than 1.
    Base(f64),
}

impl LogBase {
    /// 📏️ Natural logarithm of this base's numeric value (`ln(base)`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn ln(self) -> f64 {
        match self {
            Self::Nats => 1.0,
            Self::Bits => core::f64::consts::LN_2,
            Self::Hartleys => core::f64::consts::LN_10,
            Self::Base(b) => b.ln(),
        }
    }

    /// 📏️ Validates that a custom base is usable (`b > 0`, `b != 1`, finite).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn validate(self) -> Result<(), EntropyError> {
        if let Self::Base(b) = self {
            if !b.is_finite() || b <= 0.0 || (b - 1.0).abs() < 1e-15 {
                return Err(EntropyError::InvalidConfig { field: "log_base", reason: "custom logarithm base must be finite, positive, and not equal to 1" });
            }
        }
        Ok(())
    }

    /// 📏️ Converts a value already expressed in nats into this base.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_nats(self, nats: f64) -> f64 {
        nats / self.ln()
    }

    /// 📏️ Converts a value expressed in this base into nats.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_nats(self, value: f64) -> f64 {
        value * self.ln()
    }

    /// 📏️ Converts `value` from one base to another without an intermediate caller-visible step.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn convert(value: f64, from: LogBase, to: LogBase) -> f64 {
        to.from_nats(from.to_nats(value))
    }
}
// #endregion 🔖️Units

// #region 🔖️Estimate
/// 📦️ A `(lower, upper)` interval at a stated confidence `level` (e.g. `0.95`).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ConfidenceInterval {
    pub lower: f64,
    pub upper: f64,
    pub level: f64,
}

/// ⚠️ Non-fatal quality flags accumulated while producing an [`Estimate`]. Their presence never
/// changes `value`; they exist so a caller can decide whether to trust it.
#[derive(Clone, PartialEq, Debug)]
pub enum Warning {
    /// ⚠️ Sample count is below the method's recommended minimum for reliable results.
    SmallSample { n: usize, recommended: usize },
    /// ⚠️ The occupied support is a small fraction of the declared alphabet/bin count.
    Undersampled { occupied_bins: usize, total_bins: usize },
    /// ⚠️ A bias correction pushed the raw estimate below zero; the reported value was clamped.
    ClippedNegative,
    /// ⚠️ Ties were encountered where the method assumes strict ordering and were broken by policy.
    TiesBroken { count: usize },
    /// ⚠️ An iterative refinement stopped at its soft iteration cap without the tight convergence
    /// check succeeding, but the result is still usable.
    NotConvergedSoft { what: &'static str },
    /// ⚠️ A surrogate/permutation test could not distinguish the statistic from the null at the
    /// requested significance.
    SurrogatesInconclusive { p_value: f64 },
}

/// 📦️ The result of any estimation performed from finite data. `value` is always in `base`;
/// [`Estimate::in_base`] converts the whole struct (value, std error, CI) to a different unit.
#[derive(Clone, PartialEq, Debug)]
pub struct Estimate {
    pub value: f64,
    pub base: LogBase,
    pub method: &'static str,
    /// 📦️ Raw number of samples consumed before any weighting/embedding/deletion.
    pub n: usize,
    /// 📦️ Effective sample size after weights, embedding, or pairwise deletion are accounted for.
    pub n_effective: f64,
    pub std_error: Option<f64>,
    pub ci: Option<ConfidenceInterval>,
    pub warnings: Vec<Warning>,
    /// 📦️ Open key/value diagnostics (e.g. `("bins", 12.0)`, `("bandwidth", 0.41)`).
    pub diagnostics: Vec<(&'static str, f64)>,
}

impl Estimate {
    /// 📦️ Returns a copy of this estimate with `value`/`std_error`/`ci` converted to `base`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn in_base(&self, base: LogBase) -> Estimate {
        let convert = |v: f64| LogBase::convert(v, self.base, base);
        Estimate {
            value: convert(self.value),
            base,
            method: self.method,
            n: self.n,
            n_effective: self.n_effective,
            std_error: self.std_error.map(convert),
            ci: self.ci.map(|ci| ConfidenceInterval { lower: convert(ci.lower), upper: convert(ci.upper), level: ci.level }),
            warnings: self.warnings.clone(),
            diagnostics: self.diagnostics.clone(),
        }
    }

    /// 📦️ Value converted to bits.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn bits(&self) -> f64 {
        LogBase::convert(self.value, self.base, LogBase::Bits)
    }

    /// 📦️ Value converted to nats.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn nats(&self) -> f64 {
        LogBase::convert(self.value, self.base, LogBase::Nats)
    }
}
// #endregion 🔖️Estimate

// #region 🔖️Policies
/// 🧭️ How missing (`NaN`) values are handled by an estimator that accepts raw `f64` data.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum MissingPolicy {
    /// 🧭️ Any missing value is a hard [`EntropyError::NonFinite`].
    #[default]
    Error,
    /// 🧭️ Missing values are dropped from a single sequence before estimation.
    Skip,
    /// 🧭️ In a paired/joint computation, a row is dropped only if it is missing in a way that
    /// makes that specific pair unusable (as opposed to a listwise drop across all variables).
    PairwiseSkip,
}

/// 🧭️ The tolerance radius `r` used by regularity measures (ApEn/SampEn/FuzzyEn family).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tolerance {
    /// 🧭️ A fixed absolute radius.
    Absolute(f64),
    /// 🧭️ A radius expressed as a multiple of the series' sample standard deviation.
    RelativeToSd(f64),
    /// 🧭️ The literature-default `0.2 * sd`.
    Auto,
}

/// 🧭️ How histogram bin edges are chosen for a continuous plug-in estimator.
#[derive(Clone, PartialEq, Debug)]
pub enum BinsSpec {
    Fixed(usize),
    Sturges,
    Scott,
    FreedmanDiaconis,
    Doane,
    Edges(Vec<f64>),
}

/// 🧭️ Distance metric used by kNN-based estimators. A closed set dispatched by `match`, not a
/// trait — every implementation is exhaustively covered and testable.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Metric {
    #[default]
    Chebyshev,
    Euclidean,
    Manhattan,
}

/// 🧭️ How ties are broken when a method (ordinal patterns, spacing estimators) assumes a strict
/// total order over samples that may coincide exactly.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TiePolicy {
    /// 🧭️ Any tie is a hard error.
    Error,
    /// 🧭️ Ties are broken by original index order (stable, deterministic, no randomness).
    #[default]
    StableRank,
    /// 🧭️ Tied entries are excluded from the affected pattern/window rather than ordered.
    Jitterless,
}

/// 🧭️ How a divergence handles support mismatch (`p_i > 0` where `q_i == 0`). Default is
/// mathematical honesty: [`Smoothing::None`] returns `f64::INFINITY`, it never smooths silently.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Smoothing {
    #[default]
    None,
    /// 🧭️ Add `epsilon` to every cell of `q` and renormalize before comparing.
    Additive(f64),
    /// 🧭️ Mix `q <- (1 - lambda) * q + lambda * uniform` before comparing.
    Jeffreys(f64),
}

/// 🧭️ Overridable numerical-hygiene tolerances threaded through configs that need them. Defaults
/// match the crate-wide constants documented in `numeric.rs`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tolerances {
    /// 🧭️ Maximum `|sum(p) - 1|` that is silently renormalized rather than rejected.
    pub renormalize_sum: f64,
    /// 🧭️ Negative probability mass more negative than this is rejected outright; less negative
    /// is clamped to zero then renormalized.
    pub negative_probability: f64,
}

impl Default for Tolerances {
    fn default() -> Self {
        Self { renormalize_sum: 1e-8, negative_probability: -1e-12 }
    }
}
// #endregion 🔖️Policies

// #region 🔖️Numeric
#[path = "➕️numeric/🦀️.rs"]
pub mod numeric;
// #endregion 🔖️Numeric

// #region 🔖️Counts
#[path = "🧮️counts/🦀️.rs"]
pub mod counts;
// #endregion 🔖️Counts

// #region 🔖️Discrete
#[path = "🎲️discrete/🦀️.rs"]
pub mod discrete;
// #endregion 🔖️Discrete

// #region 🔖️Estimators
#[path = "📏️estimators/🦀️.rs"]
pub mod estimators;
// #endregion 🔖️Estimators

// #region 🔖️Knn
#[path = "📍️knn/🦀️.rs"]
pub mod knn;
// #endregion 🔖️Knn

// #region 🔖️Continuous
#[path = "〰️continuous/🦀️.rs"]
pub mod continuous;
// #endregion 🔖️Continuous

// #region 🔖️Divergence
// #endregion 🔖️Divergence

// #region 🔖️Mutual
// #endregion 🔖️Mutual

// #region 🔖️Pid
// #endregion 🔖️Pid

// #region 🔖️Fisher
// #endregion 🔖️Fisher

// #region 🔖️Symbolic
#[path = "🔠️symbolic/🦀️.rs"]
pub mod symbolic;
// #endregion 🔖️Symbolic

// #region 🔖️Regularity
// #endregion 🔖️Regularity

// #region 🔖️Ordinal
// #endregion 🔖️Ordinal

// #region 🔖️Markov
// #endregion 🔖️Markov

// #region 🔖️Multiscale
// #endregion 🔖️Multiscale

// #region 🔖️Lz
// #endregion 🔖️Lz

// #region 🔖️Fft
// #endregion 🔖️Fft

// #region 🔖️Spectral
// #endregion 🔖️Spectral

// #region 🔖️Wavelet
// #endregion 🔖️Wavelet

// #region 🔖️Matrix
// #endregion 🔖️Matrix

// #region 🔖️Inference
// #endregion 🔖️Inference

// #region 🔖️Transfer
// #endregion 🔖️Transfer

// #region 🔖️Spatial
// #endregion 🔖️Spatial

// #region 🔖️Graph
// #endregion 🔖️Graph

// #region 🔖️Ml
// #endregion 🔖️Ml

// #region 🔖️Streaming
// #endregion 🔖️Streaming

// #region 🔖️Features
// #endregion 🔖️Features
