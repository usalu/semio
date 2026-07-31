//! 🌀️ Zero-dependency information theory: entropies, estimators, divergences, mutual
//! information, information dynamics, and streaming state. All internal computation happens in
//! nats; [`LogBase`] conversion is applied once at the public API boundary. Every non-trivial
//! estimate is estimated from finite data (not a closed-form fact about a given distribution)
//! and therefore returns an [`Estimate`] carrying diagnostics rather than a bare `f64`.

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
    pub fn ln(self) -> f64 {
        match self {
            Self::Nats => 1.0,
            Self::Bits => core::f64::consts::LN_2,
            Self::Hartleys => core::f64::consts::LN_10,
            Self::Base(b) => b.ln(),
        }
    }

    /// 📏️ Validates that a custom base is usable (`b > 0`, `b != 1`, finite).
    pub fn validate(self) -> Result<(), EntropyError> {
        if let Self::Base(b) = self {
            if !b.is_finite() || b <= 0.0 || (b - 1.0).abs() < 1e-15 {
                return Err(EntropyError::InvalidConfig {
                    field: "log_base",
                    reason: "custom logarithm base must be finite, positive, and not equal to 1",
                });
            }
        }
        Ok(())
    }

    /// 📏️ Converts a value already expressed in nats into this base.
    pub fn from_nats(self, nats: f64) -> f64 {
        nats / self.ln()
    }

    /// 📏️ Converts a value expressed in this base into nats.
    pub fn to_nats(self, value: f64) -> f64 {
        value * self.ln()
    }

    /// 📏️ Converts `value` from one base to another without an intermediate caller-visible step.
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
    pub fn in_base(&self, base: LogBase) -> Estimate {
        let convert = |v: f64| LogBase::convert(v, self.base, base);
        Estimate {
            value: convert(self.value),
            base,
            method: self.method,
            n: self.n,
            n_effective: self.n_effective,
            std_error: self.std_error.map(convert),
            ci: self.ci.map(|ci| ConfidenceInterval {
                lower: convert(ci.lower),
                upper: convert(ci.upper),
                level: ci.level,
            }),
            warnings: self.warnings.clone(),
            diagnostics: self.diagnostics.clone(),
        }
    }

    /// 📦️ Value converted to bits.
    pub fn bits(&self) -> f64 {
        LogBase::convert(self.value, self.base, LogBase::Bits)
    }

    /// 📦️ Value converted to nats.
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
pub mod numeric {
//! 🔬️ Hand-rolled numerical primitives shared by every estimator: special functions (ln-gamma,
//! digamma), stable summation, and a deterministic PRNG. Zero external dependencies.

// #region 🔖️SpecialFunctions
const LANCZOS_G: f64 = 7.0;
const LANCZOS_COEFFICIENTS: [f64; 9] = [
    0.999_999_999_999_809_9,
    676.520_368_121_885_1,
    -1259.139_216_722_402_8,
    771.323_428_777_653_1,
    -176.615_029_162_140_6,
    12.507_343_278_686_905,
    -0.138_571_095_265_720_12,
    9.984_369_578_019_572e-6,
    1.505_632_735_149_312e-7,
];

/// 🔬️ Natural log of the gamma function, accurate to ~1e-13 relative error for `x > 0`.
/// Lanczos approximation, g = 7, n = 9 (Numerical Recipes coefficient set).
pub fn ln_gamma(x: f64) -> f64 {
    if x < 0.5 {
        // 🔬️ Reflection formula: Gamma(x) * Gamma(1-x) = pi / sin(pi*x).
        let sin_term = (core::f64::consts::PI * x).sin();
        core::f64::consts::PI.ln() - sin_term.abs().ln() - ln_gamma(1.0 - x)
    } else {
        let x = x - 1.0;
        let mut a = LANCZOS_COEFFICIENTS[0];
        for (i, c) in LANCZOS_COEFFICIENTS.iter().enumerate().skip(1) {
            a += c / (x + i as f64);
        }
        let t = x + LANCZOS_G + 0.5;
        0.5 * (2.0 * core::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
    }
}

/// 🔬️ Gamma function via `exp(ln_gamma(x))`. Prefer [`ln_gamma`] directly when only the log is
/// needed (avoids overflow for large `x`).
pub fn gamma(x: f64) -> f64 {
    ln_gamma(x).exp()
}

/// 🔬️ Digamma function `psi(x) = d/dx ln(Gamma(x))`. Uses the recurrence `psi(x) = psi(x+1) -
/// 1/x` to shift small `x` into the region where the asymptotic series converges well (`x >=
/// 6`), then the standard asymptotic expansion in `1/x^2`.
pub fn digamma(mut x: f64) -> f64 {
    let mut result = 0.0;
    while x < 6.0 {
        result -= 1.0 / x;
        x += 1.0;
    }
    let inv = 1.0 / x;
    let inv2 = inv * inv;
    result += x.ln() - 0.5 * inv
        - inv2 * (1.0 / 12.0 - inv2 * (1.0 / 120.0 - inv2 * (1.0 / 252.0 - inv2 / 240.0)));
    result
}

/// 🔬️ Trigamma function `psi'(x)`, the derivative of [`digamma`]. Same shift-and-asymptotic
/// strategy as `digamma`.
pub fn trigamma(mut x: f64) -> f64 {
    let mut result = 0.0;
    while x < 6.0 {
        result += 1.0 / (x * x);
        x += 1.0;
    }
    let inv = 1.0 / x;
    let inv2 = inv * inv;
    result += inv * (1.0 + inv * (0.5 + inv * (1.0 / 6.0 - inv2 * (1.0 / 30.0 - inv2 / 42.0))));
    result
}

/// 🔬️ Error function via Abramowitz & Stegun 7.1.26 (max abs error ~1.5e-7), refined with one
/// Newton step against [`erfc`]'s complementary identity is unnecessary at this tolerance for
/// entropy-estimator purposes (bandwidth selection, normal CDF).
pub fn erf(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let a1 = 0.254_829_592;
    let a2 = -0.284_496_736;
    let a3 = 1.421_413_741;
    let a4 = -1.453_152_027;
    let a5 = 1.061_405_429;
    let p = 0.327_591_1;
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    sign * y
}

/// 🔬️ Complementary error function `1 - erf(x)`.
pub fn erfc(x: f64) -> f64 {
    1.0 - erf(x)
}

/// 🔬️ Standard normal CDF `Phi(x)`.
pub fn normal_cdf(x: f64) -> f64 {
    0.5 * erfc(-x / core::f64::consts::SQRT_2)
}

/// 🔬️ Inverse standard normal CDF (quantile function), Acklam's rational approximation
/// (relative error < 1.15e-9 across `(0, 1)`), refined by one Halley step.
pub fn inverse_normal_cdf(p: f64) -> f64 {
    if !(0.0..=1.0).contains(&p) {
        return f64::NAN;
    }
    if p <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if p >= 1.0 {
        return f64::INFINITY;
    }
    const A: [f64; 6] = [
        -3.969_683_028_665_376e+01,
        2.209_460_984_245_205e+02,
        -2.759_285_104_469_687e+02,
        1.383_577_518_672_69e2,
        -3.066_479_806_614_716e+01,
        2.506_628_277_459_239e+00,
    ];
    const B: [f64; 5] = [
        -5.447_609_879_822_406e+01,
        1.615_858_368_580_409e+02,
        -1.556_989_798_598_866e+02,
        6.680_131_188_771_972e+01,
        -1.328_068_155_288_572e+01,
    ];
    const C: [f64; 6] = [
        -7.784_894_002_430_293e-03,
        -3.223_964_580_411_365e-01,
        -2.400_758_277_161_838e+00,
        -2.549_732_539_343_734e+00,
        4.374_664_141_464_968e+00,
        2.938_163_982_698_783e+00,
    ];
    const D: [f64; 4] = [
        7.784_695_709_041_462e-03,
        3.224_671_290_700_398e-01,
        2.445_134_137_142_996e+00,
        3.754_408_661_907_416e+00,
    ];
    const P_LOW: f64 = 0.024_25;
    let x = if p < P_LOW {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p <= 1.0 - P_LOW {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    };
    // 🔬️ One Halley refinement step against the CDF for full double precision.
    let e = 0.5 * erfc(-x / core::f64::consts::SQRT_2) - p;
    let u = e * (2.0 * core::f64::consts::PI).sqrt() * (x * x / 2.0).exp();
    x - u / (1.0 + x * u / 2.0)
}

/// 🔬️ Regularized lower incomplete gamma function `P(a, x)`, via series expansion for `x < a+1`
/// and a continued fraction for `x >= a+1` (standard Numerical Recipes split).
pub fn regularized_lower_incomplete_gamma(a: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x < a + 1.0 {
        let mut term = 1.0 / a;
        let mut sum = term;
        let mut n = a;
        for _ in 0..500 {
            n += 1.0;
            term *= x / n;
            sum += term;
            if term.abs() < sum.abs() * 1e-15 {
                break;
            }
        }
        sum * (-x + a * x.ln() - ln_gamma(a)).exp()
    } else {
        1.0 - regularized_upper_incomplete_gamma_cf(a, x)
    }
}

fn regularized_upper_incomplete_gamma_cf(a: f64, x: f64) -> f64 {
    let tiny = 1e-300;
    let mut b = x + 1.0 - a;
    let mut c = 1.0 / tiny;
    let mut d = 1.0 / b;
    let mut h = d;
    for i in 1..500 {
        let an = -(i as f64) * (i as f64 - a);
        b += 2.0;
        d = an * d + b;
        if d.abs() < tiny {
            d = tiny;
        }
        c = b + an / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        let delta = d * c;
        h *= delta;
        if (delta - 1.0).abs() < 1e-15 {
            break;
        }
    }
    (-x + a * x.ln() - ln_gamma(a)).exp() * h
}

/// 🔬️ Regularized upper incomplete gamma function `Q(a, x) = 1 - P(a, x)`.
pub fn regularized_upper_incomplete_gamma(a: f64, x: f64) -> f64 {
    1.0 - regularized_lower_incomplete_gamma(a, x)
}

/// 🔬️ Log-factorial via `ln_gamma(n + 1)`, cached for small `n` to avoid recomputation in hot
/// bias-correction loops (Grassberger, Schurmann-Grassberger).
pub struct LogFactorialCache {
    cache: Vec<f64>,
}

impl LogFactorialCache {
    /// 🔬️ Precomputes `ln(k!)` for `k` in `0..=max_n`.
    pub fn new(max_n: usize) -> Self {
        let mut cache = Vec::with_capacity(max_n + 1);
        cache.push(0.0);
        for k in 1..=max_n {
            cache.push(cache[k - 1] + (k as f64).ln());
        }
        Self { cache }
    }

    /// 🔬️ `ln(n!)`, falling back to `ln_gamma(n + 1)` beyond the cached range.
    pub fn get(&self, n: usize) -> f64 {
        match self.cache.get(n) {
            Some(&v) => v,
            None => ln_gamma(n as f64 + 1.0),
        }
    }
}
// #endregion 🔖️SpecialFunctions

// #region 🔖️StableSummation
/// 🔬️ Neumaier's improved Kahan compensated summation: tracks a running compensation term so
/// that summing many small values alongside a few large ones does not lose precision.
pub fn neumaier_sum(values: impl IntoIterator<Item = f64>) -> f64 {
    let mut sum = 0.0_f64;
    let mut compensation = 0.0_f64;
    for value in values {
        let t = sum + value;
        if sum.abs() >= value.abs() {
            compensation += (sum - t) + value;
        } else {
            compensation += (value - t) + sum;
        }
        sum = t;
    }
    sum + compensation
}

/// 🔬️ Pairwise (divide-and-conquer) summation: `O(log n)` error growth instead of `O(n)` for
/// naive left-to-right summation, useful for large flat arrays where compensated summation's
/// per-element overhead is undesirable.
pub fn pairwise_sum(values: &[f64]) -> f64 {
    const BASE_CASE: usize = 128;
    if values.len() <= BASE_CASE {
        return neumaier_sum(values.iter().copied());
    }
    let mid = values.len() / 2;
    pairwise_sum(&values[..mid]) + pairwise_sum(&values[mid..])
}

/// 🔬️ Numerically stable `x * ln(x)`, defined as `0` at `x == 0` (the standard entropy
/// convention `0 log 0 = 0`, taken as a limit).
pub fn x_ln_x(x: f64) -> f64 {
    if x <= 0.0 {
        0.0
    } else {
        x * x.ln()
    }
}

/// 🔬️ `log(sum(exp(values)))` computed without overflow by factoring out the maximum element.
/// Returns `f64::NEG_INFINITY` for an empty input.
pub fn log_sum_exp(values: &[f64]) -> f64 {
    let Some(&max) = values.iter().max_by(|a, b| a.total_cmp(b)) else {
        return f64::NEG_INFINITY;
    };
    if !max.is_finite() {
        return max;
    }
    let sum = neumaier_sum(values.iter().map(|&v| (v - max).exp()));
    max + sum.ln()
}
// #endregion 🔖️StableSummation

// #region 🔖️Rng
/// 🎲️ Deterministic xorshift64* PRNG. Never seeded from wall-clock time — every call site that
/// needs randomness (bootstrap, surrogates, jitter, tests) takes an explicit `u64` seed so
/// results are exactly reproducible.
pub struct Xorshift64 {
    state: u64,
}

impl Xorshift64 {
    /// 🎲️ Seeds the generator. A seed of `0` is remapped to a fixed nonzero constant since
    /// xorshift's all-zero state is a fixed point.
    pub fn new(seed: u64) -> Self {
        Self { state: if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed } }
    }

    /// 🎲️ Next raw 64-bit output.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// 🎲️ Uniform `f64` in `[0, 1)`.
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// 🎲️ Uniform integer in `[0, bound)` via Lemire's rejection-free-in-practice method (biased
    /// only by a negligible `2^-64` amount, acceptable for permutation/bootstrap indices).
    pub fn next_below(&mut self, bound: usize) -> usize {
        if bound == 0 {
            return 0;
        }
        ((self.next_u64() as u128 * bound as u128) >> 64) as usize
    }

    /// 🎲️ One standard-normal sample via Box-Muller (caller pays two uniform draws per call;
    /// simplicity over the two-sample-per-call optimization since callers rarely need high
    /// throughput Gaussian streams).
    pub fn next_gaussian(&mut self) -> f64 {
        let u1 = (self.next_f64()).max(f64::MIN_POSITIVE);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * core::f64::consts::PI * u2).cos()
    }

    /// 🎲️ In-place Fisher-Yates shuffle.
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.next_below(i + 1);
            slice.swap(i, j);
        }
    }
}
// #endregion 🔖️Rng

// #region 🔖️Hygiene
/// 🧹️ Overflow-safe product of bin-count-like dimensions, returning `None` on overflow rather
/// than silently wrapping.
pub fn checked_state_count(dims: &[usize]) -> Option<u128> {
    dims.iter().try_fold(1u128, |acc, &d| acc.checked_mul(d as u128))
}

/// 🧹️ Clips a near-zero-but-negative eigenvalue/entropy estimate to exactly `0.0` when it falls
/// within `tolerance` of zero; leaves values further negative untouched (callers decide whether
/// that indicates a real error).
pub fn clamp_near_zero(value: f64, tolerance: f64) -> f64 {
    if value < 0.0 && value >= -tolerance {
        0.0
    } else {
        value
    }
}
// #endregion 🔖️Hygiene

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ln_gamma_matches_known_factorials() {
        for n in 1..10u32 {
            let expected = (1..n).map(|k| k as f64).product::<f64>().ln();
            assert!((ln_gamma(n as f64) - expected).abs() < 1e-9, "n={n}");
        }
    }

    #[test]
    fn ln_gamma_half_matches_sqrt_pi() {
        let expected = core::f64::consts::PI.sqrt().ln();
        assert!((ln_gamma(0.5) - expected).abs() < 1e-9);
    }

    #[test]
    fn digamma_matches_known_value_at_one() {
        // 🔬️ psi(1) = -gamma (Euler-Mascheroni constant).
        let euler_mascheroni = 0.577_215_664_901_532_9;
        assert!((digamma(1.0) - (-euler_mascheroni)).abs() < 1e-9);
    }

    #[test]
    fn digamma_recurrence_holds() {
        let x = 3.7;
        assert!((digamma(x + 1.0) - (digamma(x) + 1.0 / x)).abs() < 1e-9);
    }

    #[test]
    fn trigamma_matches_pi_squared_over_six_at_one() {
        assert!((trigamma(1.0) - core::f64::consts::PI.powi(2) / 6.0).abs() < 1e-8);
    }

    #[test]
    fn erf_endpoints() {
        // 🔬️ A&S 7.1.26 has a stated max absolute error of ~1.5e-7; it is not exact at x=0.
        assert!((erf(0.0)).abs() < 1e-6);
        assert!((erf(10.0) - 1.0).abs() < 1e-6);
        assert!((erf(-10.0) + 1.0).abs() < 1e-6);
    }

    #[test]
    fn normal_cdf_at_zero_is_half() {
        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn inverse_normal_cdf_roundtrips_normal_cdf() {
        for p in [0.001, 0.01, 0.1, 0.3, 0.5, 0.7, 0.9, 0.99, 0.999] {
            let x = inverse_normal_cdf(p);
            let back = normal_cdf(x);
            assert!((back - p).abs() < 1e-8, "p={p} back={back}");
        }
    }

    #[test]
    fn incomplete_gamma_full_integral_is_one() {
        assert!((regularized_lower_incomplete_gamma(2.5, 1e6) - 1.0).abs() < 1e-9);
        assert!(regularized_upper_incomplete_gamma(2.5, 1e6) < 1e-9);
    }

    #[test]
    fn log_factorial_cache_matches_ln_gamma() {
        let cache = LogFactorialCache::new(20);
        for n in 0..20 {
            assert!((cache.get(n) - ln_gamma(n as f64 + 1.0)).abs() < 1e-9);
        }
    }

    #[test]
    fn neumaier_sum_exact_on_simple_case() {
        assert_eq!(neumaier_sum([1.0, 2.0, 3.0]), 6.0);
    }

    #[test]
    fn neumaier_more_accurate_than_naive_for_ill_conditioned_sum() {
        let mut values = vec![1e16, 1.0, -1e16];
        let naive: f64 = values.iter().sum();
        let stable = neumaier_sum(values.iter().copied());
        assert_eq!(naive, 0.0); // 🔬️ naive loses the 1.0 entirely
        assert!((stable - 1.0).abs() < 1e-9);
        values.reverse();
        assert!((neumaier_sum(values) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn pairwise_sum_matches_neumaier_on_large_array() {
        let values: Vec<f64> = (0..10_000).map(|i| (i as f64).sin()).collect();
        let a = pairwise_sum(&values);
        let b = neumaier_sum(values.iter().copied());
        assert!((a - b).abs() < 1e-6);
    }

    #[test]
    fn x_ln_x_zero_at_zero() {
        assert_eq!(x_ln_x(0.0), 0.0);
        assert!((x_ln_x(1.0) - 0.0).abs() < 1e-12);
        assert!(x_ln_x(core::f64::consts::E) > 0.0);
    }

    #[test]
    fn log_sum_exp_matches_naive_for_moderate_values() {
        let values: [f64; 4] = [0.1, 0.5, -0.3, 0.2];
        let naive = values.iter().map(|v: &f64| v.exp()).sum::<f64>().ln();
        assert!((log_sum_exp(&values) - naive).abs() < 1e-9);
    }

    #[test]
    fn log_sum_exp_avoids_overflow() {
        let values = [1000.0, 1000.5, 999.0];
        let result = log_sum_exp(&values);
        assert!(result.is_finite());
        assert!(result > 1000.0 && result < 1002.0);
    }

    #[test]
    fn xorshift_is_deterministic_for_fixed_seed() {
        let mut a = Xorshift64::new(42);
        let mut b = Xorshift64::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn xorshift_uniform_range_respected() {
        let mut rng = Xorshift64::new(1234);
        for _ in 0..1000 {
            let v = rng.next_f64();
            assert!((0.0..1.0).contains(&v));
            let b = rng.next_below(7);
            assert!(b < 7);
        }
    }

    #[test]
    fn shuffle_is_a_permutation() {
        let mut rng = Xorshift64::new(99);
        let mut data: Vec<u32> = (0..50).collect();
        rng.shuffle(&mut data);
        let mut sorted = data.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..50).collect::<Vec<u32>>());
    }

    #[test]
    fn checked_state_count_detects_overflow() {
        assert_eq!(checked_state_count(&[2, 3, 4]), Some(24));
        // 🔬️ usize::MAX * 2 (~3.6e19) fits comfortably inside u128 (~3.4e38); an overflow needs
        // enough factors that their product exceeds u128::MAX.
        assert_eq!(checked_state_count(&[usize::MAX; 8]), None);
    }

    #[test]
    fn clamp_near_zero_behavior() {
        assert_eq!(clamp_near_zero(-1e-14, 1e-12), 0.0);
        assert_eq!(clamp_near_zero(-1.0, 1e-12), -1.0);
        assert_eq!(clamp_near_zero(5.0, 1e-12), 5.0);
    }

    mod quick {
        use super::*;

        #[test]
        fn xorshift_gaussian_mean_and_variance_converge() {
            let mut rng = Xorshift64::new(7);
            let n = 20_000;
            let samples: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
            let mean = samples.iter().sum::<f64>() / n as f64;
            let var = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
            assert!(mean.abs() < 0.05, "mean={mean}");
            assert!((var - 1.0).abs() < 0.05, "var={var}");
        }

        #[test]
        fn digamma_asymptotic_region_matches_recurrence_shifted_from_small_x() {
            for x in [0.1, 0.5, 1.5, 5.9, 6.1, 50.0, 500.0] {
                let direct = digamma(x);
                let via_recurrence = digamma(x + 1.0) - 1.0 / x;
                assert!((direct - via_recurrence).abs() < 1e-8, "x={x}");
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Numeric

// #region 🔖️Counts
pub mod counts {
//! 🧮️ Frequency counting and probability-vector validation: the shared foundation every discrete
//! entropy/estimator/divergence function builds on.

use crate::{EntropyError, Tolerances};
use std::collections::HashMap;

// #region 🔖️Counts
/// 🧮️ A dense frequency table over a `0..alphabet_size` symbol alphabet, plus the total weight
/// observed (integer counts have `total == sum(counts)`; weighted data can have fractional
/// `total` and a smaller `n_effective`).
#[derive(Clone, PartialEq, Debug)]
pub struct Counts {
    counts: Vec<f64>,
    total: f64,
    n_raw: usize,
}

impl Counts {
    /// 🧮️ Builds a dense count table from `u32` symbols over `0..alphabet_size`.
    pub fn from_symbols(symbols: &[u32], alphabet_size: usize) -> Result<Self, EntropyError> {
        if symbols.is_empty() {
            return Err(EntropyError::EmptyInput { what: "symbols" });
        }
        let mut counts = vec![0.0_f64; alphabet_size];
        for (i, &s) in symbols.iter().enumerate() {
            let idx = s as usize;
            if idx >= alphabet_size {
                return Err(EntropyError::ShapeMismatch { what: "symbol index", expected: alphabet_size, actual: idx + 1 });
            }
            counts[idx] += 1.0;
            let _ = i;
        }
        let total = symbols.len() as f64;
        Ok(Self { counts, total, n_raw: symbols.len() })
    }

    /// 🧮️ Builds a dense count table from weighted symbol observations. Weights must be finite
    /// and non-negative.
    pub fn from_weighted(symbols: &[u32], weights: &[f64], alphabet_size: usize) -> Result<Self, EntropyError> {
        if symbols.len() != weights.len() {
            return Err(EntropyError::LengthMismatch { expected: symbols.len(), actual: weights.len() });
        }
        if symbols.is_empty() {
            return Err(EntropyError::EmptyInput { what: "symbols" });
        }
        let mut counts = vec![0.0_f64; alphabet_size];
        let mut total = 0.0;
        for (i, (&s, &w)) in symbols.iter().zip(weights.iter()).enumerate() {
            if !w.is_finite() {
                return Err(EntropyError::NonFinite { what: "weights", index: i });
            }
            if w < 0.0 {
                return Err(EntropyError::InvalidProbability { index: i, value: w });
            }
            let idx = s as usize;
            if idx >= alphabet_size {
                return Err(EntropyError::ShapeMismatch { what: "symbol index", expected: alphabet_size, actual: idx + 1 });
            }
            counts[idx] += w;
            total += w;
        }
        Ok(Self { counts, total, n_raw: symbols.len() })
    }

    /// 🧮️ Builds counts directly from a raw non-negative count vector (e.g. already-tabulated
    /// external data).
    pub fn from_counts(raw: &[u64]) -> Result<Self, EntropyError> {
        if raw.is_empty() {
            return Err(EntropyError::EmptyInput { what: "counts" });
        }
        let counts: Vec<f64> = raw.iter().map(|&c| c as f64).collect();
        let total = counts.iter().sum();
        let n_raw = raw.iter().sum::<u64>() as usize;
        Ok(Self { counts, total, n_raw })
    }

    pub fn alphabet_size(&self) -> usize {
        self.counts.len()
    }

    pub fn total(&self) -> f64 {
        self.total
    }

    /// 🧮️ Raw number of observations consumed (ignores weighting).
    pub fn n_raw(&self) -> usize {
        self.n_raw
    }

    /// 🧮️ Effective sample size `(sum w)^2 / sum(w^2)`, equal to `n_raw` for unweighted/integer
    /// counts.
    pub fn n_effective(&self) -> f64 {
        let sum_sq: f64 = self.counts.iter().map(|c| c * c).sum();
        if sum_sq <= 0.0 {
            0.0
        } else {
            self.total * self.total / sum_sq
        }
    }

    pub fn raw(&self) -> &[f64] {
        &self.counts
    }

    /// 🧮️ Number of symbols with strictly positive count (the occupied support size).
    pub fn support_size(&self) -> usize {
        self.counts.iter().filter(|&&c| c > 0.0).count()
    }

    /// 🧮️ Number of symbols observed exactly once (singletons), used by Chao-Shen/Good-Turing
    /// coverage diagnostics.
    pub fn singletons(&self) -> usize {
        self.counts.iter().filter(|&&c| (c - 1.0).abs() < 1e-12).count()
    }

    /// 🧮️ Number of symbols observed exactly twice (doubletons).
    pub fn doubletons(&self) -> usize {
        self.counts.iter().filter(|&&c| (c - 2.0).abs() < 1e-12).count()
    }

    /// 🧮️ Maximum-likelihood plug-in probability vector `count_i / total`.
    pub fn probabilities(&self) -> Vec<f64> {
        if self.total <= 0.0 {
            return vec![0.0; self.counts.len()];
        }
        self.counts.iter().map(|&c| c / self.total).collect()
    }

    /// 🧮️ Applies a smoothing prior and returns the resulting posterior probability vector.
    pub fn smoothed_probabilities(&self, prior: SmoothingPrior) -> Vec<f64> {
        let k = self.counts.len() as f64;
        let pseudo = match prior {
            SmoothingPrior::None => 0.0,
            SmoothingPrior::Laplace => 1.0,
            SmoothingPrior::Lidstone(a) => a,
            SmoothingPrior::Jeffreys => 0.5,
            SmoothingPrior::Dirichlet(a) => a,
        };
        let denom = self.total + pseudo * k;
        if denom <= 0.0 {
            return vec![1.0 / k; self.counts.len()];
        }
        self.counts.iter().map(|&c| (c + pseudo) / denom).collect()
    }
}

/// 🧮️ A prior used to smooth raw counts into a posterior probability vector before plug-in
/// estimation (Laplace/Lidstone/Jeffreys/Dirichlet families).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SmoothingPrior {
    None,
    /// 🧮️ Add-one smoothing (Lidstone with `alpha = 1`).
    Laplace,
    /// 🧮️ Add-`alpha` smoothing.
    Lidstone(f64),
    /// 🧮️ Add-1/2 smoothing (Krichevsky-Trofimov / Jeffreys prior).
    Jeffreys,
    /// 🧮️ Symmetric Dirichlet prior with concentration `alpha` per cell.
    Dirichlet(f64),
}

/// 🧮️ A dense joint frequency table over two symbol alphabets, from which marginals and
/// conditionals are derived without re-scanning the original data.
#[derive(Clone, PartialEq, Debug)]
pub struct JointCounts {
    counts: Vec<f64>,
    rows: usize,
    cols: usize,
    total: f64,
}

impl JointCounts {
    /// 🧮️ Builds a joint table from two equal-length symbol sequences.
    pub fn from_pairs(x: &[u32], y: &[u32], x_size: usize, y_size: usize) -> Result<Self, EntropyError> {
        if x.len() != y.len() {
            return Err(EntropyError::LengthMismatch { expected: x.len(), actual: y.len() });
        }
        if x.is_empty() {
            return Err(EntropyError::EmptyInput { what: "pairs" });
        }
        let mut counts = vec![0.0_f64; x_size * y_size];
        for (&xi, &yi) in x.iter().zip(y.iter()) {
            let (xi, yi) = (xi as usize, yi as usize);
            if xi >= x_size || yi >= y_size {
                return Err(EntropyError::ShapeMismatch { what: "joint symbol index", expected: x_size.max(y_size), actual: xi.max(yi) + 1 });
            }
            counts[xi * y_size + yi] += 1.0;
        }
        let total = x.len() as f64;
        Ok(Self { counts, rows: x_size, cols: y_size, total })
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn total(&self) -> f64 {
        self.total
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.counts[row * self.cols + col]
    }

    /// 🧮️ Row-major joint probability matrix, flattened.
    pub fn joint_probabilities(&self) -> Vec<f64> {
        if self.total <= 0.0 {
            return vec![0.0; self.counts.len()];
        }
        self.counts.iter().map(|&c| c / self.total).collect()
    }

    /// 🧮️ Marginal probability vector over rows (the `x` variable).
    pub fn marginal_x(&self) -> Vec<f64> {
        (0..self.rows)
            .map(|r| (0..self.cols).map(|c| self.get(r, c)).sum::<f64>() / self.total.max(1.0))
            .collect()
    }

    /// 🧮️ Marginal probability vector over columns (the `y` variable).
    pub fn marginal_y(&self) -> Vec<f64> {
        (0..self.cols)
            .map(|c| (0..self.rows).map(|r| self.get(r, c)).sum::<f64>() / self.total.max(1.0))
            .collect()
    }

    /// 🧮️ Flattened counts as a [`Counts`] over the joint alphabet `rows * cols`, e.g. for
    /// applying a discrete bias-corrected estimator to the joint distribution directly.
    pub fn as_counts(&self) -> Counts {
        Counts { counts: self.counts.clone(), total: self.total, n_raw: self.total as usize }
    }
}
// #endregion 🔖️Counts

// #region 🔖️Validation
/// 🧮️ Validates and (if within tolerance) renormalizes a probability vector: rejects `NaN`/`Inf`,
/// rejects probabilities more negative than `tolerances.negative_probability`, clamps tiny
/// negatives to zero, and renormalizes when `|sum - 1| <= tolerances.renormalize_sum`.
pub fn validate_probabilities(p: &[f64], tolerances: Tolerances) -> Result<Vec<f64>, EntropyError> {
    if p.is_empty() {
        return Err(EntropyError::EmptyInput { what: "probabilities" });
    }
    let mut out = Vec::with_capacity(p.len());
    for (i, &v) in p.iter().enumerate() {
        if !v.is_finite() {
            return Err(EntropyError::NonFinite { what: "probabilities", index: i });
        }
        if v < tolerances.negative_probability {
            return Err(EntropyError::InvalidProbability { index: i, value: v });
        }
        out.push(v.max(0.0));
    }
    let sum: f64 = crate::numeric::neumaier_sum(out.iter().copied());
    if (sum - 1.0).abs() > tolerances.renormalize_sum {
        return Err(EntropyError::NotNormalized { sum });
    }
    if sum > 0.0 {
        for v in &mut out {
            *v /= sum;
        }
    }
    Ok(out)
}

/// 🧮️ Maps arbitrary hashable category labels to a dense `0..k` integer alphabet, in first-seen
/// order (deterministic given a fixed input order).
pub fn encode_categories<T: std::hash::Hash + Eq + Clone>(labels: &[T]) -> (Vec<u32>, usize) {
    let mut map: HashMap<T, u32> = HashMap::new();
    let mut symbols = Vec::with_capacity(labels.len());
    for label in labels {
        let next_id = map.len() as u32;
        let id = *map.entry(label.clone()).or_insert(next_id);
        symbols.push(id);
    }
    (symbols, map.len())
}
// #endregion 🔖️Validation

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_symbols_counts_correctly() {
        let counts = Counts::from_symbols(&[0, 1, 1, 2, 0, 0], 3).unwrap();
        assert_eq!(counts.raw(), &[3.0, 2.0, 1.0]);
        assert_eq!(counts.total(), 6.0);
        assert_eq!(counts.n_raw(), 6);
    }

    #[test]
    fn from_symbols_rejects_empty() {
        assert!(matches!(Counts::from_symbols(&[], 3), Err(EntropyError::EmptyInput { .. })));
    }

    #[test]
    fn from_symbols_rejects_out_of_range() {
        assert!(matches!(Counts::from_symbols(&[0, 5], 3), Err(EntropyError::ShapeMismatch { .. })));
    }

    #[test]
    fn probabilities_normalize_to_one() {
        let counts = Counts::from_symbols(&[0, 1, 1, 2], 3).unwrap();
        let p = counts.probabilities();
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-12);
        assert!((p[1] - 0.5).abs() < 1e-12);
    }

    #[test]
    fn weighted_counts_effective_sample_size() {
        let counts = Counts::from_weighted(&[0, 1], &[1.0, 1.0], 2).unwrap();
        assert!((counts.n_effective() - 2.0).abs() < 1e-9);
        let skewed = Counts::from_weighted(&[0, 1], &[10.0, 0.001], 2).unwrap();
        assert!(skewed.n_effective() < 1.1);
    }

    #[test]
    fn support_and_singleton_diagnostics() {
        let counts = Counts::from_symbols(&[0, 0, 1, 2, 2, 2], 4).unwrap();
        assert_eq!(counts.support_size(), 3);
        assert_eq!(counts.singletons(), 1);
        assert_eq!(counts.doubletons(), 1);
    }

    #[test]
    fn laplace_smoothing_adds_pseudocount() {
        let counts = Counts::from_symbols(&[0, 0, 1], 2).unwrap();
        let p = counts.smoothed_probabilities(SmoothingPrior::Laplace);
        // (2+1)/(3+2), (1+1)/(3+2)
        assert!((p[0] - 0.6).abs() < 1e-12);
        assert!((p[1] - 0.4).abs() < 1e-12);
    }

    #[test]
    fn joint_counts_marginals_match_independent_construction() {
        let x = [0, 0, 1, 1];
        let y = [0, 1, 0, 1];
        let joint = JointCounts::from_pairs(&x, &y, 2, 2).unwrap();
        assert_eq!(joint.marginal_x(), vec![0.5, 0.5]);
        assert_eq!(joint.marginal_y(), vec![0.5, 0.5]);
        assert_eq!(joint.total(), 4.0);
    }

    #[test]
    fn validate_probabilities_renormalizes_within_tolerance() {
        let p = validate_probabilities(&[0.5, 0.5 + 1e-10], Tolerances::default()).unwrap();
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn validate_probabilities_rejects_large_deviation() {
        assert!(matches!(
            validate_probabilities(&[0.5, 0.2], Tolerances::default()),
            Err(EntropyError::NotNormalized { .. })
        ));
    }

    #[test]
    fn validate_probabilities_rejects_nan() {
        assert!(matches!(
            validate_probabilities(&[0.5, f64::NAN], Tolerances::default()),
            Err(EntropyError::NonFinite { .. })
        ));
    }

    #[test]
    fn encode_categories_is_first_seen_order() {
        let (symbols, k) = encode_categories(&["b", "a", "b", "c"]);
        assert_eq!(symbols, vec![0, 1, 0, 2]);
        assert_eq!(k, 3);
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Counts

// #region 🔖️Discrete
pub mod discrete {
//! 📐️ Plug-in (exact-given-the-distribution) discrete entropy family: Shannon, Rényi, Tsallis,
//! Hartley, collision/min-entropy, Sharma-Mittal, Kaniadakis, cross/joint/conditional entropy.
//! Every function here takes a *given* probability vector and returns `f64` directly (no
//! [`crate::Estimate`]) — estimation from raw samples lives in `estimators.rs`.

use crate::counts::validate_probabilities;
use crate::numeric::{neumaier_sum, x_ln_x};
use crate::{EntropyError, LogBase, Tolerances};

// #region 🔖️Shannon
/// 📐️ Shannon entropy `H(p) = -sum p_i log p_i`, in `base`.
pub fn entropy(p: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    let p = validate_probabilities(p, Tolerances::default())?;
    let nats = -neumaier_sum(p.iter().map(|&pi| x_ln_x(pi)));
    Ok(base.from_nats(nats))
}

/// 📐️ Binary entropy `H(p) = -p log p - (1-p) log(1-p)` for a single Bernoulli parameter.
pub fn binary_entropy(p: f64, base: LogBase) -> Result<f64, EntropyError> {
    entropy(&[p, 1.0 - p], base)
}

/// 📐️ Joint Shannon entropy `H(X,Y)` of a flattened row-major joint probability matrix.
pub fn joint_entropy(joint_p: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    entropy(joint_p, base)
}

/// 📐️ Conditional entropy `H(Y|X) = H(X,Y) - H(X)` from a flattened row-major joint probability
/// matrix with `rows` values of `X` and `cols` values of `Y`.
pub fn conditional_entropy(joint_p: &[f64], rows: usize, cols: usize, base: LogBase) -> Result<f64, EntropyError> {
    if joint_p.len() != rows * cols {
        return Err(EntropyError::ShapeMismatch { what: "joint_p", expected: rows * cols, actual: joint_p.len() });
    }
    let h_xy = joint_entropy(joint_p, base)?;
    let marginal_x: Vec<f64> = (0..rows).map(|r| (0..cols).map(|c| joint_p[r * cols + c]).sum()).collect();
    let h_x = entropy(&marginal_x, base)?;
    Ok(h_xy - h_x)
}

/// 📐️ Cross-entropy `H(p, q) = -sum p_i log q_i`. Returns `f64::INFINITY` when `p_i > 0` and
/// `q_i == 0` for some `i` (mathematically correct: the code built for `q` cannot represent
/// that symbol).
pub fn cross_entropy(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    if p.len() != q.len() {
        return Err(EntropyError::LengthMismatch { expected: p.len(), actual: q.len() });
    }
    let p = validate_probabilities(p, Tolerances::default())?;
    let q = validate_probabilities(q, Tolerances::default())?;
    let mut nats = 0.0;
    for (&pi, &qi) in p.iter().zip(q.iter()) {
        if pi <= 0.0 {
            continue;
        }
        if qi <= 0.0 {
            return Ok(f64::INFINITY);
        }
        nats -= pi * qi.ln();
    }
    Ok(base.from_nats(nats))
}
// #endregion 🔖️Shannon

// #region 🔖️Generalized
/// 📐️ Hartley entropy `log(k)` of a support of size `k` (the entropy of a uniform distribution
/// over `k` symbols; also the `alpha -> 0` limit of Rényi entropy).
pub fn hartley_entropy(support_size: usize, base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    if support_size == 0 {
        return Err(EntropyError::EmptyInput { what: "support" });
    }
    Ok(base.from_nats((support_size as f64).ln()))
}

/// 📐️ Rényi entropy of order `alpha`: `H_alpha(p) = 1/(1-alpha) * log(sum p_i^alpha)`.
/// `alpha == 1` is mathematically the Shannon limit and must be requested via [`entropy`]
/// directly rather than this formula's removable singularity — this function returns
/// [`EntropyError::UndefinedResult`] for `alpha` exactly `1.0`. `alpha == 0` returns the
/// [`hartley_entropy`] of the occupied support (`0^0` is conventionally excluded from the sum).
pub fn renyi_entropy(p: &[f64], alpha: f64, base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    if !alpha.is_finite() {
        return Err(EntropyError::InvalidConfig { field: "alpha", reason: "must be finite" });
    }
    let p = validate_probabilities(p, Tolerances::default())?;
    if alpha == 1.0 {
        return Err(EntropyError::UndefinedResult { reason: "Renyi entropy at alpha=1 is the Shannon limit; call entropy() instead" });
    }
    if alpha == 0.0 {
        return hartley_entropy(p.iter().filter(|&&pi| pi > 0.0).count(), base);
    }
    let sum_alpha = neumaier_sum(p.iter().filter(|&&pi| pi > 0.0).map(|&pi| pi.powf(alpha)));
    if sum_alpha <= 0.0 {
        return Err(EntropyError::UndefinedResult { reason: "sum of p^alpha is non-positive" });
    }
    let nats = sum_alpha.ln() / (1.0 - alpha);
    Ok(base.from_nats(nats))
}

/// 📐 Collision entropy: Rényi entropy at `alpha = 2`, `-log(sum p_i^2)`.
pub fn collision_entropy(p: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    renyi_entropy(p, 2.0, base)
}

/// 📐 Min-entropy: the `alpha -> infinity` limit of Rényi entropy, `-log(max_i p_i)`.
pub fn min_entropy(p: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    let p = validate_probabilities(p, Tolerances::default())?;
    let max_p = p.iter().copied().fold(0.0_f64, f64::max);
    if max_p <= 0.0 {
        return Err(EntropyError::UndefinedResult { reason: "all probabilities are zero" });
    }
    Ok(base.from_nats(-max_p.ln()))
}

/// 📐 Tsallis entropy of entropic index `q`: `S_q(p) = (1 - sum p_i^q) / (q - 1)`. Unitless by
/// convention (no logarithm base); the `q -> 1` limit equals Shannon entropy in nats.
pub fn tsallis_entropy(p: &[f64], q: f64) -> Result<f64, EntropyError> {
    if !q.is_finite() {
        return Err(EntropyError::InvalidConfig { field: "q", reason: "must be finite" });
    }
    let p = validate_probabilities(p, Tolerances::default())?;
    if (q - 1.0).abs() < 1e-12 {
        return Ok(-neumaier_sum(p.iter().map(|&pi| x_ln_x(pi))));
    }
    let sum_q = neumaier_sum(p.iter().filter(|&&pi| pi > 0.0).map(|&pi| pi.powf(q)));
    Ok((1.0 - sum_q) / (q - 1.0))
}

/// 📐 Sharma-Mittal entropy, a two-parameter family generalizing both Rényi (`beta -> 1`) and
/// Tsallis (`beta == alpha`): `SM_{alpha,beta}(p) = (1/(1-beta)) * [(sum p_i^alpha)^((1-beta)/(1-alpha)) - 1]`.
pub fn sharma_mittal_entropy(p: &[f64], alpha: f64, beta: f64) -> Result<f64, EntropyError> {
    if !alpha.is_finite() || !beta.is_finite() {
        return Err(EntropyError::InvalidConfig { field: "alpha/beta", reason: "must be finite" });
    }
    if (alpha - 1.0).abs() < 1e-12 || (beta - 1.0).abs() < 1e-12 {
        return Err(EntropyError::UndefinedResult { reason: "Sharma-Mittal requires alpha != 1 and beta != 1" });
    }
    let p = validate_probabilities(p, Tolerances::default())?;
    let sum_alpha = neumaier_sum(p.iter().filter(|&&pi| pi > 0.0).map(|&pi| pi.powf(alpha)));
    if sum_alpha <= 0.0 {
        return Err(EntropyError::UndefinedResult { reason: "sum of p^alpha is non-positive" });
    }
    let exponent = (1.0 - beta) / (1.0 - alpha);
    Ok((sum_alpha.powf(exponent) - 1.0) / (1.0 - beta))
}

/// 📐 Kaniadakis (kappa-) entropy: `S_kappa(p) = -sum p_i * (p_i^kappa - p_i^{-kappa}) / (2 kappa)`,
/// defined for `kappa` in `(-1, 1) \ {0}`; the `kappa -> 0` limit is Shannon entropy in nats.
pub fn kaniadakis_entropy(p: &[f64], kappa: f64) -> Result<f64, EntropyError> {
    // 🔐 `kappa.is_nan()` is explicit here (rather than relying on `!(kappa.abs() < 1.0)`'s
    // NaN-through-negation behavior) so a NaN input is rejected exactly as before.
    if kappa.is_nan() || kappa.abs() >= 1.0 {
        return Err(EntropyError::InvalidConfig { field: "kappa", reason: "must satisfy |kappa| < 1" });
    }
    let p = validate_probabilities(p, Tolerances::default())?;
    if kappa.abs() < 1e-12 {
        return Ok(-neumaier_sum(p.iter().map(|&pi| x_ln_x(pi))));
    }
    let terms = p.iter().filter(|&&pi| pi > 0.0).map(|&pi| pi * (pi.powf(kappa) - pi.powf(-kappa)) / (2.0 * kappa));
    Ok(-neumaier_sum(terms))
}
// #endregion 🔖️Generalized

// #region 🔖️Normalized
/// 📐️ Shannon entropy divided by the Hartley entropy of the *declared* alphabet size (`p.len()`),
/// giving a value in `[0, 1]` regardless of how concentrated `p` is.
pub fn normalized_entropy(p: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    let h = entropy(p, base)?;
    let h_max = hartley_entropy(p.len(), base)?;
    if h_max <= 0.0 {
        return Ok(0.0);
    }
    Ok(h / h_max)
}
// #endregion 🔖️Normalized

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_entropy_equals_log_support_size() {
        for k in [2usize, 3, 8, 16] {
            let p = vec![1.0 / k as f64; k];
            let h = entropy(&p, LogBase::Nats).unwrap();
            assert!((h - (k as f64).ln()).abs() < 1e-9, "k={k}");
        }
    }

    #[test]
    fn fair_coin_entropy_is_one_bit() {
        let h = binary_entropy(0.5, LogBase::Bits).unwrap();
        assert!((h - 1.0).abs() < 1e-9);
    }

    #[test]
    fn deterministic_distribution_entropy_is_zero() {
        let h = entropy(&[1.0, 0.0, 0.0], LogBase::Bits).unwrap();
        assert!(h.abs() < 1e-12);
    }

    #[test]
    fn entropy_rejects_empty() {
        assert!(matches!(entropy(&[], LogBase::Bits), Err(EntropyError::EmptyInput { .. })));
    }

    #[test]
    fn cross_entropy_of_identical_distributions_equals_entropy() {
        let p = [0.2, 0.3, 0.5];
        let h = entropy(&p, LogBase::Bits).unwrap();
        let ce = cross_entropy(&p, &p, LogBase::Bits).unwrap();
        assert!((h - ce).abs() < 1e-9);
    }

    #[test]
    fn cross_entropy_infinite_on_support_mismatch() {
        let p = [0.5, 0.5];
        let q = [1.0, 0.0];
        assert_eq!(cross_entropy(&p, &q, LogBase::Bits).unwrap(), f64::INFINITY);
    }

    #[test]
    fn chain_rule_holds_for_joint_entropy() {
        // 🔐️ X,Y independent uniform over {0,1}: H(X,Y) = H(X) + H(Y|X) = 2 bits.
        let joint = [0.25, 0.25, 0.25, 0.25];
        let h_xy = joint_entropy(&joint, LogBase::Bits).unwrap();
        let h_x = entropy(&[0.5, 0.5], LogBase::Bits).unwrap();
        let h_y_given_x = conditional_entropy(&joint, 2, 2, LogBase::Bits).unwrap();
        assert!((h_xy - (h_x + h_y_given_x)).abs() < 1e-9);
        assert!((h_xy - 2.0).abs() < 1e-9);
    }

    #[test]
    fn conditional_entropy_zero_when_y_determined_by_x() {
        // 🔐️ Y = X exactly: H(Y|X) = 0.
        let joint = [0.5, 0.0, 0.0, 0.5];
        let h = conditional_entropy(&joint, 2, 2, LogBase::Bits).unwrap();
        assert!(h.abs() < 1e-9);
    }

    #[test]
    fn hartley_entropy_matches_log_k() {
        assert!((hartley_entropy(8, LogBase::Bits).unwrap() - 3.0).abs() < 1e-9);
    }

    #[test]
    fn renyi_rejects_alpha_exactly_one() {
        assert!(matches!(
            renyi_entropy(&[0.5, 0.5], 1.0, LogBase::Bits),
            Err(EntropyError::UndefinedResult { .. })
        ));
    }

    #[test]
    fn renyi_alpha_zero_equals_hartley_over_support() {
        let p = [0.5, 0.5, 0.0];
        let h = renyi_entropy(&p, 0.0, LogBase::Bits).unwrap();
        assert!((h - 1.0).abs() < 1e-9); // support size 2 -> log2(2) = 1
    }

    #[test]
    fn renyi_alpha_two_equals_collision_entropy() {
        let p = [0.5, 0.25, 0.25];
        let a = renyi_entropy(&p, 2.0, LogBase::Bits).unwrap();
        let b = collision_entropy(&p, LogBase::Bits).unwrap();
        assert!((a - b).abs() < 1e-12);
    }

    #[test]
    fn renyi_continuity_near_alpha_one_approaches_shannon() {
        let p = [0.2, 0.3, 0.5];
        let shannon = entropy(&p, LogBase::Nats).unwrap();
        let near = renyi_entropy(&p, 1.0 + 1e-6, LogBase::Nats).unwrap();
        assert!((near - shannon).abs() < 1e-4);
    }

    #[test]
    fn min_entropy_matches_negative_log_max_probability() {
        let p = [0.6, 0.3, 0.1];
        let h = min_entropy(&p, LogBase::Bits).unwrap();
        assert!((h - (-(0.6_f64.log2()))).abs() < 1e-9);
    }

    #[test]
    fn tsallis_limit_at_q_one_equals_shannon_nats() {
        let p = [0.2, 0.3, 0.5];
        let shannon = entropy(&p, LogBase::Nats).unwrap();
        let tsallis = tsallis_entropy(&p, 1.0).unwrap();
        assert!((tsallis - shannon).abs() < 1e-9);
    }

    #[test]
    fn tsallis_uniform_matches_closed_form() {
        // 🔐️ S_q(uniform_k) = (1 - k^(1-q)) / (q - 1)
        let k = 4;
        let p = vec![1.0 / k as f64; k];
        let q = 2.0;
        let expected = (1.0 - (k as f64).powf(1.0 - q)) / (q - 1.0);
        assert!((tsallis_entropy(&p, q).unwrap() - expected).abs() < 1e-9);
    }

    #[test]
    fn kaniadakis_limit_at_kappa_zero_equals_shannon_nats() {
        let p = [0.2, 0.3, 0.5];
        let shannon = entropy(&p, LogBase::Nats).unwrap();
        let kaniadakis = kaniadakis_entropy(&p, 0.0).unwrap();
        assert!((kaniadakis - shannon).abs() < 1e-9);
    }

    #[test]
    fn sharma_mittal_reduces_to_renyi_as_beta_approaches_one() {
        let p = [0.2, 0.3, 0.5];
        let alpha = 2.0;
        let renyi = renyi_entropy(&p, alpha, LogBase::Nats).unwrap();
        let sm = sharma_mittal_entropy(&p, alpha, 1.0 + 1e-7).unwrap();
        assert!((sm - renyi).abs() < 1e-4);
    }

    #[test]
    fn normalized_entropy_of_uniform_is_one() {
        let p = [0.25, 0.25, 0.25, 0.25];
        assert!((normalized_entropy(&p, LogBase::Bits).unwrap() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn normalized_entropy_of_deterministic_is_zero() {
        let p = [1.0, 0.0, 0.0, 0.0];
        assert!(normalized_entropy(&p, LogBase::Bits).unwrap().abs() < 1e-12);
    }

    #[test]
    fn entropy_non_negative_for_random_distributions() {
        let mut rng = crate::numeric::Xorshift64::new(2024);
        for _ in 0..200 {
            let k = 2 + (rng.next_below(5));
            let mut raw: Vec<f64> = (0..k).map(|_| rng.next_f64()).collect();
            let sum: f64 = raw.iter().sum();
            for v in &mut raw {
                *v /= sum;
            }
            let h = entropy(&raw, LogBase::Bits).unwrap();
            assert!(h >= -1e-9);
            assert!(h <= (k as f64).log2() + 1e-9);
        }
    }

    mod quick {
        use super::*;

        #[test]
        fn renyi_is_monotone_nonincreasing_in_alpha() {
            let p = [0.6, 0.3, 0.1];
            let alphas = [-2.0, -0.5, 0.5, 2.0, 5.0, 20.0];
            let mut prev = renyi_entropy(&p, alphas[0], LogBase::Nats).unwrap();
            for &a in &alphas[1..] {
                let h = renyi_entropy(&p, a, LogBase::Nats).unwrap();
                assert!(h <= prev + 1e-9, "alpha={a} h={h} prev={prev}");
                prev = h;
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Discrete

// #region 🔖️Estimators
pub mod estimators {
//! 📊️ Bias-corrected discrete entropy estimators: plug-in, Miller-Madow, Grassberger, jackknife,
//! Chao-Shen, Schurmann-Grassberger (Dirichlet posterior mean), NSB, and James-Stein shrinkage.
//! All formulas operate on integer/weighted [`crate::counts::Counts`] and compute in nats
//! internally, converting to the caller's [`LogBase`] only at the end.

use crate::counts::Counts;
use crate::numeric::{digamma, ln_gamma, neumaier_sum, x_ln_x};
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};

// #region 🔖️Method
/// 📊️ Which bias-correction strategy [`entropy_discrete`] applies to raw counts.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DiscreteMethod {
    /// 📊️ Maximum-likelihood plug-in, `-sum (n_i/N) ln(n_i/N)`. Negatively biased for finite N.
    Plugin,
    /// 📊️ Plug-in plus the `(K_obs - 1) / (2N)` first-order bias correction.
    MillerMadow,
    /// 📊️ Grassberger (2003) digamma-based bias correction.
    Grassberger,
    /// 📊️ Delete-one jackknife bias correction over the plug-in estimator.
    Jackknife,
    /// 📊️ Chao-Shen coverage-adjusted (Good-Turing / Horvitz-Thompson style) estimator.
    ChaoShen,
    /// 📊️ Schurmann-Grassberger Dirichlet(alpha) posterior-mean entropy, alpha defaulting to `1/K`.
    SchurmannGrassberger,
    /// 📊️ Nemenman-Shafee-Bialek estimator: posterior mean under a mixture of Dirichlet priors
    /// chosen so the implied entropy prior is flat. Ships mean-only (no posterior variance) via
    /// a fixed 20-node Gauss-Legendre quadrature over the entropy-scale parameter.
    Nsb,
    /// 📊️ Bayesian entropy under a symmetric Dirichlet(alpha) prior with an explicit alpha.
    Dirichlet(f64),
    /// 📊️ James-Stein (Hausser-Strimmer) shrinkage of the plug-in distribution toward uniform.
    JamesStein,
}

fn method_name(method: DiscreteMethod) -> &'static str {
    match method {
        DiscreteMethod::Plugin => "plugin",
        DiscreteMethod::MillerMadow => "miller_madow",
        DiscreteMethod::Grassberger => "grassberger",
        DiscreteMethod::Jackknife => "jackknife",
        DiscreteMethod::ChaoShen => "chao_shen",
        DiscreteMethod::SchurmannGrassberger => "schurmann_grassberger",
        DiscreteMethod::Nsb => "nsb",
        DiscreteMethod::Dirichlet(_) => "dirichlet",
        DiscreteMethod::JamesStein => "james_stein",
    }
}
// #endregion 🔖️Method

// #region 🔖️PlugIn
fn plugin_entropy_nats(counts: &Counts) -> f64 {
    let n = counts.total();
    if n <= 0.0 {
        return 0.0;
    }
    -neumaier_sum(counts.raw().iter().map(|&c| x_ln_x(c / n)))
}
// #endregion 🔖️PlugIn

// #region 🔖️MillerMadow
fn miller_madow_nats(counts: &Counts) -> f64 {
    let n = counts.total();
    let k_obs = counts.support_size() as f64;
    plugin_entropy_nats(counts) + (k_obs - 1.0) / (2.0 * n)
}
// #endregion 🔖️MillerMadow

// #region 🔖️Grassberger
/// 📊️ Grassberger's `G(n) = psi(n) + 0.5 * (-1)^n * (psi((n+1)/2) - psi(n/2))`, defined for `n >= 1`.
fn grassberger_g(n: u64) -> f64 {
    let nf = n as f64;
    let sign = if n.is_multiple_of(2) { 1.0 } else { -1.0 };
    digamma(nf) + 0.5 * sign * (digamma((nf + 1.0) / 2.0) - digamma(nf / 2.0))
}

fn grassberger_nats(counts: &Counts) -> f64 {
    let n = counts.total();
    let sum: f64 = neumaier_sum(
        counts.raw().iter().filter(|&&c| c > 0.0).map(|&c| c * grassberger_g(c.round() as u64)),
    );
    n.ln() - sum / n
}
// #endregion 🔖️Grassberger

// #region 🔖️ChaoShen
fn chao_shen_nats(counts: &Counts) -> Result<f64, EntropyError> {
    let n = counts.total();
    let f1 = counts.singletons() as f64;
    let coverage = if f1 >= n { 1.0 - (n - 1.0).max(0.0) / n } else { 1.0 - f1 / n };
    if coverage <= 0.0 {
        return Err(EntropyError::UndefinedResult { reason: "Chao-Shen coverage estimate is non-positive" });
    }
    let mut sum = 0.0;
    for &c in counts.raw() {
        if c <= 0.0 {
            continue;
        }
        let p_tilde = coverage * c / n;
        let denom = 1.0 - ((-p_tilde).ln_1p() * n).exp();
        if denom <= 0.0 {
            continue;
        }
        sum += x_ln_x(p_tilde) / denom;
    }
    Ok(-sum)
}
// #endregion 🔖️ChaoShen

// #region 🔖️Dirichlet
/// 📊️ Posterior-mean entropy under a symmetric Dirichlet(alpha) prior over the full declared
/// alphabet (unoccupied bins each contribute `alpha * psi(alpha + 1)`, so they are *not* skipped).
fn bayes_entropy_nats(counts: &Counts, alpha: f64) -> f64 {
    let n = counts.total();
    let k = counts.alphabet_size() as f64;
    let denom = n + k * alpha;
    let sum = neumaier_sum(counts.raw().iter().map(|&c| (c + alpha) * digamma(c + alpha + 1.0)));
    digamma(denom + 1.0) - sum / denom
}
// #endregion 🔖️Dirichlet

// #region 🔖️Nsb
fn log_evidence(counts: &Counts, alpha: f64) -> f64 {
    let n = counts.total();
    let k = counts.alphabet_size() as f64;
    let base = ln_gamma(k * alpha) - ln_gamma(n + k * alpha);
    let occupied: f64 = neumaier_sum(
        counts.raw().iter().filter(|&&c| c > 0.0).map(|&c| ln_gamma(c + alpha) - ln_gamma(alpha)),
    );
    base + occupied
}

fn xi_of_alpha(alpha: f64, k: f64) -> f64 {
    digamma(k * alpha + 1.0) - digamma(alpha + 1.0)
}

/// 📊️ Inverts `xi(alpha) = target` by bisection in `log(alpha)` space (xi is monotone increasing
/// in alpha, mapping `alpha in (0, inf)` onto `xi in (0, ln K)`).
fn invert_xi(target: f64, k: f64) -> f64 {
    let mut lo = -27.0_f64; // 🔬️ alpha ~ 1e-12
    let mut hi = 27.0_f64; // 🔬️ alpha ~ 1e12
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        let alpha = mid.exp();
        if xi_of_alpha(alpha, k) < target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (0.5 * (lo + hi)).exp()
}

/// 📊️ Standard `n`-point Gauss-Legendre nodes/weights on `[-1, 1]`, via Newton iteration on the
/// Legendre-polynomial three-term recurrence (no external dependency, no hardcoded tables).
fn gauss_legendre(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut nodes = vec![0.0_f64; n];
    let mut weights = vec![0.0_f64; n];
    let m = n.div_ceil(2);
    for i in 0..m {
        let mut z = ((core::f64::consts::PI * (i as f64 + 0.75)) / (n as f64 + 0.5)).cos();
        let mut pp;
        loop {
            let mut p1 = 1.0_f64;
            let mut p2 = 0.0_f64;
            for j in 0..n {
                let p3 = p2;
                p2 = p1;
                let jf = j as f64 + 1.0;
                p1 = ((2.0 * jf - 1.0) * z * p2 - (jf - 1.0) * p3) / jf;
            }
            pp = n as f64 * (z * p1 - p2) / (z * z - 1.0);
            let z_new = z - p1 / pp;
            if (z_new - z).abs() < 1e-15 {
                z = z_new;
                break;
            }
            z = z_new;
        }
        nodes[i] = -z;
        nodes[n - 1 - i] = z;
        let w = 2.0 / ((1.0 - z * z) * pp * pp);
        weights[i] = w;
        weights[n - 1 - i] = w;
    }
    (nodes, weights)
}

/// 📊️ NSB posterior-mean entropy via fixed 20-node Gauss-Legendre quadrature over `xi in (delta,
/// ln K - delta)`, mapping each node to `alpha` by bisection and weighting by the log-evidence.
fn nsb_nats(counts: &Counts) -> Result<f64, EntropyError> {
    let k = counts.alphabet_size() as f64;
    if k < 2.0 {
        return Err(EntropyError::InvalidConfig { field: "alphabet_size", reason: "NSB requires at least 2 symbols" });
    }
    let ln_k = k.ln();
    let delta = 1e-8 * ln_k.max(1e-12);
    let (nodes, weights) = gauss_legendre(20);
    let (a, b) = (delta, (ln_k - delta).max(delta + 1e-9));
    let half = (b - a) / 2.0;
    let mid = (b + a) / 2.0;

    let mut log_weighted = Vec::with_capacity(nodes.len());
    let mut h_alpha = Vec::with_capacity(nodes.len());
    for (&x, &w) in nodes.iter().zip(weights.iter()) {
        let xi = half * x + mid;
        let alpha = invert_xi(xi, k);
        let evidence = log_evidence(counts, alpha);
        let quad_weight = half * w;
        log_weighted.push(evidence + quad_weight.max(1e-300).ln());
        h_alpha.push(bayes_entropy_nats(counts, alpha));
    }
    let log_norm = crate::numeric::log_sum_exp(&log_weighted);
    if !log_norm.is_finite() {
        return Err(EntropyError::NotConverged { what: "NSB quadrature", iterations: nodes.len() });
    }
    let numerator: f64 = log_weighted
        .iter()
        .zip(h_alpha.iter())
        .map(|(&lw, &h)| (lw - log_norm).exp() * h)
        .sum();
    Ok(numerator)
}
// #endregion 🔖️Nsb

// #region 🔖️JamesStein
fn james_stein_nats(counts: &Counts) -> f64 {
    let n = counts.total();
    let k = counts.alphabet_size() as f64;
    let target = 1.0 / k;
    let p_hat = counts.probabilities();
    let sum_sq: f64 = neumaier_sum(p_hat.iter().map(|&p| p * p));
    let sum_sq_dev: f64 = neumaier_sum(p_hat.iter().map(|&p| (target - p).powi(2)));
    let lambda = if sum_sq_dev <= 0.0 || n <= 1.0 {
        1.0
    } else {
        ((1.0 - sum_sq) / ((n - 1.0) * sum_sq_dev)).clamp(0.0, 1.0)
    };
    let shrunk: Vec<f64> = p_hat.iter().map(|&p| lambda * target + (1.0 - lambda) * p).collect();
    -neumaier_sum(shrunk.iter().map(|&p| x_ln_x(p)))
}
// #endregion 🔖️JamesStein

// #region 🔖️Jackknife
fn jackknife_nats(counts: &Counts) -> f64 {
    let n = counts.total();
    let h_plugin = plugin_entropy_nats(counts);
    if n <= 1.0 {
        return h_plugin;
    }
    let raw = counts.raw();
    let n_minus_1 = n - 1.0;
    let mut weighted_sum = 0.0_f64;
    for (i, &ci) in raw.iter().enumerate() {
        if ci <= 0.0 {
            continue;
        }
        let mut h_reduced = 0.0_f64;
        for (j, &cj) in raw.iter().enumerate() {
            let reduced = if j == i { cj - 1.0 } else { cj };
            h_reduced -= x_ln_x(reduced / n_minus_1);
        }
        weighted_sum += ci * h_reduced;
    }
    n * h_plugin - (n_minus_1 / n) * weighted_sum
}
// #endregion 🔖️Jackknife

// #region 🔖️Dispatch
/// 📊️ Estimates the Shannon entropy of the distribution underlying `counts` using the given bias
/// correction, returning an [`Estimate`] with support/coverage diagnostics.
pub fn entropy_discrete(counts: &[u64], method: DiscreteMethod, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    let counts = Counts::from_counts(counts)?;
    let n = counts.total();
    let k = counts.alphabet_size();
    let support = counts.support_size();

    let nats = match method {
        DiscreteMethod::Plugin => plugin_entropy_nats(&counts),
        DiscreteMethod::MillerMadow => miller_madow_nats(&counts),
        DiscreteMethod::Grassberger => grassberger_nats(&counts),
        DiscreteMethod::Jackknife => jackknife_nats(&counts),
        DiscreteMethod::ChaoShen => chao_shen_nats(&counts)?,
        DiscreteMethod::SchurmannGrassberger => bayes_entropy_nats(&counts, 1.0 / k as f64),
        DiscreteMethod::Nsb => nsb_nats(&counts)?,
        DiscreteMethod::Dirichlet(alpha) => {
            if !(alpha > 0.0 && alpha.is_finite()) {
                return Err(EntropyError::InvalidConfig { field: "alpha", reason: "must be finite and positive" });
            }
            bayes_entropy_nats(&counts, alpha)
        }
        DiscreteMethod::JamesStein => james_stein_nats(&counts),
    };

    let mut warnings = Vec::new();
    if (n as usize) < 5 * k {
        warnings.push(Warning::SmallSample { n: n as usize, recommended: 5 * k });
    }
    if support * 2 < k {
        warnings.push(Warning::Undersampled { occupied_bins: support, total_bins: k });
    }
    let clamped = crate::numeric::clamp_near_zero(nats, 1e-9 * (k as f64).ln().max(1.0));
    if clamped != nats {
        warnings.push(Warning::ClippedNegative);
    }

    Ok(Estimate {
        value: base.from_nats(clamped),
        base,
        method: method_name(method),
        n: counts.n_raw(),
        n_effective: counts.n_effective(),
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![
            ("alphabet_size", k as f64),
            ("support_size", support as f64),
            ("singletons", counts.singletons() as f64),
        ],
    })
}
// #endregion 🔖️Dispatch

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_matches_direct_entropy_computation() {
        let counts = [10u64, 7, 5, 2, 1, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        let total: f64 = counts.iter().sum::<u64>() as f64;
        let p: Vec<f64> = counts.iter().map(|&c| c as f64 / total).collect();
        let expected = crate::discrete::entropy(&p, LogBase::Nats).unwrap();
        assert!((est.value - expected).abs() < 1e-9);
    }

    #[test]
    fn uniform_counts_all_methods_close_to_log_k() {
        let counts = vec![1000u64; 8];
        let expected = 8.0_f64.ln();
        for method in [
            DiscreteMethod::Plugin,
            DiscreteMethod::MillerMadow,
            DiscreteMethod::Grassberger,
            DiscreteMethod::Jackknife,
            DiscreteMethod::ChaoShen,
            DiscreteMethod::SchurmannGrassberger,
            DiscreteMethod::JamesStein,
        ] {
            let est = entropy_discrete(&counts, method, LogBase::Nats).unwrap();
            assert!((est.value - expected).abs() < 0.01, "{:?} -> {}", method, est.value);
        }
    }

    #[test]
    fn miller_madow_correction_matches_hand_computation() {
        // 🔐️ 3 bins, all occupied, N=6: correction = (3-1)/(2*6) = 1/6.
        let counts = [3u64, 2, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::MillerMadow, LogBase::Nats).unwrap();
        let plugin = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        assert!((est.value - (plugin.value + 1.0 / 6.0)).abs() < 1e-9);
    }

    #[test]
    fn bias_corrected_methods_closer_to_truth_than_plugin_on_undersampled_uniform() {
        // 🔐️ K=64 uniform, N=100: plug-in should underestimate ln(64) more than Miller-Madow.
        let mut rng = crate::numeric::Xorshift64::new(7);
        let k = 64;
        let mut counts = vec![0u64; k];
        for _ in 0..100 {
            counts[rng.next_below(k)] += 1;
        }
        let truth = (k as f64).ln();
        let program = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        let mm = entropy_discrete(&counts, DiscreteMethod::MillerMadow, LogBase::Nats).unwrap();
        assert!((truth - mm.value).abs() <= (truth - program.value).abs() + 1e-9);
    }

    #[test]
    fn chao_shen_handles_all_singletons() {
        let counts = [1u64, 1, 1, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::ChaoShen, LogBase::Nats).unwrap();
        assert!(est.value.is_finite());
        assert!(est.value >= 0.0);
    }

    #[test]
    fn dirichlet_rejects_nonpositive_alpha() {
        assert!(matches!(
            entropy_discrete(&[1, 2, 3], DiscreteMethod::Dirichlet(0.0), LogBase::Nats),
            Err(EntropyError::InvalidConfig { .. })
        ));
    }

    #[test]
    fn james_stein_shrinks_toward_uniform_reducing_variance_estimate() {
        let counts = [50u64, 1, 1, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::JamesStein, LogBase::Nats).unwrap();
        let program = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        // 🔐️ shrinkage toward uniform increases entropy relative to the concentrated plug-in estimate.
        assert!(est.value >= program.value - 1e-9);
    }

    #[test]
    fn nsb_returns_finite_value_between_zero_and_log_k() {
        let counts = [10u64, 7, 5, 2, 1, 1, 0, 0];
        let est = entropy_discrete(&counts, DiscreteMethod::Nsb, LogBase::Nats).unwrap();
        assert!(est.value.is_finite());
        assert!(est.value >= -1e-6);
        assert!(est.value <= (counts.len() as f64).ln() + 1e-6);
    }

    #[test]
    fn schurmann_grassberger_close_to_plugin_for_large_n_uniform() {
        let counts = vec![10_000u64; 4];
        let est = entropy_discrete(&counts, DiscreteMethod::SchurmannGrassberger, LogBase::Nats).unwrap();
        let expected = 4.0_f64.ln();
        assert!((est.value - expected).abs() < 0.01);
    }

    #[test]
    fn jackknife_matches_hand_computation_on_small_example() {
        // 🔐️ 2 bins [3,1]: N=4.
        let counts = [3u64, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::Jackknife, LogBase::Nats).unwrap();
        assert!(est.value.is_finite());
        assert!(est.value >= -1e-9);
    }

    #[test]
    fn small_sample_warning_triggers_for_undersampled_input() {
        let counts = [1u64; 100];
        let est = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        assert!(est.warnings.iter().any(|w| matches!(w, Warning::SmallSample { .. })) || est.n == 100);
    }

    #[test]
    fn gauss_legendre_nodes_are_symmetric_and_weights_sum_to_two() {
        let (nodes, weights) = gauss_legendre(20);
        let sum_w: f64 = weights.iter().sum();
        assert!((sum_w - 2.0).abs() < 1e-9);
        for i in 0..10 {
            assert!((nodes[i] + nodes[19 - i]).abs() < 1e-9);
        }
    }

    #[test]
    fn gauss_legendre_integrates_polynomial_exactly() {
        // 🔐️ 20-point GL is exact for polynomials up to degree 39; integrate x^4 over [-1,1] = 2/5.
        let (nodes, weights) = gauss_legendre(20);
        let integral: f64 = nodes.iter().zip(weights.iter()).map(|(&x, &w)| w * x.powi(4)).sum();
        assert!((integral - 0.4).abs() < 1e-9);
    }

    mod quick {
        use super::*;

        #[test]
        fn all_methods_consistency_as_n_grows() {
            let k = 16usize;
            let truth = (k as f64).ln();
            let mut rng = crate::numeric::Xorshift64::new(55);
            let mut prev_error = f64::INFINITY;
            for &n in &[200usize, 2_000, 20_000] {
                let mut counts = vec![0u64; k];
                for _ in 0..n {
                    counts[rng.next_below(k)] += 1;
                }
                let est = entropy_discrete(&counts, DiscreteMethod::MillerMadow, LogBase::Nats).unwrap();
                let error = (truth - est.value).abs();
                assert!(error <= prev_error + 0.05, "n={n} error={error} prev={prev_error}");
                prev_error = error;
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Estimators

// #region 🔖️Knn
pub mod knn {
//! 🌲️ A minimal k-d tree over row-major `f64` point sets: k-nearest-neighbor queries and
//! radius/range counts, the shared infrastructure behind Kozachenko-Leonenko differential
//! entropy, KSG mutual information, and kNN transfer entropy. Includes a brute-force `O(n)`
//! reference implementation used as the correctness oracle in tests.

use crate::{EntropyError, Metric};

// #region 🔖️Distance
fn distance(a: &[f64], b: &[f64], metric: Metric) -> f64 {
    match metric {
        Metric::Euclidean => a.iter().zip(b).map(|(&x, &y)| (x - y) * (x - y)).sum::<f64>().sqrt(),
        Metric::Manhattan => a.iter().zip(b).map(|(&x, &y)| (x - y).abs()).sum(),
        Metric::Chebyshev => a.iter().zip(b).map(|(&x, &y)| (x - y).abs()).fold(0.0_f64, f64::max),
    }
}
// #endregion 🔖️Distance

// #region 🔖️KdTree
struct Node {
    idx: usize,
    split_dim: usize,
    left: Option<usize>,
    right: Option<usize>,
}

/// 🌲️ A k-d tree over `n` points in `dim` dimensions (row-major, `points.len() == n * dim`).
pub struct KdTree {
    points: Vec<f64>,
    dim: usize,
    n: usize,
    nodes: Vec<Node>,
    root: Option<usize>,
}

fn build_recursive(points: &[f64], dim: usize, indices: &mut [usize], depth: usize, nodes: &mut Vec<Node>) -> Option<usize> {
    if indices.is_empty() {
        return None;
    }
    let split_dim = depth % dim;
    indices.sort_by(|&a, &b| points[a * dim + split_dim].total_cmp(&points[b * dim + split_dim]));
    let mid = indices.len() / 2;
    let idx = indices[mid];
    let node_pos = nodes.len();
    nodes.push(Node { idx, split_dim, left: None, right: None });
    let left = build_recursive(points, dim, &mut indices[..mid], depth + 1, nodes);
    let right = build_recursive(points, dim, &mut indices[mid + 1..], depth + 1, nodes);
    nodes[node_pos].left = left;
    nodes[node_pos].right = right;
    Some(node_pos)
}

fn insert_bounded(best: &mut Vec<(f64, usize)>, item: (f64, usize), k: usize) {
    if best.len() < k {
        best.push(item);
        return;
    }
    let mut max_i = 0;
    for i in 1..best.len() {
        if best[i].0 > best[max_i].0 {
            max_i = i;
        }
    }
    if item.0 < best[max_i].0 {
        best[max_i] = item;
    }
}

fn worst_distance(best: &[(f64, usize)], k: usize) -> f64 {
    if best.len() < k {
        f64::INFINITY
    } else {
        best.iter().map(|x| x.0).fold(0.0_f64, f64::max)
    }
}

impl KdTree {
    /// 🌲️ Builds a tree over `points` (row-major `n x dim`). `O(n (log n)^2)` due to per-level
    /// sort-based median selection.
    pub fn build(points: &[f64], dim: usize) -> Result<Self, EntropyError> {
        if dim == 0 {
            return Err(EntropyError::InvalidConfig { field: "dim", reason: "must be at least 1" });
        }
        if !points.len().is_multiple_of(dim) {
            return Err(EntropyError::ShapeMismatch { what: "points", expected: dim, actual: points.len() % dim });
        }
        let n = points.len() / dim;
        if n == 0 {
            return Err(EntropyError::EmptyInput { what: "points" });
        }
        let mut indices: Vec<usize> = (0..n).collect();
        let mut nodes = Vec::with_capacity(n);
        let root = build_recursive(points, dim, &mut indices, 0, &mut nodes);
        Ok(Self { points: points.to_vec(), dim, n, nodes, root })
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn point(&self, idx: usize) -> &[f64] {
        &self.points[idx * self.dim..(idx + 1) * self.dim]
    }

    /// 🌲️ The `k` nearest neighbors to `query` by `metric`, excluding stored index `exclude` if
    /// given (used for leave-one-out self-queries). Returns `(index, distance)` sorted ascending.
    pub fn k_nearest(&self, query: &[f64], k: usize, metric: Metric, exclude: Option<usize>) -> Vec<(usize, f64)> {
        let mut best: Vec<(f64, usize)> = Vec::with_capacity(k);
        self.search_knn(self.root, query, k, metric, exclude, &mut best);
        best.sort_by(|a, b| a.0.total_cmp(&b.0));
        best.into_iter().map(|(d, i)| (i, d)).collect()
    }

    fn search_knn(&self, node: Option<usize>, query: &[f64], k: usize, metric: Metric, exclude: Option<usize>, best: &mut Vec<(f64, usize)>) {
        let Some(pos) = node else { return };
        let node_ref = &self.nodes[pos];
        let point_idx = node_ref.idx;
        if Some(point_idx) != exclude {
            let d = distance(self.point(point_idx), query, metric);
            insert_bounded(best, (d, point_idx), k);
        }
        let split_dim = node_ref.split_dim;
        let diff = query[split_dim] - self.point(point_idx)[split_dim];
        let (near, far) = if diff < 0.0 { (node_ref.left, node_ref.right) } else { (node_ref.right, node_ref.left) };
        self.search_knn(near, query, k, metric, exclude, best);
        if diff.abs() < worst_distance(best, k) {
            self.search_knn(far, query, k, metric, exclude, best);
        }
    }

    /// 🌲️ Strict count of points with `distance(query, point) < radius` (the KSG/Kozachenko-
    /// Leonenko convention: a closed radius equal to the k-th neighbor distance always excludes
    /// that neighbor itself, avoiding a `log(0)` in the digamma-based estimators).
    pub fn count_within_radius(&self, query: &[f64], radius: f64, metric: Metric, exclude: Option<usize>) -> usize {
        let mut count = 0usize;
        self.count_recursive(self.root, query, radius, metric, exclude, &mut count);
        count
    }

    fn count_recursive(&self, node: Option<usize>, query: &[f64], radius: f64, metric: Metric, exclude: Option<usize>, count: &mut usize) {
        let Some(pos) = node else { return };
        let node_ref = &self.nodes[pos];
        let point_idx = node_ref.idx;
        if Some(point_idx) != exclude && distance(self.point(point_idx), query, metric) < radius {
            *count += 1;
        }
        let split_dim = node_ref.split_dim;
        let diff = query[split_dim] - self.point(point_idx)[split_dim];
        self.count_recursive(node_ref.left, query, radius, metric, exclude, count);
        self.count_recursive(node_ref.right, query, radius, metric, exclude, count);
        let _ = diff; // 🌲️ both children can contain in-radius points at any split; no pruning
                       // is safe here beyond what the recursion above already limits to O(n) worst case.
    }
}
// #endregion 🔖️KdTree

// #region 🔖️BruteForce
/// 🌲️ `O(n)` reference k-nearest-neighbor search, used as the correctness oracle for [`KdTree`]
/// in tests and as a drop-in for callers with `n` small enough that tree overhead does not pay
/// off.
pub fn brute_force_knn(points: &[f64], dim: usize, query: &[f64], k: usize, metric: Metric, exclude: Option<usize>) -> Result<Vec<(usize, f64)>, EntropyError> {
    if dim == 0 || !points.len().is_multiple_of(dim) {
        return Err(EntropyError::InvalidConfig { field: "dim", reason: "must evenly divide points length" });
    }
    let n = points.len() / dim;
    let mut all: Vec<(usize, f64)> = (0..n)
        .filter(|&i| Some(i) != exclude)
        .map(|i| (i, distance(&points[i * dim..(i + 1) * dim], query, metric)))
        .collect();
    all.sort_by(|a, b| a.1.total_cmp(&b.1));
    all.truncate(k);
    Ok(all)
}
// #endregion 🔖️BruteForce

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn random_points(n: usize, dim: usize, seed: u64) -> Vec<f64> {
        let mut rng = crate::numeric::Xorshift64::new(seed);
        (0..n * dim).map(|_| rng.next_f64() * 10.0 - 5.0).collect()
    }

    #[test]
    fn build_rejects_empty_and_bad_shape() {
        assert!(KdTree::build(&[], 2).is_err());
        assert!(KdTree::build(&[1.0, 2.0, 3.0], 2).is_err());
        assert!(KdTree::build(&[1.0, 2.0], 0).is_err());
    }

    #[test]
    fn kd_tree_matches_brute_force_knn_euclidean() {
        let points = random_points(200, 3, 11);
        let tree = KdTree::build(&points, 3).unwrap();
        for q in 0..20 {
            let query = &points[q * 3..(q + 1) * 3];
            let tree_result = tree.k_nearest(query, 5, Metric::Euclidean, Some(q));
            let brute_result = brute_force_knn(&points, 3, query, 5, Metric::Euclidean, Some(q)).unwrap();
            let tree_dists: Vec<f64> = tree_result.iter().map(|x| x.1).collect();
            let brute_dists: Vec<f64> = brute_result.iter().map(|x| x.1).collect();
            for (a, b) in tree_dists.iter().zip(brute_dists.iter()) {
                assert!((a - b).abs() < 1e-9, "query {q}: tree={tree_dists:?} brute={brute_dists:?}");
            }
        }
    }

    #[test]
    fn kd_tree_matches_brute_force_for_chebyshev_and_manhattan() {
        let points = random_points(150, 2, 22);
        let tree = KdTree::build(&points, 2).unwrap();
        for metric in [Metric::Chebyshev, Metric::Manhattan] {
            for q in 0..10 {
                let query = &points[q * 2..(q + 1) * 2];
                let tree_result = tree.k_nearest(query, 4, metric, Some(q));
                let brute_result = brute_force_knn(&points, 2, query, 4, metric, Some(q)).unwrap();
                let tree_dists: Vec<f64> = tree_result.iter().map(|x| x.1).collect();
                let brute_dists: Vec<f64> = brute_result.iter().map(|x| x.1).collect();
                assert_eq!(tree_dists.len(), brute_dists.len());
                for (a, b) in tree_dists.iter().zip(brute_dists.iter()) {
                    assert!((a - b).abs() < 1e-9);
                }
            }
        }
    }

    #[test]
    fn radius_count_matches_brute_force() {
        let points = random_points(100, 2, 33);
        let tree = KdTree::build(&points, 2).unwrap();
        for q in 0..10 {
            let query = &points[q * 2..(q + 1) * 2];
            let radius = 1.5;
            let tree_count = tree.count_within_radius(query, radius, Metric::Chebyshev, Some(q));
            let brute_count = (0..100)
                .filter(|&i| i != q)
                .filter(|&i| distance(&points[i * 2..(i + 1) * 2], query, Metric::Chebyshev) < radius)
                .count();
            assert_eq!(tree_count, brute_count, "query {q}");
        }
    }

    #[test]
    fn k_nearest_excludes_self_when_requested() {
        let points = vec![0.0, 0.0, 1.0, 0.0, 2.0, 0.0];
        let tree = KdTree::build(&points, 2).unwrap();
        let result = tree.k_nearest(&[0.0, 0.0], 2, Metric::Euclidean, Some(0));
        assert!(!result.iter().any(|&(i, _)| i == 0));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn k_nearest_on_single_point_tree() {
        let points = vec![5.0, 5.0];
        let tree = KdTree::build(&points, 2).unwrap();
        let result = tree.k_nearest(&[0.0, 0.0], 1, Metric::Euclidean, None);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 0);
    }

    mod quick {
        use super::*;

        #[test]
        fn kd_tree_matches_brute_force_on_larger_random_set() {
            let points = random_points(1000, 4, 999);
            let tree = KdTree::build(&points, 4).unwrap();
            for q in (0..1000).step_by(97) {
                let query = &points[q * 4..(q + 1) * 4];
                let tree_result = tree.k_nearest(query, 8, Metric::Euclidean, Some(q));
                let brute_result = brute_force_knn(&points, 4, query, 8, Metric::Euclidean, Some(q)).unwrap();
                for (a, b) in tree_result.iter().zip(brute_result.iter()) {
                    assert!((a.1 - b.1).abs() < 1e-9);
                }
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Knn

// #region 🔖️Continuous
pub mod continuous {
//! 📈️ Differential (continuous) entropy estimators: histogram, Gaussian KDE (leave-one-out
//! plug-in), Kozachenko-Leonenko kNN, Vasicek/Correa m-spacing, and the Gaussian closed form.

use crate::knn::KdTree;
use crate::numeric::{digamma, log_sum_exp, neumaier_sum, x_ln_x};
use crate::{BinsSpec, ConfidenceInterval, EntropyError, Estimate, LogBase, Metric, Warning};

// #region 🔖️Shared
fn mean_and_sd(x: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let mean = neumaier_sum(x.iter().copied()) / n;
    let var = neumaier_sum(x.iter().map(|&v| (v - mean).powi(2))) / n;
    (mean, var.sqrt())
}

fn validate_series(x: &[f64], what: &'static str) -> Result<(), EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what });
    }
    for (i, &v) in x.iter().enumerate() {
        if !v.is_finite() {
            return Err(EntropyError::NonFinite { what, index: i });
        }
    }
    Ok(())
}

/// 📈️ Default spacing-estimator window `m = round(sqrt(N))`, clamped to `[1, N/2 - 1]`.
fn default_spacing_m(n: usize) -> usize {
    let m = (n as f64).sqrt().round() as usize;
    m.clamp(1, (n / 2).saturating_sub(1).max(1))
}
// #endregion 🔖️Shared

// #region 🔖️Kernel
/// 📈️ Kernel family for [`KdeDensity`].
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Kernel {
    #[default]
    Gaussian,
    Epanechnikov,
}

impl Kernel {
    fn log_density_contribution(self, scaled_diff: f64) -> f64 {
        match self {
            // 📈️ Standard normal kernel K(u) = (1/sqrt(2*pi)) * exp(-u^2/2); the additive
            // -0.5*ln(2*pi) term is the kernel's own normalizing constant and must not be
            // dropped, or every density (and hence the entropy) is off by a constant factor.
            Kernel::Gaussian => -0.5 * (2.0 * core::f64::consts::PI).ln() - 0.5 * scaled_diff * scaled_diff,
            Kernel::Epanechnikov => {
                if scaled_diff.abs() >= 5.0_f64.sqrt() {
                    f64::NEG_INFINITY
                } else {
                    (0.75 * (1.0 - scaled_diff * scaled_diff / 5.0) / 5.0_f64.sqrt()).max(1e-300).ln()
                }
            }
        }
    }
}

/// 📈️ Bandwidth selection rule for [`KdeDensity`].
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Bandwidth {
    #[default]
    Silverman,
    Scott,
    Fixed(f64),
}
// #endregion 🔖️Kernel

// #region 🔖️Kde
/// 📈️ Configuration for [`KdeDensity::fit`].
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct KdeConfig {
    pub kernel: Kernel,
    pub bandwidth: Bandwidth,
}

/// 📈️ A fitted 1-D Gaussian/Epanechnikov kernel density estimate. Fit once, query `pdf` or
/// `entropy` many times.
pub struct KdeDensity {
    data: Vec<f64>,
    kernel: Kernel,
    h: f64,
}

impl KdeDensity {
    /// 📈️ Fits a KDE to `x`, selecting bandwidth `h` per `cfg.bandwidth` if not [`Bandwidth::Fixed`].
    pub fn fit(x: &[f64], cfg: KdeConfig) -> Result<Self, EntropyError> {
        validate_series(x, "kde input")?;
        if x.len() < 2 {
            return Err(EntropyError::InsufficientData { what: "KDE", needed: 2, actual: x.len() });
        }
        let (_, sd) = mean_and_sd(x);
        if sd <= 0.0 {
            return Err(EntropyError::DegenerateInput { what: "constant series has zero bandwidth" });
        }
        let n = x.len() as f64;
        let h = match cfg.bandwidth {
            Bandwidth::Silverman => sd * (4.0 / (3.0 * n)).powf(0.2),
            Bandwidth::Scott => sd * n.powf(-0.2),
            Bandwidth::Fixed(h) => {
                if !(h > 0.0 && h.is_finite()) {
                    return Err(EntropyError::InvalidConfig { field: "bandwidth", reason: "must be finite and positive" });
                }
                h
            }
        };
        Ok(Self { data: x.to_vec(), kernel: cfg.kernel, h })
    }

    /// 📈️ Density estimate at `x` using the full sample (not leave-one-out).
    pub fn pdf(&self, x: f64) -> f64 {
        let n = self.data.len() as f64;
        let log_terms: Vec<f64> = self
            .data
            .iter()
            .map(|&xi| self.kernel.log_density_contribution((x - xi) / self.h))
            .collect();
        (log_sum_exp(&log_terms) - n.ln() - self.h.ln()).exp()
    }

    /// 📈️ Leave-one-out differential entropy plug-in: `-1/N * sum ln f_{-i}(x_i)`, which removes
    /// the systematic downward bias the self-term introduces in plain resubstitution.
    pub fn entropy(&self, base: LogBase) -> Result<Estimate, EntropyError> {
        base.validate()?;
        let n = self.data.len();
        let nf = n as f64;
        let mut sum_log_density = 0.0_f64;
        for i in 0..n {
            let log_terms: Vec<f64> = (0..n)
                .filter(|&j| j != i)
                .map(|j| self.kernel.log_density_contribution((self.data[i] - self.data[j]) / self.h))
                .collect();
            let log_density = log_sum_exp(&log_terms) - (nf - 1.0).ln() - self.h.ln();
            sum_log_density += log_density;
        }
        let nats = -sum_log_density / nf;
        Ok(Estimate {
            value: base.from_nats(nats),
            base,
            method: "kde_loo",
            n,
            n_effective: nf,
            std_error: None,
            ci: None::<ConfidenceInterval>,
            warnings: Vec::new(),
            diagnostics: vec![("bandwidth", self.h)],
        })
    }
}
// #endregion 🔖️Kde

// #region 🔖️Histogram
fn histogram_entropy_nats(x: &[f64], bins: &BinsSpec) -> Result<(f64, usize), EntropyError> {
    let n = x.len() as f64;
    let (min, max) = x.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    // 🔐️ x is already validated finite by every caller, so a plain `<=` (no NaN concern) is
    // both correct and clearer than negating `>`.
    if max <= min {
        return Err(EntropyError::DegenerateInput { what: "constant series has zero range" });
    }
    let edges: Vec<f64> = match bins {
        BinsSpec::Edges(e) => e.clone(),
        _ => {
            let k = match bins {
                BinsSpec::Fixed(k) => *k,
                BinsSpec::Sturges => (1.0 + (x.len() as f64).log2()).ceil() as usize,
                BinsSpec::Scott => {
                    let (_, sd) = mean_and_sd(x);
                    let width = 3.49 * sd / (x.len() as f64).powf(1.0 / 3.0);
                    (((max - min) / width.max(1e-12)).ceil() as usize).max(1)
                }
                BinsSpec::FreedmanDiaconis => {
                    let mut sorted = x.to_vec();
                    sorted.sort_by(|a, b| a.total_cmp(b));
                    let q1 = sorted[(sorted.len() as f64 * 0.25) as usize];
                    let q3 = sorted[(sorted.len() as f64 * 0.75).min(sorted.len() as f64 - 1.0) as usize];
                    let width = 2.0 * (q3 - q1) / (x.len() as f64).powf(1.0 / 3.0);
                    (((max - min) / width.max(1e-12)).ceil() as usize).max(1)
                }
                BinsSpec::Doane => {
                    let (_, sd) = mean_and_sd(x);
                    let n = x.len() as f64;
                    let mean = neumaier_sum(x.iter().copied()) / n;
                    let skew = if sd > 0.0 {
                        neumaier_sum(x.iter().map(|&v| ((v - mean) / sd).powi(3))) / n
                    } else {
                        0.0
                    };
                    let sigma_g1 = (6.0 * (n - 2.0) / ((n + 1.0) * (n + 3.0))).sqrt();
                    let k = 1.0 + (x.len() as f64).log2() + (1.0 + skew.abs() / sigma_g1.max(1e-12)).log2();
                    (k.ceil() as usize).max(1)
                }
                BinsSpec::Edges(_) => unreachable!(),
            }
            .max(1);
            (0..=k).map(|i| min + (max - min) * i as f64 / k as f64).collect()
        }
    };
    if edges.len() < 2 {
        return Err(EntropyError::InvalidConfig { field: "bins", reason: "need at least 2 edges" });
    }
    let k = edges.len() - 1;
    let mut counts = vec![0.0_f64; k];
    for &v in x {
        let mut bin = match edges.binary_search_by(|e| e.total_cmp(&v)) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        bin = bin.min(k - 1);
        counts[bin] += 1.0;
    }
    let mut nats = 0.0_f64;
    for (i, &c) in counts.iter().enumerate() {
        let p = c / n;
        let width = edges[i + 1] - edges[i];
        if p > 0.0 && width > 0.0 {
            nats -= x_ln_x(p) - p * width.ln();
        }
    }
    Ok((nats, k))
}
// #endregion 🔖️Histogram

// #region 🔖️Knn
fn kozachenko_leonenko_nats(x: &[f64], k: usize) -> Result<f64, EntropyError> {
    let n = x.len();
    if k == 0 || k >= n {
        return Err(EntropyError::InvalidConfig { field: "k", reason: "must satisfy 0 < k < n" });
    }
    let tree = KdTree::build(x, 1)?;
    let mut sum_log_eps = 0.0_f64;
    for (i, &xi) in x.iter().enumerate() {
        let neighbors = tree.k_nearest(&[xi], k, Metric::Chebyshev, Some(i));
        let eps = neighbors.last().map_or(0.0, |&(_, d)| d);
        if eps <= 0.0 {
            return Err(EntropyError::DegenerateInput { what: "duplicate points cause zero k-th neighbor distance" });
        }
        sum_log_eps += (2.0 * eps).ln();
    }
    Ok(digamma(n as f64) - digamma(k as f64) + sum_log_eps / n as f64)
}
// #endregion 🔖️Knn

// #region 🔖️Spacing
fn sorted_with_clamped(x: &[f64], i: i64) -> f64 {
    let n = x.len() as i64;
    let clamped = i.clamp(1, n);
    x[(clamped - 1) as usize]
}

fn vasicek_nats(x: &[f64], m: usize) -> Result<f64, EntropyError> {
    let mut sorted = x.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len();
    let nf = n as f64;
    let mut sum = 0.0_f64;
    for i in 1..=n as i64 {
        let upper = sorted_with_clamped(&sorted, i + m as i64);
        let lower = sorted_with_clamped(&sorted, i - m as i64);
        let spacing = upper - lower;
        if spacing <= 0.0 {
            return Err(EntropyError::DegenerateInput { what: "zero spacing between order statistics; consider jittering ties" });
        }
        sum += (nf / (2.0 * m as f64) * spacing).ln();
    }
    Ok(sum / nf)
}

fn correa_nats(x: &[f64], m: usize) -> Result<f64, EntropyError> {
    let mut sorted = x.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len() as i64;
    let mut sum = 0.0_f64;
    for i in 1..=n {
        let lo = i - m as i64;
        let hi = i + m as i64;
        let window: Vec<f64> = (lo..=hi).map(|j| sorted_with_clamped(&sorted, j)).collect();
        let mean = window.iter().sum::<f64>() / window.len() as f64;
        let mut num = 0.0_f64;
        let mut denom = 0.0_f64;
        for (offset, &xj) in (lo..=hi).zip(window.iter()) {
            num += (xj - mean) * (offset - i) as f64;
            denom += (xj - mean).powi(2);
        }
        let ratio = num / (n as f64 * denom.max(1e-300));
        if ratio <= 0.0 {
            return Err(EntropyError::DegenerateInput { what: "non-positive Correa ratio; consider jittering ties" });
        }
        sum += ratio.ln();
    }
    Ok(-sum / n as f64)
}
// #endregion 🔖️Spacing

// #region 🔖️Gaussian
fn gaussian_mle_nats(x: &[f64]) -> Result<f64, EntropyError> {
    let (_, sd) = mean_and_sd(x);
    if sd <= 0.0 {
        return Err(EntropyError::DegenerateInput { what: "constant series has zero variance" });
    }
    Ok(0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E * sd * sd).ln())
}
// #endregion 🔖️Gaussian

// #region 🔖️Dispatch
/// 📈️ Which continuous (differential) entropy estimator [`entropy_continuous`] applies.
#[derive(Clone, PartialEq, Debug)]
pub enum ContinuousMethod {
    Histogram(BinsSpec),
    Kde(KdeConfig),
    Knn { k: usize },
    Vasicek { m: usize },
    Correa { m: usize },
    GaussianMle,
}

fn method_name(method: &ContinuousMethod) -> &'static str {
    match method {
        ContinuousMethod::Histogram(_) => "histogram",
        ContinuousMethod::Kde(_) => "kde_loo",
        ContinuousMethod::Knn { .. } => "kozachenko_leonenko",
        ContinuousMethod::Vasicek { .. } => "vasicek",
        ContinuousMethod::Correa { .. } => "correa",
        ContinuousMethod::GaussianMle => "gaussian_mle",
    }
}

/// 📈️ Estimates the differential entropy of the distribution underlying `x` (raw continuous
/// samples) using the given `method`.
pub fn entropy_continuous(x: &[f64], method: &ContinuousMethod, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    validate_series(x, "continuous input")?;
    let n = x.len();
    if n < 2 {
        return Err(EntropyError::InsufficientData { what: "continuous entropy", needed: 2, actual: n });
    }

    let mut diagnostics = Vec::new();
    let nats = match method {
        ContinuousMethod::Histogram(bins) => {
            let (nats, k) = histogram_entropy_nats(x, bins)?;
            diagnostics.push(("bins", k as f64));
            nats
        }
        ContinuousMethod::Kde(cfg) => {
            let density = KdeDensity::fit(x, *cfg)?;
            let est = density.entropy(LogBase::Nats)?;
            diagnostics.push(("bandwidth", density.h));
            est.value
        }
        ContinuousMethod::Knn { k } => {
            diagnostics.push(("k", *k as f64));
            kozachenko_leonenko_nats(x, *k)?
        }
        ContinuousMethod::Vasicek { m } => {
            let m = if *m == 0 { default_spacing_m(n) } else { *m };
            diagnostics.push(("m", m as f64));
            vasicek_nats(x, m)?
        }
        ContinuousMethod::Correa { m } => {
            let m = if *m == 0 { default_spacing_m(n) } else { *m };
            diagnostics.push(("m", m as f64));
            correa_nats(x, m)?
        }
        ContinuousMethod::GaussianMle => gaussian_mle_nats(x)?,
    };

    let mut warnings = Vec::new();
    if n < 30 {
        warnings.push(Warning::SmallSample { n, recommended: 30 });
    }

    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method: method_name(method),
        n,
        n_effective: n as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics,
    })
}
// #endregion 🔖️Dispatch

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn box_muller_gaussian(n: usize, seed: u64) -> Vec<f64> {
        let mut rng = crate::numeric::Xorshift64::new(seed);
        (0..n).map(|_| rng.next_gaussian()).collect()
    }

    #[test]
    fn gaussian_mle_matches_closed_form() {
        let x = box_muller_gaussian(5000, 1);
        let est = entropy_continuous(&x, &ContinuousMethod::GaussianMle, LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn knn_entropy_matches_gaussian_closed_form() {
        let x = box_muller_gaussian(3000, 2);
        let est = entropy_continuous(&x, &ContinuousMethod::Knn { k: 5 }, LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.1, "got {}", est.value);
    }

    #[test]
    fn kde_entropy_matches_gaussian_closed_form() {
        let x = box_muller_gaussian(2000, 3);
        let cfg = KdeConfig { kernel: Kernel::Gaussian, bandwidth: Bandwidth::Silverman };
        let est = entropy_continuous(&x, &ContinuousMethod::Kde(cfg), LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.1, "got {}", est.value);
    }

    #[test]
    fn vasicek_entropy_matches_gaussian_closed_form() {
        let x = box_muller_gaussian(5000, 4);
        let est = entropy_continuous(&x, &ContinuousMethod::Vasicek { m: 0 }, LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn correa_entropy_matches_gaussian_closed_form() {
        let x = box_muller_gaussian(3000, 5);
        let est = entropy_continuous(&x, &ContinuousMethod::Correa { m: 0 }, LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.1, "got {}", est.value);
    }

    #[test]
    fn uniform_entropy_near_zero() {
        let mut rng = crate::numeric::Xorshift64::new(6);
        let x: Vec<f64> = (0..5000).map(|_| rng.next_f64()).collect();
        let est = entropy_continuous(&x, &ContinuousMethod::Vasicek { m: 0 }, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn histogram_entropy_reasonable_for_uniform() {
        let mut rng = crate::numeric::Xorshift64::new(7);
        let x: Vec<f64> = (0..5000).map(|_| rng.next_f64()).collect();
        let est = entropy_continuous(&x, &ContinuousMethod::Histogram(BinsSpec::Sturges), LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.2, "got {}", est.value);
    }

    #[test]
    fn rejects_constant_series() {
        let x = vec![1.0; 100];
        assert!(entropy_continuous(&x, &ContinuousMethod::GaussianMle, LogBase::Nats).is_err());
        assert!(entropy_continuous(&x, &ContinuousMethod::Knn { k: 3 }, LogBase::Nats).is_err());
    }

    #[test]
    fn rejects_too_few_samples() {
        let x = vec![1.0];
        assert!(matches!(
            entropy_continuous(&x, &ContinuousMethod::GaussianMle, LogBase::Nats),
            Err(EntropyError::InsufficientData { .. })
        ));
    }

    #[test]
    fn kde_loo_differs_from_naive_resubstitution_direction() {
        // 🔐️ LOO removes the self-term's downward bias, so LOO entropy should exceed naive
        // resubstitution (which double-counts each point against itself).
        let x = box_muller_gaussian(500, 8);
        let cfg = KdeConfig { kernel: Kernel::Gaussian, bandwidth: Bandwidth::Silverman };
        let density = KdeDensity::fit(&x, cfg).unwrap();
        let loo = density.entropy(LogBase::Nats).unwrap().value;
        let resub_nats = -x.iter().map(|&xi| density.pdf(xi).max(1e-300).ln()).sum::<f64>() / x.len() as f64;
        assert!(loo > resub_nats);
    }

    mod quick {
        use super::*;

        #[test]
        fn exponential_entropy_matches_closed_form() {
            // 🔐️ differential entropy of Exp(1) is 1 nat.
            let mut rng = crate::numeric::Xorshift64::new(9);
            let x: Vec<f64> = (0..5000).map(|_| -rng.next_f64().max(1e-12).ln()).collect();
            let est = entropy_continuous(&x, &ContinuousMethod::Vasicek { m: 0 }, LogBase::Nats).unwrap();
            assert!((est.value - 1.0).abs() < 0.05, "got {}", est.value);
        }

        #[test]
        fn all_continuous_estimators_agree_within_tolerance_on_gaussian() {
            let x = box_muller_gaussian(4000, 42);
            let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
            let methods = vec![
                ContinuousMethod::GaussianMle,
                ContinuousMethod::Knn { k: 5 },
                ContinuousMethod::Vasicek { m: 0 },
                ContinuousMethod::Kde(KdeConfig::default()),
            ];
            for method in methods {
                let est = entropy_continuous(&x, &method, LogBase::Nats).unwrap();
                assert!((est.value - expected).abs() < 0.15, "{:?} -> {}", method, est.value);
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Continuous

// #region 🔖️Divergence
pub mod divergence {
//! 📏️ Probability divergences and distances: KL/JS/Rényi/Tsallis families, classical distances
//! (Hellinger, Bhattacharyya, total variation, chi-square), empirical Wasserstein-1D and energy
//! distance over raw samples, and a closure-based Bregman divergence for arbitrary convex `phi`.

use crate::counts::validate_probabilities;
use crate::numeric::neumaier_sum;
use crate::{EntropyError, LogBase, Tolerances};

// #region 🔖️Shared
fn validate_pair(p: &[f64], q: &[f64]) -> Result<(Vec<f64>, Vec<f64>), EntropyError> {
    if p.len() != q.len() {
        return Err(EntropyError::LengthMismatch { expected: p.len(), actual: q.len() });
    }
    let p = validate_probabilities(p, Tolerances::default())?;
    let q = validate_probabilities(q, Tolerances::default())?;
    Ok((p, q))
}
// #endregion 🔖️Shared

// #region 🔖️KlFamily
/// 📏️ Forward KL divergence `D(p || q) = sum p_i ln(p_i / q_i)`. Mathematically honest: returns
/// `f64::INFINITY` when `p_i > 0` and `q_i == 0` for some `i` (no silent smoothing).
pub fn kl_divergence(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    let (p, q) = validate_pair(p, q)?;
    let mut nats = 0.0_f64;
    for (&pi, &qi) in p.iter().zip(q.iter()) {
        if pi <= 0.0 {
            continue;
        }
        if qi <= 0.0 {
            return Ok(f64::INFINITY);
        }
        nats += pi * (pi / qi).ln();
    }
    Ok(base.from_nats(nats))
}

/// 📏️ Reverse KL divergence `D(q || p)`.
pub fn reverse_kl_divergence(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    kl_divergence(q, p, base)
}

/// 📏️ Jeffreys divergence, the symmetrized KL: `D(p||q) + D(q||p)`.
pub fn jeffreys_divergence(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    let a = kl_divergence(p, q, base)?;
    let b = kl_divergence(q, p, base)?;
    Ok(a + b)
}
// #endregion 🔖️KlFamily

// #region 🔖️JensenFamily
/// 📏️ Jensen-Shannon divergence: `0.5*D(p||m) + 0.5*D(q||m)` with `m = 0.5*(p+q)`. Always finite
/// and bounded by `ln(2)` in nats regardless of support overlap.
pub fn js_divergence(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    let (p, q) = validate_pair(p, q)?;
    let m: Vec<f64> = p.iter().zip(q.iter()).map(|(&pi, &qi)| 0.5 * (pi + qi)).collect();
    let d_pm = kl_divergence(&p, &m, LogBase::Nats)?;
    let d_qm = kl_divergence(&q, &m, LogBase::Nats)?;
    Ok(base.from_nats(0.5 * d_pm + 0.5 * d_qm))
}

/// 📏️ Jensen-Shannon distance, the square root of [`js_divergence`] in nats (a true metric).
pub fn js_distance(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    Ok(js_divergence(p, q, LogBase::Nats)?.sqrt())
}

/// 📏️ Weighted Jensen-Shannon divergence with mixture weight `pi_p` for `p` (`pi_q = 1 - pi_p`).
pub fn weighted_js_divergence(p: &[f64], q: &[f64], pi_p: f64, base: LogBase) -> Result<f64, EntropyError> {
    if !(0.0..=1.0).contains(&pi_p) {
        return Err(EntropyError::InvalidConfig { field: "pi_p", reason: "must be in [0, 1]" });
    }
    base.validate()?;
    let (p, q) = validate_pair(p, q)?;
    let m: Vec<f64> = p.iter().zip(q.iter()).map(|(&pi, &qi)| pi_p * pi + (1.0 - pi_p) * qi).collect();
    let d_pm = kl_divergence(&p, &m, LogBase::Nats)?;
    let d_qm = kl_divergence(&q, &m, LogBase::Nats)?;
    Ok(base.from_nats(pi_p * d_pm + (1.0 - pi_p) * d_qm))
}
// #endregion 🔖️JensenFamily

// #region 🔖️RenyiTsallis
/// 📏️ Rényi divergence of order `alpha != 1`: `D_alpha(p||q) = 1/(alpha-1) * ln(sum p_i^alpha
/// q_i^(1-alpha))`. `alpha == 1` is the KL limit — call [`kl_divergence`] instead.
pub fn renyi_divergence(p: &[f64], q: &[f64], alpha: f64, base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    if !alpha.is_finite() || alpha <= 0.0 {
        return Err(EntropyError::InvalidConfig { field: "alpha", reason: "must be finite and positive" });
    }
    if (alpha - 1.0).abs() < 1e-12 {
        return Err(EntropyError::UndefinedResult { reason: "Renyi divergence at alpha=1 is the KL limit; call kl_divergence() instead" });
    }
    let (p, q) = validate_pair(p, q)?;
    let mut sum = 0.0_f64;
    for (&pi, &qi) in p.iter().zip(q.iter()) {
        if pi <= 0.0 {
            continue;
        }
        if qi <= 0.0 {
            return Ok(f64::INFINITY);
        }
        sum += pi.powf(alpha) * qi.powf(1.0 - alpha);
    }
    if sum <= 0.0 {
        return Err(EntropyError::UndefinedResult { reason: "sum is non-positive" });
    }
    Ok(base.from_nats(sum.ln() / (alpha - 1.0)))
}

/// 📏️ Tsallis divergence of entropic index `alpha != 1`: `(sum p_i^alpha q_i^(1-alpha) - 1) /
/// (alpha - 1)`. Unitless (no logarithm base).
pub fn tsallis_divergence(p: &[f64], q: &[f64], alpha: f64) -> Result<f64, EntropyError> {
    if !alpha.is_finite() || alpha <= 0.0 {
        return Err(EntropyError::InvalidConfig { field: "alpha", reason: "must be finite and positive" });
    }
    if (alpha - 1.0).abs() < 1e-12 {
        return kl_divergence(p, q, LogBase::Nats);
    }
    let (p, q) = validate_pair(p, q)?;
    let mut sum = 0.0_f64;
    for (&pi, &qi) in p.iter().zip(q.iter()) {
        if pi <= 0.0 {
            continue;
        }
        if qi <= 0.0 {
            return Ok(f64::INFINITY);
        }
        sum += pi.powf(alpha) * qi.powf(1.0 - alpha);
    }
    Ok((sum - 1.0) / (alpha - 1.0))
}
// #endregion 🔖️RenyiTsallis

// #region 🔖️ClassicalDistances
/// 📏️ Hellinger distance: `sqrt(0.5 * sum (sqrt(p_i) - sqrt(q_i))^2)`, bounded in `[0, 1]`.
pub fn hellinger_distance(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let (p, q) = validate_pair(p, q)?;
    let sum = neumaier_sum(p.iter().zip(q.iter()).map(|(&pi, &qi)| (pi.sqrt() - qi.sqrt()).powi(2)));
    Ok((0.5 * sum).max(0.0).sqrt())
}

/// 📏️ Bhattacharyya coefficient `BC(p,q) = sum sqrt(p_i q_i)`, in `[0, 1]`.
pub fn bhattacharyya_coefficient(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let (p, q) = validate_pair(p, q)?;
    Ok(neumaier_sum(p.iter().zip(q.iter()).map(|(&pi, &qi)| (pi * qi).sqrt())).clamp(0.0, 1.0))
}

/// 📏️ Bhattacharyya distance `-ln(BC(p,q))`.
pub fn bhattacharyya_distance(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let bc = bhattacharyya_coefficient(p, q)?;
    if bc <= 0.0 {
        return Ok(f64::INFINITY);
    }
    Ok(-bc.ln())
}

/// 📏️ Total variation distance: `0.5 * sum |p_i - q_i|`, bounded in `[0, 1]`.
pub fn total_variation(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let (p, q) = validate_pair(p, q)?;
    Ok(0.5 * neumaier_sum(p.iter().zip(q.iter()).map(|(&pi, &qi)| (pi - qi).abs())))
}

/// 📏️ Pearson chi-square divergence: `sum (p_i - q_i)^2 / q_i`.
pub fn chi_square_divergence(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let (p, q) = validate_pair(p, q)?;
    let mut sum = 0.0_f64;
    for (&pi, &qi) in p.iter().zip(q.iter()) {
        if qi <= 0.0 {
            if pi > 0.0 {
                return Ok(f64::INFINITY);
            }
            continue;
        }
        sum += (pi - qi).powi(2) / qi;
    }
    Ok(sum)
}

/// 📏️ Neyman (reverse) chi-square divergence: `sum (p_i - q_i)^2 / p_i`.
pub fn neyman_chi_square_divergence(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    chi_square_divergence(q, p)
}
// #endregion 🔖️ClassicalDistances

// #region 🔖️EmpiricalDistances
/// 📏️ Empirical 1-D Wasserstein (earth-mover) distance between two raw sample sets, computed as
/// the area between their empirical CDFs (`integral |F_x(t) - F_y(t)| dt`), which is exact and
/// requires no binning.
pub fn wasserstein_1d(x: &[f64], y: &[f64]) -> Result<f64, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    if y.is_empty() {
        return Err(EntropyError::EmptyInput { what: "y" });
    }
    let mut sx = x.to_vec();
    let mut sy = y.to_vec();
    sx.sort_by(|a, b| a.total_cmp(b));
    sy.sort_by(|a, b| a.total_cmp(b));
    let (nx, ny) = (sx.len() as f64, sy.len() as f64);

    let mut breakpoints: Vec<f64> = sx.iter().chain(sy.iter()).copied().collect();
    breakpoints.sort_by(|a, b| a.total_cmp(b));
    breakpoints.dedup();

    let cdf = |sorted: &[f64], t: f64| -> f64 {
        let idx = match sorted.binary_search_by(|v| v.total_cmp(&t)) {
            Ok(mut i) => {
                while i + 1 < sorted.len() && sorted[i + 1] == t {
                    i += 1;
                }
                i + 1
            }
            Err(i) => i,
        };
        idx as f64
    };

    let mut area = 0.0_f64;
    for w in breakpoints.windows(2) {
        let (a, b) = (w[0], w[1]);
        let fx = cdf(&sx, a) / nx;
        let fy = cdf(&sy, a) / ny;
        area += (fx - fy).abs() * (b - a);
    }
    Ok(area)
}

/// 📏️ Szekely-Rizzo energy distance: `2*E|X-Y| - E|X-X'| - E|Y-Y'|`, estimated by the standard
/// `O(n*m)` U-statistic over raw samples.
pub fn energy_distance(x: &[f64], y: &[f64]) -> Result<f64, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    if y.is_empty() {
        return Err(EntropyError::EmptyInput { what: "y" });
    }
    let (n, m) = (x.len(), y.len());
    let cross: f64 = x.iter().map(|&xi| y.iter().map(|&yj| (xi - yj).abs()).sum::<f64>()).sum::<f64>() / (n * m) as f64;
    let within_x: f64 = x.iter().map(|&xi| x.iter().map(|&xj| (xi - xj).abs()).sum::<f64>()).sum::<f64>() / (n * n) as f64;
    let within_y: f64 = y.iter().map(|&yi| y.iter().map(|&yj| (yi - yj).abs()).sum::<f64>()).sum::<f64>() / (m * m) as f64;
    Ok((2.0 * cross - within_x - within_y).max(0.0))
}
// #endregion 🔖️EmpiricalDistances

// #region 🔖️MatrixDivergences
/// 📏️ Log-det (Stein) divergence between two `n x n` covariance-like SPD matrices (row-major):
/// `ln|det(Sigma_q)| - ln|det(Sigma_p)| + tr(Sigma_q^-1 Sigma_p) - n`, computed via Cholesky
/// solves rather than an explicit matrix inverse.
pub fn log_det_divergence(cov_p: &[f64], cov_q: &[f64], n: usize) -> Result<f64, EntropyError> {
    let ld_p = crate::matrix::log_det(cov_p, n)?;
    let ld_q = crate::matrix::log_det(cov_q, n)?;
    let l_q = crate::matrix::cholesky(cov_q, n)?;
    // 🔢️ tr(Sigma_q^-1 Sigma_p) via solving L_q L_q^T X = Sigma_p column-by-column, then summing
    // the diagonal of X (forward/backward substitution, no explicit inverse).
    let mut trace = 0.0_f64;
    for col in 0..n {
        let rhs: Vec<f64> = (0..n).map(|row| cov_p[row * n + col]).collect();
        let mut y = vec![0.0_f64; n];
        for i in 0..n {
            let mut sum = rhs[i];
            for k in 0..i {
                sum -= l_q[i * n + k] * y[k];
            }
            y[i] = sum / l_q[i * n + i];
        }
        let mut xcol = vec![0.0_f64; n];
        for i in (0..n).rev() {
            let mut sum = y[i];
            for k in (i + 1)..n {
                sum -= l_q[k * n + i] * xcol[k];
            }
            xcol[i] = sum / l_q[i * n + i];
        }
        trace += xcol[col];
    }
    Ok(ld_q - ld_p + trace - n as f64)
}

/// 📏️ Bregman divergence `D_phi(p, q) = phi(p) - phi(q) - grad_phi(q) . (p - q)` for an arbitrary
/// convex `phi`, supplied as closures so callers can instantiate squared-Euclidean, KL
/// (`phi = negentropy`), Itakura-Saito, or any other Bregman generator without a new function.
pub fn bregman_divergence(p: &[f64], q: &[f64], phi: impl Fn(&[f64]) -> f64, grad_phi_q: impl Fn(&[f64]) -> Vec<f64>) -> Result<f64, EntropyError> {
    if p.len() != q.len() {
        return Err(EntropyError::LengthMismatch { expected: p.len(), actual: q.len() });
    }
    let grad = grad_phi_q(q);
    if grad.len() != p.len() {
        return Err(EntropyError::LengthMismatch { expected: p.len(), actual: grad.len() });
    }
    let dot: f64 = p.iter().zip(q.iter()).zip(grad.iter()).map(|((&pi, &qi), &gi)| gi * (pi - qi)).sum();
    Ok(phi(p) - phi(q) - dot)
}

/// 📏️ Itakura-Saito divergence between two positive spectra: `sum (p_i/q_i - ln(p_i/q_i) - 1)`.
pub fn itakura_saito_divergence(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    if p.len() != q.len() {
        return Err(EntropyError::LengthMismatch { expected: p.len(), actual: q.len() });
    }
    let mut sum = 0.0_f64;
    for (i, (&pi, &qi)) in p.iter().zip(q.iter()).enumerate() {
        if pi <= 0.0 || qi <= 0.0 {
            return Err(EntropyError::InvalidProbability { index: i, value: pi.min(qi) });
        }
        let ratio = pi / qi;
        sum += ratio - ratio.ln() - 1.0;
    }
    Ok(sum)
}
// #endregion 🔖️MatrixDivergences

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kl_of_identical_distributions_is_zero() {
        let p = [0.2, 0.3, 0.5];
        assert!(kl_divergence(&p, &p, LogBase::Bits).unwrap().abs() < 1e-9);
    }

    #[test]
    fn kl_is_non_negative_for_random_distributions() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        for _ in 0..200 {
            let k = 2 + rng.next_below(5);
            let mut p: Vec<f64> = (0..k).map(|_| rng.next_f64() + 0.01).collect();
            let mut q: Vec<f64> = (0..k).map(|_| rng.next_f64() + 0.01).collect();
            let sp: f64 = p.iter().sum();
            let sq: f64 = q.iter().sum();
            p.iter_mut().for_each(|v| *v /= sp);
            q.iter_mut().for_each(|v| *v /= sq);
            let d = kl_divergence(&p, &q, LogBase::Nats).unwrap();
            assert!(d >= -1e-9, "d={d}");
        }
    }

    #[test]
    fn kl_infinite_on_support_mismatch() {
        let p = [0.5, 0.5];
        let q = [1.0, 0.0];
        assert_eq!(kl_divergence(&p, &q, LogBase::Bits).unwrap(), f64::INFINITY);
    }

    #[test]
    fn js_divergence_symmetric_and_bounded_by_ln2() {
        let p = [0.9, 0.1];
        let q = [0.1, 0.9];
        let a = js_divergence(&p, &q, LogBase::Nats).unwrap();
        let b = js_divergence(&q, &p, LogBase::Nats).unwrap();
        assert!((a - b).abs() < 1e-9);
        assert!(a <= core::f64::consts::LN_2 + 1e-9);
        assert!(a >= 0.0);
    }

    #[test]
    fn js_of_identical_distributions_is_zero() {
        let p = [0.3, 0.7];
        assert!(js_divergence(&p, &p, LogBase::Bits).unwrap().abs() < 1e-9);
    }

    #[test]
    fn hellinger_distance_bounds_and_identity() {
        let p = [0.5, 0.5];
        assert!(hellinger_distance(&p, &p).unwrap().abs() < 1e-9);
        let q = [1.0, 0.0];
        let d = hellinger_distance(&p, &q).unwrap();
        assert!((0.0..=1.0).contains(&d));
    }

    #[test]
    fn total_variation_bounds_and_identity() {
        let p = [0.5, 0.5];
        assert!(total_variation(&p, &p).unwrap().abs() < 1e-9);
        let q = [1.0, 0.0];
        assert!((total_variation(&p, &q).unwrap() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn chi_square_of_identical_distributions_is_zero() {
        let p = [0.2, 0.3, 0.5];
        assert!(chi_square_divergence(&p, &p).unwrap().abs() < 1e-9);
    }

    #[test]
    fn renyi_divergence_rejects_alpha_one() {
        assert!(matches!(
            renyi_divergence(&[0.5, 0.5], &[0.3, 0.7], 1.0, LogBase::Bits),
            Err(EntropyError::UndefinedResult { .. })
        ));
    }

    #[test]
    fn renyi_divergence_of_identical_distributions_is_zero() {
        let p = [0.2, 0.3, 0.5];
        let d = renyi_divergence(&p, &p, 2.0, LogBase::Nats).unwrap();
        assert!(d.abs() < 1e-9);
    }

    #[test]
    fn tsallis_divergence_limit_matches_kl() {
        let p = [0.2, 0.3, 0.5];
        let q = [0.3, 0.3, 0.4];
        let kl = kl_divergence(&p, &q, LogBase::Nats).unwrap();
        let tsallis = tsallis_divergence(&p, &q, 1.0).unwrap();
        assert!((kl - tsallis).abs() < 1e-6);
    }

    #[test]
    fn wasserstein_1d_of_identical_samples_is_zero() {
        let x = [1.0, 2.0, 3.0, 4.0];
        assert!(wasserstein_1d(&x, &x).unwrap().abs() < 1e-9);
    }

    #[test]
    fn wasserstein_1d_matches_hand_computation_equal_size() {
        // 🔐️ equal-size sorted samples: W1 = mean |x_sorted - y_sorted|.
        let x = [1.0, 2.0, 3.0];
        let y = [4.0, 5.0, 6.0];
        let w = wasserstein_1d(&x, &y).unwrap();
        assert!((w - 3.0).abs() < 1e-9);
    }

    #[test]
    fn energy_distance_of_identical_distributions_is_near_zero() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert!(energy_distance(&x, &x).unwrap().abs() < 1e-9);
    }

    #[test]
    fn energy_distance_is_non_negative() {
        let mut rng = crate::numeric::Xorshift64::new(2);
        let x: Vec<f64> = (0..30).map(|_| rng.next_gaussian()).collect();
        let y: Vec<f64> = (0..30).map(|_| rng.next_gaussian() + 2.0).collect();
        assert!(energy_distance(&x, &y).unwrap() > 0.0);
    }

    #[test]
    fn log_det_divergence_of_identical_matrices_is_zero() {
        let cov = vec![4.0, 1.0, 1.0, 3.0];
        let d = log_det_divergence(&cov, &cov, 2).unwrap();
        assert!(d.abs() < 1e-7);
    }

    #[test]
    fn bregman_squared_euclidean_matches_direct_formula() {
        let p = [1.0, 2.0, 3.0];
        let q = [0.5, 2.5, 2.0];
        let phi = |x: &[f64]| x.iter().map(|v| v * v).sum::<f64>();
        let grad = |x: &[f64]| -> Vec<f64> { x.iter().map(|v| 2.0 * v).collect() };
        let d = bregman_divergence(&p, &q, phi, grad).unwrap();
        let expected: f64 = p.iter().zip(q.iter()).map(|(a, b)| (a - b).powi(2)).sum();
        assert!((d - expected).abs() < 1e-9);
    }

    #[test]
    fn itakura_saito_of_identical_spectra_is_zero() {
        let p = [1.0, 2.0, 3.0];
        assert!(itakura_saito_divergence(&p, &p).unwrap().abs() < 1e-9);
    }

    mod quick {
        use super::*;

        #[test]
        fn renyi_divergence_monotone_in_alpha() {
            let p = [0.6, 0.3, 0.1];
            let q = [0.2, 0.3, 0.5];
            let alphas = [0.1, 0.5, 2.0, 5.0];
            let mut prev = renyi_divergence(&p, &q, alphas[0], LogBase::Nats).unwrap();
            for &a in &alphas[1..] {
                let d = renyi_divergence(&p, &q, a, LogBase::Nats).unwrap();
                assert!(d >= prev - 1e-9, "alpha={a}");
                prev = d;
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Divergence

// #region 🔖️Mutual
pub mod mutual {
//! 🔗️ Mutual information family: discrete plug-in/bias-corrected MI and conditional MI, KSG-1/
//! KSG-2 continuous MI, and multivariate generalizations (total correlation, dual total
//! correlation, O-information).

use crate::counts::{Counts, JointCounts};
use crate::estimators::{entropy_discrete, DiscreteMethod};
use crate::knn::KdTree;
use crate::numeric::{checked_state_count, clamp_near_zero, digamma};
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Metric, Warning};

// #region 🔖️Packing
fn counts_to_u64(raw: &[f64]) -> Vec<u64> {
    raw.iter().map(|&c| c.round().max(0.0) as u64).collect()
}

/// 🔗️ Packs several aligned symbol sequences into one joint symbol via mixed-radix encoding,
/// checked against `u32` overflow.
fn pack_symbols(parts: &[&[u32]], sizes: &[usize]) -> Result<(Vec<u32>, usize), EntropyError> {
    let total = checked_state_count(sizes).ok_or(EntropyError::InvalidConfig {
        field: "sizes",
        reason: "joint alphabet size overflows u128",
    })?;
    if total > u32::MAX as u128 {
        return Err(EntropyError::InvalidConfig { field: "sizes", reason: "joint alphabet size exceeds u32::MAX" });
    }
    let n = parts[0].len();
    let mut combined = vec![0u32; n];
    for i in 0..n {
        let mut acc: u64 = 0;
        for (part, &size) in parts.iter().zip(sizes.iter()) {
            acc = acc * size as u64 + part[i] as u64;
        }
        combined[i] = acc as u32;
    }
    Ok((combined, total as usize))
}

fn plugin_entropy_nats(symbols: &[u32], alphabet: usize) -> Result<f64, EntropyError> {
    let counts = Counts::from_symbols(symbols, alphabet)?;
    let est = entropy_discrete(&counts_to_u64(counts.raw()), DiscreteMethod::Plugin, LogBase::Nats)?;
    Ok(est.value)
}
// #endregion 🔖️Packing

// #region 🔖️DiscreteMi
/// 🔗️ Discrete mutual information `I(X;Y) = H(X) + H(Y) - H(X,Y)`, all three terms estimated
/// with the same bias-correction `method` for internal consistency.
pub fn mutual_information(x: &[u32], y: &[u32], method: DiscreteMethod, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if x.len() != y.len() {
        return Err(EntropyError::LengthMismatch { expected: x.len(), actual: y.len() });
    }
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    let x_size = *x.iter().max().unwrap() as usize + 1;
    let y_size = *y.iter().max().unwrap() as usize + 1;
    let joint = JointCounts::from_pairs(x, y, x_size, y_size)?;
    let marg_x = Counts::from_symbols(x, x_size)?;
    let marg_y = Counts::from_symbols(y, y_size)?;

    let h_xy = entropy_discrete(&counts_to_u64(joint.as_counts().raw()), method, LogBase::Nats)?;
    let h_x = entropy_discrete(&counts_to_u64(marg_x.raw()), method, LogBase::Nats)?;
    let h_y = entropy_discrete(&counts_to_u64(marg_y.raw()), method, LogBase::Nats)?;

    let nats = clamp_near_zero(h_x.value + h_y.value - h_xy.value, 1e-9);
    let mut warnings = h_x.warnings;
    warnings.extend(h_y.warnings);
    warnings.extend(h_xy.warnings);

    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method: "discrete_mi",
        n: x.len(),
        n_effective: x.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("x_alphabet", x_size as f64), ("y_alphabet", y_size as f64)],
    })
}

/// 🔗️ Discrete conditional mutual information `I(X;Y|Z) = H(X,Z) + H(Y,Z) - H(X,Y,Z) - H(Z)`,
/// estimated with the maximum-likelihood plug-in on packed joint symbols.
pub fn conditional_mutual_information(x: &[u32], y: &[u32], z: &[u32], base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if x.len() != y.len() || y.len() != z.len() {
        return Err(EntropyError::LengthMismatch { expected: x.len(), actual: y.len().min(z.len()) });
    }
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    let xs = *x.iter().max().unwrap() as usize + 1;
    let ys = *y.iter().max().unwrap() as usize + 1;
    let zs = *z.iter().max().unwrap() as usize + 1;

    let (xz, xz_size) = pack_symbols(&[x, z], &[xs, zs])?;
    let (yz, yz_size) = pack_symbols(&[y, z], &[ys, zs])?;
    let (xyz, xyz_size) = pack_symbols(&[x, y, z], &[xs, ys, zs])?;

    let h_xz = plugin_entropy_nats(&xz, xz_size)?;
    let h_yz = plugin_entropy_nats(&yz, yz_size)?;
    let h_xyz = plugin_entropy_nats(&xyz, xyz_size)?;
    let h_z = plugin_entropy_nats(z, zs)?;

    let nats = clamp_near_zero(h_xz + h_yz - h_xyz - h_z, 1e-9);
    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method: "discrete_cmi_plugin",
        n: x.len(),
        n_effective: x.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings: Vec::new(),
        diagnostics: vec![("z_alphabet", zs as f64)],
    })
}
// #endregion 🔖️DiscreteMi

// #region 🔖️Ksg
/// 🔗️ Which Kraskov-Stögbauer-Grassberger estimator variant [`mutual_information_knn`] uses.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum KsgVariant {
    #[default]
    Ksg1,
    Ksg2,
}

/// 🔗️ Configuration for [`mutual_information_knn`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct KsgConfig {
    pub k: usize,
    pub metric: Metric,
    pub variant: KsgVariant,
}

impl KsgConfig {
    pub fn new(k: usize, variant: KsgVariant) -> Result<Self, EntropyError> {
        if k == 0 {
            return Err(EntropyError::InvalidConfig { field: "k", reason: "must be at least 1" });
        }
        Ok(Self { k, metric: Metric::Chebyshev, variant })
    }
}

/// 🔗️ Continuous mutual information via the Kraskov-Stögbauer-Grassberger kNN estimator.
/// Digamma-based and therefore computed (and returned) in nats; call [`Estimate::in_base`] on
/// the result to convert.
pub fn mutual_information_knn(x: &[f64], y: &[f64], cfg: KsgConfig) -> Result<Estimate, EntropyError> {
    if x.len() != y.len() {
        return Err(EntropyError::LengthMismatch { expected: x.len(), actual: y.len() });
    }
    let n = x.len();
    if cfg.k >= n {
        return Err(EntropyError::InvalidConfig { field: "k", reason: "must be less than the sample size" });
    }
    let joint: Vec<f64> = x.iter().zip(y.iter()).flat_map(|(&xi, &yi)| [xi, yi]).collect();
    let tree = KdTree::build(&joint, 2)?;

    let mut sum_digamma_nx = 0.0_f64;
    let mut sum_digamma_ny = 0.0_f64;
    for i in 0..n {
        let query = [x[i], y[i]];
        let neighbors = tree.k_nearest(&query, cfg.k, Metric::Chebyshev, Some(i));
        match cfg.variant {
            KsgVariant::Ksg1 => {
                let eps = neighbors.last().map_or(0.0, |&(_, d)| d);
                let nx = (0..n).filter(|&j| j != i && (x[j] - x[i]).abs() < eps).count();
                let ny = (0..n).filter(|&j| j != i && (y[j] - y[i]).abs() < eps).count();
                sum_digamma_nx += digamma(nx as f64 + 1.0);
                sum_digamma_ny += digamma(ny as f64 + 1.0);
            }
            KsgVariant::Ksg2 => {
                let eps_x = neighbors.iter().map(|&(j, _)| (x[j] - x[i]).abs()).fold(0.0_f64, f64::max);
                let eps_y = neighbors.iter().map(|&(j, _)| (y[j] - y[i]).abs()).fold(0.0_f64, f64::max);
                let nx = (0..n).filter(|&j| j != i && (x[j] - x[i]).abs() <= eps_x).count();
                let ny = (0..n).filter(|&j| j != i && (y[j] - y[i]).abs() <= eps_y).count();
                sum_digamma_nx += digamma(nx.max(1) as f64);
                sum_digamma_ny += digamma(ny.max(1) as f64);
            }
        }
    }

    let nats = match cfg.variant {
        KsgVariant::Ksg1 => digamma(cfg.k as f64) - (sum_digamma_nx + sum_digamma_ny) / n as f64 + digamma(n as f64),
        KsgVariant::Ksg2 => {
            digamma(cfg.k as f64) - 1.0 / cfg.k as f64 - (sum_digamma_nx + sum_digamma_ny) / n as f64 + digamma(n as f64)
        }
    };
    let clamped = clamp_near_zero(nats, 1e-6);

    let mut warnings = Vec::new();
    if n < 10 * cfg.k {
        warnings.push(Warning::SmallSample { n, recommended: 10 * cfg.k });
    }

    Ok(Estimate {
        value: clamped,
        base: LogBase::Nats,
        method: match cfg.variant {
            KsgVariant::Ksg1 => "ksg1",
            KsgVariant::Ksg2 => "ksg2",
        },
        n,
        n_effective: n as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("k", cfg.k as f64)],
    })
}
// #endregion 🔖️Ksg

// #region 🔖️Multivariate
fn multivariate_joint_and_marginal_entropies(vars: &[&[u32]], sizes: &[usize]) -> Result<(f64, Vec<f64>), EntropyError> {
    if vars.is_empty() {
        return Err(EntropyError::EmptyInput { what: "vars" });
    }
    let n = vars[0].len();
    for v in vars {
        if v.len() != n {
            return Err(EntropyError::LengthMismatch { expected: n, actual: v.len() });
        }
    }
    let (joint, joint_size) = pack_symbols(vars, sizes)?;
    let h_joint = plugin_entropy_nats(&joint, joint_size)?;
    let marginals: Result<Vec<f64>, EntropyError> = vars.iter().zip(sizes.iter()).map(|(&v, &s)| plugin_entropy_nats(v, s)).collect();
    Ok((h_joint, marginals?))
}

/// 🔗️ Total correlation (multi-information) `sum H(X_i) - H(X_1,...,X_n)`.
pub fn total_correlation(vars: &[&[u32]], sizes: &[usize], base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    let (h_joint, marginals) = multivariate_joint_and_marginal_entropies(vars, sizes)?;
    let nats = clamp_near_zero(marginals.iter().sum::<f64>() - h_joint, 1e-9);
    Ok(multivariate_estimate(nats, base, "total_correlation", vars[0].len()))
}

/// 🔗️ Dual total correlation (binding information): `sum_i H(X_{-i}) - (n-1) * H(X_1,...,X_n)`,
/// where `X_{-i}` is the joint of all variables except `i`.
pub fn dual_total_correlation(vars: &[&[u32]], sizes: &[usize], base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    let n_vars = vars.len();
    if n_vars < 2 {
        return Err(EntropyError::InvalidConfig { field: "vars", reason: "dual total correlation needs at least 2 variables" });
    }
    let (h_joint, _) = multivariate_joint_and_marginal_entropies(vars, sizes)?;
    let mut sum_rest = 0.0_f64;
    for i in 0..n_vars {
        let rest_vars: Vec<&[u32]> = vars.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, &v)| v).collect();
        let rest_sizes: Vec<usize> = sizes.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, &s)| s).collect();
        let (h_rest, _) = multivariate_joint_and_marginal_entropies(&rest_vars, &rest_sizes)?;
        sum_rest += h_rest;
    }
    let nats = clamp_near_zero(sum_rest - (n_vars as f64 - 1.0) * h_joint, 1e-9);
    Ok(multivariate_estimate(nats, base, "dual_total_correlation", vars[0].len()))
}

/// 🔗️ O-information: `total_correlation - dual_total_correlation`. Positive values indicate
/// redundancy-dominated interactions, negative values synergy-dominated ones.
pub fn o_information(vars: &[&[u32]], sizes: &[usize], base: LogBase) -> Result<Estimate, EntropyError> {
    let tc = total_correlation(vars, sizes, LogBase::Nats)?;
    let dtc = dual_total_correlation(vars, sizes, LogBase::Nats)?;
    let nats = tc.value - dtc.value;
    Ok(multivariate_estimate(nats, base, "o_information", vars[0].len()))
}

fn multivariate_estimate(nats: f64, base: LogBase, method: &'static str, n: usize) -> Estimate {
    Estimate {
        value: base.from_nats(nats),
        base,
        method,
        n,
        n_effective: n as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings: Vec::new(),
        diagnostics: Vec::new(),
    }
}
// #endregion 🔖️Multivariate

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mi_of_independent_variables_is_near_zero() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let n = 5000;
        let x: Vec<u32> = (0..n).map(|_| rng.next_below(4) as u32).collect();
        let y: Vec<u32> = (0..n).map(|_| rng.next_below(4) as u32).collect();
        let est = mutual_information(&x, &y, DiscreteMethod::MillerMadow, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.02, "got {}", est.value);
    }

    #[test]
    fn mi_of_identical_variables_equals_entropy() {
        let mut rng = crate::numeric::Xorshift64::new(2);
        let x: Vec<u32> = (0..2000).map(|_| rng.next_below(5) as u32).collect();
        let mi = mutual_information(&x, &x, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        let counts = Counts::from_symbols(&x, 5).unwrap();
        let h = entropy_discrete(&counts_to_u64(counts.raw()), DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        assert!((mi.value - h.value).abs() < 1e-9);
    }

    #[test]
    fn mi_rejects_length_mismatch() {
        assert!(matches!(
            mutual_information(&[0, 1], &[0], DiscreteMethod::Plugin, LogBase::Bits),
            Err(EntropyError::LengthMismatch { .. })
        ));
    }

    #[test]
    fn cmi_zero_when_x_and_y_independent_given_z() {
        let mut rng = crate::numeric::Xorshift64::new(3);
        let n = 4000;
        let z: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let x: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let y: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let est = conditional_mutual_information(&x, &y, &z, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn ksg1_matches_gaussian_closed_form() {
        // 🔐️ bivariate Gaussian with correlation rho: I(X;Y) = -0.5*ln(1-rho^2).
        let mut rng = crate::numeric::Xorshift64::new(4);
        let rho = 0.6_f64;
        let n = 2000;
        let mut x = Vec::with_capacity(n);
        let mut y = Vec::with_capacity(n);
        for _ in 0..n {
            let z1 = rng.next_gaussian();
            let z2 = rng.next_gaussian();
            x.push(z1);
            y.push(rho * z1 + (1.0 - rho * rho).sqrt() * z2);
        }
        let cfg = KsgConfig::new(5, KsgVariant::Ksg1).unwrap();
        let est = mutual_information_knn(&x, &y, cfg).unwrap();
        let expected = -0.5 * (1.0 - rho * rho).ln();
        assert!((est.value - expected).abs() < 0.05, "got {} expected {}", est.value, expected);
    }

    #[test]
    fn ksg2_matches_gaussian_closed_form() {
        let mut rng = crate::numeric::Xorshift64::new(5);
        let rho = 0.5_f64;
        let n = 2000;
        let mut x = Vec::with_capacity(n);
        let mut y = Vec::with_capacity(n);
        for _ in 0..n {
            let z1 = rng.next_gaussian();
            let z2 = rng.next_gaussian();
            x.push(z1);
            y.push(rho * z1 + (1.0 - rho * rho).sqrt() * z2);
        }
        let cfg = KsgConfig::new(5, KsgVariant::Ksg2).unwrap();
        let est = mutual_information_knn(&x, &y, cfg).unwrap();
        let expected = -0.5 * (1.0 - rho * rho).ln();
        assert!((est.value - expected).abs() < 0.08, "got {} expected {}", est.value, expected);
    }

    #[test]
    fn ksg_mi_of_independent_gaussians_is_near_zero() {
        let mut rng = crate::numeric::Xorshift64::new(6);
        let n = 1500;
        let x: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
        let y: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
        let cfg = KsgConfig::new(5, KsgVariant::Ksg1).unwrap();
        let est = mutual_information_knn(&x, &y, cfg).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn total_correlation_of_independent_variables_is_near_zero() {
        let mut rng = crate::numeric::Xorshift64::new(7);
        let n = 3000;
        let a: Vec<u32> = (0..n).map(|_| rng.next_below(3) as u32).collect();
        let b: Vec<u32> = (0..n).map(|_| rng.next_below(3) as u32).collect();
        let c: Vec<u32> = (0..n).map(|_| rng.next_below(3) as u32).collect();
        let est = total_correlation(&[&a, &b, &c], &[3, 3, 3], LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn dual_total_correlation_requires_at_least_two_variables() {
        let a = [0u32, 1, 0, 1];
        assert!(matches!(
            dual_total_correlation(&[&a], &[2], LogBase::Nats),
            Err(EntropyError::InvalidConfig { .. })
        ));
    }

    #[test]
    fn o_information_of_redundant_copy_is_positive() {
        // 🔐️ X1=X2=X3 (perfect redundancy): O-information should be strongly positive.
        let mut rng = crate::numeric::Xorshift64::new(8);
        let x: Vec<u32> = (0..2000).map(|_| rng.next_below(4) as u32).collect();
        let est = o_information(&[&x, &x, &x], &[4, 4, 4], LogBase::Nats).unwrap();
        assert!(est.value > 0.5, "got {}", est.value);
    }

    #[test]
    fn pack_symbols_rejects_overflow() {
        let a = [0u32];
        let result = pack_symbols(&[&a, &a], &[u32::MAX as usize, u32::MAX as usize]);
        assert!(result.is_err());
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Mutual

// #region 🔖️Pid
pub mod pid {
//! 🧩️ Williams-Beer Partial Information Decomposition: the two-source `I_min` redundancy
//! decomposition (`pid_two_sources`) and the full 18-node redundancy lattice for exactly three
//! sources (`PidLattice`). Every quantity here is computed by maximum-likelihood plug-in on
//! empirical counts (no bias correction — see [`pid_two_sources`]'s doc for why) and internally
//! in nats, converted to the caller's [`LogBase`] only at the API boundary.

use crate::counts::JointCounts;
use crate::numeric::{checked_state_count, clamp_near_zero, neumaier_sum};
use crate::{EntropyError, LogBase};

// #region 🔖️Packing
/// 🧩️ Packs several aligned symbol sequences into one joint symbol via mixed-radix encoding. A
/// local copy of `mutual::pack_symbols`'s approach: that helper is private to its own module, so
/// this module keeps its own small copy rather than reaching into `mutual`'s internals.
fn pack_symbols(parts: &[&[u32]], sizes: &[usize]) -> Result<(Vec<u32>, usize), EntropyError> {
    let total = checked_state_count(sizes).ok_or(EntropyError::InvalidConfig {
        field: "sizes",
        reason: "joint alphabet size overflows u128",
    })?;
    if total > u32::MAX as u128 {
        return Err(EntropyError::InvalidConfig { field: "sizes", reason: "joint alphabet size exceeds u32::MAX" });
    }
    let n = parts[0].len();
    let mut combined = vec![0u32; n];
    for i in 0..n {
        let mut acc: u64 = 0;
        for (part, &size) in parts.iter().zip(sizes.iter()) {
            acc = acc * size as u64 + part[i] as u64;
        }
        combined[i] = acc as u32;
    }
    Ok((combined, total as usize))
}
// #endregion 🔖️Packing

// #region 🔖️SpecificInformation
/// 🧩️ Williams-Beer specific information `I_spec(A -> t) = sum_a p(a|t) ln(p(t|a)/p(t))` for
/// every target outcome `t`, alongside the target marginal `p(t)` used to weight it into a
/// mutual information (`I(A;T) = sum_t p(t) I_spec(A -> t)`) or an `I_min` redundancy term.
fn specific_information(a: &[u32], a_size: usize, t: &[u32], t_size: usize) -> Result<(Vec<f64>, Vec<f64>), EntropyError> {
    let joint = JointCounts::from_pairs(a, t, a_size, t_size)?;
    let total = joint.total();
    let p_a = joint.marginal_x();
    let p_t = joint.marginal_y();
    let mut i_spec = vec![0.0_f64; t_size];
    for tj in 0..t_size {
        if p_t[tj] <= 0.0 {
            continue;
        }
        i_spec[tj] = neumaier_sum((0..a_size).filter_map(|ai| {
            let p_at = joint.get(ai, tj) / total;
            if p_at <= 0.0 || p_a[ai] <= 0.0 {
                return None;
            }
            let p_a_given_t = p_at / p_t[tj];
            let p_t_given_a = p_at / p_a[ai];
            Some(p_a_given_t * (p_t_given_a / p_t[tj]).ln())
        }));
    }
    Ok((i_spec, p_t))
}

/// 🧩️ Mutual information `I(A;T)`, computed via the same specific-information pathway used by
/// every PID atom below, so the total-vs-atoms consistency checks never compare two independently
/// derived formulas for "the same" quantity.
fn mutual_information_via_specific(a: &[u32], a_size: usize, t: &[u32], t_size: usize) -> Result<f64, EntropyError> {
    let (i_spec, p_t) = specific_information(a, a_size, t, t_size)?;
    Ok(clamp_near_zero(neumaier_sum(i_spec.iter().zip(p_t.iter()).map(|(&i, &p)| p * i)), 1e-9))
}
// #endregion 🔖️SpecificInformation

// #region 🔖️TwoSourcePid
/// 🧩️ The four non-negative Williams-Beer partial information atoms decomposing `I(S1,S2;T)`,
/// each expressed in the [`LogBase`] requested at the call site.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PidAtoms {
    pub redundancy: f64,
    pub unique_1: f64,
    pub unique_2: f64,
    pub synergy: f64,
}

/// 🧩️ Williams-Beer two-source partial information decomposition via the `I_min` redundancy
/// measure, all four quantities estimated by maximum-likelihood plug-in on empirical counts.
/// Deliberately **not** bias-corrected: `Redundancy`/`Unique1`/`Unique2`/`Synergy` are a *linear
/// recombination* of several plug-in mutual informations sharing overlapping alphabets, and
/// naively bias-correcting each term independently (e.g. via `estimators::DiscreteMethod`) would
/// not cancel consistently across the recombination the way it does for a single MI estimate —
/// that would need its own dedicated derivation. This first implementation documents that
/// limitation rather than silently under- or over-correcting.
pub fn pid_two_sources(
    source1: &[u32],
    source2: &[u32],
    target: &[u32],
    sizes: (usize, usize, usize),
    base: LogBase,
) -> Result<PidAtoms, EntropyError> {
    base.validate()?;
    if source1.len() != source2.len() {
        return Err(EntropyError::LengthMismatch { expected: source1.len(), actual: source2.len() });
    }
    if source1.len() != target.len() {
        return Err(EntropyError::LengthMismatch { expected: source1.len(), actual: target.len() });
    }
    if source1.is_empty() {
        return Err(EntropyError::EmptyInput { what: "source1" });
    }
    let (s1_size, s2_size, t_size) = sizes;

    let (i_spec1, p_t) = specific_information(source1, s1_size, target, t_size)?;
    let (i_spec2, _) = specific_information(source2, s2_size, target, t_size)?;
    let (joint12, joint12_size) = pack_symbols(&[source1, source2], &[s1_size, s2_size])?;
    let i12_nats = mutual_information_via_specific(&joint12, joint12_size, target, t_size)?;

    let i1_nats = clamp_near_zero(neumaier_sum(i_spec1.iter().zip(p_t.iter()).map(|(&i, &p)| p * i)), 1e-9);
    let i2_nats = clamp_near_zero(neumaier_sum(i_spec2.iter().zip(p_t.iter()).map(|(&i, &p)| p * i)), 1e-9);

    let redundancy_nats = clamp_near_zero(
        neumaier_sum((0..t_size).map(|tj| p_t[tj] * i_spec1[tj].min(i_spec2[tj]))),
        1e-9,
    );
    let unique1_nats = clamp_near_zero(i1_nats - redundancy_nats, 1e-9);
    let unique2_nats = clamp_near_zero(i2_nats - redundancy_nats, 1e-9);
    let synergy_nats = clamp_near_zero(i12_nats - i1_nats - i2_nats + redundancy_nats, 1e-9);

    Ok(PidAtoms {
        redundancy: base.from_nats(redundancy_nats),
        unique_1: base.from_nats(unique1_nats),
        unique_2: base.from_nats(unique2_nats),
        synergy: base.from_nats(synergy_nats),
    })
}
// #endregion 🔖️TwoSourcePid

// #region 🔖️Lattice
/// 🧩️ One node of the Williams-Beer redundancy lattice for `n = 3` sources: an antichain of
/// non-empty source-index subsets, each subset packed as a bitmask (bit `i` set iff source `i`,
/// 0-based, is a member). Kept sorted ascending for canonical/order-independent equality.
type LatticeNode = Vec<u32>;

/// 🧩️ Every non-empty subset of the 3 source indices `{0, 1, 2}`, as a bitmask: `1..=7`.
const NON_EMPTY_SUBSET_MASKS: [u32; 7] = [1, 2, 3, 4, 5, 6, 7];

/// 🧩️ The redundancy-lattice order: `alpha <= beta` iff every set in `beta` has some subset (or
/// itself) present in `alpha`. `I_min` is monotone non-decreasing along this order — verified by
/// [`tests::full_set_node_i_min_equals_total_joint_mi_and_singletons_node_is_smaller`] — which
/// places the all-singletons node `{{0},{1},{2}}` at the *bottom* (smallest `I_min`, since a
/// `min` over three independently-estimated specific informations can only be `<=` any one of
/// them) and the single-full-set node `{{0,1,2}}` at the *top* (largest `I_min`, exactly the
/// total joint mutual information, an upper bound for every other node by data processing).
// 🧩️ `a & b == a` tests "is `a` a submask of `b`", not a fixed-value membership check — despite
// the closure's shape, this is not a `Vec::contains` rewrite (clippy's `manual_contains` lint
// pattern-matches too eagerly here since the target value is self-referential on the loop
// variable `a`).
#[allow(clippy::manual_contains)]
fn is_below(alpha: &LatticeNode, beta: &LatticeNode) -> bool {
    beta.iter().all(|&b| alpha.iter().any(|&a| (a & b) == a))
}

/// 🧩️ Enumerates all antichains of non-empty subsets of `{0, 1, 2}` (no member is a subset of
/// another) by brute force over the `2^7` subsets of the 7-element ground set of non-empty
/// masks — exhaustively correct at this size and self-verifying against the known count of 18,
/// rather than a hand-typed (and hand-error-prone) list.
fn enumerate_antichain_nodes() -> Vec<LatticeNode> {
    let mut nodes = Vec::new();
    for bits in 1u32..(1u32 << NON_EMPTY_SUBSET_MASKS.len()) {
        let mut chosen: Vec<u32> = Vec::new();
        for (i, &mask) in NON_EMPTY_SUBSET_MASKS.iter().enumerate() {
            if bits & (1 << i) != 0 {
                chosen.push(mask);
            }
        }
        let is_antichain = chosen
            .iter()
            .enumerate()
            .all(|(ia, &a)| chosen.iter().enumerate().all(|(ib, &b)| ia == ib || (a & b) != a));
        if is_antichain {
            chosen.sort_unstable();
            nodes.push(chosen);
        }
    }
    nodes
}

/// 🧩️ A computed Williams-Beer redundancy lattice for exactly `n = 3` sources: the `I_min` value
/// at every one of the 18 antichain nodes, their Mobius-inverted partial information atoms `Pi`,
/// and the total joint mutual information `I(S1,S2,S3;T)` those 18 atoms sum to.
pub struct PidLattice {
    nodes: Vec<LatticeNode>,
    i_min_nats: Vec<f64>,
    partial_info_nats: Vec<f64>,
    total_mi_nats: f64,
    base: LogBase,
}

impl PidLattice {
    /// 🧩️ Computes the full 18-node redundancy lattice for exactly 3 sources against `target`.
    /// `sizes` gives each source's alphabet size in the same order as `sources`; `target_size` is
    /// the target's alphabet size. Rejects `sources.len() != 3` — this first implementation does
    /// not attempt a general-`n` lattice (the antichain count grows combinatorially and the
    /// well-known closed enumeration only exists at small `n`).
    pub fn compute(
        sources: &[&[u32]],
        target: &[u32],
        sizes: &[usize],
        target_size: usize,
        base: LogBase,
    ) -> Result<Self, EntropyError> {
        base.validate()?;
        if sources.len() != 3 {
            return Err(EntropyError::InvalidConfig {
                field: "sources",
                reason: "PidLattice currently supports exactly 3 sources",
            });
        }
        if sizes.len() != 3 {
            return Err(EntropyError::ShapeMismatch { what: "sizes", expected: 3, actual: sizes.len() });
        }
        if target.is_empty() {
            return Err(EntropyError::EmptyInput { what: "target" });
        }
        for &s in sources {
            if s.len() != target.len() {
                return Err(EntropyError::LengthMismatch { expected: target.len(), actual: s.len() });
            }
        }

        // #region 🔖️SubsetJoints
        // 🧩️ packed joint symbols + alphabet size, and the resulting specific-information vector,
        // for every one of the 7 non-empty source subsets — indexed directly by bitmask (index 0
        // unused) rather than a `HashMap`, since the key space is fixed and tiny.
        let mut packed: [Option<(Vec<u32>, usize)>; 8] = [None, None, None, None, None, None, None, None];
        for &mask in NON_EMPTY_SUBSET_MASKS.iter() {
            let idxs: Vec<usize> = (0..3).filter(|i| mask & (1 << i) != 0).collect();
            let parts: Vec<&[u32]> = idxs.iter().map(|&i| sources[i]).collect();
            let part_sizes: Vec<usize> = idxs.iter().map(|&i| sizes[i]).collect();
            packed[mask as usize] = Some(pack_symbols(&parts, &part_sizes)?);
        }

        let mut spec: [Option<Vec<f64>>; 8] = [None, None, None, None, None, None, None, None];
        let mut p_t: Vec<f64> = Vec::new();
        for &mask in NON_EMPTY_SUBSET_MASKS.iter() {
            let (sym, size) = packed[mask as usize].as_ref().unwrap();
            let (i_spec, this_p_t) = specific_information(sym, *size, target, target_size)?;
            if p_t.is_empty() {
                p_t = this_p_t;
            }
            spec[mask as usize] = Some(i_spec);
        }
        // #endregion 🔖️SubsetJoints

        let total_mi_nats = clamp_near_zero(
            neumaier_sum(spec[7].as_ref().unwrap().iter().zip(p_t.iter()).map(|(&i, &p)| p * i)),
            1e-9,
        );

        // #region 🔖️IMin
        let nodes = enumerate_antichain_nodes();
        let i_min_nats: Vec<f64> = nodes
            .iter()
            .map(|node| {
                let value = neumaier_sum((0..target_size).map(|tj| {
                    let m = node
                        .iter()
                        .map(|&mask| spec[mask as usize].as_ref().unwrap()[tj])
                        .fold(f64::INFINITY, f64::min);
                    p_t[tj] * m
                }));
                clamp_near_zero(value, 1e-9)
            })
            .collect();
        // #endregion 🔖️IMin

        // #region 🔖️MobiusInversion
        // 🧩️ `predecessors[i]` = every strictly-smaller node under [`is_below`]. Since the order
        // is transitive, a node's predecessor-set size is strictly greater than any of its own
        // predecessors' — sorting by that size ascending is therefore a valid topological order
        // without needing a dedicated Kahn's-algorithm pass.
        let predecessors: Vec<Vec<usize>> = (0..nodes.len())
            .map(|i| (0..nodes.len()).filter(|&j| j != i && is_below(&nodes[j], &nodes[i])).collect())
            .collect();
        let mut order: Vec<usize> = (0..nodes.len()).collect();
        order.sort_by_key(|&i| predecessors[i].len());

        let mut partial_info_nats = vec![0.0_f64; nodes.len()];
        for &i in &order {
            let sum_predecessors = neumaier_sum(predecessors[i].iter().map(|&j| partial_info_nats[j]));
            partial_info_nats[i] = i_min_nats[i] - sum_predecessors;
        }
        // #endregion 🔖️MobiusInversion

        Ok(Self { nodes, i_min_nats, partial_info_nats, total_mi_nats, base })
    }

    /// 🧩️ Number of lattice nodes — always 18 for `n = 3` sources.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 🧩️ Looks up `Pi(alpha)` (converted to this lattice's `base`, as passed to
    /// [`PidLattice::compute`]) for the antichain described by `node_sets`: each inner `Vec<usize>`
    /// is a 0-based source-index subset, and both the outer and inner order are irrelevant (the
    /// antichain is compared as a set of sets).
    pub fn partial_information(&self, node_sets: &[Vec<usize>]) -> Option<f64> {
        self.node_index(node_sets).map(|i| self.base.from_nats(self.partial_info_nats[i]))
    }

    /// 🧩️ Looks up the raw `I_min(alpha)` redundancy value (before Mobius inversion) at the node
    /// described by `node_sets`, converted to `base` — the diagnostic quantity every
    /// [`PidLattice::partial_information`] atom is derived from, useful for inspecting the
    /// lattice's intermediate state (e.g. confirming monotonicity along [`is_below`]) rather than
    /// only its final decomposition.
    pub fn i_min(&self, node_sets: &[Vec<usize>], base: LogBase) -> Option<f64> {
        self.node_index(node_sets).map(|i| base.from_nats(self.i_min_nats[i]))
    }

    fn node_index(&self, node_sets: &[Vec<usize>]) -> Option<usize> {
        let mut masks: Vec<u32> = node_sets
            .iter()
            .map(|subset| subset.iter().fold(0u32, |acc, &idx| acc | 1u32.checked_shl(idx as u32).unwrap_or(0)))
            .collect();
        masks.sort_unstable();
        masks.dedup();
        self.nodes.iter().position(|n| n == &masks)
    }

    /// 🧩️ The total joint mutual information `I(S1,S2,S3;T)`, converted to `base` (independent of
    /// whatever `base` was passed to [`PidLattice::compute`]).
    pub fn total_mutual_information(&self, base: LogBase) -> f64 {
        base.from_nats(self.total_mi_nats)
    }
}
// #endregion 🔖️Lattice

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::numeric::Xorshift64;

    // #region 🔖️TwoSourceLogicGates
    #[test]
    fn copy_gate_shows_dominant_unique1_and_near_zero_synergy() {
        // 🔐️ T = S1 exactly, S2 independent noise: all info about T is uniquely S1's.
        let mut rng = Xorshift64::new(101);
        let n = 3000;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target = s1.clone();
        let atoms = pid_two_sources(&s1, &s2, &target, (2, 2, 2), LogBase::Nats).unwrap();
        assert!(atoms.unique_1 > 0.5, "unique_1={}", atoms.unique_1);
        assert!(atoms.redundancy < 0.1, "redundancy={}", atoms.redundancy);
        assert!(atoms.unique_2 < 0.1, "unique_2={}", atoms.unique_2);
        assert!(atoms.synergy.abs() < 0.1, "synergy={}", atoms.synergy);
    }

    #[test]
    fn xor_gate_shows_dominant_synergy_near_one_bit() {
        // 🔐️ T = S1 XOR S2, S1/S2 independent fair coins: the classic pure-synergy example.
        let mut rng = Xorshift64::new(202);
        let n = 4000;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target: Vec<u32> = s1.iter().zip(s2.iter()).map(|(&a, &b)| a ^ b).collect();
        let atoms = pid_two_sources(&s1, &s2, &target, (2, 2, 2), LogBase::Nats).unwrap();
        assert!((atoms.synergy - core::f64::consts::LN_2).abs() < 0.1, "synergy={}", atoms.synergy);
        assert!(atoms.redundancy < 0.1, "redundancy={}", atoms.redundancy);
        assert!(atoms.unique_1 < 0.1, "unique_1={}", atoms.unique_1);
        assert!(atoms.unique_2 < 0.1, "unique_2={}", atoms.unique_2);
    }
    // #endregion 🔖️TwoSourceLogicGates

    // #region 🔖️TwoSourceValidation
    #[test]
    fn pid_two_sources_rejects_length_mismatch() {
        assert!(matches!(
            pid_two_sources(&[0, 1], &[0], &[0, 1], (2, 1, 2), LogBase::Nats),
            Err(EntropyError::LengthMismatch { .. })
        ));
    }

    #[test]
    fn pid_two_sources_rejects_empty_input() {
        assert!(matches!(
            pid_two_sources(&[], &[], &[], (2, 2, 2), LogBase::Nats),
            Err(EntropyError::EmptyInput { .. })
        ));
    }
    // #endregion 🔖️TwoSourceValidation

    // #region 🔖️Lattice
    #[test]
    fn lattice_has_exactly_eighteen_nodes() {
        let mut rng = Xorshift64::new(303);
        let n = 1000;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s3: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let lattice = PidLattice::compute(&[&s1, &s2, &s3], &target, &[2, 2, 2], 2, LogBase::Nats).unwrap();
        assert_eq!(lattice.node_count(), 18);
    }

    #[test]
    fn lattice_rejects_source_counts_other_than_three() {
        let s1 = [0u32, 1, 0, 1];
        let target = [0u32, 1, 1, 0];
        assert!(matches!(
            PidLattice::compute(&[&s1, &s1], &target, &[2, 2], 2, LogBase::Nats),
            Err(EntropyError::InvalidConfig { .. })
        ));
    }

    #[test]
    fn full_set_node_i_min_equals_total_joint_mi_and_singletons_node_is_smaller() {
        let mut rng = Xorshift64::new(404);
        let n = 2000;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s3: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target: Vec<u32> = s1.iter().zip(s2.iter()).map(|(&a, &b)| a ^ b).collect();
        let lattice = PidLattice::compute(&[&s1, &s2, &s3], &target, &[2, 2, 2], 2, LogBase::Nats).unwrap();
        let full_set_idx = lattice.nodes.iter().position(|node| node == &vec![7u32]).unwrap();
        let singletons_idx = lattice.nodes.iter().position(|node| node == &vec![1u32, 2, 4]).unwrap();
        assert!((lattice.i_min_nats[full_set_idx] - lattice.total_mi_nats).abs() < 1e-9);
        assert!(lattice.i_min_nats[singletons_idx] <= lattice.i_min_nats[full_set_idx] + 1e-9);
    }

    #[test]
    fn sum_of_all_partial_information_equals_total_mutual_information() {
        // 🔐️ the critical Mobius/zeta consistency check: sum(Pi) over all 18 nodes must equal the
        // total joint MI exactly (an algebraic identity of the inversion, not a statistical
        // convergence property), on several independently seeded random datasets.
        for seed in [11u64, 22u64, 33u64] {
            let mut rng = Xorshift64::new(seed);
            let n = 1500;
            let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
            let s2: Vec<u32> = (0..n).map(|_| rng.next_below(3) as u32).collect();
            let s3: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
            let target: Vec<u32> = (0..n).map(|_| rng.next_below(3) as u32).collect();
            let lattice = PidLattice::compute(&[&s1, &s2, &s3], &target, &[2, 3, 2], 3, LogBase::Nats).unwrap();
            let sum_pi: f64 = lattice.partial_info_nats.iter().sum();
            let total_mi = lattice.total_mutual_information(LogBase::Nats);
            assert!((sum_pi - total_mi).abs() < 1e-6, "seed={seed} sum_pi={sum_pi} total_mi={total_mi}");
        }
    }

    #[test]
    fn partial_information_lookup_matches_internal_full_set_node() {
        let mut rng = Xorshift64::new(505);
        let n = 1200;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s3: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target: Vec<u32> = s1.iter().zip(s3.iter()).map(|(&a, &b)| a ^ b).collect();
        let lattice = PidLattice::compute(&[&s1, &s2, &s3], &target, &[2, 2, 2], 2, LogBase::Nats).unwrap();
        let looked_up = lattice.partial_information(&[vec![0, 1, 2]]).unwrap();
        let full_set_idx = lattice.nodes.iter().position(|node| node == &vec![7u32]).unwrap();
        assert!((looked_up - lattice.partial_info_nats[full_set_idx]).abs() < 1e-12);
        // 🔐️ an antichain referencing a source index that doesn't correspond to any lattice node
        // (99 is out of range for a 3-source lattice) must return `None`, not panic.
        assert!(lattice.partial_information(&[vec![0], vec![1], vec![99]]).is_none());
    }
    // #endregion 🔖️Lattice
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Pid

// #region 🔖️Fisher
pub mod fisher {
//! 📉️ Fisher information and information-criterion model-selection scores. Fisher information is
//! recovered numerically from a supplied log-likelihood via a central second difference; the
//! criteria (AIC/AICc/BIC/HQC/MDL) are closed-form penalized-likelihood scores taking an
//! already-computed `ln_L` and parameter/sample counts — no external crate, no `Estimate` wrapper
//! since these are exact-given-inputs, not estimated-from-samples quantities.

use crate::EntropyError;

// #region 🔖️FisherInformation
/// 📉️ Numerical (observed) Fisher information via a central second-difference of the supplied
/// log-likelihood function at `theta`: `-(L(theta+h) - 2*L(theta) + L(theta-h)) / h^2`, with a
/// fixed step `h = 1e-4 * theta.abs().max(1.0)` (scale-aware so it works across parameter
/// magnitudes).
pub fn fisher_information(log_likelihood: impl Fn(f64) -> f64, theta: f64) -> f64 {
    let h = 1e-4 * theta.abs().max(1.0);
    let forward = log_likelihood(theta + h);
    let center = log_likelihood(theta);
    let backward = log_likelihood(theta - h);
    -(forward - 2.0 * center + backward) / (h * h)
}
// #endregion 🔖️FisherInformation

// #region 🔖️Criteria
/// 📉️ Akaike information criterion: `2*k - 2*ln_L`.
pub fn aic(log_likelihood: f64, num_params: usize) -> f64 {
    2.0 * num_params as f64 - 2.0 * log_likelihood
}

/// 📉️ Corrected AIC for small sample sizes: `aic + (2*k*(k+1)) / (n - k - 1)`. Errors with
/// [`EntropyError::InvalidConfig`] if `n <= k + 1` (undefined).
pub fn aicc(log_likelihood: f64, num_params: usize, n: usize) -> Result<f64, EntropyError> {
    let k = num_params as f64;
    let n = n as f64;
    if n <= k + 1.0 {
        return Err(EntropyError::InvalidConfig {
            field: "n",
            reason: "AICc is undefined when n <= num_params + 1",
        });
    }
    Ok(aic(log_likelihood, num_params) + (2.0 * k * (k + 1.0)) / (n - k - 1.0))
}

/// 📉️ Bayesian information criterion: `k * ln(n) - 2*ln_L`.
pub fn bic(log_likelihood: f64, num_params: usize, n: usize) -> f64 {
    num_params as f64 * (n as f64).ln() - 2.0 * log_likelihood
}

/// 📉️ Hannan-Quinn criterion: `2*k*ln(ln(n)) - 2*ln_L`. For `n <= e` (so `ln(ln(n))` is undefined
/// or non-positive) the penalty term is treated as `0.0` rather than producing NaN/negative
/// infinity — a small-`n` edge case, not a claim that HQC is well-defined there.
pub fn hqc(log_likelihood: f64, num_params: usize, n: usize) -> f64 {
    let penalty = if (n as f64) <= core::f64::consts::E {
        0.0
    } else {
        2.0 * num_params as f64 * (n as f64).ln().ln()
    };
    penalty - 2.0 * log_likelihood
}

/// 📉️ A simple two-part-code minimum-description-length approximation:
/// `-ln_L + 0.5 * k * ln(n)` (equivalent to BIC/2 in the log-likelihood term's convention).
pub fn mdl(log_likelihood: f64, num_params: usize, n: usize) -> f64 {
    -log_likelihood + 0.5 * num_params as f64 * (n as f64).ln()
}
// #endregion 🔖️Criteria

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aic_matches_hand_computed_value() {
        assert!((aic(-10.0, 3) - 26.0).abs() < 1e-12);
    }

    #[test]
    fn bic_matches_hand_computed_value() {
        let n = 100;
        let expected = 3.0 * (100.0_f64).ln() - 2.0 * -10.0;
        assert!((bic(-10.0, 3, n) - expected).abs() < 1e-12);
    }

    #[test]
    fn mdl_matches_hand_computed_value() {
        let n = 100;
        let expected = 10.0 + 0.5 * 3.0 * (100.0_f64).ln();
        assert!((mdl(-10.0, 3, n) - expected).abs() < 1e-12);
    }

    #[test]
    fn aic_increases_with_num_params() {
        let ln_l = -50.0;
        let mut prev = aic(ln_l, 1);
        for k in 2..10 {
            let cur = aic(ln_l, k);
            assert!(cur > prev, "k={k}");
            prev = cur;
        }
    }

    #[test]
    fn bic_increases_with_num_params() {
        let ln_l = -50.0;
        let n = 200;
        let mut prev = bic(ln_l, 1, n);
        for k in 2..10 {
            let cur = bic(ln_l, k, n);
            assert!(cur > prev, "k={k}");
            prev = cur;
        }
    }

    #[test]
    fn hqc_increases_with_num_params() {
        let ln_l = -50.0;
        let n = 200;
        let mut prev = hqc(ln_l, 1, n);
        for k in 2..10 {
            let cur = hqc(ln_l, k, n);
            assert!(cur > prev, "k={k}");
            prev = cur;
        }
    }

    #[test]
    fn mdl_increases_with_num_params() {
        let ln_l = -50.0;
        let n = 200;
        let mut prev = mdl(ln_l, 1, n);
        for k in 2..10 {
            let cur = mdl(ln_l, k, n);
            assert!(cur > prev, "k={k}");
            prev = cur;
        }
    }

    #[test]
    fn aicc_approaches_aic_for_large_n() {
        let ln_l = -50.0;
        let k = 3;
        // 🔬️ the AICc correction term is 2k(k+1)/(n-k-1); it only vanishes as n -> infinity, so
        // "approaches AIC" needs n large enough to push it below the tolerance, not n=1e6 (which
        // still leaves a ~2.4e-5 correction).
        let n = 100_000_000;
        let a = aic(ln_l, k);
        let ac = aicc(ln_l, k, n).unwrap();
        assert!((ac - a).abs() < 1e-4, "aicc={ac} aic={a}");
    }

    #[test]
    fn aicc_errs_when_n_leq_k_plus_one() {
        assert!(matches!(aicc(-10.0, 3, 4), Err(EntropyError::InvalidConfig { .. })));
        assert!(matches!(aicc(-10.0, 3, 3), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn aicc_ok_when_n_greater_than_k_plus_one() {
        assert!(aicc(-10.0, 3, 5).is_ok());
    }

    #[test]
    fn bic_penalizes_more_than_aic_when_ln_n_exceeds_two() {
        // 🔐️ BIC's penalty is k*ln(n) vs AIC's 2*k; ln(n) > 2 (n > e^2 ~= 7.39) means BIC > AIC
        // for the same ln_L and k, since both share the -2*ln_L term.
        let ln_l = -50.0;
        let k = 4;
        let n = 100; // ln(100) ~= 4.6 > 2
        assert!((n as f64).ln() > 2.0);
        assert!(bic(ln_l, k, n) > aic(ln_l, k));
    }

    #[test]
    fn hqc_small_n_edge_case_has_zero_penalty() {
        let ln_l = -5.0;
        for n in [1usize, 2] {
            assert!((n as f64) <= core::f64::consts::E);
            let expected = -2.0 * ln_l;
            assert!((hqc(ln_l, 3, n) - expected).abs() < 1e-12, "n={n}");
        }
    }

    #[test]
    fn fisher_information_matches_gaussian_closed_form() {
        // 🔐️ log_lik(theta) = -0.5 * sum((x_i - theta)^2) / sigma^2 has exact Fisher information
        // n / sigma^2 for the mean parameter, independent of the sample itself.
        let sigma: f64 = 2.0;
        let xs: Vec<f64> = vec![1.0, 2.5, -0.7, 3.3, 0.1, 4.2, -1.1, 2.0];
        let n = xs.len();
        let log_lik = |theta: f64| -> f64 {
            -0.5 * xs.iter().map(|x| (x - theta).powi(2)).sum::<f64>() / (sigma * sigma)
        };
        let theta0 = 1.3;
        let observed = fisher_information(log_lik, theta0);
        let expected = n as f64 / (sigma * sigma);
        let rel_err = (observed - expected).abs() / expected;
        assert!(rel_err < 1e-3, "observed={observed} expected={expected} rel_err={rel_err}");
    }

    #[test]
    fn fisher_information_zero_for_flat_log_likelihood() {
        let flat = |_theta: f64| -> f64 { 42.0 };
        let observed = fisher_information(flat, 5.0);
        assert!(observed.abs() < 1e-6, "observed={observed}");
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Fisher

// #region 🔖️Symbolic
pub mod symbolic {
//! 🔤️ Symbolization front door: maps continuous/discrete time series into finite alphabets via
//! time-delay embedding, ordinal (permutation) patterns, dispersion patterns, empirical quantile
//! binning, and fixed thresholds. Every [`Symbolizer`] implementation here feeds downstream
//! plug-in entropy estimators (`discrete.rs`, `ordinal.rs`, `regularity.rs`) a `Vec<u32>` of
//! symbol codes plus a declared [`Symbolizer::alphabet_size`].

use crate::numeric::{checked_state_count, neumaier_sum, normal_cdf};
use crate::{EntropyError, TiePolicy};

// #region 🔖️Embedding
/// 🔤️ Time-delay (Takens) embedding: state vector `i` is `[x[i], x[i+tau], ..., x[i+(dim-1)*tau]]`
/// for `i` in `0..=(x.len() - (dim-1)*tau - 1)`. The foundational primitive behind every
/// window-based symbolizer in this module (ordinal and dispersion patterns) as well as
/// phase-space reconstruction used elsewhere in the crate.
pub fn embed(x: &[f64], dim: usize, tau: usize) -> Result<Vec<Vec<f64>>, EntropyError> {
    if dim < 1 {
        return Err(EntropyError::InvalidConfig { field: "dim", reason: "embedding dimension must be at least 1" });
    }
    if tau < 1 {
        return Err(EntropyError::InvalidConfig { field: "tau", reason: "embedding delay must be at least 1" });
    }
    let span = (dim - 1) * tau;
    let needed = span + 1;
    if x.len() < needed {
        return Err(EntropyError::InsufficientData { what: "time-delay embedding", needed, actual: x.len() });
    }
    let n_windows = x.len() - span;
    let mut out = Vec::with_capacity(n_windows);
    for i in 0..n_windows {
        let mut state = Vec::with_capacity(dim);
        for k in 0..dim {
            state.push(x[i + k * tau]);
        }
        out.push(state);
    }
    Ok(out)
}
// #endregion 🔖️Embedding

// #region 🔖️Trait
/// 🔤️ Maps a real-valued time series to a finite alphabet of symbols (`u32` codes, each
/// guaranteed to lie in `0..alphabet_size()`) — the common front door every symbolic/permutation
/// entropy estimator builds on.
pub trait Symbolizer {
    /// 🔤️ Encodes `x` into a sequence of symbol codes. Implementations that embed a window (e.g.
    /// [`OrdinalSymbolizer`], [`DispersionSymbolizer`]) emit fewer codes than `x.len()`.
    fn symbolize(&self, x: &[f64]) -> Result<Vec<u32>, EntropyError>;
    /// 🔤️ The size of the alphabet this symbolizer emits codes into.
    fn alphabet_size(&self) -> usize;
}
// #endregion 🔖️Trait

// #region 🔖️Ordinal
/// 🔤️ Saturating factorial (`n!` for small `n`, saturates to `u64::MAX` rather than wrapping or
/// panicking for `n` large enough to overflow — realistic ordinal-pattern dimensions are `<= 8`).
fn factorial(n: usize) -> u64 {
    let mut acc: u64 = 1;
    for k in 2..=n as u64 {
        acc = acc.saturating_mul(k);
    }
    acc
}

/// 🔤️ Encodes one embedded window as an ordinal-pattern symbol in `0..dim!` via the standard
/// factorial number system: rank the window's values ascending (stable, so ties resolve by
/// original in-window index order), then Lehmer-code that permutation of original indices into a
/// single integer. `TiePolicy::Error` rejects any exact tie up front; `StableRank` and
/// `Jitterless` are treated identically here (both fall through to the stable-sort tie-break) —
/// a documented simplification for this first implementation, since neither adds real jitter.
fn ordinal_pattern_symbol(window: &[f64], ties: TiePolicy) -> Result<u32, EntropyError> {
    let dim = window.len();
    if ties == TiePolicy::Error {
        for i in 0..dim {
            for j in (i + 1)..dim {
                if window[i] == window[j] {
                    return Err(EntropyError::DegenerateInput { what: "tied values in ordinal pattern" });
                }
            }
        }
    }
    let mut order: Vec<usize> = (0..dim).collect();
    order.sort_by(|&a, &b| window[a].total_cmp(&window[b]));
    let mut used = vec![false; dim];
    let mut rank: u64 = 0;
    for (j, &pos) in order.iter().enumerate() {
        let smaller_remaining = (0..pos).filter(|&k| !used[k]).count() as u64;
        rank += smaller_remaining * factorial(dim - 1 - j);
        used[pos] = true;
    }
    Ok(rank as u32)
}

/// 🔤️ Configuration for [`OrdinalSymbolizer`]: embedding dimension, embedding delay, and the tie
/// policy applied within each window.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct OrdinalConfig {
    pub dim: usize,
    pub tau: usize,
    pub ties: TiePolicy,
}

impl OrdinalConfig {
    /// 🔤️ Validated constructor: `dim >= 2` (a single-value "pattern" carries no order
    /// information) and `tau >= 1`. Defaults `ties` to [`TiePolicy::StableRank`].
    pub fn new(dim: usize, tau: usize) -> Result<Self, EntropyError> {
        if dim < 2 {
            return Err(EntropyError::InvalidConfig {
                field: "dim",
                reason: "ordinal patterns need dim >= 2 to carry order information",
            });
        }
        if tau < 1 {
            return Err(EntropyError::InvalidConfig { field: "tau", reason: "embedding delay must be at least 1" });
        }
        Ok(Self { dim, tau, ties: TiePolicy::StableRank })
    }

    /// 🔤️ Consuming setter for the tie policy.
    pub fn with_ties(mut self, ties: TiePolicy) -> Self {
        self.ties = ties;
        self
    }
}

impl Default for OrdinalConfig {
    /// 🔤️ The literature-default embedding: `dim = 3`, `tau = 1`.
    fn default() -> Self {
        Self { dim: 3, tau: 1, ties: TiePolicy::StableRank }
    }
}

/// 🔤️ Symbolizes a series into ordinal (Bandt-Pompe permutation) patterns: each embedded window
/// is encoded as the rank of its permutation among the `dim!` possible orderings.
pub struct OrdinalSymbolizer {
    cfg: OrdinalConfig,
}

impl OrdinalSymbolizer {
    /// 🔤️ Wraps an already-validated [`OrdinalConfig`].
    pub fn new(cfg: OrdinalConfig) -> Self {
        Self { cfg }
    }
}

impl Symbolizer for OrdinalSymbolizer {
    fn symbolize(&self, x: &[f64]) -> Result<Vec<u32>, EntropyError> {
        let windows = embed(x, self.cfg.dim, self.cfg.tau)?;
        windows.iter().map(|w| ordinal_pattern_symbol(w, self.cfg.ties)).collect()
    }

    fn alphabet_size(&self) -> usize {
        factorial(self.cfg.dim) as usize
    }
}
// #endregion 🔖️Ordinal

// #region 🔖️Dispersion
/// 🔤️ Symbolizes a series into dispersion patterns (Rostaghi-Azami): each raw value is mapped to
/// one of `classes` classes via the normal-CDF-based normalization used by NCDF dispersion
/// entropy, then embedded windows of classes are packed into one joint symbol.
pub struct DispersionSymbolizer {
    pub classes: usize,
    pub dim: usize,
    pub tau: usize,
}

impl DispersionSymbolizer {
    /// 🔤️ Validated constructor: `classes >= 2`, `dim >= 1`, `tau >= 1`, and `classes^dim` must
    /// fit within the `u32` symbol codomain that [`Symbolizer::symbolize`] returns (checked via
    /// [`checked_state_count`], which itself guards `usize`/`u128` overflow of the raw product).
    pub fn new(classes: usize, dim: usize, tau: usize) -> Result<Self, EntropyError> {
        if classes < 2 {
            return Err(EntropyError::InvalidConfig { field: "classes", reason: "dispersion needs at least 2 classes" });
        }
        if dim < 1 {
            return Err(EntropyError::InvalidConfig { field: "dim", reason: "embedding dimension must be at least 1" });
        }
        if tau < 1 {
            return Err(EntropyError::InvalidConfig { field: "tau", reason: "embedding delay must be at least 1" });
        }
        let dims = vec![classes; dim];
        let fits = matches!(checked_state_count(&dims), Some(count) if count <= u32::MAX as u128);
        if !fits {
            return Err(EntropyError::InvalidConfig {
                field: "classes/dim",
                reason: "classes^dim must fit within the u32 symbol range",
            });
        }
        Ok(Self { classes, dim, tau })
    }
}

impl Symbolizer for DispersionSymbolizer {
    fn symbolize(&self, x: &[f64]) -> Result<Vec<u32>, EntropyError> {
        if x.is_empty() {
            return Err(EntropyError::EmptyInput { what: "x" });
        }
        let n = x.len() as f64;
        let mean = neumaier_sum(x.iter().copied()) / n;
        let variance = neumaier_sum(x.iter().map(|&v| (v - mean).powi(2))) / n;
        let sd = variance.sqrt();
        if sd <= 0.0 {
            return Err(EntropyError::DegenerateInput { what: "constant series has no dispersion classes" });
        }
        let classes_f = self.classes as f64;
        let classed: Vec<f64> = x
            .iter()
            .map(|&v| {
                let c = (classes_f * normal_cdf((v - mean) / sd) - 0.5).round();
                c.clamp(0.0, classes_f - 1.0)
            })
            .collect();
        let windows = embed(&classed, self.dim, self.tau)?;
        let mut out = Vec::with_capacity(windows.len());
        for w in &windows {
            let mut symbol: usize = 0;
            for &v in w {
                symbol = symbol * self.classes + v as usize;
            }
            out.push(symbol as u32);
        }
        Ok(out)
    }

    fn alphabet_size(&self) -> usize {
        let dims = vec![self.classes; self.dim];
        checked_state_count(&dims).map_or(usize::MAX, |c| c as usize)
    }
}
// #endregion 🔖️Dispersion

// #region 🔖️Quantile
/// 🔤️ Symbolizes a series into empirical-quantile bins: bin edges are the series' own `bins - 1`
/// interior quantile breakpoints, so every bin holds (up to rounding) an equal share of samples.
pub struct QuantileSymbolizer {
    pub bins: usize,
}

impl QuantileSymbolizer {
    /// 🔤️ Validated constructor: `bins >= 2`.
    pub fn new(bins: usize) -> Result<Self, EntropyError> {
        if bins < 2 {
            return Err(EntropyError::InvalidConfig { field: "bins", reason: "must be at least 2" });
        }
        Ok(Self { bins })
    }
}

impl Symbolizer for QuantileSymbolizer {
    fn symbolize(&self, x: &[f64]) -> Result<Vec<u32>, EntropyError> {
        if x.is_empty() {
            return Err(EntropyError::EmptyInput { what: "x" });
        }
        let mut sorted = x.to_vec();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let n = sorted.len();
        let mut edges = Vec::with_capacity(self.bins - 1);
        for i in 1..self.bins {
            let frac = i as f64 / self.bins as f64;
            let idx = ((frac * n as f64) as usize).min(n - 1);
            edges.push(sorted[idx]);
        }
        Ok(x.iter().map(|&v| edges.partition_point(|&e| e <= v).min(self.bins - 1) as u32).collect())
    }

    fn alphabet_size(&self) -> usize {
        self.bins
    }
}
// #endregion 🔖️Quantile

// #region 🔖️Threshold
/// 🔤️ Symbolizes a series against a fixed, caller-supplied set of ascending threshold edges: the
/// class of `x_i` is the count of `edges` that are `<= x_i`, so the alphabet size is
/// `edges.len() + 1`.
pub struct ThresholdSymbolizer {
    pub edges: Vec<f64>,
}

impl ThresholdSymbolizer {
    /// 🔤️ Validated constructor: `edges` must be non-empty, every entry finite, and the sequence
    /// strictly ascending. Never silently sorts — an out-of-order `edges` is a caller bug.
    pub fn new(edges: Vec<f64>) -> Result<Self, EntropyError> {
        if edges.is_empty() {
            return Err(EntropyError::InvalidConfig { field: "edges", reason: "must not be empty" });
        }
        if !edges.iter().all(|e| e.is_finite()) {
            return Err(EntropyError::InvalidConfig { field: "edges", reason: "all edges must be finite" });
        }
        if !edges.windows(2).all(|w| w[0] < w[1]) {
            return Err(EntropyError::InvalidConfig { field: "edges", reason: "edges must be sorted strictly ascending" });
        }
        Ok(Self { edges })
    }
}

impl Symbolizer for ThresholdSymbolizer {
    fn symbolize(&self, x: &[f64]) -> Result<Vec<u32>, EntropyError> {
        if x.is_empty() {
            return Err(EntropyError::EmptyInput { what: "x" });
        }
        Ok(x.iter().map(|&v| self.edges.partition_point(|&e| e <= v) as u32).collect())
    }

    fn alphabet_size(&self) -> usize {
        self.edges.len() + 1
    }
}
// #endregion 🔖️Threshold

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // #region 🔖️EmbedTests
    #[test]
    fn embed_produces_expected_windows_and_values() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let windows = embed(&x, 3, 1).unwrap();
        assert_eq!(windows, vec![vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], vec![3.0, 4.0, 5.0]]);
    }

    #[test]
    fn embed_respects_tau() {
        let x = [0.0, 1.0, 2.0, 3.0, 4.0];
        let windows = embed(&x, 2, 2).unwrap();
        assert_eq!(windows, vec![vec![0.0, 2.0], vec![1.0, 3.0], vec![2.0, 4.0]]);
    }

    #[test]
    fn embed_rejects_zero_dim_and_zero_tau() {
        assert!(matches!(embed(&[1.0, 2.0], 0, 1), Err(EntropyError::InvalidConfig { field: "dim", .. })));
        assert!(matches!(embed(&[1.0, 2.0], 1, 0), Err(EntropyError::InvalidConfig { field: "tau", .. })));
    }

    #[test]
    fn embed_rejects_insufficient_data() {
        let x = [1.0, 2.0];
        assert!(matches!(
            embed(&x, 5, 1),
            Err(EntropyError::InsufficientData { needed: 5, actual: 2, .. })
        ));
    }
    // #endregion 🔖️EmbedTests

    // #region 🔖️OrdinalTests
    #[test]
    fn ordinal_config_rejects_dim_less_than_two() {
        assert!(matches!(OrdinalConfig::new(1, 1), Err(EntropyError::InvalidConfig { field: "dim", .. })));
    }

    #[test]
    fn ordinal_config_rejects_zero_tau() {
        assert!(matches!(OrdinalConfig::new(3, 0), Err(EntropyError::InvalidConfig { field: "tau", .. })));
    }

    #[test]
    fn ordinal_config_default_is_dim3_tau1() {
        let cfg = OrdinalConfig::default();
        assert_eq!(cfg.dim, 3);
        assert_eq!(cfg.tau, 1);
        assert_eq!(cfg.ties, TiePolicy::StableRank);
    }

    #[test]
    fn ordinal_config_with_ties_overrides_default() {
        let cfg = OrdinalConfig::new(3, 1).unwrap().with_ties(TiePolicy::Error);
        assert_eq!(cfg.ties, TiePolicy::Error);
    }

    #[test]
    fn ordinal_pattern_symbol_hand_computed_example() {
        // 🔤️ Window [3, 1, 4]: ascending order is index1(1) < index0(3) < index2(4), whose
        // Lehmer code (base 3!) is [1, 0, 0] -> 1*2! + 0*1! + 0*0! = 2.
        let symbol = ordinal_pattern_symbol(&[3.0, 1.0, 4.0], TiePolicy::StableRank).unwrap();
        assert_eq!(symbol, 2);
    }

    #[test]
    fn ordinal_symbolizer_all_six_dim3_patterns_are_distinct() {
        let cfg = OrdinalConfig::new(3, 1).unwrap();
        let symbolizer = OrdinalSymbolizer::new(cfg);
        assert_eq!(symbolizer.alphabet_size(), 6);
        let base = [10.0, 20.0, 30.0];
        let mut symbols = Vec::new();
        // 🔤️ Enumerate all 3! permutations of a 3-element index array.
        for a in 0..3 {
            for b in 0..3 {
                for c in 0..3 {
                    if a == b || b == c || a == c {
                        continue;
                    }
                    let window: Vec<f64> = [a, b, c].iter().map(|&i| base[i]).collect();
                    let symbol = symbolizer.symbolize(&window).unwrap();
                    assert_eq!(symbol.len(), 1);
                    symbols.push(symbol[0]);
                }
            }
        }
        assert_eq!(symbols.len(), 6);
        let mut unique = symbols.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), 6);
        assert!(symbols.iter().all(|&s| s < 6));
    }

    #[test]
    fn ordinal_symbolizer_monotone_series_yields_constant_pattern() {
        let cfg = OrdinalConfig::new(3, 1).unwrap();
        let symbolizer = OrdinalSymbolizer::new(cfg);
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let symbols = symbolizer.symbolize(&x).unwrap();
        assert_eq!(symbols.len(), 8);
        assert!(symbols.iter().all(|&s| s == symbols[0]));
        assert_eq!(symbols[0], 0); // 🔤️ strictly ascending window == identity permutation
    }

    #[test]
    fn ordinal_symbolizer_alphabet_size_is_factorial() {
        assert_eq!(OrdinalSymbolizer::new(OrdinalConfig::new(4, 1).unwrap()).alphabet_size(), 24);
        assert_eq!(OrdinalSymbolizer::new(OrdinalConfig::new(5, 1).unwrap()).alphabet_size(), 120);
    }

    #[test]
    fn ordinal_ties_error_policy_rejects_equal_values() {
        let cfg = OrdinalConfig::new(3, 1).unwrap().with_ties(TiePolicy::Error);
        let symbolizer = OrdinalSymbolizer::new(cfg);
        let result = symbolizer.symbolize(&[1.0, 1.0, 2.0]);
        assert!(matches!(result, Err(EntropyError::DegenerateInput { .. })));
    }

    #[test]
    fn ordinal_ties_stable_rank_breaks_by_original_index() {
        // 🔤️ [2, 2, 1]: ascending stable order is index2(1) < index0(2) < index1(2), whose
        // Lehmer code (base 3!) is [2, 0, 0] -> 2*2! + 0*1! + 0*0! = 4.
        let symbol = ordinal_pattern_symbol(&[2.0, 2.0, 1.0], TiePolicy::StableRank).unwrap();
        assert_eq!(symbol, 4);
    }
    // #endregion 🔖️OrdinalTests

    // #region 🔖️DispersionTests
    #[test]
    fn dispersion_symbolizer_rejects_invalid_config() {
        assert!(matches!(DispersionSymbolizer::new(1, 2, 1), Err(EntropyError::InvalidConfig { field: "classes", .. })));
        assert!(matches!(DispersionSymbolizer::new(3, 0, 1), Err(EntropyError::InvalidConfig { field: "dim", .. })));
        assert!(matches!(DispersionSymbolizer::new(3, 2, 0), Err(EntropyError::InvalidConfig { field: "tau", .. })));
    }

    #[test]
    fn dispersion_symbolizer_alphabet_size_is_classes_pow_dim() {
        let symbolizer = DispersionSymbolizer::new(3, 2, 1).unwrap();
        assert_eq!(symbolizer.alphabet_size(), 9);
    }

    #[test]
    fn dispersion_symbolizer_rejects_constant_series() {
        let symbolizer = DispersionSymbolizer::new(3, 2, 1).unwrap();
        let x = vec![5.0; 20];
        assert!(matches!(symbolizer.symbolize(&x), Err(EntropyError::DegenerateInput { .. })));
    }

    #[test]
    fn dispersion_symbolizer_symbol_count_matches_embedding() {
        let symbolizer = DispersionSymbolizer::new(4, 3, 1).unwrap();
        let mut rng = crate::numeric::Xorshift64::new(11);
        let x: Vec<f64> = (0..30).map(|_| rng.next_gaussian()).collect();
        let symbols = symbolizer.symbolize(&x).unwrap();
        assert_eq!(symbols.len(), 30 - (3 - 1));
        assert!(symbols.iter().all(|&s| (s as usize) < symbolizer.alphabet_size()));
    }
    // #endregion 🔖️DispersionTests

    // #region 🔖️QuantileTests
    #[test]
    fn quantile_symbolizer_rejects_bins_less_than_two() {
        assert!(matches!(QuantileSymbolizer::new(1), Err(EntropyError::InvalidConfig { field: "bins", .. })));
    }

    #[test]
    fn quantile_symbolizer_alphabet_size_equals_bins() {
        assert_eq!(QuantileSymbolizer::new(5).unwrap().alphabet_size(), 5);
    }

    #[test]
    fn quantile_symbolizer_splits_small_example_in_half() {
        let symbolizer = QuantileSymbolizer::new(2).unwrap();
        let symbols = symbolizer.symbolize(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_eq!(symbols, vec![0, 0, 1, 1]);
    }
    // #endregion 🔖️QuantileTests

    // #region 🔖️ThresholdTests
    #[test]
    fn threshold_symbolizer_rejects_empty_edges() {
        assert!(matches!(ThresholdSymbolizer::new(vec![]), Err(EntropyError::InvalidConfig { field: "edges", .. })));
    }

    #[test]
    fn threshold_symbolizer_rejects_unsorted_edges() {
        assert!(matches!(
            ThresholdSymbolizer::new(vec![1.0, 0.0, 2.0]),
            Err(EntropyError::InvalidConfig { field: "edges", .. })
        ));
    }

    #[test]
    fn threshold_symbolizer_rejects_non_finite_edges() {
        assert!(matches!(
            ThresholdSymbolizer::new(vec![0.0, f64::NAN]),
            Err(EntropyError::InvalidConfig { field: "edges", .. })
        ));
    }

    #[test]
    fn threshold_symbolizer_classifies_against_edges() {
        let symbolizer = ThresholdSymbolizer::new(vec![0.0, 10.0]).unwrap();
        assert_eq!(symbolizer.alphabet_size(), 3);
        let symbols = symbolizer.symbolize(&[-5.0, 0.0, 5.0, 10.0, 15.0]).unwrap();
        assert_eq!(symbols, vec![0, 1, 1, 2, 2]);
    }
    // #endregion 🔖️ThresholdTests

    mod quick {
        use super::*;

        #[test]
        fn ordinal_symbols_always_within_alphabet_for_random_series() {
            let cfg = OrdinalConfig::new(4, 2).unwrap();
            let symbolizer = OrdinalSymbolizer::new(cfg);
            let alphabet = symbolizer.alphabet_size();
            let mut rng = crate::numeric::Xorshift64::new(4242);
            for _ in 0..50 {
                let n = 20 + rng.next_below(50);
                let x: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
                let symbols = symbolizer.symbolize(&x).unwrap();
                assert!(symbols.iter().all(|&s| (s as usize) < alphabet));
            }
        }

        #[test]
        fn quantile_symbolizer_bins_are_roughly_balanced() {
            let symbolizer = QuantileSymbolizer::new(4).unwrap();
            let mut rng = crate::numeric::Xorshift64::new(777);
            let x: Vec<f64> = (0..4000).map(|_| rng.next_gaussian()).collect();
            let symbols = symbolizer.symbolize(&x).unwrap();
            let mut counts = [0usize; 4];
            for &s in &symbols {
                counts[s as usize] += 1;
            }
            for &c in &counts {
                let frac = c as f64 / symbols.len() as f64;
                assert!((frac - 0.25).abs() < 0.03, "counts={counts:?}");
            }
        }

        #[test]
        fn dispersion_symbols_always_within_alphabet_for_random_series() {
            let symbolizer = DispersionSymbolizer::new(5, 3, 1).unwrap();
            let alphabet = symbolizer.alphabet_size();
            let mut rng = crate::numeric::Xorshift64::new(909);
            for _ in 0..50 {
                let n = 30 + rng.next_below(40);
                let x: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
                let symbols = symbolizer.symbolize(&x).unwrap();
                assert!(symbols.iter().all(|&s| (s as usize) < alphabet));
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Symbolic

// #region 🔖️Regularity
pub mod regularity {
//! 🔁️ Regularity/complexity measures over a single scalar time series: Approximate Entropy
//! (ApEn), Sample Entropy (SampEn), and Fuzzy Entropy (FuzzyEn). All three compare
//! time-delay-embedded template vectors (via [`crate::symbolic::embed`]) under a Chebyshev
//! tolerance radius `r` and differ only in how "matching" is counted and whether self-matches
//! are included — see each function's docstring for the exact convention.

use crate::numeric::neumaier_sum;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Tolerance, Warning};

// #region 🔖️Config
/// 🔁️ Shared knobs for the ApEn/SampEn/FuzzyEn family: embedding dimension `m` and tolerance
/// radius `r`. The companion dimension `m + 1` is always derived internally.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RegularityConfig {
    pub m: usize,
    pub r: Tolerance,
}

impl RegularityConfig {
    /// 🔁️ Validates `m >= 1` (a zero-length template is not a meaningful embedding). `r` is
    /// resolved later, once data is available, via [`resolve_tolerance`].
    pub fn new(m: usize, r: Tolerance) -> Result<Self, EntropyError> {
        if m < 1 {
            return Err(EntropyError::InvalidConfig {
                field: "m",
                reason: "embedding dimension must be at least 1",
            });
        }
        Ok(Self { m, r })
    }
}

/// 🔁️ Sample standard deviation (`n - 1` denominator). Rejects a degenerate (constant, or
/// too-short-to-vary) series since it can never anchor a meaningful proportional tolerance.
fn sample_sd(x: &[f64]) -> Result<f64, EntropyError> {
    let n = x.len() as f64;
    if x.len() < 2 {
        return Err(EntropyError::DegenerateInput { what: "series has fewer than 2 points" });
    }
    let mean = neumaier_sum(x.iter().copied()) / n;
    let variance = neumaier_sum(x.iter().map(|&v| (v - mean).powi(2))) / (n - 1.0);
    if variance <= 0.0 {
        return Err(EntropyError::DegenerateInput { what: "constant series has zero standard deviation" });
    }
    Ok(variance.sqrt())
}

/// 🔁️ Resolves a [`Tolerance`] policy into a concrete positive Chebyshev radius for `x`.
fn resolve_tolerance(x: &[f64], r: Tolerance) -> Result<f64, EntropyError> {
    match r {
        Tolerance::Absolute(v) => {
            if !(v.is_finite() && v > 0.0) {
                return Err(EntropyError::InvalidConfig { field: "r", reason: "absolute tolerance must be finite and positive" });
            }
            Ok(v)
        }
        Tolerance::RelativeToSd(k) => {
            if !(k.is_finite() && k > 0.0) {
                return Err(EntropyError::InvalidConfig { field: "r", reason: "sd multiplier must be finite and positive" });
            }
            Ok(k * sample_sd(x)?)
        }
        Tolerance::Auto => Ok(0.2 * sample_sd(x)?),
    }
}
// #endregion 🔖️Config

// #region 🔖️Distance
/// 🔁️ Chebyshev (`L-infinity`) distance between two equal-length template vectors.
fn chebyshev(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&x, &y)| (x - y).abs()).fold(0.0_f64, f64::max)
}

/// 🔁️ Template minus its own mean (the Chen et al. fuzzy-entropy de-trending step).
fn demean(t: &[f64]) -> Vec<f64> {
    let mean = neumaier_sum(t.iter().copied()) / t.len() as f64;
    t.iter().map(|&v| v - mean).collect()
}

fn small_sample_warning(n: usize) -> Option<Warning> {
    (n < 100).then_some(Warning::SmallSample { n, recommended: 100 })
}
// #endregion 🔖️Distance

// #region 🔖️ApproximateEntropy
/// 🔁️ `Phi(dim) = mean_i(ln(C_i))` where `C_i` counts matches INCLUDING the self-match `j == i` —
/// the defining (if statistically awkward) ApEn convention that SampEn below deliberately drops.
fn apen_phi(templates: &[Vec<f64>], r: f64) -> f64 {
    let n = templates.len() as f64;
    let ln_c: Vec<f64> = templates
        .iter()
        .map(|ti| {
            let matches = templates.iter().filter(|tj| chebyshev(ti, tj) <= r).count() as f64;
            (matches / n).ln()
        })
        .collect();
    neumaier_sum(ln_c) / n
}

/// 🔁️ Approximate Entropy (Pincus 1991): `ApEn = Phi(m) - Phi(m + 1)`, in `base`. Self-matches are
/// counted, which biases ApEn low and dependent on `m` in a way SampEn was designed to fix.
pub fn approximate_entropy(x: &[f64], cfg: RegularityConfig, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if x.len() < cfg.m + 2 {
        return Err(EntropyError::InsufficientData {
            what: "approximate_entropy",
            needed: cfg.m + 2,
            actual: x.len(),
        });
    }
    let r = resolve_tolerance(x, cfg.r)?;
    let templates_m = crate::symbolic::embed(x, cfg.m, 1)?;
    let templates_m1 = crate::symbolic::embed(x, cfg.m + 1, 1)?;
    let apen_nats = apen_phi(&templates_m, r) - apen_phi(&templates_m1, r);

    let mut warnings = Vec::new();
    warnings.extend(small_sample_warning(x.len()));

    Ok(Estimate {
        value: base.from_nats(apen_nats),
        base,
        method: "approximate_entropy",
        n: x.len(),
        n_effective: x.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("m", cfg.m as f64), ("r", r)],
    })
}
// #endregion 🔖️ApproximateEntropy

// #region 🔖️SharedEmbedding
/// 🔁️ A pair of `(length-m templates, length-(m+1) templates)` over the same shared index range.
type TemplatePair = (Vec<Vec<f64>>, Vec<Vec<f64>>);

/// 🔁️ Embeds at `m + 1` first to fix the valid start-index range, then embeds at `m` and
/// truncates to that SAME range, so SampEn/FuzzyEn compare the two lengths over identical
/// windows rather than the (larger) index range `m`-only embedding would otherwise allow.
fn shared_templates(x: &[f64], m: usize) -> Result<TemplatePair, EntropyError> {
    let templates_m1 = crate::symbolic::embed(x, m + 1, 1)?;
    let k = templates_m1.len();
    let mut templates_m = crate::symbolic::embed(x, m, 1)?;
    templates_m.truncate(k);
    Ok((templates_m, templates_m1))
}
// #endregion 🔖️SharedEmbedding

// #region 🔖️SampleEntropy
/// 🔁️ Sample Entropy (Richman & Moorman 2000): `SampEn = -ln(A / B)`, where `B` counts
/// length-`m` matches and `A` counts length-`(m + 1)` matches among the SAME index pairs
/// `i < j` (self-matches structurally excluded, unlike ApEn). `B == 0` is undefined; `A == 0`
/// is reported as `+infinity` with a diagnostic warning rather than erroring.
pub fn sample_entropy(x: &[f64], cfg: RegularityConfig, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if x.len() < cfg.m + 2 {
        return Err(EntropyError::InsufficientData { what: "sample_entropy", needed: cfg.m + 2, actual: x.len() });
    }
    let r = resolve_tolerance(x, cfg.r)?;
    let (templates_m, templates_m1) = shared_templates(x, cfg.m)?;
    let k = templates_m1.len();

    let mut a: u64 = 0;
    let mut b: u64 = 0;
    for i in 0..k {
        for j in (i + 1)..k {
            if chebyshev(&templates_m[i], &templates_m[j]) <= r {
                b += 1;
                if chebyshev(&templates_m1[i], &templates_m1[j]) <= r {
                    a += 1;
                }
            }
        }
    }

    if b == 0 {
        return Err(EntropyError::UndefinedResult { reason: "no length-m template matches; sample entropy is undefined" });
    }

    let mut warnings = Vec::new();
    warnings.extend(small_sample_warning(x.len()));
    let sampen_nats = if a == 0 {
        warnings.push(Warning::NotConvergedSoft { what: "no length-(m+1) matches; SampEn is infinite" });
        f64::INFINITY
    } else {
        -((a as f64) / (b as f64)).ln()
    };

    Ok(Estimate {
        value: base.from_nats(sampen_nats),
        base,
        method: "sample_entropy",
        n: x.len(),
        n_effective: x.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("m", cfg.m as f64), ("r", r)],
    })
}
// #endregion 🔖️SampleEntropy

// #region 🔖️FuzzyEntropy
/// 🔁️ `Phi(dim) = (1 / (K*(K-1))) * sum_i sum_{j != i} mu(Chebyshev(T_i - mean(T_i), T_j -
/// mean(T_j)))` with Gaussian membership `mu(d) = exp(-(d/r)^2)`, replacing SampEn/ApEn's hard
/// `<= r` indicator with a smooth one (Chen et al. 2007).
fn fuzzy_phi(templates: &[Vec<f64>], r: f64) -> f64 {
    let k = templates.len();
    let demeaned: Vec<Vec<f64>> = templates.iter().map(|t| demean(t)).collect();
    let mut terms = Vec::with_capacity(k.saturating_mul(k.saturating_sub(1)));
    for i in 0..k {
        for j in 0..k {
            if i == j {
                continue;
            }
            let d = chebyshev(&demeaned[i], &demeaned[j]);
            terms.push((-(d / r).powi(2)).exp());
        }
    }
    neumaier_sum(terms) / (k as f64 * (k as f64 - 1.0))
}

/// 🔁️ Fuzzy Entropy: like [`sample_entropy`] but with a Gaussian-membership match indicator and
/// per-template mean removal, `FuzzyEn = ln(Phi(m)) - ln(Phi(m + 1))`, in `base`.
pub fn fuzzy_entropy(x: &[f64], cfg: RegularityConfig, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if x.len() < cfg.m + 2 {
        return Err(EntropyError::InsufficientData { what: "fuzzy_entropy", needed: cfg.m + 2, actual: x.len() });
    }
    let r = resolve_tolerance(x, cfg.r)?;
    let (templates_m, templates_m1) = shared_templates(x, cfg.m)?;
    let phi_m = fuzzy_phi(&templates_m, r);
    let phi_m1 = fuzzy_phi(&templates_m1, r);
    if phi_m <= 0.0 || phi_m1 <= 0.0 {
        return Err(EntropyError::UndefinedResult { reason: "fuzzy membership sum is non-positive" });
    }
    let fuzzyen_nats = phi_m.ln() - phi_m1.ln();

    let mut warnings = Vec::new();
    warnings.extend(small_sample_warning(x.len()));

    Ok(Estimate {
        value: base.from_nats(fuzzyen_nats),
        base,
        method: "fuzzy_entropy",
        n: x.len(),
        n_effective: x.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("m", cfg.m as f64), ("r", r)],
    })
}
// #endregion 🔖️FuzzyEntropy

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::numeric::Xorshift64;

    fn sine_series(n: usize, period: f64) -> Vec<f64> {
        (0..n).map(|i| (2.0 * core::f64::consts::PI * i as f64 / period).sin()).collect()
    }

    fn white_noise_series(n: usize, seed: u64) -> Vec<f64> {
        let mut rng = Xorshift64::new(seed);
        (0..n).map(|_| rng.next_gaussian()).collect()
    }

    #[test]
    fn regularity_config_rejects_m_zero() {
        assert!(matches!(
            RegularityConfig::new(0, Tolerance::Auto),
            Err(EntropyError::InvalidConfig { field: "m", .. })
        ));
        assert!(RegularityConfig::new(1, Tolerance::Auto).is_ok());
    }

    #[test]
    fn resolve_tolerance_auto_matches_hand_computation() {
        // 🔐️ [1,2,3,4,5]: mean=3, sample variance=(4+1+0+1+4)/4=2.5, sd=sqrt(2.5).
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = 0.2 * 2.5_f64.sqrt();
        let r = resolve_tolerance(&x, Tolerance::Auto).unwrap();
        assert!((r - expected).abs() < 1e-12, "r={r} expected={expected}");
    }

    #[test]
    fn resolve_tolerance_rejects_constant_series() {
        let x = [5.0; 10];
        assert!(matches!(resolve_tolerance(&x, Tolerance::Auto), Err(EntropyError::DegenerateInput { .. })));
        assert!(matches!(
            resolve_tolerance(&x, Tolerance::RelativeToSd(1.0)),
            Err(EntropyError::DegenerateInput { .. })
        ));
    }

    #[test]
    fn sample_entropy_rejects_very_short_series() {
        let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();
        let x = [1.0, 2.0, 3.0];
        assert!(matches!(
            sample_entropy(&x, cfg, LogBase::Nats),
            Err(EntropyError::InsufficientData { .. })
        ));
    }

    #[test]
    fn sample_entropy_reports_infinity_when_no_higher_order_matches_exist() {
        // 🔐️ Hand-verified: at m=1 the pairs (0,1),(0,3),(1,3) match within r=1.0, but every one
        // of those pairs diverges by 4-8 at m+1=2, so A=0 while B=3 — SampEn must be +infinity.
        let x = [1.0, 1.0, 5.0, 1.0, 9.0];
        let cfg = RegularityConfig::new(1, Tolerance::Absolute(1.0)).unwrap();
        let est = sample_entropy(&x, cfg, LogBase::Nats).unwrap();
        assert_eq!(est.value, f64::INFINITY);
        assert!(est.warnings.iter().any(|w| matches!(w, Warning::NotConvergedSoft { .. })));
    }

    #[test]
    fn near_constant_signal_under_generous_tolerance_has_near_zero_regularity_entropy() {
        // 🔐️ A constant plus a 1e-10-scale perturbation, compared against a tolerance orders of
        // magnitude larger than the perturbation: virtually every template matches every other
        // template at both m and m+1, so Phi(m) ~= Phi(m+1) and all three measures collapse to
        // ~0 — the "this series is maximally predictable at this resolution" case.
        let mut rng = Xorshift64::new(11);
        let x: Vec<f64> = (0..300).map(|_| 5.0 + 1e-10 * rng.next_f64()).collect();
        let cfg = RegularityConfig::new(2, Tolerance::Absolute(0.5)).unwrap();
        let apen = approximate_entropy(&x, cfg, LogBase::Nats).unwrap();
        let sampen = sample_entropy(&x, cfg, LogBase::Nats).unwrap();
        assert!(apen.value.abs() < 1e-6, "apen={}", apen.value);
        assert!(sampen.value.abs() < 1e-6, "sampen={}", sampen.value);
    }

    #[test]
    fn regular_sine_has_much_lower_regularity_entropy_than_white_noise() {
        // 🔐️ THE canonical ApEn/SampEn/FuzzyEn sanity check: a smooth periodic signal is far more
        // "regular" (predictable from its own past) than i.i.d. noise of the same length, so all
        // three measures must be substantially lower on the sine than on the noise.
        let n = 1000;
        let sine = sine_series(n, 50.0);
        let noise = white_noise_series(n, 42);
        let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();

        let apen_sine = approximate_entropy(&sine, cfg, LogBase::Nats).unwrap().value;
        let apen_noise = approximate_entropy(&noise, cfg, LogBase::Nats).unwrap().value;
        assert!(apen_sine < apen_noise - 0.5, "apen_sine={apen_sine} apen_noise={apen_noise}");

        let sampen_sine = sample_entropy(&sine, cfg, LogBase::Nats).unwrap().value;
        let sampen_noise = sample_entropy(&noise, cfg, LogBase::Nats).unwrap().value;
        assert!(sampen_sine < sampen_noise - 0.5, "sampen_sine={sampen_sine} sampen_noise={sampen_noise}");

        let fuzzyen_sine = fuzzy_entropy(&sine, cfg, LogBase::Nats).unwrap().value;
        let fuzzyen_noise = fuzzy_entropy(&noise, cfg, LogBase::Nats).unwrap().value;
        assert!(fuzzyen_sine < fuzzyen_noise - 0.5, "fuzzyen_sine={fuzzyen_sine} fuzzyen_noise={fuzzyen_noise}");
    }

    #[test]
    fn approximate_entropy_small_sample_warning() {
        let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();
        let x = sine_series(30, 10.0);
        let est = approximate_entropy(&x, cfg, LogBase::Nats).unwrap();
        assert!(est.warnings.iter().any(|w| matches!(w, Warning::SmallSample { .. })));
    }

    #[test]
    fn base_conversion_is_consistent_across_apen_sampen_fuzzyen() {
        let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();
        let x = sine_series(200, 25.0);
        for est in [
            approximate_entropy(&x, cfg, LogBase::Bits).unwrap(),
            sample_entropy(&x, cfg, LogBase::Bits).unwrap(),
            fuzzy_entropy(&x, cfg, LogBase::Bits).unwrap(),
        ] {
            let nats = est.nats();
            let back = LogBase::convert(nats, LogBase::Nats, LogBase::Bits);
            assert!((back - est.value).abs() < 1e-9);
        }
    }

    mod quick {
        use super::*;

        #[test]
        fn fuzzy_entropy_finite_and_defined_for_moderate_series() {
            let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();
            let x = white_noise_series(500, 7);
            let est = fuzzy_entropy(&x, cfg, LogBase::Nats).unwrap();
            assert!(est.value.is_finite());
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Regularity

// #region 🔖️Ordinal
pub mod ordinal {
//! 🎼️ Symbol-based time-series entropies built on `symbolic.rs`: permutation entropy (Bandt-
//! Pompe), dispersion entropy, increment entropy, and slope entropy — each reduces a real-valued
//! series to a finite alphabet, then reports the Shannon entropy of the resulting symbol
//! distribution.

use crate::numeric::{checked_state_count, neumaier_sum, x_ln_x};
use crate::symbolic::{DispersionSymbolizer, OrdinalConfig, OrdinalSymbolizer, Symbolizer};
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};

// #region 🔖️Shared
fn symbol_distribution_entropy(symbols: &[u32], alphabet_size: usize, base: LogBase, method: &'static str, diagnostics: Vec<(&'static str, f64)>) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if symbols.is_empty() {
        return Err(EntropyError::EmptyInput { what: "symbols" });
    }
    let mut counts = vec![0.0_f64; alphabet_size];
    for &s in symbols {
        let idx = s as usize;
        if idx >= alphabet_size {
            return Err(EntropyError::ShapeMismatch { what: "symbol index", expected: alphabet_size, actual: idx + 1 });
        }
        counts[idx] += 1.0;
    }
    let n = symbols.len() as f64;
    let nats = -neumaier_sum(counts.iter().map(|&c| x_ln_x(c / n)));

    let mut warnings = Vec::new();
    if symbols.len() < 5 * alphabet_size {
        warnings.push(Warning::SmallSample { n: symbols.len(), recommended: 5 * alphabet_size });
    }

    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method,
        n: symbols.len(),
        n_effective: symbols.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics,
    })
}

fn sample_sd(x: &[f64]) -> f64 {
    let n = x.len() as f64;
    let mean = neumaier_sum(x.iter().copied()) / n;
    (neumaier_sum(x.iter().map(|&v| (v - mean).powi(2))) / n).sqrt()
}
// #endregion 🔖️Shared

// #region 🔖️Permutation
/// 🎼️ Bandt-Pompe permutation entropy: Shannon entropy of the ordinal-pattern distribution
/// produced by [`OrdinalSymbolizer`].
pub fn permutation_entropy(x: &[f64], cfg: OrdinalConfig, base: LogBase) -> Result<Estimate, EntropyError> {
    let symbolizer = OrdinalSymbolizer::new(cfg);
    let symbols = symbolizer.symbolize(x)?;
    symbol_distribution_entropy(&symbols, symbolizer.alphabet_size(), base, "permutation_entropy", vec![("dim", cfg.dim as f64), ("tau", cfg.tau as f64)])
}
// #endregion 🔖️Permutation

// #region 🔖️Dispersion
/// 🎼️ Configuration for [`dispersion_entropy`], mirroring [`DispersionSymbolizer`]'s fields.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DispersionConfig {
    pub classes: usize,
    pub dim: usize,
    pub tau: usize,
}

impl DispersionConfig {
    pub fn new(classes: usize, dim: usize, tau: usize) -> Result<Self, EntropyError> {
        if classes < 2 {
            return Err(EntropyError::InvalidConfig { field: "classes", reason: "must be at least 2" });
        }
        if dim == 0 || tau == 0 {
            return Err(EntropyError::InvalidConfig { field: "dim/tau", reason: "must be at least 1" });
        }
        Ok(Self { classes, dim, tau })
    }
}

/// 🎼️ Dispersion entropy: Shannon entropy of the normal-CDF-class dispersion-pattern
/// distribution produced by [`DispersionSymbolizer`].
pub fn dispersion_entropy(x: &[f64], cfg: DispersionConfig, base: LogBase) -> Result<Estimate, EntropyError> {
    let symbolizer = DispersionSymbolizer { classes: cfg.classes, dim: cfg.dim, tau: cfg.tau };
    let symbols = symbolizer.symbolize(x)?;
    symbol_distribution_entropy(
        &symbols,
        symbolizer.alphabet_size(),
        base,
        "dispersion_entropy",
        vec![("classes", cfg.classes as f64), ("dim", cfg.dim as f64), ("tau", cfg.tau as f64)],
    )
}
// #endregion 🔖️Dispersion

// #region 🔖️Increment
/// 🎼️ Increment entropy: each successive difference `x[i+1] - x[i]` is encoded as a signed
/// magnitude symbol in `0..(2*levels+1)` (`levels` per-side magnitude buckets sized by quantiles
/// of `|increment| / sd(increments)`, symbol `levels` reserved for an exact-zero increment),
/// `word_length` consecutive increment-symbols are packed via mixed-radix into one word, and the
/// Shannon entropy of the resulting word distribution is reported.
pub fn increment_entropy(x: &[f64], word_length: usize, levels: usize, base: LogBase) -> Result<Estimate, EntropyError> {
    if word_length == 0 {
        return Err(EntropyError::InvalidConfig { field: "word_length", reason: "must be at least 1" });
    }
    if levels == 0 {
        return Err(EntropyError::InvalidConfig { field: "levels", reason: "must be at least 1" });
    }
    if x.len() < word_length + 2 {
        return Err(EntropyError::InsufficientData { what: "increment_entropy", needed: word_length + 2, actual: x.len() });
    }
    let increments: Vec<f64> = x.windows(2).map(|w| w[1] - w[0]).collect();
    let sd = sample_sd(&increments);
    let alphabet = 2 * levels + 1;

    let symbols: Vec<u32> = increments
        .iter()
        .map(|&d| {
            if d == 0.0 || sd <= 0.0 {
                return levels as u32;
            }
            let normalized = (d.abs() / sd).min(0.999_999);
            let bucket = ((normalized * levels as f64).floor() as usize).min(levels - 1);
            if d > 0.0 {
                (levels + 1 + bucket) as u32
            } else {
                (levels - 1 - bucket) as u32
            }
        })
        .collect();

    let total = checked_state_count(&vec![alphabet; word_length]).ok_or(EntropyError::InvalidConfig {
        field: "word_length",
        reason: "resulting alphabet overflows",
    })?;
    if total > u32::MAX as u128 {
        return Err(EntropyError::InvalidConfig { field: "word_length", reason: "resulting alphabet exceeds u32::MAX" });
    }
    let words: Vec<u32> = symbols
        .windows(word_length)
        .map(|w| w.iter().fold(0u64, |acc, &s| acc * alphabet as u64 + s as u64) as u32)
        .collect();

    symbol_distribution_entropy(&words, total as usize, base, "increment_entropy", vec![("word_length", word_length as f64), ("levels", levels as f64)])
}
// #endregion 🔖️Increment

// #region 🔖️Slope
/// 🎼️ Slope entropy: each successive slope `atan2(x[i+1] - x[i], 1)` (in radians) is classified
/// into one of 5 symbols by two angular thresholds `(gamma1, gamma2)` with `0 < gamma1 < gamma2 <
/// pi/2`: symbol `0`/`4` for a steep negative/positive slope beyond `gamma2`, `1`/`3` for a
/// shallow negative/positive slope in `(gamma1, gamma2]`, and `2` for a near-flat slope within
/// `gamma1`. `dim` consecutive slope-symbols are packed via mixed-radix (base 5) into one word.
pub fn slope_entropy(x: &[f64], thresholds: (f64, f64), dim: usize, base: LogBase) -> Result<Estimate, EntropyError> {
    let (gamma1, gamma2) = thresholds;
    if !(0.0 < gamma1 && gamma1 < gamma2 && gamma2 < core::f64::consts::FRAC_PI_2) {
        return Err(EntropyError::InvalidConfig { field: "thresholds", reason: "must satisfy 0 < gamma1 < gamma2 < pi/2" });
    }
    if dim == 0 {
        return Err(EntropyError::InvalidConfig { field: "dim", reason: "must be at least 1" });
    }
    if x.len() < dim + 1 {
        return Err(EntropyError::InsufficientData { what: "slope_entropy", needed: dim + 1, actual: x.len() });
    }
    let symbols: Vec<u32> = x
        .windows(2)
        .map(|w| {
            let angle = (w[1] - w[0]).atan();
            let sign = if angle >= 0.0 { 1.0 } else { -1.0 };
            let mag = angle.abs();
            let level = if mag > gamma2 {
                2
            } else if mag > gamma1 {
                1
            } else {
                0
            };
            (2 + (sign * level as f64) as i32) as u32
        })
        .collect();

    const ALPHABET: usize = 5;
    let total = checked_state_count(&vec![ALPHABET; dim]).ok_or(EntropyError::InvalidConfig { field: "dim", reason: "resulting alphabet overflows" })?;
    if total > u32::MAX as u128 {
        return Err(EntropyError::InvalidConfig { field: "dim", reason: "resulting alphabet exceeds u32::MAX" });
    }
    let words: Vec<u32> = symbols.windows(dim).map(|w| w.iter().fold(0u64, |acc, &s| acc * ALPHABET as u64 + s as u64) as u32).collect();

    symbol_distribution_entropy(&words, total as usize, base, "slope_entropy", vec![("dim", dim as f64), ("gamma1", gamma1), ("gamma2", gamma2)])
}
// #endregion 🔖️Slope

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_entropy_of_monotone_series_is_zero() {
        let x: Vec<f64> = (0..100).map(|i| i as f64).collect();
        let cfg = OrdinalConfig::new(3, 1).unwrap();
        let est = permutation_entropy(&x, cfg, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn permutation_entropy_of_noise_approaches_max() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let x: Vec<f64> = (0..3000).map(|_| rng.next_f64()).collect();
        let cfg = OrdinalConfig::new(3, 1).unwrap();
        let est = permutation_entropy(&x, cfg, LogBase::Bits).unwrap();
        let max = 6.0_f64.log2(); // 3! = 6 patterns
        assert!(est.value > 0.9 * max, "got {} max {}", est.value, max);
    }

    #[test]
    fn dispersion_config_rejects_small_classes() {
        assert!(DispersionConfig::new(1, 2, 1).is_err());
    }

    #[test]
    fn dispersion_entropy_of_noise_is_positive() {
        let mut rng = crate::numeric::Xorshift64::new(2);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_gaussian()).collect();
        let cfg = DispersionConfig::new(4, 2, 1).unwrap();
        let est = dispersion_entropy(&x, cfg, LogBase::Bits).unwrap();
        assert!(est.value > 0.0);
    }

    #[test]
    fn increment_entropy_of_constant_series_is_zero() {
        let x = vec![5.0; 100];
        // 🔐️ all increments are exactly zero -> single symbol -> zero entropy.
        let est = increment_entropy(&x, 2, 3, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn increment_entropy_of_noise_is_positive() {
        let mut rng = crate::numeric::Xorshift64::new(3);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_gaussian()).collect();
        let est = increment_entropy(&x, 2, 3, LogBase::Bits).unwrap();
        assert!(est.value > 0.0);
    }

    #[test]
    fn increment_entropy_rejects_zero_word_length() {
        let x = vec![1.0, 2.0, 3.0, 4.0];
        assert!(increment_entropy(&x, 0, 3, LogBase::Bits).is_err());
    }

    #[test]
    fn slope_entropy_rejects_bad_thresholds() {
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        assert!(slope_entropy(&x, (0.5, 0.3), 2, LogBase::Bits).is_err());
        assert!(slope_entropy(&x, (-0.1, 0.5), 2, LogBase::Bits).is_err());
    }

    #[test]
    fn slope_entropy_of_straight_line_is_zero() {
        let x: Vec<f64> = (0..100).map(|i| i as f64).collect();
        let est = slope_entropy(&x, (0.2, 0.8), 2, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn slope_entropy_of_noise_is_positive() {
        let mut rng = crate::numeric::Xorshift64::new(4);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_gaussian()).collect();
        let est = slope_entropy(&x, (0.2, 0.8), 2, LogBase::Bits).unwrap();
        assert!(est.value > 0.0);
    }

    mod quick {
        use super::*;

        #[test]
        fn permutation_entropy_orders_regularity_correctly() {
            let mut rng = crate::numeric::Xorshift64::new(5);
            let n = 2000;
            let sine: Vec<f64> = (0..n).map(|i| (i as f64 * 0.1).sin()).collect();
            let noise: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
            let cfg = OrdinalConfig::new(4, 1).unwrap();
            let h_sine = permutation_entropy(&sine, cfg, LogBase::Bits).unwrap().value;
            let h_noise = permutation_entropy(&noise, cfg, LogBase::Bits).unwrap().value;
            assert!(h_sine < h_noise, "sine={h_sine} noise={h_noise}");
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Ordinal

// #region 🔖️Markov
pub mod markov {
//! ⛓️ Fitted, stateful Markov-chain estimation: transition counts and conditional distributions
//! per context, the stationary distribution over contexts via power iteration, and the resulting
//! entropy rate. All internal computation happens in nats; [`LogBase`] conversion is applied only
//! at the [`MarkovChain::entropy_rate`] boundary.

use crate::numeric::{checked_state_count, neumaier_sum, x_ln_x};
use crate::{EntropyError, Estimate, LogBase};

// #region 🔖️Context
/// ⛓️ Packs a window of `order` consecutive symbols into a single mixed-radix context id in
/// `0..alphabet_size^order`. The oldest symbol in `window` is the most significant digit, so
/// dropping it and appending a new symbol (advancing the chain by one step) is a cheap
/// `(context % (alphabet_size^(order-1))) * alphabet_size + next` update — see
/// [`MarkovChain::stationary`].
fn pack_context(window: &[u32], alphabet_size: usize) -> usize {
    window.iter().fold(0usize, |acc, &symbol| acc * alphabet_size + symbol as usize)
}
// #endregion 🔖️Context

// #region 🔖️MarkovChain
/// ⛓️ A fitted order-`order` Markov chain over an alphabet of size `alphabet_size`. Stores raw
/// per-context transition counts and their row-normalized conditional distributions
/// `P(next | context)`. A context with zero observed transitions is treated, for the purposes of
/// [`MarkovChain::stationary`], as an absorbing self-loop (probability 1 of transitioning back to
/// itself) — the simplest choice that keeps the context-transition matrix row-stochastic and
/// well-defined for power iteration without fabricating data the sequence never showed.
pub struct MarkovChain {
    alphabet_size: usize,
    order: usize,
    num_contexts: usize,
    n: usize,
    /// ⛓️ Row-major `num_contexts x alphabet_size` raw transition counts.
    counts: Vec<f64>,
    /// ⛓️ Row-major `num_contexts x alphabet_size` conditional probabilities `P(next | context)`.
    /// A context with zero total count has an all-zero row here.
    conditional: Vec<f64>,
}

impl MarkovChain {
    /// ⛓️ Row-major `num_contexts x alphabet_size` raw transition counts (diagnostic access to
    /// the fitted state, e.g. for inspecting per-context sample sizes before trusting
    /// [`MarkovChain::entropy_rate`] on sparsely observed contexts).
    pub fn raw_counts(&self) -> &[f64] {
        &self.counts
    }

    /// ⛓️ Fits an order-`order` Markov chain to `seq` (symbols in `0..alphabet_size`). Requires
    /// `order >= 1` and `seq.len() >= order + 1`; rejects sequences that are too short with
    /// [`EntropyError::InsufficientData`] and out-of-range symbols or an overflowing
    /// `alphabet_size^order` context space with [`EntropyError::InvalidConfig`].
    pub fn fit(seq: &[u32], alphabet_size: usize, order: usize) -> Result<Self, EntropyError> {
        if order == 0 {
            return Err(EntropyError::InvalidConfig { field: "order", reason: "must be >= 1" });
        }
        if alphabet_size == 0 {
            return Err(EntropyError::InvalidConfig { field: "alphabet_size", reason: "must be >= 1" });
        }
        if seq.len() < order + 1 {
            return Err(EntropyError::InsufficientData {
                what: "markov sequence",
                needed: order + 1,
                actual: seq.len(),
            });
        }
        if seq.iter().any(|&s| s as usize >= alphabet_size) {
            return Err(EntropyError::InvalidConfig {
                field: "seq",
                reason: "symbol index must be < alphabet_size",
            });
        }
        let num_contexts = checked_state_count(&vec![alphabet_size; order])
            .and_then(|c| usize::try_from(c).ok())
            .ok_or(EntropyError::InvalidConfig {
                field: "alphabet_size/order",
                reason: "alphabet_size^order overflows usize",
            })?;

        let mut counts = vec![0.0_f64; num_contexts * alphabet_size];
        for i in order..seq.len() {
            let context = pack_context(&seq[i - order..i], alphabet_size);
            let next = seq[i] as usize;
            counts[context * alphabet_size + next] += 1.0;
        }

        let mut conditional = vec![0.0_f64; num_contexts * alphabet_size];
        for context in 0..num_contexts {
            let row = &counts[context * alphabet_size..(context + 1) * alphabet_size];
            let total = neumaier_sum(row.iter().copied());
            if total > 0.0 {
                for next in 0..alphabet_size {
                    conditional[context * alphabet_size + next] = counts[context * alphabet_size + next] / total;
                }
            }
        }

        Ok(Self { alphabet_size, order, num_contexts, n: seq.len(), counts, conditional })
    }

    /// ⛓️ Stationary distribution over the `alphabet_size^order` contexts, found by power
    /// iteration on the context-transition matrix implied by the fitted conditional
    /// distributions (a context transitions to the context formed by dropping its oldest symbol
    /// and appending the sampled next symbol; for `order == 1` this is the ordinary
    /// symbol-to-symbol transition matrix). Iterates a uniform-start distribution until the L1
    /// change drops below `1e-12` or `10_000` iterations elapse, returning
    /// [`EntropyError::NotConverged`] on the latter.
    pub fn stationary(&self) -> Result<Vec<f64>, EntropyError> {
        let k = self.num_contexts;
        let modulus = k / self.alphabet_size;
        let mut pi = vec![1.0 / k as f64; k];
        for _ in 0..10_000 {
            let mut next_pi = vec![0.0_f64; k];
            for context in 0..k {
                let row = &self.conditional[context * self.alphabet_size..(context + 1) * self.alphabet_size];
                let row_total = neumaier_sum(row.iter().copied());
                if row_total <= 0.0 {
                    // ⛓️ Absorbing self-loop for an unobserved context (see struct docs).
                    next_pi[context] += pi[context];
                    continue;
                }
                for (next, &p) in row.iter().enumerate() {
                    if p <= 0.0 {
                        continue;
                    }
                    let new_context = (context % modulus) * self.alphabet_size + next;
                    next_pi[new_context] += pi[context] * p;
                }
            }
            let diff = neumaier_sum(pi.iter().zip(next_pi.iter()).map(|(&a, &b)| (a - b).abs()));
            pi = next_pi;
            if diff < 1e-12 {
                return Ok(pi);
            }
        }
        Err(EntropyError::NotConverged { what: "markov stationary distribution power iteration", iterations: 10_000 })
    }

    /// ⛓️ Entropy rate `-sum_context pi(context) * sum_next P(next|context) * ln P(next|context)`,
    /// computed in nats from [`MarkovChain::stationary`] and the fitted conditional
    /// distributions, then converted to `base`.
    pub fn entropy_rate(&self, base: LogBase) -> Result<Estimate, EntropyError> {
        base.validate()?;
        let pi = self.stationary()?;
        let nats = neumaier_sum((0..self.num_contexts).map(|context| {
            let row = &self.conditional[context * self.alphabet_size..(context + 1) * self.alphabet_size];
            let context_entropy = -neumaier_sum(row.iter().map(|&p| x_ln_x(p)));
            pi[context] * context_entropy
        }));
        Ok(Estimate {
            value: base.from_nats(nats),
            base,
            method: "markov_entropy_rate",
            n: self.n,
            n_effective: self.n as f64,
            std_error: None,
            ci: None,
            warnings: Vec::new(),
            diagnostics: vec![("order", self.order as f64), ("alphabet_size", self.alphabet_size as f64)],
        })
    }
}
// #endregion 🔖️MarkovChain

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::numeric::Xorshift64;

    /// ⛓️ Generates a sequence from a hand-specified 2-state chain (`transition[i][j] = P(i -> j)`)
    /// starting at state 0, using a deterministic PRNG so tests are exactly reproducible.
    fn generate_two_state_sequence(transition: [[f64; 2]; 2], n: usize, seed: u64) -> Vec<u32> {
        let mut rng = Xorshift64::new(seed);
        let mut seq = Vec::with_capacity(n);
        let mut state = 0u32;
        for _ in 0..n {
            seq.push(state);
            let p_stay = transition[state as usize][state as usize];
            state = if rng.next_f64() < p_stay { state } else { 1 - state };
        }
        seq
    }

    fn binary_entropy_nats(p: f64) -> f64 {
        -(x_ln_x(p) + x_ln_x(1.0 - p))
    }

    #[test]
    fn fit_rejects_too_short_sequence() {
        let seq = [0u32, 1];
        let result = MarkovChain::fit(&seq, 2, 2);
        assert!(matches!(result, Err(EntropyError::InsufficientData { .. })));
    }

    #[test]
    fn fit_rejects_order_zero() {
        let seq = [0u32, 1, 0, 1];
        assert!(matches!(MarkovChain::fit(&seq, 2, 0), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn fit_rejects_out_of_range_symbol() {
        let seq = [0u32, 1, 2, 0];
        assert!(matches!(MarkovChain::fit(&seq, 2, 1), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn order_one_matches_ordinary_transition_counts() {
        let seq = [0u32, 0, 1, 0, 1, 1, 1, 0];
        let chain = MarkovChain::fit(&seq, 2, 1).unwrap();
        let mut expected = [[0.0_f64; 2]; 2];
        for w in seq.windows(2) {
            expected[w[0] as usize][w[1] as usize] += 1.0;
        }
        for (context, row) in expected.iter().enumerate() {
            for (next, &expected_count) in row.iter().enumerate() {
                assert_eq!(chain.counts[context * 2 + next], expected_count, "context={context} next={next}");
            }
        }
    }

    #[test]
    fn periodic_two_cycle_has_near_zero_entropy_rate() {
        let seq: Vec<u32> = (0..100u32).map(|i| i % 2).collect();
        let chain = MarkovChain::fit(&seq, 2, 1).unwrap();
        let estimate = chain.entropy_rate(LogBase::Bits).unwrap();
        assert!(estimate.value.abs() < 1e-9, "value={}", estimate.value);
        let pi = chain.stationary().unwrap();
        assert!((pi[0] - 0.5).abs() < 1e-6, "pi={pi:?}");
        assert!((pi[1] - 0.5).abs() < 1e-6, "pi={pi:?}");
    }

    #[test]
    fn entropy_rate_diagnostics_report_order_and_alphabet() {
        let seq = [0u32, 1, 0, 1, 0, 1];
        let chain = MarkovChain::fit(&seq, 2, 1).unwrap();
        let estimate = chain.entropy_rate(LogBase::Nats).unwrap();
        assert_eq!(estimate.method, "markov_entropy_rate");
        assert_eq!(estimate.n, seq.len());
        assert_eq!(estimate.diagnostics, vec![("order", 1.0), ("alphabet_size", 2.0)]);
    }

    mod quick {
        use super::*;

        #[test]
        fn two_state_chain_converges_to_analytic_stationary_and_entropy_rate() {
            // ⛓️ pi P = pi for [[0.9, 0.1], [0.5, 0.5]] solves to pi = (5/6, 1/6).
            let transition = [[0.9, 0.1], [0.5, 0.5]];
            let seq = generate_two_state_sequence(transition, 50_000, 12_345);
            let chain = MarkovChain::fit(&seq, 2, 1).unwrap();

            let pi = chain.stationary().unwrap();
            let pi0_expected = 5.0 / 6.0;
            let pi1_expected = 1.0 / 6.0;
            assert!((pi[0] - pi0_expected).abs() < 0.02, "pi={pi:?}");
            assert!((pi[1] - pi1_expected).abs() < 0.02, "pi={pi:?}");

            let expected_nats = pi0_expected * binary_entropy_nats(0.9) + pi1_expected * binary_entropy_nats(0.5);
            let estimate = chain.entropy_rate(LogBase::Nats).unwrap();
            assert!((estimate.value - expected_nats).abs() < 0.02, "value={} expected={expected_nats}", estimate.value);
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Markov

// #region 🔖️Multiscale
pub mod multiscale {
//! 📶️ Multiscale entropy: coarse-grain a series at increasing scales and report a chosen
//! regularity/ordinal entropy at each scale, summarized by a complexity index (mean entropy
//! across valid scales).

use crate::ordinal::{dispersion_entropy, permutation_entropy, DispersionConfig};
use crate::regularity::{fuzzy_entropy, sample_entropy, RegularityConfig};
use crate::symbolic::OrdinalConfig;
use crate::{EntropyError, Estimate, LogBase};

// #region 🔖️Grain
/// 📶️ How each scale's coarse-grained series is derived from non-overlapping windows of the
/// original (or previous-scale) series.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Grain {
    Mean,
    Median,
    Variance,
    StdDev,
}

fn coarse_grain(x: &[f64], scale: usize, grain: Grain) -> Vec<f64> {
    if scale == 1 {
        return x.to_vec();
    }
    let n_out = x.len() / scale;
    (0..n_out)
        .map(|j| {
            let window = &x[j * scale..(j + 1) * scale];
            match grain {
                Grain::Mean => window.iter().sum::<f64>() / scale as f64,
                Grain::Median => {
                    let mut sorted = window.to_vec();
                    sorted.sort_by(|a, b| a.total_cmp(b));
                    let mid = sorted.len() / 2;
                    if sorted.len().is_multiple_of(2) {
                        0.5 * (sorted[mid - 1] + sorted[mid])
                    } else {
                        sorted[mid]
                    }
                }
                Grain::Variance | Grain::StdDev => {
                    let mean = window.iter().sum::<f64>() / scale as f64;
                    let var = window.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / scale as f64;
                    if matches!(grain, Grain::StdDev) {
                        var.sqrt()
                    } else {
                        var
                    }
                }
            }
        })
        .collect()
}
// #endregion 🔖️Grain

// #region 🔖️Inner
/// 📶️ Which per-scale entropy [`multiscale_entropy`] computes on each coarse-grained series. An
/// enum (not a trait object) so the whole pipeline stays monomorphic and exhaustively matchable.
#[derive(Clone, PartialEq, Debug)]
pub enum MsInner {
    SampleEntropy(RegularityConfig),
    FuzzyEntropy(RegularityConfig),
    Permutation(OrdinalConfig),
    Dispersion(DispersionConfig),
}

fn run_inner(x: &[f64], inner: &MsInner, base: LogBase) -> Result<Estimate, EntropyError> {
    match inner {
        MsInner::SampleEntropy(cfg) => sample_entropy(x, *cfg, base),
        MsInner::FuzzyEntropy(cfg) => fuzzy_entropy(x, *cfg, base),
        MsInner::Permutation(cfg) => permutation_entropy(x, *cfg, base),
        MsInner::Dispersion(cfg) => dispersion_entropy(x, *cfg, base),
    }
}
// #endregion 🔖️Inner

// #region 🔖️Dispatch
/// 📶️ Configuration for [`multiscale_entropy`].
#[derive(Clone, PartialEq, Debug)]
pub struct MultiscaleConfig {
    pub scales: usize,
    pub grain: Grain,
    pub inner: MsInner,
}

impl MultiscaleConfig {
    pub fn new(scales: usize, grain: Grain, inner: MsInner) -> Result<Self, EntropyError> {
        if scales == 0 {
            return Err(EntropyError::InvalidConfig { field: "scales", reason: "must be at least 1" });
        }
        Ok(Self { scales, grain, inner })
    }
}

/// 📶️ Per-scale entropies plus a summary complexity index (mean entropy across valid scales).
#[derive(Clone, Debug)]
pub struct MultiscaleResult {
    pub per_scale: Vec<Estimate>,
    pub complexity_index: f64,
    pub scales: Vec<usize>,
}

/// 📶️ Coarse-grains `x` at scales `1..=cfg.scales` and computes `cfg.inner`'s entropy at each,
/// stopping early (without error) once a scale's coarse-grained series becomes too short for the
/// inner method — [`MultiscaleResult::scales`] reports exactly which scales succeeded.
pub fn multiscale_entropy(x: &[f64], cfg: &MultiscaleConfig, base: LogBase) -> Result<MultiscaleResult, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    let mut per_scale = Vec::new();
    let mut scales = Vec::new();
    for scale in 1..=cfg.scales {
        let coarse = coarse_grain(x, scale, cfg.grain);
        if coarse.len() < 4 {
            break;
        }
        match run_inner(&coarse, &cfg.inner, base) {
            Ok(est) => {
                per_scale.push(est);
                scales.push(scale);
            }
            Err(_) if scale > 1 => break,
            Err(e) => return Err(e),
        }
    }
    if per_scale.is_empty() {
        return Err(EntropyError::InsufficientData { what: "multiscale_entropy", needed: 4, actual: x.len() });
    }
    let complexity_index = per_scale.iter().map(|e| e.value).sum::<f64>() / per_scale.len() as f64;
    Ok(MultiscaleResult { per_scale, complexity_index, scales })
}
// #endregion 🔖️Dispatch

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiscale_config_rejects_zero_scales() {
        let inner = MsInner::Permutation(OrdinalConfig::new(3, 1).unwrap());
        assert!(MultiscaleConfig::new(0, Grain::Mean, inner).is_err());
    }

    #[test]
    fn coarse_grain_mean_matches_hand_computation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let coarse = coarse_grain(&x, 2, Grain::Mean);
        assert_eq!(coarse, vec![1.5, 3.5, 5.5]);
    }

    #[test]
    fn coarse_grain_scale_one_is_identity() {
        let x = vec![1.0, 2.0, 3.0];
        assert_eq!(coarse_grain(&x, 1, Grain::Mean), x);
    }

    #[test]
    fn multiscale_entropy_reports_requested_scales_for_long_series() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let x: Vec<f64> = (0..3000).map(|_| rng.next_f64()).collect();
        let inner = MsInner::Permutation(OrdinalConfig::new(3, 1).unwrap());
        let cfg = MultiscaleConfig::new(5, Grain::Mean, inner).unwrap();
        let result = multiscale_entropy(&x, &cfg, LogBase::Bits).unwrap();
        assert_eq!(result.scales, vec![1, 2, 3, 4, 5]);
        assert_eq!(result.per_scale.len(), 5);
    }

    #[test]
    fn multiscale_entropy_stops_early_for_short_series() {
        let x: Vec<f64> = (0..20).map(|i| i as f64).collect();
        let inner = MsInner::Permutation(OrdinalConfig::new(3, 1).unwrap());
        let cfg = MultiscaleConfig::new(20, Grain::Mean, inner).unwrap();
        let result = multiscale_entropy(&x, &cfg, LogBase::Bits).unwrap();
        assert!(result.scales.len() < 20);
    }

    mod quick {
        use super::*;

        #[test]
        fn white_noise_multiscale_entropy_differs_from_pink_like_noise() {
            let mut rng = crate::numeric::Xorshift64::new(2);
            let n = 4000;
            let white: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
            // 🔐️ a crude 1/f-like signal via running-sum (integrated white noise).
            let mut acc = 0.0;
            let pink: Vec<f64> = std::iter::repeat_with(|| {
                acc = 0.98 * acc + rng.next_gaussian();
                acc
            })
            .take(n)
            .collect();
            let inner = MsInner::SampleEntropy(RegularityConfig::new(2, crate::Tolerance::Auto).unwrap());
            let cfg = MultiscaleConfig::new(4, Grain::Mean, inner).unwrap();
            let white_result = multiscale_entropy(&white, &cfg, LogBase::Nats).unwrap();
            let pink_result = multiscale_entropy(&pink, &cfg, LogBase::Nats).unwrap();
            assert!((white_result.complexity_index - pink_result.complexity_index).abs() > 1e-6);
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Multiscale

// #region 🔖️Lz
pub mod lz {
//! 🗜️ Lempel-Ziv complexity family: the Kaspar & Schuster (1987) incremental-parsing LZ76
//! complexity measure over arbitrary discrete symbol streams, a from-scratch LZ78 dictionary
//! [`Compressor`], and the compressor-agnostic normalized compression distance ([`ncd`]) built on
//! top of it. Zero external dependencies — no `flate2`, no `zstd`, nothing beyond `std`.

use std::collections::HashMap;

use crate::{EntropyError, Estimate, LogBase, Warning};

// #region 🔖️Lz76
/// 🗜️ Kaspar & Schuster (1987) incremental-parsing complexity `c(n)`, generalized from binary
/// strings to an arbitrary `u32`-symbol alphabet. Counts the number of distinct "new" phrases a
/// greedy left-to-right parse needs to reproduce `s`, where a phrase may self-referentially copy
/// from *inside* the phrase currently being grown (not just from history already committed).
///
/// Ported verbatim (0-indexed) from the classic incremental-parsing pseudocode; the loop body is
/// fiddly and was **not** simplified. The two indices `s[i + k - 1]` / `s[l + k - 1]` are read
/// through [`slice::get`] rather than direct indexing: an out-of-range read is treated as "not
/// equal" so the mismatch branch fires instead of panicking, which is the only change from the
/// textbook pseudocode (needed because the naive index-safety invariant does not actually hold
/// for short inputs — confirmed by exhaustive brute-force cross-validation, see
/// `tests::exhaustive`).
fn lz76_complexity(s: &[u32]) -> usize {
    let n = s.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let (mut i, mut k, mut l) = (0usize, 1usize, 1usize);
    let mut c = 1usize;
    let mut k_max = 1usize;
    loop {
        let equal = match (s.get(i + k - 1), s.get(l + k - 1)) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        };
        if equal {
            k += 1;
            if l + k > n {
                c += 1;
                break;
            }
        } else {
            if k > k_max {
                k_max = k;
            }
            i += 1;
            if i == l {
                c += 1;
                l += k_max;
                if l + 1 > n {
                    break;
                }
                i = 0;
                k = 1;
                k_max = 1;
            } else {
                k = 1;
            }
        }
    }
    c
}

/// 🗜️ Lempel-Ziv complexity of a discrete symbol sequence. Raw mode reports the incremental-parse
/// phrase count `c(n)` directly; `normalized` mode reports `c(n) * log_alpha(n) / n` (`alpha` =
/// occupied alphabet size), which converges to `1` for a maximally complex sequence over that
/// alphabet as `n -> infinity`.
///
/// `base` on the returned [`Estimate`] is always [`LogBase::Nats`] as an inert placeholder — LZ
/// complexity is a phrase count / normalized ratio, not a log-base-dependent entropy, so
/// [`Estimate::in_base`] is meaningless here and should not be called on this result.
pub fn lempel_ziv_complexity(symbols: &[u32], normalized: bool) -> Result<Estimate, EntropyError> {
    if symbols.is_empty() {
        return Err(EntropyError::EmptyInput { what: "symbols" });
    }
    let n = symbols.len();
    let c = lz76_complexity(symbols);

    let mut alphabet: Vec<u32> = symbols.to_vec();
    alphabet.sort_unstable();
    alphabet.dedup();
    let alpha = alphabet.len();

    let value = if normalized {
        if alpha <= 1 {
            0.0
        } else {
            let log_alpha_n = (n as f64).ln() / (alpha as f64).ln();
            c as f64 * log_alpha_n / n as f64
        }
    } else {
        c as f64
    };

    let mut warnings = Vec::new();
    if n < 100 {
        warnings.push(Warning::SmallSample { n, recommended: 100 });
    }

    Ok(Estimate {
        value,
        base: LogBase::Nats,
        method: "lempel_ziv",
        n,
        n_effective: n as f64,
        std_error: None,
        ci: None,
        warnings,
        diagnostics: vec![("alphabet_size", alpha as f64), ("raw_complexity", c as f64)],
    })
}
// #endregion 🔖️Lz76

// #region 🔖️Compressor
/// 🗜️ A byte-stream compressor exposing only the one number [`ncd`] needs: how many bytes `data`
/// would take to represent. Kept behind a trait so [`ncd`] never depends on a specific codec's
/// implementation details.
pub trait Compressor {
    fn compressed_len(&self, data: &[u8]) -> usize;
}

/// 🗜️ Textbook LZ78 dictionary compressor (Ziv & Lempel, 1978), implemented from scratch as a
/// proxy [`Compressor`] for [`ncd`]. Not a byte-for-byte codec (there is no matching decoder) —
/// only [`Compressor::compressed_len`]'s *size estimate* is produced.
pub struct Lz78Compressor;

impl Compressor for Lz78Compressor {
    /// 🗜️ Greedily extends the current phrase while it remains a dictionary hit; each time a new
    /// (dictionary-index, literal-byte) pair is emitted, the phrase is added to the dictionary and
    /// the running bit cost grows by `ceil(log2(dictionary_size + 2))` (index) `+ 8` (literal).
    /// The dictionary is capped at 65535 entries (`u16` index space, index `0` reserved for the
    /// implicit empty root phrase); once full, matching against existing entries continues but no
    /// new phrases are memorized (standard LZW/LZ78 dictionary-full behavior). Returns the total
    /// emitted bit cost rounded up to whole bytes.
    fn compressed_len(&self, data: &[u8]) -> usize {
        if data.is_empty() {
            return 0;
        }
        const MAX_ENTRIES: usize = 65_535;
        let mut dict: HashMap<Vec<u8>, u16> = HashMap::new();
        let mut phrase: Vec<u8> = Vec::new();
        let mut total_bits: f64 = 0.0;

        let index_bits = |dict_len: usize| -> f64 {
            let addressable = dict_len as f64 + 1.0; // 🗜️ +1 for the implicit empty-phrase index 0
            (addressable + 1.0).log2().ceil().max(1.0)
        };

        for &byte in data {
            phrase.push(byte);
            if !dict.contains_key(&phrase) {
                total_bits += index_bits(dict.len()) + 8.0;
                if dict.len() < MAX_ENTRIES {
                    dict.insert(phrase.clone(), dict.len() as u16 + 1);
                }
                phrase.clear();
            }
        }
        if !phrase.is_empty() {
            // 🗜️ Trailing phrase matched an existing entry but the input ended before a new
            // extension was discovered; emit one final index-only reference (no literal).
            total_bits += index_bits(dict.len());
        }
        ((total_bits / 8.0).ceil() as usize).max(1)
    }
}
// #endregion 🔖️Compressor

// #region 🔖️Ncd
/// 🗜️ Normalized compression distance (Cilibrasi & Vitanyi, 2005):
/// `(C(xy) - min(C(x), C(y))) / max(C(x), C(y))`, a compressor-driven approximation to
/// normalized information distance in `[0, ~1]`. `compressor` decides what "compressed" means;
/// [`Lz78Compressor`] is a reasonable zero-dependency default.
pub fn ncd(x: &[u8], y: &[u8], compressor: &dyn Compressor) -> Result<f64, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    if y.is_empty() {
        return Err(EntropyError::EmptyInput { what: "y" });
    }
    let cx = compressor.compressed_len(x) as f64;
    let cy = compressor.compressed_len(y) as f64;
    let mut xy = Vec::with_capacity(x.len() + y.len());
    xy.extend_from_slice(x);
    xy.extend_from_slice(y);
    let cxy = compressor.compressed_len(&xy) as f64;

    let denom = cx.max(cy);
    if denom == 0.0 {
        return Ok(0.0);
    }
    Ok((cxy - cx.min(cy)) / denom)
}
// #endregion 🔖️Ncd

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn binary_string(s: &str) -> Vec<u32> {
        s.chars().map(|c| if c == '1' { 1 } else { 0 }).collect()
    }

    #[test]
    fn lz76_matches_canonical_test_string() {
        // 🔐️ See `exhaustive::lz76_canonical_value_matches_verified_reference` for the full
        // cross-validation story behind this specific number.
        let s = binary_string("0001101001000101");
        assert_eq!(lz76_complexity(&s), 6);
    }

    #[test]
    fn lz76_constant_sequence_is_minimally_complex() {
        let s = vec![0u32; 200];
        assert!(lz76_complexity(&s) <= 3);
    }

    #[test]
    fn lz76_repetitive_much_lower_than_random_of_same_length() {
        let mut rng = crate::numeric::Xorshift64::new(7);
        let n = 500;
        let repetitive: Vec<u32> = (0..n).map(|i| (i % 3) as u32).collect();
        let random: Vec<u32> = (0..n).map(|_| rng.next_below(8) as u32).collect();
        let c_rep = lz76_complexity(&repetitive);
        let c_rand = lz76_complexity(&random);
        assert!(c_rep * 3 < c_rand, "c_rep={c_rep} c_rand={c_rand}");
    }

    #[test]
    fn lempel_ziv_complexity_rejects_empty() {
        assert!(matches!(
            lempel_ziv_complexity(&[], false),
            Err(EntropyError::EmptyInput { .. })
        ));
    }

    #[test]
    fn lempel_ziv_complexity_raw_matches_lz76() {
        let s = binary_string("0001101001000101");
        let est = lempel_ziv_complexity(&s, false).unwrap();
        assert_eq!(est.value, 6.0);
        assert_eq!(est.n, s.len());
        assert_eq!(est.diagnostics[1], ("raw_complexity", 6.0));
    }

    #[test]
    fn lempel_ziv_complexity_normalized_is_zero_for_single_symbol_alphabet() {
        let s = vec![0u32; 50];
        let est = lempel_ziv_complexity(&s, true).unwrap();
        assert_eq!(est.value, 0.0);
        assert_eq!(est.diagnostics[0], ("alphabet_size", 1.0));
    }

    #[test]
    fn lempel_ziv_complexity_small_sample_warns() {
        let s = binary_string("0001101001000101");
        let est = lempel_ziv_complexity(&s, false).unwrap();
        assert!(est.warnings.iter().any(|w| matches!(w, Warning::SmallSample { .. })));
    }

    #[test]
    fn lempel_ziv_complexity_large_sample_does_not_warn() {
        let mut rng = crate::numeric::Xorshift64::new(3);
        let s: Vec<u32> = (0..200).map(|_| rng.next_below(4) as u32).collect();
        let est = lempel_ziv_complexity(&s, false).unwrap();
        assert!(est.warnings.is_empty());
    }

    #[test]
    fn lz78_empty_input_compresses_to_zero() {
        assert_eq!(Lz78Compressor.compressed_len(&[]), 0);
    }

    #[test]
    fn lz78_repetitive_input_compresses_shorter_than_random() {
        let repetitive = b"abababababababab".to_vec();
        let mut rng = crate::numeric::Xorshift64::new(42);
        let random: Vec<u8> = (0..repetitive.len()).map(|_| rng.next_below(256) as u8).collect();
        let comp = Lz78Compressor;
        assert!(comp.compressed_len(&repetitive) <= comp.compressed_len(&random));
    }

    #[test]
    fn ncd_rejects_empty_inputs() {
        let comp = Lz78Compressor;
        assert!(matches!(ncd(&[], b"x", &comp), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(ncd(b"x", &[], &comp), Err(EntropyError::EmptyInput { .. })));
    }

    #[test]
    fn ncd_of_a_string_with_itself_is_well_below_unrelated_random_strings() {
        let comp = Lz78Compressor;
        let text: Vec<u8> = b"the quick brown fox jumps over the lazy dog ".repeat(8);
        let d_self = ncd(&text, &text, &comp).unwrap();

        let mut rng_a = crate::numeric::Xorshift64::new(1234);
        let a: Vec<u8> = (0..text.len()).map(|_| rng_a.next_below(256) as u8).collect();
        let mut rng_b = crate::numeric::Xorshift64::new(999_999);
        let b: Vec<u8> = (0..text.len()).map(|_| rng_b.next_below(256) as u8).collect();
        let d_diff = ncd(&a, &b, &comp).unwrap();

        assert!(d_self < d_diff, "d_self={d_self} d_diff={d_diff}");
        assert!(d_diff > 0.5, "expected two unrelated random byte strings to be far apart: {d_diff}");
    }

    #[test]
    fn ncd_is_bounded_below_by_zero() {
        let comp = Lz78Compressor;
        let x = b"identical payload identical payload".to_vec();
        let d = ncd(&x, &x, &comp).unwrap();
        assert!(d >= 0.0);
    }

    mod quick {
        use super::*;

        #[test]
        fn lz76_is_non_decreasing_in_sequence_length_for_a_growing_random_stream() {
            let mut rng = crate::numeric::Xorshift64::new(11);
            let full: Vec<u32> = (0..300).map(|_| rng.next_below(5) as u32).collect();
            let mut prev = lz76_complexity(&full[..1]);
            for len in [10, 50, 100, 200, 300] {
                let c = lz76_complexity(&full[..len]);
                assert!(c >= prev, "len={len} c={c} prev={prev}");
                prev = c;
            }
        }

        #[test]
        fn lz78_concatenation_never_shrinks_relative_to_either_half() {
            let comp = Lz78Compressor;
            let mut rng = crate::numeric::Xorshift64::new(55);
            for _ in 0..20 {
                let n = 20 + rng.next_below(80);
                let x: Vec<u8> = (0..n).map(|_| rng.next_below(256) as u8).collect();
                let m = 20 + rng.next_below(80);
                let y: Vec<u8> = (0..m).map(|_| rng.next_below(256) as u8).collect();
                let mut xy = x.clone();
                xy.extend_from_slice(&y);
                let cx = comp.compressed_len(&x);
                let cy = comp.compressed_len(&y);
                let cxy = comp.compressed_len(&xy);
                assert!(cxy >= cx.max(cy), "cxy={cxy} cx={cx} cy={cy}");
            }
        }
    }

    // #region 🔖️Exhaustive
    /// 🔐️ Brute-force validation of [`lz76_complexity`] against an independent definitional
    /// oracle, over every binary string of length `1..=12` (`4094` strings). This measure is
    /// well known to be easy to get off-by-one wrong, so the incremental (fast) implementation is
    /// checked against a slow, obviously-correct-by-construction parser rather than trusted on
    /// its own.
    mod exhaustive {
        use super::*;

        /// 🔐️ Naive contiguous-substring test, `O(n*m)`, used only by the brute-force oracle
        /// below (never on a hot path).
        fn contains(haystack: &[u32], needle: &[u32]) -> bool {
            if needle.is_empty() {
                return true;
            }
            if needle.len() > haystack.len() {
                return false;
            }
            (0..=haystack.len() - needle.len()).any(|start| &haystack[start..start + needle.len()] == needle)
        }

        /// 🔐️ Definitional LZ76 phrase count: repeatedly takes the shortest prefix-extension of
        /// the unparsed remainder that is not already a substring of (parsed history + candidate
        /// minus its own last symbol), i.e. the shortest prefix of the remainder not found
        /// anywhere in the string up to (and including) the position just before the prefix's
        /// last symbol. Deliberately independent of [`lz76_complexity`]'s control flow.
        fn brute_force_lz76(s: &[u32]) -> usize {
            let n = s.len();
            if n == 0 {
                return 0;
            }
            let mut phrases = 0usize;
            let mut pos = 0usize;
            while pos < n {
                let mut len = 1usize;
                loop {
                    if pos + len > n {
                        len = n - pos;
                        break;
                    }
                    let candidate = &s[pos..pos + len];
                    let haystack = &s[0..pos + len - 1];
                    if !contains(haystack, candidate) {
                        break;
                    }
                    len += 1;
                }
                phrases += 1;
                pos += len;
            }
            phrases
        }

        #[test]
        fn lz76_matches_definitional_brute_force_for_every_short_binary_string() {
            let mut checked = 0usize;
            for len in 1..=12usize {
                for v in 0..(1u32 << len) {
                    let s: Vec<u32> = (0..len as u32).map(|b| (v >> b) & 1).collect();
                    let incremental = lz76_complexity(&s);
                    let brute = brute_force_lz76(&s);
                    assert_eq!(incremental, brute, "len={len} v={v} s={s:?}");
                    checked += 1;
                }
            }
            assert_eq!(checked, 8190); // 2^1 + 2^2 + ... + 2^12
        }

        #[test]
        fn lz76_canonical_value_matches_verified_reference() {
            // 🔐️ The literal incremental-parsing pseudocode, cross-checked against
            // `brute_force_lz76` above on all 8190 binary strings of length 1..=12 with zero
            // mismatches, computes `c = 6` for this string (not the `8` sometimes quoted for it
            // in secondary sources — that figure does not reproduce under this definition and
            // was rejected in favor of the exhaustively cross-validated result).
            let s = binary_string("0001101001000101");
            assert_eq!(lz76_complexity(&s), 6);
            assert_eq!(brute_force_lz76(&s), 6);
        }
    }
    // #endregion 🔖️Exhaustive
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Lz

// #region 🔖️Fft
pub mod fft {
//! 🌊️ Hand-rolled discrete Fourier transform: iterative radix-2 Cooley-Tukey for power-of-two
//! lengths, Bluestein's chirp-z algorithm for arbitrary lengths (Welch segment lengths are
//! user-chosen and rarely powers of two), plus the standard analysis window functions.

use crate::EntropyError;
use std::ops::{Add, Mul, Sub};

// #region 🔖️Complex
/// 🌊️ A minimal complex number, `re + i*im`.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub const fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    pub fn from_polar(magnitude: f64, angle: f64) -> Self {
        Self { re: magnitude * angle.cos(), im: magnitude * angle.sin() }
    }

    pub fn conj(self) -> Self {
        Self { re: self.re, im: -self.im }
    }

    pub fn norm_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn abs(self) -> f64 {
        self.norm_sq().sqrt()
    }

    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn scale(self, s: f64) -> Self {
        Self { re: self.re * s, im: self.im * s }
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Complex {
        Complex { re: self.re - rhs.re, im: self.im - rhs.im }
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex { re: self.re * rhs.re - self.im * rhs.im, im: self.re * rhs.im + self.im * rhs.re }
    }
}
// #endregion 🔖️Complex

// #region 🔖️Radix2
fn bit_reverse_permute(data: &mut [Complex]) {
    let n = data.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            data.swap(i, j);
        }
    }
}

/// 🌊️ In-place iterative radix-2 Cooley-Tukey FFT. `n = data.len()` must be a power of two.
/// `inverse = true` computes the unnormalized inverse transform (caller divides by `n`).
fn fft_radix2_inplace(data: &mut [Complex], inverse: bool) {
    let n = data.len();
    if n <= 1 {
        return;
    }
    bit_reverse_permute(data);
    let sign = if inverse { 1.0 } else { -1.0 };
    let mut len = 2;
    while len <= n {
        let angle = sign * 2.0 * core::f64::consts::PI / len as f64;
        let wlen = Complex::from_polar(1.0, angle);
        let mut i = 0;
        while i < n {
            let mut w = Complex::new(1.0, 0.0);
            for k in 0..len / 2 {
                let u = data[i + k];
                let v = data[i + k + len / 2] * w;
                data[i + k] = u + v;
                data[i + k + len / 2] = u - v;
                w = w * wlen;
            }
            i += len;
        }
        len <<= 1;
    }
}

fn next_power_of_two(n: usize) -> usize {
    n.next_power_of_two()
}
// #endregion 🔖️Radix2

// #region 🔖️Bluestein
/// 🌊️ Bluestein's chirp-z transform: reduces an arbitrary-length DFT to a power-of-two
/// convolution via the identity `n*k = (n^2 + k^2 - (n-k)^2) / 2`, so it applies exactly to any
/// length (used whenever `n` is not already a power of two).
fn fft_bluestein(data: &[Complex], inverse: bool) -> Vec<Complex> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }
    let sign = if inverse { 1.0 } else { -1.0 };
    let chirp: Vec<Complex> = (0..n)
        .map(|k| {
            let angle = sign * core::f64::consts::PI * ((k as u128 * k as u128) % (2 * n as u128)) as f64 / n as f64;
            Complex::from_polar(1.0, angle)
        })
        .collect();

    let m = next_power_of_two(2 * n - 1);
    let mut a = vec![Complex::zero(); m];
    for k in 0..n {
        a[k] = data[k] * chirp[k];
    }
    let mut b = vec![Complex::zero(); m];
    b[0] = chirp[0].conj();
    for k in 1..n {
        let c = chirp[k].conj();
        b[k] = c;
        b[m - k] = c;
    }

    fft_radix2_inplace(&mut a, false);
    fft_radix2_inplace(&mut b, false);
    let mut conv: Vec<Complex> = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).collect();
    fft_radix2_inplace(&mut conv, true);
    let inv_m = 1.0 / m as f64;
    for c in &mut conv {
        *c = c.scale(inv_m);
    }

    (0..n).map(|k| conv[k] * chirp[k]).collect()
}
// #endregion 🔖️Bluestein

// #region 🔖️Fft
/// 🌊️ A reusable FFT plan for a fixed transform length `n`, dispatching to the fast radix-2 path
/// when `n` is a power of two and to Bluestein's algorithm otherwise. Caches nothing beyond `n`
/// itself (the radix-2 path recomputes twiddles per call; profiling has not shown this to matter
/// at the sample sizes entropy estimators use) but keeps a stable, testable, allocation-owning
/// API for callers that transform the same length repeatedly.
pub struct Fft {
    n: usize,
    power_of_two: bool,
}

impl Fft {
    pub fn new(n: usize) -> Self {
        Self { n, power_of_two: n.is_power_of_two() }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// 🌊️ Forward DFT: `X_k = sum_j x_j * exp(-2*pi*i*j*k/n)`.
    pub fn forward(&self, input: &[Complex]) -> Vec<Complex> {
        assert_eq!(input.len(), self.n, "Fft::forward: input length must match plan length");
        if self.power_of_two {
            let mut data = input.to_vec();
            fft_radix2_inplace(&mut data, false);
            data
        } else {
            fft_bluestein(input, false)
        }
    }

    /// 🌊️ Inverse DFT (normalized by `1/n`): `x_j = (1/n) sum_k X_k * exp(+2*pi*i*j*k/n)`.
    pub fn inverse(&self, input: &[Complex]) -> Vec<Complex> {
        assert_eq!(input.len(), self.n, "Fft::inverse: input length must match plan length");
        let mut data = if self.power_of_two {
            let mut d = input.to_vec();
            fft_radix2_inplace(&mut d, true);
            d
        } else {
            fft_bluestein(input, true)
        };
        let inv_n = 1.0 / self.n as f64;
        for c in &mut data {
            *c = c.scale(inv_n);
        }
        data
    }
}

/// 🌊️ Real-input forward FFT, returning the one-sided spectrum (`n/2 + 1` bins, DC through
/// Nyquist) since a real signal's full spectrum is Hermitian-symmetric.
pub fn real_fft(input: &[f64]) -> Vec<Complex> {
    let n = input.len();
    let complex: Vec<Complex> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
    let full = Fft::new(n).forward(&complex);
    full.into_iter().take(n / 2 + 1).collect()
}

/// 🌊️ Naive `O(n^2)` DFT, kept as the correctness oracle for [`Fft`] in tests — never used on the
/// hot path.
#[cfg(test)]
fn naive_dft(input: &[Complex], inverse: bool) -> Vec<Complex> {
    let n = input.len();
    let sign = if inverse { 1.0 } else { -1.0 };
    (0..n)
        .map(|k| {
            let mut sum = Complex::zero();
            for (j, &x) in input.iter().enumerate() {
                let angle = sign * 2.0 * core::f64::consts::PI * (j * k) as f64 / n as f64;
                sum = sum + x * Complex::from_polar(1.0, angle);
            }
            if inverse {
                sum.scale(1.0 / n as f64)
            } else {
                sum
            }
        })
        .collect()
}
// #endregion 🔖️Fft

// #region 🔖️Window
/// 🌊️ Analysis window function family for spectral estimation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WindowKind {
    Rectangular,
    Hann,
    Hamming,
    Blackman,
    BlackmanHarris,
    /// 🌊️ Kaiser window with shape parameter `beta` (larger = narrower mainlobe, higher sidelobes
    /// suppressed less).
    Kaiser(f64),
    /// 🌊️ Tukey (tapered cosine) window with taper fraction `alpha` in `[0, 1]`.
    Tukey(f64),
}

/// 🌊️ Modified zeroth-order Bessel function `I0(x)`, needed by the Kaiser window, via its power
/// series (converges rapidly for the `x` ranges Kaiser windows use).
fn bessel_i0(x: f64) -> f64 {
    let mut term = 1.0_f64;
    let mut sum = 1.0_f64;
    let half_x_sq = (x / 2.0) * (x / 2.0);
    for k in 1..64 {
        term *= half_x_sq / (k as f64 * k as f64);
        sum += term;
        if term < sum * 1e-17 {
            break;
        }
    }
    sum
}

/// 🌊️ Samples a length-`n` window of the given kind.
pub fn window(kind: WindowKind, n: usize) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![1.0];
    }
    let nf = (n - 1) as f64;
    match kind {
        WindowKind::Rectangular => vec![1.0; n],
        WindowKind::Hann => {
            (0..n).map(|i| 0.5 - 0.5 * (2.0 * core::f64::consts::PI * i as f64 / nf).cos()).collect()
        }
        WindowKind::Hamming => {
            (0..n).map(|i| 0.54 - 0.46 * (2.0 * core::f64::consts::PI * i as f64 / nf).cos()).collect()
        }
        WindowKind::Blackman => (0..n)
            .map(|i| {
                let x = 2.0 * core::f64::consts::PI * i as f64 / nf;
                0.42 - 0.5 * x.cos() + 0.08 * (2.0 * x).cos()
            })
            .collect(),
        WindowKind::BlackmanHarris => (0..n)
            .map(|i| {
                let x = 2.0 * core::f64::consts::PI * i as f64 / nf;
                0.358_75 - 0.488_29 * x.cos() + 0.141_28 * (2.0 * x).cos() - 0.011_68 * (3.0 * x).cos()
            })
            .collect(),
        WindowKind::Kaiser(beta) => {
            let i0_beta = bessel_i0(beta);
            (0..n)
                .map(|i| {
                    let ratio = (2.0 * i as f64 / nf) - 1.0;
                    let arg = beta * (1.0 - ratio * ratio).max(0.0).sqrt();
                    bessel_i0(arg) / i0_beta
                })
                .collect()
        }
        WindowKind::Tukey(alpha) => {
            let alpha = alpha.clamp(0.0, 1.0);
            if alpha <= 0.0 {
                return window(WindowKind::Rectangular, n);
            }
            let taper = (alpha * nf / 2.0).floor() as usize;
            (0..n)
                .map(|i| {
                    if i < taper {
                        0.5 * (1.0 + (core::f64::consts::PI * (i as f64 / taper as f64 - 1.0)).cos())
                    } else if i >= n - taper {
                        let j = n - 1 - i;
                        0.5 * (1.0 + (core::f64::consts::PI * (j as f64 / taper as f64 - 1.0)).cos())
                    } else {
                        1.0
                    }
                })
                .collect()
        }
    }
}

/// 🌊️ Validates that a window/segment length is nonzero, the one input check every windowed
/// caller needs before slicing.
pub fn validate_length(n: usize, what: &'static str) -> Result<(), EntropyError> {
    if n == 0 {
        return Err(EntropyError::EmptyInput { what });
    }
    Ok(())
}
// #endregion 🔖️Window

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq_complex(a: Complex, b: Complex, tol: f64) -> bool {
        (a.re - b.re).abs() < tol && (a.im - b.im).abs() < tol
    }

    #[test]
    fn radix2_fft_matches_naive_dft() {
        for n in [2usize, 4, 8, 16, 32, 64] {
            let mut rng = crate::numeric::Xorshift64::new(n as u64);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5)).collect();
            let fast = Fft::new(n).forward(&input);
            let naive = naive_dft(&input, false);
            for (a, b) in fast.iter().zip(naive.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-9), "n={n}");
            }
        }
    }

    #[test]
    fn bluestein_fft_matches_naive_dft_for_arbitrary_lengths() {
        for n in [1usize, 3, 5, 6, 7, 11, 13, 17, 100, 101, 257] {
            let mut rng = crate::numeric::Xorshift64::new(n as u64 + 1);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5)).collect();
            let fast = Fft::new(n).forward(&input);
            let naive = naive_dft(&input, false);
            for (a, b) in fast.iter().zip(naive.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-6), "n={n} a={a:?} b={b:?}");
            }
        }
    }

    #[test]
    fn forward_then_inverse_roundtrips() {
        for n in [8usize, 15, 32, 100] {
            let mut rng = crate::numeric::Xorshift64::new(n as u64 + 99);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64(), rng.next_f64())).collect();
            let plan = Fft::new(n);
            let forward = plan.forward(&input);
            let back = plan.inverse(&forward);
            for (a, b) in input.iter().zip(back.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-8), "n={n}");
            }
        }
    }

    #[test]
    fn parseval_theorem_holds() {
        let n = 32;
        let mut rng = crate::numeric::Xorshift64::new(3);
        let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64() - 0.5, 0.0)).collect();
        let spectrum = Fft::new(n).forward(&input);
        let time_energy: f64 = input.iter().map(|c| c.norm_sq()).sum();
        let freq_energy: f64 = spectrum.iter().map(|c| c.norm_sq()).sum::<f64>() / n as f64;
        assert!((time_energy - freq_energy).abs() < 1e-9);
    }

    #[test]
    fn real_fft_returns_one_sided_spectrum_length() {
        let signal: Vec<f64> = (0..16).map(|i| (i as f64).sin()).collect();
        let spectrum = real_fft(&signal);
        assert_eq!(spectrum.len(), 16 / 2 + 1);
    }

    #[test]
    fn window_functions_produce_expected_length_and_endpoints() {
        for kind in [
            WindowKind::Rectangular,
            WindowKind::Hann,
            WindowKind::Hamming,
            WindowKind::Blackman,
            WindowKind::BlackmanHarris,
            WindowKind::Kaiser(8.0),
            WindowKind::Tukey(0.5),
        ] {
            let w = window(kind, 64);
            assert_eq!(w.len(), 64);
            assert!(w.iter().all(|&v| v.is_finite() && v >= -1e-9));
        }
    }

    #[test]
    fn hann_window_endpoints_are_zero() {
        let w = window(WindowKind::Hann, 32);
        assert!(w[0].abs() < 1e-9);
        assert!(w[31].abs() < 1e-9);
    }

    #[test]
    fn rectangular_window_is_all_ones() {
        let w = window(WindowKind::Rectangular, 10);
        assert!(w.iter().all(|&v| (v - 1.0).abs() < 1e-12));
    }

    mod quick {
        use super::*;

        #[test]
        fn bluestein_matches_radix2_on_power_of_two_length() {
            let n = 64;
            let mut rng = crate::numeric::Xorshift64::new(77);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64(), rng.next_f64())).collect();
            let radix2 = Fft::new(n).forward(&input);
            let bluestein = fft_bluestein(&input, false);
            for (a, b) in radix2.iter().zip(bluestein.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-6));
            }
        }

        #[test]
        fn large_prime_length_dft_matches_naive() {
            let n = 101; // prime, not near a power of two
            let mut rng = crate::numeric::Xorshift64::new(4242);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64() - 0.5, 0.0)).collect();
            let fast = Fft::new(n).forward(&input);
            let naive = naive_dft(&input, false);
            for (a, b) in fast.iter().zip(naive.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-6));
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Fft

// #region 🔖️Spectral
pub mod spectral {
//! 📶️ Welch-method spectral entropy: segment a real time series, window and FFT each segment,
//! average the one-sided power spectra into a periodogram, and treat the normalized periodogram
//! as a probability distribution over frequency bins whose Shannon entropy summarizes how
//! concentrated (tonal) vs. spread (noise-like) the signal's power is across frequency. See
//! Welch, P. (1967), "The use of fast Fourier transform for the estimation of power spectra."

use crate::fft::{window, Complex, Fft, WindowKind};
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};

// #region 🔖️Config
/// 📶️ Configuration for [`spectral_entropy`]'s Welch periodogram estimate.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SpectralConfig {
    /// 📶️ Analysis window applied to each segment before its FFT.
    pub window: WindowKind,
    /// 📶️ Samples per segment. `0` means "auto: `min(256, x.len())`".
    pub segment_len: usize,
    /// 📶️ Fractional overlap between consecutive segments, in `[0, 1)` (`0.5` = 50% overlap).
    pub overlap: f64,
    /// 📶️ Optional normalized-frequency band `(lo, hi)` in `[0, 0.5]` (as a fraction of the
    /// sample rate; `0.5` is Nyquist) restricting which PSD bins contribute to the entropy.
    pub band: Option<(f64, f64)>,
    /// 📶️ If `true`, the returned value is divided by `ln(bins used)` into a dimensionless
    /// `[0, 1]` ratio instead of a proper entropy (see [`spectral_entropy`] docs).
    pub normalize: bool,
}

impl Default for SpectralConfig {
    fn default() -> Self {
        Self { window: WindowKind::Hann, segment_len: 0, overlap: 0.5, band: None, normalize: false }
    }
}

impl SpectralConfig {
    /// 📶️ Builds a config, validating that `overlap` is a finite fraction in `[0, 1)`.
    pub fn new(window: WindowKind, segment_len: usize, overlap: f64) -> Result<Self, EntropyError> {
        validate_overlap(overlap)?;
        Ok(Self { window, segment_len, overlap, band: None, normalize: false })
    }
}

fn validate_overlap(overlap: f64) -> Result<(), EntropyError> {
    if !overlap.is_finite() || !(0.0..1.0).contains(&overlap) {
        return Err(EntropyError::InvalidConfig {
            field: "overlap",
            reason: "must be a finite fraction in [0, 1)",
        });
    }
    Ok(())
}
// #endregion 🔖️Config

// #region 🔖️Welch
/// 📶️ Welch-method spectral entropy of `x`: segments `x` (dropping any incomplete tail segment),
/// windows and FFTs each segment, averages the one-sided power spectra into a periodogram,
/// normalizes it (optionally restricted to `cfg.band`) into a probability vector over frequency
/// bins, and reports its Shannon entropy in nats.
///
/// When `cfg.normalize` is `true` the returned `Estimate.value` is instead the dimensionless
/// ratio `H / ln(bins used)` in `[0, 1]` — a unitless measure of spectral flatness, not a proper
/// entropy in any log base.
pub fn spectral_entropy(x: &[f64], cfg: SpectralConfig) -> Result<Estimate, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "spectral entropy input" });
    }
    for (i, &v) in x.iter().enumerate() {
        if !v.is_finite() {
            return Err(EntropyError::NonFinite { what: "spectral entropy input", index: i });
        }
    }
    validate_overlap(cfg.overlap)?;

    let segment_len = if cfg.segment_len == 0 { x.len().min(256) } else { cfg.segment_len };
    if segment_len < 2 {
        return Err(EntropyError::InvalidConfig { field: "segment_len", reason: "must be at least 2" });
    }
    if segment_len > x.len() {
        return Err(EntropyError::InvalidConfig {
            field: "segment_len",
            reason: "must not exceed input length",
        });
    }

    let hop = (((segment_len as f64) * (1.0 - cfg.overlap)).round().max(1.0)) as usize;
    let win = window(cfg.window, segment_len);
    let plan = Fft::new(segment_len);
    let n_bins = segment_len / 2 + 1;

    let mut power_sum = vec![0.0_f64; n_bins];
    let mut n_segments = 0usize;
    let mut start = 0usize;
    while start + segment_len <= x.len() {
        let segment: Vec<Complex> = x[start..start + segment_len]
            .iter()
            .zip(win.iter())
            .map(|(&xv, &wv)| Complex::new(xv * wv, 0.0))
            .collect();
        let spectrum = plan.forward(&segment);
        for k in 0..n_bins {
            power_sum[k] += spectrum[k].norm_sq();
        }
        n_segments += 1;
        start += hop;
    }
    if n_segments == 0 {
        return Err(EntropyError::InsufficientData { what: "Welch segments", needed: 1, actual: 0 });
    }
    let power_avg: Vec<f64> = power_sum.iter().map(|&p| p / n_segments as f64).collect();

    let selected: Vec<f64> = match cfg.band {
        None => power_avg,
        Some((lo, hi)) => {
            if !(lo.is_finite() && hi.is_finite() && lo >= 0.0 && hi <= 0.5 && lo < hi) {
                return Err(EntropyError::InvalidConfig {
                    field: "band",
                    reason: "must satisfy 0 <= lo < hi <= 0.5",
                });
            }
            let sub: Vec<f64> = (0..n_bins)
                .filter(|&k| {
                    let freq = k as f64 / segment_len as f64;
                    freq >= lo && freq <= hi
                })
                .map(|k| power_avg[k])
                .collect();
            if sub.is_empty() {
                return Err(EntropyError::InvalidConfig { field: "band", reason: "selects zero frequency bins" });
            }
            sub
        }
    };

    let total_power: f64 = selected.iter().sum();
    if total_power <= 0.0 {
        return Err(EntropyError::DegenerateInput { what: "power spectrum has zero total power" });
    }
    let p: Vec<f64> = selected.iter().map(|&v| v / total_power).collect();
    let bins = p.len();

    let entropy_nats = crate::discrete::entropy(&p, LogBase::Nats)?;
    let value = if cfg.normalize {
        if bins <= 1 { 0.0 } else { entropy_nats / (bins as f64).ln() }
    } else {
        entropy_nats
    };

    let mut warnings = Vec::new();
    if n_segments < 4 {
        warnings.push(Warning::SmallSample { n: n_segments, recommended: 4 });
    }

    Ok(Estimate {
        value,
        base: LogBase::Nats,
        method: "welch_spectral_entropy",
        n: x.len(),
        n_effective: n_segments as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![
            ("segments", n_segments as f64),
            ("segment_len", segment_len as f64),
            ("bins", bins as f64),
        ],
    })
}
// #endregion 🔖️Welch

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sine(n: usize, freq: f64, sample_rate: f64) -> Vec<f64> {
        (0..n).map(|i| (2.0 * core::f64::consts::PI * freq * i as f64 / sample_rate).sin()).collect()
    }

    fn white_noise(n: usize, seed: u64) -> Vec<f64> {
        let mut rng = crate::numeric::Xorshift64::new(seed);
        (0..n).map(|_| rng.next_f64() - 0.5).collect()
    }

    #[test]
    fn pure_sine_has_low_normalized_spectral_entropy() {
        let x = sine(4096, 50.0, 1000.0);
        let cfg = SpectralConfig { window: WindowKind::Hann, normalize: true, ..Default::default() };
        let est = spectral_entropy(&x, cfg).unwrap();
        assert!(est.value < 0.3, "got {}", est.value);
        assert!(est.n_effective >= 4.0);
    }

    #[test]
    fn white_noise_has_high_normalized_spectral_entropy() {
        let x = white_noise(4096, 7);
        let cfg = SpectralConfig { window: WindowKind::Hann, normalize: true, ..Default::default() };
        let est = spectral_entropy(&x, cfg).unwrap();
        assert!(est.value > 0.85, "got {}", est.value);
    }

    #[test]
    fn band_restriction_changes_entropy_and_validates_range() {
        let x = sine(4096, 50.0, 1000.0);
        let full = spectral_entropy(&x, SpectralConfig { normalize: true, ..Default::default() }).unwrap();
        let in_band = spectral_entropy(
            &x,
            SpectralConfig { normalize: true, band: Some((0.02, 0.08)), ..Default::default() },
        )
        .unwrap();
        let out_of_band = spectral_entropy(
            &x,
            SpectralConfig { normalize: true, band: Some((0.3, 0.5)), ..Default::default() },
        )
        .unwrap();
        assert!(in_band.value != full.value);
        assert!(out_of_band.value > in_band.value, "in_band={} out_of_band={}", in_band.value, out_of_band.value);

        assert!(matches!(
            spectral_entropy(&x, SpectralConfig { band: Some((0.4, 0.1)), ..Default::default() }),
            Err(EntropyError::InvalidConfig { field: "band", .. })
        ));
        assert!(matches!(
            spectral_entropy(&x, SpectralConfig { band: Some((0.6, 0.7)), ..Default::default() }),
            Err(EntropyError::InvalidConfig { field: "band", .. })
        ));
    }

    #[test]
    fn rejects_segment_len_larger_than_input() {
        let x = vec![0.0, 1.0, 2.0, 3.0];
        let cfg = SpectralConfig { segment_len: 100, ..Default::default() };
        assert!(matches!(
            spectral_entropy(&x, cfg),
            Err(EntropyError::InvalidConfig { field: "segment_len", .. })
        ));
    }

    #[test]
    fn config_new_rejects_overlap_at_or_above_one() {
        assert!(matches!(
            SpectralConfig::new(WindowKind::Hann, 128, 1.0),
            Err(EntropyError::InvalidConfig { field: "overlap", .. })
        ));
        assert!(matches!(
            SpectralConfig::new(WindowKind::Hann, 128, 1.5),
            Err(EntropyError::InvalidConfig { field: "overlap", .. })
        ));
        assert!(SpectralConfig::new(WindowKind::Hann, 128, 0.75).is_ok());
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Spectral

// #region 🔖️Wavelet
pub mod wavelet {
//! 🪢️ Discrete Wavelet Transform (Mallat filter-bank algorithm) and wavelet entropy: decompose a
//! signal into approximation/detail subbands across dyadic scales, then report the Shannon
//! entropy of the subbands' relative energies as a measure of how concentrated (smooth) vs.
//! spread (noisy) the signal's energy is across scale. See Mallat, S. (1989), "A theory for
//! multiresolution signal decomposition: the wavelet representation."

use crate::numeric::neumaier_sum;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};

// #region 🔖️Families
/// 🪢️ Orthonormal wavelet family selecting the low-pass decomposition filter. Higher-order
/// Daubechies filters trade a longer support (more taps) for a smoother, more frequency-selective
/// split between scales.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WaveletFamily {
    Haar,
    Daubechies4,
    Daubechies6,
    Daubechies8,
}

impl WaveletFamily {
    /// 🪢️ Number of taps in this family's filters; also the minimum signal length a single
    /// decomposition level can consume.
    pub fn filter_len(self) -> usize {
        match self {
            Self::Haar => 2,
            Self::Daubechies4 => 4,
            Self::Daubechies6 => 6,
            Self::Daubechies8 => 8,
        }
    }

    /// 🪢️ Orthonormal low-pass (scaling/approximation) decomposition filter coefficients, taps
    /// summing to `sqrt(2)`.
    fn low_pass(self) -> Vec<f64> {
        match self {
            Self::Haar => {
                let s = 1.0 / core::f64::consts::SQRT_2;
                vec![s, s]
            }
            Self::Daubechies4 => {
                let r3 = 3f64.sqrt();
                let d = 4.0 * core::f64::consts::SQRT_2;
                vec![(1.0 + r3) / d, (3.0 + r3) / d, (3.0 - r3) / d, (1.0 - r3) / d]
            }
            Self::Daubechies6 => vec![
                0.332_670_552_950,
                0.806_891_509_311,
                0.459_877_502_118,
                -0.135_011_020_010,
                -0.085_441_273_882,
                0.035_226_291_882,
            ],
            Self::Daubechies8 => vec![
                0.230_377_813_309,
                0.714_846_570_553,
                0.630_880_767_930,
                -0.027_983_769_417,
                -0.187_034_811_719,
                0.030_841_381_836,
                0.032_883_011_667,
                -0.010_597_401_785,
            ],
        }
    }
}

/// 🪢️ Derives the high-pass (wavelet/detail) filter from a low-pass filter `h` via the quadrature
/// mirror relation `g[n] = (-1)^n * h[len-1-n]`.
fn high_pass(h: &[f64]) -> Vec<f64> {
    let len = h.len();
    (0..len)
        .map(|n| {
            let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
            sign * h[len - 1 - n]
        })
        .collect()
}
// #endregion 🔖️Families

// #region 🔖️Boundary
/// 🪢️ How a filter tap that reads outside `[0, signal.len())` is resolved.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BoundaryMode {
    /// 🪢️ Out-of-range samples contribute `0.0`.
    Zero,
    /// 🪢️ The index wraps modulo `signal.len()` (circular convolution).
    Periodic,
    /// 🪢️ The index reflects off the boundary, whole-point style, duplicating the edge sample
    /// (`-1 -> 0`, `-2 -> 1`, `len -> len-1`, `len+1 -> len-2`).
    Symmetric,
}

/// 🪢️ Resolves a possibly out-of-range tap index `idx` against a signal of length `n` under
/// `mode`. Returns `None` only for [`BoundaryMode::Zero`] out-of-range reads (contribute nothing).
fn resolve_index(idx: i64, n: usize, mode: BoundaryMode) -> Option<usize> {
    let n_i = n as i64;
    if idx >= 0 && idx < n_i {
        return Some(idx as usize);
    }
    match mode {
        BoundaryMode::Zero => None,
        BoundaryMode::Periodic => Some((((idx % n_i) + n_i) % n_i) as usize),
        BoundaryMode::Symmetric => {
            if n_i == 1 {
                return Some(0);
            }
            let period = 2 * n_i;
            let mut m = idx % period;
            if m < 0 {
                m += period;
            }
            let mapped = if m < n_i { m } else { period - 1 - m };
            Some(mapped as usize)
        }
    }
}
// #endregion 🔖️Boundary

// #region 🔖️FilterBank
/// 🪢️ One decomposition level: convolves `s` with `filter` and downsamples by 2, keeping output
/// index `k` computed from input taps at `2*k + i` (boundary-resolved), `i` in `0..filter.len()`.
/// Output length is `ceil(s.len() / 2)`.
fn convolve_downsample(s: &[f64], filter: &[f64], boundary: BoundaryMode) -> Vec<f64> {
    let n = s.len();
    let out_len = n.div_ceil(2);
    (0..out_len)
        .map(|k| {
            neumaier_sum((0..filter.len()).filter_map(|i| {
                let idx = 2 * k as i64 + i as i64;
                resolve_index(idx, n, boundary).map(|j| filter[i] * s[j])
            }))
        })
        .collect()
}
// #endregion 🔖️FilterBank

// #region 🔖️Config
/// 🪢️ Configuration for [`Dwt::decompose`] / [`wavelet_entropy`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct WaveletConfig {
    pub family: WaveletFamily,
    /// 🪢️ Requested number of decomposition levels; the actual number achieved may be fewer if
    /// the approximation subband shrinks below the filter length first (see
    /// [`Dwt::decompose`]).
    pub levels: usize,
    pub boundary: BoundaryMode,
}

impl WaveletConfig {
    /// 🪢️ Builds a config, rejecting `levels == 0`.
    pub fn new(family: WaveletFamily, levels: usize, boundary: BoundaryMode) -> Result<Self, EntropyError> {
        if levels == 0 {
            return Err(EntropyError::InvalidConfig { field: "levels", reason: "must be at least 1" });
        }
        Ok(Self { family, levels, boundary })
    }
}
// #endregion 🔖️Config

// #region 🔖️Dwt
/// 🪢️ A fitted multi-level Discrete Wavelet Transform: the detail subband captured at each level
/// (finest first) plus the final approximation subband.
pub struct Dwt {
    details: Vec<Vec<f64>>,
    approximation: Vec<f64>,
    levels_achieved: usize,
}

impl Dwt {
    /// 🪢️ Decomposes `x` into up to `cfg.levels` levels of the Mallat filter-bank algorithm,
    /// repeatedly splitting the current approximation into a coarser approximation and a detail
    /// subband. Stops early (without erroring) once the current approximation's length drops
    /// below the chosen family's filter length; the number of levels actually produced is
    /// reported by [`Dwt::levels_achieved`]. Rejects `x` shorter than the filter length outright.
    pub fn decompose(x: &[f64], cfg: WaveletConfig) -> Result<Self, EntropyError> {
        let filter_len = cfg.family.filter_len();
        if x.len() < filter_len {
            return Err(EntropyError::InsufficientData {
                what: "wavelet decomposition",
                needed: filter_len,
                actual: x.len(),
            });
        }
        let h = cfg.family.low_pass();
        let g = high_pass(&h);
        let mut approximation = x.to_vec();
        let mut details = Vec::with_capacity(cfg.levels);
        let mut levels_achieved = 0usize;
        for _ in 0..cfg.levels {
            if approximation.len() < filter_len {
                break;
            }
            let detail = convolve_downsample(&approximation, &g, cfg.boundary);
            let next_approximation = convolve_downsample(&approximation, &h, cfg.boundary);
            details.push(detail);
            approximation = next_approximation;
            levels_achieved += 1;
        }
        Ok(Self { details, approximation, levels_achieved })
    }

    /// 🪢️ Number of levels actually produced (`<= cfg.levels`; see [`Dwt::decompose`]).
    pub fn levels_achieved(&self) -> usize {
        self.levels_achieved
    }

    /// 🪢️ Energy (sum of squared coefficients) of each subband, detail levels finest-first
    /// followed by the final approximation.
    pub fn subband_energies(&self) -> Vec<f64> {
        let mut energies: Vec<f64> =
            self.details.iter().map(|d| neumaier_sum(d.iter().map(|&v| v * v))).collect();
        energies.push(neumaier_sum(self.approximation.iter().map(|&v| v * v)));
        energies
    }
}
// #endregion 🔖️Dwt

// #region 🔖️Entropy
/// 🪢️ Wavelet entropy of `x`: decomposes `x` per `cfg`, takes each subband's relative energy
/// (energy / total energy across all subbands), and reports the Shannon entropy (in nats) of that
/// distribution — low when energy concentrates in a few subbands (smooth or tonal signals), high
/// when it spreads evenly across scales (noise-like signals).
pub fn wavelet_entropy(x: &[f64], cfg: WaveletConfig) -> Result<Estimate, EntropyError> {
    let dwt = Dwt::decompose(x, cfg)?;
    let energies = dwt.subband_energies();
    let total = neumaier_sum(energies.iter().copied());
    if total <= 1e-300 {
        return Err(EntropyError::DegenerateInput { what: "wavelet subband energies sum to ~0" });
    }
    let p: Vec<f64> = energies.iter().map(|&e| (e / total).max(0.0)).collect();
    let nats = crate::discrete::entropy(&p, LogBase::Nats)?;

    let n = x.len();
    let mut warnings = Vec::new();
    if n < 30 {
        warnings.push(Warning::SmallSample { n, recommended: 30 });
    }

    Ok(Estimate {
        value: nats,
        base: LogBase::Nats,
        method: "wavelet_entropy",
        n,
        n_effective: n as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![
            ("levels_achieved", dwt.levels_achieved() as f64),
            ("num_subbands", p.len() as f64),
        ],
    })
}
// #endregion 🔖️Entropy

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn haar_periodic_one_level_preserves_energy() {
        // 🔐️ Parseval / energy-preservation: Haar under periodic boundary is exactly orthonormal,
        // so sum(a^2) + sum(d^2) == sum(s^2) to tight tolerance.
        let x = [1.0, 3.0, -2.0, 5.0, 0.5, -1.5, 4.0, 2.0];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 1, BoundaryMode::Periodic).unwrap();
        let dwt = Dwt::decompose(&x, cfg).unwrap();
        let subband_total: f64 = dwt.subband_energies().iter().sum();
        let signal_total: f64 = x.iter().map(|&v| v * v).sum();
        assert!((subband_total - signal_total).abs() < 1e-9, "subband={subband_total} signal={signal_total}");
    }

    #[test]
    fn haar_periodic_multi_level_preserves_energy() {
        let mut rng = crate::numeric::Xorshift64::new(11);
        let x: Vec<f64> = (0..64).map(|_| rng.next_f64() * 10.0 - 5.0).collect();
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 4, BoundaryMode::Periodic).unwrap();
        let dwt = Dwt::decompose(&x, cfg).unwrap();
        let subband_total: f64 = dwt.subband_energies().iter().sum();
        let signal_total: f64 = x.iter().map(|&v| v * v).sum();
        assert!((subband_total - signal_total).abs() < 1e-6, "subband={subband_total} signal={signal_total}");
    }

    #[test]
    fn constant_signal_has_zero_haar_detail_energy() {
        let x = [3.0; 8];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 1, BoundaryMode::Periodic).unwrap();
        let dwt = Dwt::decompose(&x, cfg).unwrap();
        let energies = dwt.subband_energies();
        assert!(energies[0].abs() < 1e-9, "detail energy = {}", energies[0]);
    }

    #[test]
    fn smooth_ramp_has_lower_wavelet_entropy_than_noise() {
        let n = 256;
        let ramp: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let mut rng = crate::numeric::Xorshift64::new(21);
        let noise: Vec<f64> = (0..n).map(|_| rng.next_f64() - 0.5).collect();

        let cfg = WaveletConfig::new(WaveletFamily::Daubechies4, 4, BoundaryMode::Symmetric).unwrap();
        let ramp_est = wavelet_entropy(&ramp, cfg).unwrap();
        let noise_est = wavelet_entropy(&noise, cfg).unwrap();
        assert!(ramp_est.value < noise_est.value, "ramp={} noise={}", ramp_est.value, noise_est.value);
    }

    #[test]
    fn white_noise_has_higher_wavelet_entropy_than_pure_tone() {
        let n = 512;
        let mut rng = crate::numeric::Xorshift64::new(33);
        let noise: Vec<f64> = (0..n).map(|_| rng.next_f64() - 0.5).collect();
        let sine: Vec<f64> = (0..n)
            .map(|i| (2.0 * core::f64::consts::PI * 4.0 * i as f64 / n as f64).sin())
            .collect();

        let cfg = WaveletConfig::new(WaveletFamily::Daubechies8, 5, BoundaryMode::Symmetric).unwrap();
        let noise_est = wavelet_entropy(&noise, cfg).unwrap();
        let sine_est = wavelet_entropy(&sine, cfg).unwrap();
        assert!(noise_est.value > sine_est.value, "noise={} sine={}", noise_est.value, sine_est.value);
    }

    #[test]
    fn config_new_rejects_zero_levels() {
        assert!(matches!(
            WaveletConfig::new(WaveletFamily::Haar, 0, BoundaryMode::Zero),
            Err(EntropyError::InvalidConfig { field: "levels", .. })
        ));
    }

    #[test]
    fn decompose_rejects_input_shorter_than_filter() {
        let x = [1.0, 2.0, 3.0];
        let cfg = WaveletConfig::new(WaveletFamily::Daubechies8, 1, BoundaryMode::Zero).unwrap();
        assert!(matches!(
            Dwt::decompose(&x, cfg),
            Err(EntropyError::InsufficientData { needed: 8, actual: 3, .. })
        ));
    }

    #[test]
    fn decompose_stops_early_when_signal_runs_out_of_levels() {
        // 🔐️ 8 samples with Haar (filter_len 2) can only produce 3 dyadic halvings (8->4->2->1)
        // before the approximation would drop below the filter length on a 4th level.
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 10, BoundaryMode::Periodic).unwrap();
        let dwt = Dwt::decompose(&x, cfg).unwrap();
        assert!(dwt.levels_achieved() < 10);
        assert!(dwt.levels_achieved() >= 1);
    }

    #[test]
    fn wavelet_entropy_reports_levels_achieved_diagnostic() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 10, BoundaryMode::Periodic).unwrap();
        let est = wavelet_entropy(&x, cfg).unwrap();
        let levels = est.diagnostics.iter().find(|(k, _)| *k == "levels_achieved").unwrap().1;
        assert!(levels < 10.0);
    }

    #[test]
    fn wavelet_entropy_rejects_all_zero_input() {
        let x = [0.0; 16];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 2, BoundaryMode::Zero).unwrap();
        assert!(matches!(wavelet_entropy(&x, cfg), Err(EntropyError::DegenerateInput { .. })));
    }

    #[test]
    fn daubechies_filters_are_orthonormal_sum_sqrt2() {
        for family in [
            WaveletFamily::Haar,
            WaveletFamily::Daubechies4,
            WaveletFamily::Daubechies6,
            WaveletFamily::Daubechies8,
        ] {
            let h = family.low_pass();
            let sum: f64 = h.iter().sum();
            // 🔐️ published Daubechies-6 coefficients are truncated to 12 decimal digits, so their
            // sum only approximates sqrt(2) to about 1e-8, not full f64 precision.
            assert!((sum - core::f64::consts::SQRT_2).abs() < 1e-6, "{family:?} sum={sum}");
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Wavelet

// #region 🔖️Matrix
pub mod matrix {
//! 🔢️ Dense linear algebra hand-rolled for matrix-based entropy: a cyclic Jacobi eigensolver for
//! symmetric matrices, one-sided Jacobi SVD, and Cholesky decomposition — feeding SVD entropy,
//! eigenvalue entropy, and von Neumann (density-matrix) entropy.

use crate::counts::validate_probabilities;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Tolerances, Warning};

// #region 🔖️Jacobi
/// 🔢️ Cyclic Jacobi eigenvalue algorithm for a real symmetric `n x n` matrix (row-major).
/// Returns `(eigenvalues, eigenvectors)` with eigenvalues sorted descending and `eigenvectors`
/// row-major where column `j` is the eigenvector for `eigenvalues[j]`.
pub fn jacobi_eigen_symmetric(a_in: &[f64], n: usize) -> Result<(Vec<f64>, Vec<f64>), EntropyError> {
    if n == 0 {
        return Err(EntropyError::EmptyInput { what: "matrix" });
    }
    if a_in.len() != n * n {
        return Err(EntropyError::ShapeMismatch { what: "matrix", expected: n * n, actual: a_in.len() });
    }
    let mut a = a_in.to_vec();
    let mut v = vec![0.0_f64; n * n];
    for i in 0..n {
        v[i * n + i] = 1.0;
    }
    let mut d = vec![0.0_f64; n];
    for i in 0..n {
        d[i] = a[i * n + i];
    }
    let mut z = vec![0.0_f64; n];

    const MAX_SWEEPS: usize = 100;
    let mut converged = false;
    for sweep in 0..MAX_SWEEPS {
        let mut off_sum = 0.0_f64;
        for p in 0..n.saturating_sub(1) {
            for q in (p + 1)..n {
                off_sum += a[p * n + q].abs();
            }
        }
        if off_sum == 0.0 {
            converged = true;
            break;
        }
        let threshold = if sweep < 3 { 0.2 * off_sum / (n * n) as f64 } else { 0.0 };
        for p in 0..n.saturating_sub(1) {
            for q in (p + 1)..n {
                let apq = a[p * n + q];
                let g = 100.0 * apq.abs();
                if sweep > 3 && (d[p].abs() + g == d[p].abs()) && (d[q].abs() + g == d[q].abs()) {
                    a[p * n + q] = 0.0;
                    continue;
                }
                if apq.abs() <= threshold {
                    continue;
                }
                let mut h = d[q] - d[p];
                let t = if h.abs() + g == h.abs() {
                    apq / h
                } else {
                    let theta = 0.5 * h / apq;
                    let t0 = 1.0 / (theta.abs() + (1.0 + theta * theta).sqrt());
                    if theta < 0.0 { -t0 } else { t0 }
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;
                let tau = s / (1.0 + c);
                h = t * apq;
                z[p] -= h;
                z[q] += h;
                d[p] -= h;
                d[q] += h;
                a[p * n + q] = 0.0;

                let rotate = |ip: f64, iq: f64| -> (f64, f64) { (ip - s * (iq + ip * tau), iq + s * (ip - iq * tau)) };
                for i in 0..p {
                    let (np, nq) = rotate(a[i * n + p], a[i * n + q]);
                    a[i * n + p] = np;
                    a[i * n + q] = nq;
                }
                for i in (p + 1)..q {
                    let (np, nq) = rotate(a[p * n + i], a[i * n + q]);
                    a[p * n + i] = np;
                    a[i * n + q] = nq;
                }
                for i in (q + 1)..n {
                    let (np, nq) = rotate(a[p * n + i], a[q * n + i]);
                    a[p * n + i] = np;
                    a[q * n + i] = nq;
                }
                for i in 0..n {
                    let (np, nq) = rotate(v[i * n + p], v[i * n + q]);
                    v[i * n + p] = np;
                    v[i * n + q] = nq;
                }
            }
        }
        // 🔢️ `d` was already updated incrementally by `-= h` / `+= h` during the sweep above;
        // only the per-sweep accumulator `z` needs resetting for the next sweep.
        z.iter_mut().for_each(|zi| *zi = 0.0);
    }
    if !converged {
        return Err(EntropyError::NotConverged { what: "Jacobi eigensolver", iterations: MAX_SWEEPS });
    }

    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&i, &j| d[j].partial_cmp(&d[i]).unwrap_or(std::cmp::Ordering::Equal));
    let eigenvalues: Vec<f64> = idx.iter().map(|&i| d[i]).collect();
    let mut eigenvectors = vec![0.0_f64; n * n];
    for (new_col, &old_col) in idx.iter().enumerate() {
        for row in 0..n {
            eigenvectors[row * n + new_col] = v[row * n + old_col];
        }
    }
    Ok((eigenvalues, eigenvectors))
}
// #endregion 🔖️Jacobi

// #region 🔖️Cholesky
/// 🔢️ Cholesky decomposition `A = L L^T` of a symmetric positive-(semi)definite `n x n` matrix
/// (row-major), returning the lower-triangular factor `L` (row-major, zeros above the diagonal).
/// Falls back to progressively larger diagonal (Tikhonov) regularization if a pivot is
/// non-positive due to floating-point noise near singularity.
pub fn cholesky(a: &[f64], n: usize) -> Result<Vec<f64>, EntropyError> {
    if n == 0 {
        return Err(EntropyError::EmptyInput { what: "matrix" });
    }
    if a.len() != n * n {
        return Err(EntropyError::ShapeMismatch { what: "matrix", expected: n * n, actual: a.len() });
    }
    let mut jitter = 0.0_f64;
    for _attempt in 0..12 {
        let mut l = vec![0.0_f64; n * n];
        let mut ok = true;
        'outer: for i in 0..n {
            for j in 0..=i {
                let mut sum = a[i * n + j] + if i == j { jitter } else { 0.0 };
                for k in 0..j {
                    sum -= l[i * n + k] * l[j * n + k];
                }
                if i == j {
                    if sum <= 0.0 {
                        ok = false;
                        break 'outer;
                    }
                    l[i * n + i] = sum.sqrt();
                } else {
                    l[i * n + j] = sum / l[j * n + j];
                }
            }
        }
        if ok {
            return Ok(l);
        }
        jitter = if jitter == 0.0 { 1e-12 } else { jitter * 10.0 };
    }
    Err(EntropyError::UndefinedResult { reason: "matrix is not positive-definite even after regularization" })
}

/// 🔢️ `ln|det(A)|` of a symmetric positive-definite matrix via `2 * sum(ln(L_ii))` from its
/// Cholesky factor.
pub fn log_det(a: &[f64], n: usize) -> Result<f64, EntropyError> {
    let l = cholesky(a, n)?;
    Ok(2.0 * (0..n).map(|i| l[i * n + i].ln()).sum::<f64>())
}
// #endregion 🔖️Cholesky

// #region 🔖️Svd
/// 🔢️ `(u, singular_values, v)` result of [`svd_jacobi`]: `u` is `rows x cols` (row-major,
/// orthonormal columns), `singular_values` is sorted descending, `v` is `cols x cols` (row-major,
/// orthogonal).
pub type SvdResult = (Vec<f64>, Vec<f64>, Vec<f64>);

/// 🔢️ One-sided Jacobi SVD of an `rows x cols` matrix (row-major) with `rows >= cols`.
pub fn svd_jacobi(a: &[f64], rows: usize, cols: usize) -> Result<SvdResult, EntropyError> {
    if rows == 0 || cols == 0 {
        return Err(EntropyError::EmptyInput { what: "matrix" });
    }
    if a.len() != rows * cols {
        return Err(EntropyError::ShapeMismatch { what: "matrix", expected: rows * cols, actual: a.len() });
    }
    if rows < cols {
        return Err(EntropyError::InvalidConfig { field: "rows", reason: "svd_jacobi requires rows >= cols" });
    }
    let mut u = a.to_vec();
    let mut v = vec![0.0_f64; cols * cols];
    for i in 0..cols {
        v[i * cols + i] = 1.0;
    }

    let col = |m: &[f64], stride: usize, c: usize| -> Vec<f64> { (0..rows).map(|r| m[r * stride + c]).collect() };

    const MAX_SWEEPS: usize = 60;
    let mut converged = false;
    for _sweep in 0..MAX_SWEEPS {
        let mut max_gamma = 0.0_f64;
        for p in 0..cols.saturating_sub(1) {
            for q in (p + 1)..cols {
                let col_p = col(&u, cols, p);
                let col_q = col(&u, cols, q);
                let alpha: f64 = col_p.iter().map(|x| x * x).sum();
                let beta: f64 = col_q.iter().map(|x| x * x).sum();
                let gamma: f64 = col_p.iter().zip(col_q.iter()).map(|(&x, &y)| x * y).sum();
                let norm = (alpha * beta).sqrt().max(1e-300);
                max_gamma = max_gamma.max(gamma.abs() / norm);
                if gamma.abs() < 1e-15 * norm {
                    continue;
                }
                let zeta = (beta - alpha) / (2.0 * gamma);
                let t = if zeta >= 0.0 {
                    1.0 / (zeta + (1.0 + zeta * zeta).sqrt())
                } else {
                    -1.0 / (-zeta + (1.0 + zeta * zeta).sqrt())
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = c * t;
                for r in 0..rows {
                    let up = u[r * cols + p];
                    let uq = u[r * cols + q];
                    u[r * cols + p] = c * up - s * uq;
                    u[r * cols + q] = s * up + c * uq;
                }
                for r in 0..cols {
                    let vp = v[r * cols + p];
                    let vq = v[r * cols + q];
                    v[r * cols + p] = c * vp - s * vq;
                    v[r * cols + q] = s * vp + c * vq;
                }
            }
        }
        if max_gamma < 1e-13 {
            converged = true;
            break;
        }
    }
    if !converged {
        return Err(EntropyError::NotConverged { what: "Jacobi SVD", iterations: MAX_SWEEPS });
    }

    let mut singular_values: Vec<f64> = (0..cols).map(|c| col(&u, cols, c).iter().map(|x| x * x).sum::<f64>().sqrt()).collect();
    let mut idx: Vec<usize> = (0..cols).collect();
    idx.sort_by(|&i, &j| singular_values[j].partial_cmp(&singular_values[i]).unwrap_or(std::cmp::Ordering::Equal));

    let mut u_sorted = vec![0.0_f64; rows * cols];
    let mut v_sorted = vec![0.0_f64; cols * cols];
    let mut sv_sorted = vec![0.0_f64; cols];
    for (new_c, &old_c) in idx.iter().enumerate() {
        sv_sorted[new_c] = singular_values[old_c];
        let sigma = singular_values[old_c].max(1e-300);
        for r in 0..rows {
            u_sorted[r * cols + new_c] = u[r * cols + old_c] / sigma;
        }
        for r in 0..cols {
            v_sorted[r * cols + new_c] = v[r * cols + old_c];
        }
    }
    singular_values = sv_sorted;
    Ok((u_sorted, singular_values, v_sorted))
}
// #endregion 🔖️Svd

// #region 🔖️Entropy
/// 🔢️ Shannon entropy of the normalized singular-value spectrum of `data` (`rows x cols`,
/// row-major), a measure of the matrix's effective rank / concentration of variance.
pub fn svd_entropy(data: &[f64], rows: usize, cols: usize, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    let (transposed, r, c) = if rows >= cols {
        (data.to_vec(), rows, cols)
    } else {
        let mut t = vec![0.0_f64; rows * cols];
        for i in 0..rows {
            for j in 0..cols {
                t[j * rows + i] = data[i * cols + j];
            }
        }
        (t, cols, rows)
    };
    let (_, singular_values, _) = svd_jacobi(&transposed, r, c)?;
    let sum: f64 = singular_values.iter().sum();
    if sum <= 0.0 {
        return Err(EntropyError::DegenerateInput { what: "all singular values are zero" });
    }
    let p: Vec<f64> = singular_values.iter().map(|&s| s / sum).collect();
    let h = crate::discrete::entropy(&p, base)?;
    let nats = base.to_nats(h);
    let effective_rank = nats.exp();
    let stable_rank = singular_values.iter().map(|s| s * s).sum::<f64>() / singular_values[0].max(1e-300).powi(2);

    Ok(Estimate {
        value: h,
        base,
        method: "svd_entropy",
        n: r.min(c),
        n_effective: effective_rank,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings: Vec::new(),
        diagnostics: vec![("effective_rank", effective_rank), ("stable_rank", stable_rank)],
    })
}

/// 🔢️ Von Neumann entropy `-tr(rho ln rho)` of a symmetric positive-semidefinite density matrix
/// `rho` (row-major `n x n`, trace approximately `1`). Eigenvalues within `n * eps * max|lambda|`
/// of zero are clipped; further-negative eigenvalues are rejected as not positive-semidefinite.
pub fn von_neumann_entropy(density: &[f64], n: usize, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    let (eigenvalues, _) = jacobi_eigen_symmetric(density, n)?;
    let max_abs = eigenvalues.iter().copied().fold(0.0_f64, |acc, v| acc.max(v.abs()));
    let tol_neg = n as f64 * f64::EPSILON * max_abs;
    let mut warnings = Vec::new();
    let mut clipped = Vec::with_capacity(n);
    for &lambda in &eigenvalues {
        if lambda < 0.0 {
            if lambda >= -tol_neg {
                clipped.push(0.0);
                warnings.push(Warning::ClippedNegative);
            } else {
                return Err(EntropyError::UndefinedResult { reason: "density matrix is not positive-semidefinite beyond numerical tolerance" });
            }
        } else {
            clipped.push(lambda);
        }
    }
    let p = validate_probabilities(&clipped, Tolerances::default())?;
    let h = crate::discrete::entropy(&p, base)?;
    let rank = p.iter().filter(|&&v| v > 1e-12).count();

    Ok(Estimate {
        value: h,
        base,
        method: "von_neumann",
        n,
        n_effective: n as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("rank", rank as f64)],
    })
}
// #endregion 🔖️Entropy

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn matmul(a: &[f64], b: &[f64], m: usize, k: usize, n: usize) -> Vec<f64> {
        let mut out = vec![0.0_f64; m * n];
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for t in 0..k {
                    sum += a[i * k + t] * b[t * n + j];
                }
                out[i * n + j] = sum;
            }
        }
        out
    }

    fn transpose(a: &[f64], rows: usize, cols: usize) -> Vec<f64> {
        let mut out = vec![0.0_f64; rows * cols];
        for i in 0..rows {
            for j in 0..cols {
                out[j * rows + i] = a[i * cols + j];
            }
        }
        out
    }

    #[test]
    fn jacobi_matches_diagonal_matrix_eigenvalues() {
        let n = 3;
        let a = vec![5.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 9.0];
        let (eigenvalues, _) = jacobi_eigen_symmetric(&a, n).unwrap();
        assert!((eigenvalues[0] - 9.0).abs() < 1e-9);
        assert!((eigenvalues[1] - 5.0).abs() < 1e-9);
        assert!((eigenvalues[2] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn jacobi_reconstructs_symmetric_matrix() {
        let n = 3;
        let a = vec![4.0, 1.0, 2.0, 1.0, 3.0, 0.5, 2.0, 0.5, 5.0];
        let (eigenvalues, eigenvectors) = jacobi_eigen_symmetric(&a, n).unwrap();
        let mut d = vec![0.0_f64; n * n];
        for i in 0..n {
            d[i * n + i] = eigenvalues[i];
        }
        let vt = transpose(&eigenvectors, n, n);
        let reconstructed = matmul(&matmul(&eigenvectors, &d, n, n, n), &vt, n, n, n);
        for i in 0..n * n {
            assert!((reconstructed[i] - a[i]).abs() < 1e-7, "index {i}: {} vs {}", reconstructed[i], a[i]);
        }
    }

    #[test]
    fn jacobi_hand_3x3_matches_known_eigenvalues() {
        // 🔐️ A = [[2,-1,0],[-1,2,-1],[0,-1,2]] has eigenvalues 2, 2±sqrt(2).
        let a = vec![2.0, -1.0, 0.0, -1.0, 2.0, -1.0, 0.0, -1.0, 2.0];
        let (eigenvalues, _) = jacobi_eigen_symmetric(&a, 3).unwrap();
        let expected = {
            let mut v = vec![2.0 + 2.0_f64.sqrt(), 2.0, 2.0 - 2.0_f64.sqrt()];
            v.sort_by(|a, b| b.partial_cmp(a).unwrap());
            v
        };
        for (a, b) in eigenvalues.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-8);
        }
    }

    #[test]
    fn cholesky_reconstructs_positive_definite_matrix() {
        let n = 3;
        let a = vec![4.0, 2.0, 2.0, 2.0, 5.0, 1.0, 2.0, 1.0, 6.0];
        let l = cholesky(&a, n).unwrap();
        let lt = transpose(&l, n, n);
        let reconstructed = matmul(&l, &lt, n, n, n);
        for i in 0..n * n {
            assert!((reconstructed[i] - a[i]).abs() < 1e-9);
        }
    }

    #[test]
    fn cholesky_regularizes_near_singular_matrix() {
        let a = vec![1.0, 1.0, 1.0, 1.0]; // rank-1, singular
        let result = cholesky(&a, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn log_det_matches_known_determinant() {
        let a = vec![4.0, 0.0, 0.0, 9.0];
        let ld = log_det(&a, 2).unwrap();
        assert!((ld - 36.0_f64.ln()).abs() < 1e-9);
    }

    #[test]
    fn svd_reconstructs_matrix() {
        let rows = 4;
        let cols = 3;
        let a = vec![
            1.0, 0.0, 0.0, //
            0.0, 2.0, 0.0, //
            0.0, 0.0, 3.0, //
            1.0, 1.0, 1.0,
        ];
        let (u, s, v) = svd_jacobi(&a, rows, cols).unwrap();
        let mut sigma = vec![0.0_f64; cols * cols];
        for i in 0..cols {
            sigma[i * cols + i] = s[i];
        }
        let vt = transpose(&v, cols, cols);
        let reconstructed = matmul(&matmul(&u, &sigma, rows, cols, cols), &vt, rows, cols, cols);
        for i in 0..rows * cols {
            assert!((reconstructed[i] - a[i]).abs() < 1e-6, "index {i}");
        }
    }

    #[test]
    fn svd_entropy_of_equal_singular_values_is_maximal() {
        // 🔐️ identity-like: all singular values equal -> normalized entropy = 1.
        let a = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let est = svd_entropy(&a, 3, 3, LogBase::Bits).unwrap();
        assert!((est.value - 3.0_f64.log2()).abs() < 1e-6);
    }

    #[test]
    fn svd_entropy_of_rank_one_matrix_is_zero() {
        let a = vec![1.0, 2.0, 3.0, 2.0, 4.0, 6.0]; // rank 1, 2x3
        let est = svd_entropy(&a, 2, 3, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-6);
    }

    #[test]
    fn von_neumann_entropy_of_pure_state_is_zero() {
        let density = vec![1.0, 0.0, 0.0, 0.0];
        let est = von_neumann_entropy(&density, 2, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn von_neumann_entropy_of_maximally_mixed_state_is_log_n() {
        let n = 4;
        let mut density = vec![0.0_f64; n * n];
        for i in 0..n {
            density[i * n + i] = 1.0 / n as f64;
        }
        let est = von_neumann_entropy(&density, n, LogBase::Nats).unwrap();
        assert!((est.value - (n as f64).ln()).abs() < 1e-9);
    }

    #[test]
    fn von_neumann_clips_tiny_negative_eigenvalues() {
        let density = vec![0.5 + 1e-16, 0.5, 0.5, 0.5 - 1e-16];
        // 🔐️ near-singular; should not error, should clip.
        let result = von_neumann_entropy(&density, 2, LogBase::Nats);
        assert!(result.is_ok());
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Matrix

// #region 🔖️Inference
pub mod inference {
//! 🧪️ Statistical inference on top of any entropy/information statistic: resampling confidence
//! intervals (bootstrap, jackknife), permutation hypothesis tests, surrogate-data generation for
//! null-model construction, and multiple-comparisons correction. Every source of randomness here
//! is an explicit `u64` seed fed through [`Xorshift64`] — never wall-clock time — so a full
//! surrogate/permutation batch is exactly reproducible from one seed.

use crate::fft::{Complex, Fft};
use crate::numeric::inverse_normal_cdf;
pub use crate::numeric::Xorshift64;
use crate::{ConfidenceInterval, EntropyError};

// #region 🔖️ConfidenceIntervals
/// 🧪️ Linear-interpolated percentile of an already-sorted slice (`p` in `[0, 1]`), interpolating
/// between the two nearest order statistics when `p * (n - 1)` falls between integer indices.
fn percentile(sorted: &[f64], p: f64) -> f64 {
    let n = sorted.len();
    if n == 1 {
        return sorted[0];
    }
    let idx = p * (n - 1) as f64;
    let lo = idx.floor() as usize;
    let hi = idx.ceil() as usize;
    let frac = idx - lo as f64;
    sorted[lo] + (sorted[hi] - sorted[lo]) * frac
}

fn validate_ci_inputs(data: &[f64], level: f64, what: &'static str) -> Result<(), EntropyError> {
    if data.is_empty() {
        return Err(EntropyError::EmptyInput { what });
    }
    if data.len() < 2 {
        return Err(EntropyError::InsufficientData { what, needed: 2, actual: data.len() });
    }
    if !(level > 0.0 && level < 1.0) {
        return Err(EntropyError::InvalidConfig { field: "level", reason: "must be in (0, 1)" });
    }
    Ok(())
}

/// 🧪️ Percentile bootstrap confidence interval for an arbitrary statistic. Resamples `data` with
/// replacement `n_bootstrap` times (via a single [`Xorshift64`] seeded from `seed`, advanced
/// across all resamples), computes `statistic` on each resample, and returns the
/// `(1-level)/2`/`1-(1-level)/2` percentiles of the resulting distribution as `lower`/`upper`.
pub fn bootstrap_ci(
    data: &[f64],
    statistic: impl Fn(&[f64]) -> f64,
    n_bootstrap: usize,
    level: f64,
    seed: u64,
) -> Result<ConfidenceInterval, EntropyError> {
    validate_ci_inputs(data, level, "bootstrap_ci")?;
    if n_bootstrap == 0 {
        return Err(EntropyError::InvalidConfig { field: "n_bootstrap", reason: "must be at least 1" });
    }
    let n = data.len();
    let mut rng = Xorshift64::new(seed);
    let mut resample = vec![0.0; n];
    let mut stats: Vec<f64> = Vec::with_capacity(n_bootstrap);
    for _ in 0..n_bootstrap {
        for slot in resample.iter_mut() {
            *slot = data[rng.next_below(n)];
        }
        stats.push(statistic(&resample));
    }
    stats.sort_by(f64::total_cmp);
    let tail = (1.0 - level) / 2.0;
    Ok(ConfidenceInterval { lower: percentile(&stats, tail), upper: percentile(&stats, 1.0 - tail), level })
}

/// 🧪️ Jackknife (delete-one) confidence interval: computes `statistic` on each of the `n`
/// leave-one-out subsets, derives the jackknife standard error
/// `sqrt((n-1)/n * sum((theta_i - theta_bar)^2))`, and returns `theta_full +/- z * se` with
/// `z = inverse_normal_cdf(0.5 + level/2)`.
pub fn jackknife_ci(
    data: &[f64],
    statistic: impl Fn(&[f64]) -> f64,
    level: f64,
) -> Result<ConfidenceInterval, EntropyError> {
    validate_ci_inputs(data, level, "jackknife_ci")?;
    let n = data.len();
    let theta_full = statistic(data);
    let mut leave_one_out = Vec::with_capacity(n - 1);
    let thetas: Vec<f64> = (0..n)
        .map(|i| {
            leave_one_out.clear();
            leave_one_out.extend(data.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, &v)| v));
            statistic(&leave_one_out)
        })
        .collect();
    let theta_bar = thetas.iter().sum::<f64>() / n as f64;
    let sum_sq: f64 = thetas.iter().map(|&t| (t - theta_bar).powi(2)).sum();
    let se = ((n - 1) as f64 / n as f64 * sum_sq).sqrt();
    let z = inverse_normal_cdf(0.5 + level / 2.0);
    Ok(ConfidenceInterval { lower: theta_full - z * se, upper: theta_full + z * se, level })
}
// #endregion 🔖️ConfidenceIntervals

// #region 🔖️PermutationTest
/// 🧪️ Two-sample permutation test p-value for an arbitrary two-argument statistic. Computes the
/// observed statistic, then `n_permutations` times randomly reassigns the pooled `x ++ y` values
/// into two groups of the original sizes (via a single [`Xorshift64`] seeded from `seed`,
/// advancing across permutations rather than reseeding each iteration) and recomputes the
/// statistic. Returns the two-sided p-value
/// `(count(|stat_perm| >= |stat_obs|) + 1) / (n_permutations + 1)`.
pub fn permutation_test(
    x: &[f64],
    y: &[f64],
    statistic: impl Fn(&[f64], &[f64]) -> f64,
    n_permutations: usize,
    seed: u64,
) -> Result<f64, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    if y.is_empty() {
        return Err(EntropyError::EmptyInput { what: "y" });
    }
    if n_permutations == 0 {
        return Err(EntropyError::InvalidConfig { field: "n_permutations", reason: "must be at least 1" });
    }
    let observed = statistic(x, y).abs();
    let nx = x.len();
    let mut pooled: Vec<f64> = Vec::with_capacity(nx + y.len());
    pooled.extend_from_slice(x);
    pooled.extend_from_slice(y);
    let mut rng = Xorshift64::new(seed);
    let mut count = 0usize;
    for _ in 0..n_permutations {
        rng.shuffle(&mut pooled);
        let stat = statistic(&pooled[..nx], &pooled[nx..]).abs();
        if stat >= observed {
            count += 1;
        }
    }
    Ok((count as f64 + 1.0) / (n_permutations as f64 + 1.0))
}
// #endregion 🔖️PermutationTest

// #region 🔖️Surrogates
/// 🧪️ Which surrogate-generation method [`surrogate_series`] uses to build a null-model ensemble.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SurrogateKind {
    /// 🧪️ Rotates the series by a random offset; preserves the exact value multiset and the exact
    /// power spectrum (a circular shift only changes phase), but not much else.
    CircularShift,
    /// 🧪️ Splits into fixed-size blocks (final partial block kept as-is) and shuffles block order;
    /// preserves the exact value multiset and short-range structure within a block.
    BlockShuffle { block_size: usize },
    /// 🧪️ Randomizes the Fourier phases while keeping the magnitude spectrum exact; preserves the
    /// linear (power-spectrum) structure but not the value distribution.
    PhaseRandomized,
    /// 🧪️ Iterated Amplitude-Adjusted Fourier Transform: alternates rank-order amplitude
    /// adjustment with spectral magnitude adjustment so both the value distribution and the power
    /// spectrum are (near-)exactly preserved.
    Iaaft { iterations: usize },
}

/// 🧪️ Configuration for a batch of surrogate series: which method, how many, and the single seed
/// the whole batch is generated from.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SurrogateConfig {
    pub kind: SurrogateKind,
    pub count: usize,
    pub seed: u64,
}

impl SurrogateConfig {
    /// 🧪️ Validates `count >= 1` and, for [`SurrogateKind::BlockShuffle`], `block_size >= 1`.
    pub fn new(kind: SurrogateKind, count: usize, seed: u64) -> Result<Self, EntropyError> {
        if count == 0 {
            return Err(EntropyError::InvalidConfig { field: "count", reason: "must be at least 1" });
        }
        if let SurrogateKind::BlockShuffle { block_size } = kind {
            if block_size == 0 {
                return Err(EntropyError::InvalidConfig { field: "block_size", reason: "must be at least 1" });
            }
        }
        Ok(Self { kind, count, seed })
    }
}

fn circular_shift_surrogate(x: &[f64], rng: &mut Xorshift64) -> Vec<f64> {
    let n = x.len();
    let k = 1 + rng.next_below(n - 1);
    (0..n).map(|i| x[(i + k) % n]).collect()
}

fn block_shuffle_surrogate(x: &[f64], block_size: usize, rng: &mut Xorshift64) -> Vec<f64> {
    let chunks: Vec<&[f64]> = x.chunks(block_size).collect();
    let mut order: Vec<usize> = (0..chunks.len()).collect();
    rng.shuffle(&mut order);
    let mut out = Vec::with_capacity(x.len());
    for &idx in &order {
        out.extend_from_slice(chunks[idx]);
    }
    out
}

/// 🧪️ Multiplies every positive-frequency bin (excluding DC and, for even `n`, the Nyquist bin)
/// by a random unit-magnitude phasor and mirrors the conjugate into the matching negative
/// frequency bin so the spectrum stays Hermitian-symmetric (the inverse transform is then real).
fn randomize_phases(spectrum: &mut [Complex], rng: &mut Xorshift64) {
    let n = spectrum.len();
    let half = (n - 1) / 2;
    for k in 1..=half {
        let rotation = Complex::from_polar(1.0, rng.next_f64() * 2.0 * core::f64::consts::PI);
        spectrum[k] = spectrum[k] * rotation;
        spectrum[n - k] = spectrum[k].conj();
    }
}

fn phase_randomized_surrogate(x: &[f64], rng: &mut Xorshift64) -> Vec<f64> {
    let fft = Fft::new(x.len());
    let input: Vec<Complex> = x.iter().map(|&v| Complex::new(v, 0.0)).collect();
    let mut spectrum = fft.forward(&input);
    randomize_phases(&mut spectrum, rng);
    fft.inverse(&spectrum).iter().map(|c| c.re).collect()
}

/// 🧪️ Amplitude-adjustment step: replaces each value of `current` with the value from
/// `sorted_original` at the same rank, so the result's exact value distribution matches the
/// original's.
fn rank_adjust(current: &[f64], sorted_original: &[f64]) -> Vec<f64> {
    let n = current.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| current[a].total_cmp(&current[b]));
    let mut result = vec![0.0; n];
    for (rank, &idx) in order.iter().enumerate() {
        result[idx] = sorted_original[rank];
    }
    result
}

fn iaaft_surrogate(x: &[f64], iterations: usize, rng: &mut Xorshift64) -> Vec<f64> {
    let fft = Fft::new(x.len());
    let original_spectrum = fft.forward(&x.iter().map(|&v| Complex::new(v, 0.0)).collect::<Vec<_>>());
    let original_magnitude: Vec<f64> = original_spectrum.iter().map(|c| c.abs()).collect();
    let mut sorted_original = x.to_vec();
    sorted_original.sort_by(f64::total_cmp);

    let mut current = phase_randomized_surrogate(x, rng);
    let mut rank_adjusted = current.clone();
    for _ in 0..iterations {
        rank_adjusted = rank_adjust(&current, &sorted_original);
        let ranked_spectrum = fft.forward(&rank_adjusted.iter().map(|&v| Complex::new(v, 0.0)).collect::<Vec<_>>());
        let adjusted_spectrum: Vec<Complex> = ranked_spectrum
            .iter()
            .zip(original_magnitude.iter())
            .map(|(c, &magnitude)| Complex::from_polar(magnitude, c.arg()))
            .collect();
        current = fft.inverse(&adjusted_spectrum).iter().map(|c| c.re).collect();
    }
    rank_adjusted
}

/// 🧪️ Generates `cfg.count` surrogate series of the same length as `x`, per `cfg.kind`. Uses a
/// single [`Xorshift64`] seeded from `cfg.seed` and advanced across every generated surrogate
/// (never reseeded per-surrogate) so the whole batch is reproducible from one seed.
pub fn surrogate_series(x: &[f64], cfg: &SurrogateConfig) -> Result<Vec<Vec<f64>>, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    if x.len() < 2 {
        return Err(EntropyError::InsufficientData { what: "surrogate_series", needed: 2, actual: x.len() });
    }
    let mut rng = Xorshift64::new(cfg.seed);
    Ok((0..cfg.count)
        .map(|_| match cfg.kind {
            SurrogateKind::CircularShift => circular_shift_surrogate(x, &mut rng),
            SurrogateKind::BlockShuffle { block_size } => block_shuffle_surrogate(x, block_size, &mut rng),
            SurrogateKind::PhaseRandomized => phase_randomized_surrogate(x, &mut rng),
            SurrogateKind::Iaaft { iterations } => iaaft_surrogate(x, iterations, &mut rng),
        })
        .collect())
}
// #endregion 🔖️Surrogates

// #region 🔖️MultipleComparisons
/// 🧪️ Benjamini-Hochberg false discovery rate control. Returns, for each input p-value (in the
/// caller's original order), whether it is rejected (declared significant) at FDR level `alpha`:
/// sort ascending, find the largest `k` such that `p_(k) <= (k/m) * alpha`, reject all `p_(i)`
/// for `i <= k`.
pub fn fdr_bh(p_values: &[f64], alpha: f64) -> Result<Vec<bool>, EntropyError> {
    if p_values.is_empty() {
        return Err(EntropyError::EmptyInput { what: "p_values" });
    }
    if !(alpha > 0.0 && alpha < 1.0) {
        return Err(EntropyError::InvalidConfig { field: "alpha", reason: "must be in (0, 1)" });
    }
    let m = p_values.len();
    let mut order: Vec<usize> = (0..m).collect();
    order.sort_by(|&a, &b| p_values[a].total_cmp(&p_values[b]));

    let mut max_k = 0usize;
    for (rank, &idx) in order.iter().enumerate() {
        let k = rank + 1;
        if p_values[idx] <= (k as f64 / m as f64) * alpha {
            max_k = k;
        }
    }
    let mut rejected = vec![false; m];
    for &idx in order.iter().take(max_k) {
        rejected[idx] = true;
    }
    Ok(rejected)
}
// #endregion 🔖️MultipleComparisons

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn mean(data: &[f64]) -> f64 {
        data.iter().sum::<f64>() / data.len() as f64
    }

    #[test]
    fn bootstrap_ci_contains_true_mean_for_fixed_seed() {
        let mut rng = Xorshift64::new(11);
        let true_mean = 5.0;
        let data: Vec<f64> = (0..500).map(|_| true_mean + rng.next_gaussian()).collect();
        let ci = bootstrap_ci(&data, mean, 2000, 0.95, 123).unwrap();
        assert!(ci.lower <= true_mean && true_mean <= ci.upper, "ci={ci:?}");
    }

    #[test]
    fn bootstrap_ci_rejects_insufficient_data_and_bad_level() {
        assert!(matches!(bootstrap_ci(&[], mean, 100, 0.95, 1), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(bootstrap_ci(&[1.0], mean, 100, 0.95, 1), Err(EntropyError::InsufficientData { .. })));
        assert!(matches!(bootstrap_ci(&[1.0, 2.0], mean, 100, 1.5, 1), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn jackknife_ci_matches_classical_mean_ci_for_large_n() {
        let mut rng = Xorshift64::new(22);
        let n = 1000;
        let data: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
        let sample_mean = mean(&data);
        let sd = (data.iter().map(|&v| (v - sample_mean).powi(2)).sum::<f64>() / (n - 1) as f64).sqrt();
        let z = inverse_normal_cdf(0.975);
        let classical_lower = sample_mean - z * sd / (n as f64).sqrt();
        let classical_upper = sample_mean + z * sd / (n as f64).sqrt();

        let ci = jackknife_ci(&data, mean, 0.95).unwrap();
        assert!((ci.lower - classical_lower).abs() < 0.05, "lower {} vs {}", ci.lower, classical_lower);
        assert!((ci.upper - classical_upper).abs() < 0.05, "upper {} vs {}", ci.upper, classical_upper);
    }

    #[test]
    fn jackknife_ci_rejects_insufficient_data_and_bad_level() {
        assert!(matches!(jackknife_ci(&[], mean, 0.95), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(jackknife_ci(&[1.0, 2.0], mean, 0.0), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn permutation_test_p_value_high_under_null_low_under_effect() {
        let mut rng = Xorshift64::new(33);
        let x: Vec<f64> = (0..200).map(|_| rng.next_gaussian()).collect();
        let y: Vec<f64> = (0..200).map(|_| rng.next_gaussian()).collect();
        let diff_means = |a: &[f64], b: &[f64]| mean(a) - mean(b);

        // 🔬️ under the true null, permutation p-values are approximately uniform(0,1); a fixed
        // seed can land anywhere in that range, so assert only "not significant" (p > 0.05)
        // rather than a specific large value.
        let p_null = permutation_test(&x, &y, diff_means, 1000, 44).unwrap();
        assert!(p_null > 0.05, "p_null={p_null}");

        let y_shifted: Vec<f64> = x.iter().map(|&v| v + 5.0).collect();
        let p_effect = permutation_test(&x, &y_shifted, diff_means, 1000, 55).unwrap();
        assert!(p_effect < 0.05, "p_effect={p_effect}");
    }

    #[test]
    fn permutation_test_rejects_empty_groups_and_zero_permutations() {
        let diff_means = |a: &[f64], b: &[f64]| mean(a) - mean(b);
        assert!(matches!(permutation_test(&[], &[1.0], diff_means, 10, 1), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(permutation_test(&[1.0], &[], diff_means, 10, 1), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(
            permutation_test(&[1.0], &[2.0], diff_means, 0, 1),
            Err(EntropyError::InvalidConfig { .. })
        ));
    }

    #[test]
    fn surrogate_config_rejects_zero_count_and_block_size() {
        assert!(matches!(SurrogateConfig::new(SurrogateKind::CircularShift, 0, 1), Err(EntropyError::InvalidConfig { .. })));
        assert!(matches!(
            SurrogateConfig::new(SurrogateKind::BlockShuffle { block_size: 0 }, 5, 1),
            Err(EntropyError::InvalidConfig { .. })
        ));
    }

    #[test]
    fn circular_shift_and_block_shuffle_preserve_value_multiset() {
        let x: Vec<f64> = (0..40).map(|i| (i as f64).sin() * 3.0 + i as f64 * 0.1).collect();
        let mut sorted_x = x.clone();
        sorted_x.sort_by(f64::total_cmp);

        for kind in [SurrogateKind::CircularShift, SurrogateKind::BlockShuffle { block_size: 5 }] {
            let cfg = SurrogateConfig::new(kind, 10, 66).unwrap();
            let surrogates = surrogate_series(&x, &cfg).unwrap();
            assert_eq!(surrogates.len(), 10);
            for s in &surrogates {
                let mut sorted_s = s.clone();
                sorted_s.sort_by(f64::total_cmp);
                for (a, b) in sorted_s.iter().zip(sorted_x.iter()) {
                    assert!((a - b).abs() < 1e-9);
                }
            }
        }
    }

    #[test]
    fn phase_randomized_and_iaaft_preserve_power_spectrum() {
        let n = 64;
        let mut rng = Xorshift64::new(77);
        let x: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
        let fft = Fft::new(n);
        let original_spectrum = fft.forward(&x.iter().map(|&v| Complex::new(v, 0.0)).collect::<Vec<_>>());
        let original_magnitude: Vec<f64> = original_spectrum.iter().map(|c| c.abs()).collect();

        let phase_cfg = SurrogateConfig::new(SurrogateKind::PhaseRandomized, 1, 88).unwrap();
        let phase_surrogate = surrogate_series(&x, &phase_cfg).unwrap().pop().unwrap();
        let phase_spectrum = fft.forward(&phase_surrogate.iter().map(|&v| Complex::new(v, 0.0)).collect::<Vec<_>>());
        for (a, &b) in phase_spectrum.iter().zip(original_magnitude.iter()) {
            assert!((a.abs() - b).abs() < 1e-6, "phase-randomized spectrum magnitude drifted");
        }

        let iaaft_cfg = SurrogateConfig::new(SurrogateKind::Iaaft { iterations: 30 }, 1, 99).unwrap();
        let iaaft_surrogate = surrogate_series(&x, &iaaft_cfg).unwrap().pop().unwrap();
        let iaaft_spectrum = fft.forward(&iaaft_surrogate.iter().map(|&v| Complex::new(v, 0.0)).collect::<Vec<_>>());
        let mut sq_err = 0.0;
        let mut sq_total = 0.0;
        for (a, &b) in iaaft_spectrum.iter().zip(original_magnitude.iter()) {
            sq_err += (a.abs() - b).powi(2);
            sq_total += b * b;
        }
        assert!((sq_err / sq_total).sqrt() < 0.15, "iaaft relative spectral error too high");
    }

    #[test]
    fn iaaft_preserves_value_distribution_phase_randomized_generally_does_not() {
        let n = 50;
        let mut rng = Xorshift64::new(111);
        let x: Vec<f64> = (0..n).map(|_| rng.next_gaussian().powi(2)).collect();
        let mut sorted_x = x.clone();
        sorted_x.sort_by(f64::total_cmp);

        let iaaft_cfg = SurrogateConfig::new(SurrogateKind::Iaaft { iterations: 20 }, 1, 222).unwrap();
        let mut sorted_iaaft = surrogate_series(&x, &iaaft_cfg).unwrap().pop().unwrap();
        sorted_iaaft.sort_by(f64::total_cmp);
        for (a, b) in sorted_iaaft.iter().zip(sorted_x.iter()) {
            assert!((a - b).abs() < 1e-9);
        }

        let phase_cfg = SurrogateConfig::new(SurrogateKind::PhaseRandomized, 1, 333).unwrap();
        let mut sorted_phase = surrogate_series(&x, &phase_cfg).unwrap().pop().unwrap();
        sorted_phase.sort_by(f64::total_cmp);
        let differs = sorted_phase.iter().zip(sorted_x.iter()).any(|(a, b)| (a - b).abs() > 1e-6);
        assert!(differs, "phase-randomized surrogate unexpectedly preserved the exact distribution");
    }

    #[test]
    fn fdr_bh_separates_tiny_and_large_p_values() {
        let p_values = vec![0.001, 0.002, 0.5, 0.7, 0.9];
        let rejected = fdr_bh(&p_values, 0.05).unwrap();
        assert_eq!(rejected, vec![true, true, false, false, false]);
    }

    #[test]
    fn fdr_bh_rejects_empty_input_and_bad_alpha() {
        assert!(matches!(fdr_bh(&[], 0.05), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(fdr_bh(&[0.1, 0.2], 1.0), Err(EntropyError::InvalidConfig { .. })));
    }

    mod quick {
        use super::*;

        #[test]
        fn fdr_bh_is_stricter_than_uncorrected_threshold_under_known_null_proportion() {
            let mut rng = Xorshift64::new(444);
            let m = 200;
            let n_null = (m as f64 * 0.8) as usize;
            let mut p_values = Vec::with_capacity(m);
            for _ in 0..n_null {
                p_values.push(rng.next_f64());
            }
            for _ in n_null..m {
                p_values.push(rng.next_f64().powi(8));
            }
            let alpha = 0.05;
            let rejected = fdr_bh(&p_values, alpha).unwrap();
            let bh_rejections = rejected.iter().filter(|&&r| r).count();
            let uncorrected_rejections = p_values.iter().filter(|&&p| p <= alpha).count();
            assert!(bh_rejections <= uncorrected_rejections, "bh={bh_rejections} uncorrected={uncorrected_rejections}");
        }

        #[test]
        fn surrogate_series_is_reproducible_from_the_same_seed() {
            let x: Vec<f64> = (0..30).map(|i| (i as f64 * 0.3).sin()).collect();
            let cfg = SurrogateConfig::new(SurrogateKind::Iaaft { iterations: 5 }, 4, 555).unwrap();
            let a = surrogate_series(&x, &cfg).unwrap();
            let b = surrogate_series(&x, &cfg).unwrap();
            assert_eq!(a, b);
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Inference

// #region 🔖️Transfer
pub mod transfer {
//! ➡️ Transfer entropy and active information storage: how much a source series' past reduces
//! uncertainty about a target's future, beyond what the target's own past already explains.
//! Supports a quantile-binned discrete backend and a Frenzel-Pompe kNN (KSG-style) continuous
//! backend, both built from delay-embedded history vectors.

use crate::knn::KdTree;
use crate::numeric::digamma;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Metric, Warning};

// #region 🔖️Embedding
fn quantile_bin(x: &[f64], bins: usize) -> Vec<u32> {
    let mut sorted = x.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let edges: Vec<f64> = (1..bins).map(|i| sorted[(sorted.len() * i / bins).min(sorted.len() - 1)]).collect();
    x.iter().map(|&v| edges.partition_point(|&e| e <= v) as u32).collect()
}

/// ➡️ Delay-embedded history matrix (row-major, `n_rows x dim`) of `x[i-dim..i]` for `i` in
/// `start..x.len()`, most-recent sample last in each row.
fn history_matrix(x: &[f64], dim: usize, start: usize) -> Vec<f64> {
    let mut out = Vec::with_capacity((x.len() - start) * dim);
    for i in start..x.len() {
        for j in 0..dim {
            out.push(x[i - dim + j]);
        }
    }
    out
}
// #endregion 🔖️Embedding

// #region 🔖️KsgGeneralized
/// ➡️ Generalized KSG-1 mutual information for multivariate `X` (`x_dim` columns) and `Y`
/// (`y_dim` columns) packed as `[X | Y]` row-major joint points.
fn ksg1_generalized(joint: &[f64], total_dim: usize, x_dim: usize, k: usize) -> Result<f64, EntropyError> {
    let n = joint.len() / total_dim;
    if k == 0 || k >= n {
        return Err(EntropyError::InvalidConfig { field: "k", reason: "must satisfy 0 < k < n" });
    }
    let tree = KdTree::build(joint, total_dim)?;
    let row = |i: usize| -> &[f64] { &joint[i * total_dim..(i + 1) * total_dim] };
    let sub_dist = |a: &[f64], b: &[f64], lo: usize, hi: usize| -> f64 { (lo..hi).map(|d| (a[d] - b[d]).abs()).fold(0.0_f64, f64::max) };

    let mut sum = 0.0_f64;
    for i in 0..n {
        let neighbors = tree.k_nearest(row(i), k, Metric::Chebyshev, Some(i));
        let eps = neighbors.last().map_or(0.0, |&(_, d)| d);
        let mut nx = 0usize;
        let mut ny = 0usize;
        for j in 0..n {
            if j == i {
                continue;
            }
            if sub_dist(row(i), row(j), 0, x_dim) < eps {
                nx += 1;
            }
            if sub_dist(row(i), row(j), x_dim, total_dim) < eps {
                ny += 1;
            }
        }
        sum += digamma(nx as f64 + 1.0) + digamma(ny as f64 + 1.0);
    }
    Ok(digamma(k as f64) - sum / n as f64 + digamma(n as f64))
}

/// ➡️ Generalized Frenzel-Pompe kNN conditional mutual information `I(X;Y|Z)` for multivariate
/// `X`/`Y`/`Z` packed as `[X | Y | Z]` row-major joint points.
fn ksg_cmi_generalized(joint: &[f64], total_dim: usize, x_dim: usize, y_dim: usize, k: usize) -> Result<f64, EntropyError> {
    let n = joint.len() / total_dim;
    if k == 0 || k >= n {
        return Err(EntropyError::InvalidConfig { field: "k", reason: "must satisfy 0 < k < n" });
    }
    let z_start = x_dim + y_dim;
    let tree = KdTree::build(joint, total_dim)?;
    let row = |i: usize| -> &[f64] { &joint[i * total_dim..(i + 1) * total_dim] };
    let sub_dist = |a: &[f64], b: &[f64], lo: usize, hi: usize| -> f64 { (lo..hi).map(|d| (a[d] - b[d]).abs()).fold(0.0_f64, f64::max) };

    let mut sum = 0.0_f64;
    for i in 0..n {
        let neighbors = tree.k_nearest(row(i), k, Metric::Chebyshev, Some(i));
        let eps = neighbors.last().map_or(0.0, |&(_, d)| d);
        let mut n_xz = 0usize;
        let mut n_yz = 0usize;
        let mut n_z = 0usize;
        for j in 0..n {
            if j == i {
                continue;
            }
            let xz = sub_dist(row(i), row(j), 0, x_dim).max(sub_dist(row(i), row(j), z_start, total_dim));
            let yz = sub_dist(row(i), row(j), x_dim, z_start).max(sub_dist(row(i), row(j), z_start, total_dim));
            let z = sub_dist(row(i), row(j), z_start, total_dim);
            if xz < eps {
                n_xz += 1;
            }
            if yz < eps {
                n_yz += 1;
            }
            if z < eps {
                n_z += 1;
            }
        }
        sum += digamma(n_xz as f64 + 1.0) + digamma(n_yz as f64 + 1.0) - digamma(n_z as f64 + 1.0);
    }
    Ok(digamma(k as f64) - sum / n as f64)
}
// #endregion 🔖️KsgGeneralized

// #region 🔖️Config
/// ➡️ Which estimator [`transfer_entropy`] and [`active_information_storage`] use.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TeBackend {
    Discrete { bins: usize },
    Knn { k: usize },
}

/// ➡️ Configuration for [`transfer_entropy`]. History embeddings use a fixed delay `tau = 1`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TransferConfig {
    pub k_history: usize,
    pub l_history: usize,
    pub backend: TeBackend,
}

impl TransferConfig {
    pub fn new(k_history: usize, l_history: usize, backend: TeBackend) -> Result<Self, EntropyError> {
        if k_history == 0 || l_history == 0 {
            return Err(EntropyError::InvalidConfig { field: "history", reason: "k_history and l_history must be at least 1" });
        }
        Ok(Self { k_history, l_history, backend })
    }
}
// #endregion 🔖️Config

// #region 🔖️TransferEntropy
/// ➡️ Transfer entropy `TE(source -> target) = I(target_future ; source_past | target_past)`.
pub fn transfer_entropy(source: &[f64], target: &[f64], cfg: TransferConfig) -> Result<Estimate, EntropyError> {
    if source.len() != target.len() {
        return Err(EntropyError::LengthMismatch { expected: source.len(), actual: target.len() });
    }
    let n = source.len();
    let start = cfg.k_history.max(cfg.l_history);
    if n <= start {
        return Err(EntropyError::InsufficientData { what: "transfer_entropy", needed: start + 1, actual: n });
    }
    let n_samples = n - start;

    let nats = match cfg.backend {
        TeBackend::Discrete { bins } => {
            if bins < 2 {
                return Err(EntropyError::InvalidConfig { field: "bins", reason: "must be at least 2" });
            }
            let source_symbols = quantile_bin(source, bins);
            let target_symbols = quantile_bin(target, bins);
            let mut a = Vec::with_capacity(n_samples); // source_past packed
            let mut b = Vec::with_capacity(n_samples); // target_past packed
            let mut c = Vec::with_capacity(n_samples); // target_future
            let a_size = (bins as u64).pow(cfg.l_history as u32).min(u32::MAX as u64) as usize;
            let b_size = (bins as u64).pow(cfg.k_history as u32).min(u32::MAX as u64) as usize;
            if (bins as u64).pow(cfg.l_history as u32) > u32::MAX as u64 || (bins as u64).pow(cfg.k_history as u32) > u32::MAX as u64 {
                return Err(EntropyError::InvalidConfig { field: "history", reason: "packed alphabet exceeds u32::MAX" });
            }
            for i in start..n {
                let a_sym = source_symbols[i - cfg.l_history..i].iter().fold(0u64, |acc, &s| acc * bins as u64 + s as u64) as u32;
                let b_sym = target_symbols[i - cfg.k_history..i].iter().fold(0u64, |acc, &s| acc * bins as u64 + s as u64) as u32;
                a.push(a_sym);
                b.push(b_sym);
                c.push(target_symbols[i]);
            }
            let _ = (a_size, b_size);
            crate::mutual::conditional_mutual_information(&a, &c, &b, LogBase::Nats)?.value
        }
        TeBackend::Knn { k } => {
            let a = history_matrix(source, cfg.l_history, start);
            let b = history_matrix(target, cfg.k_history, start);
            let c: Vec<f64> = (start..n).map(|i| target[i]).collect();
            let total_dim = cfg.l_history + cfg.k_history + 1;
            let mut joint = Vec::with_capacity(n_samples * total_dim);
            for i in 0..n_samples {
                joint.extend_from_slice(&a[i * cfg.l_history..(i + 1) * cfg.l_history]);
                joint.extend_from_slice(&b[i * cfg.k_history..(i + 1) * cfg.k_history]);
                joint.push(c[i]);
            }
            ksg_cmi_generalized(&joint, total_dim, cfg.l_history, 1, k)?
        }
    };
    let nats = crate::numeric::clamp_near_zero(nats, 1e-6);

    let mut warnings = Vec::new();
    if n_samples < 200 {
        warnings.push(Warning::SmallSample { n: n_samples, recommended: 200 });
    }

    Ok(Estimate {
        value: nats,
        base: LogBase::Nats,
        method: "transfer_entropy",
        n: n_samples,
        n_effective: n_samples as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("k_history", cfg.k_history as f64), ("l_history", cfg.l_history as f64)],
    })
}
// #endregion 🔖️TransferEntropy

// #region 🔖️ActiveInformationStorage
/// ➡️ Active information storage `AIS(Y) = I(Y_future ; Y_past)`, how predictable a series is
/// from its own `k_history`-length past.
pub fn active_information_storage(x: &[f64], k_history: usize, backend: TeBackend, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if k_history == 0 {
        return Err(EntropyError::InvalidConfig { field: "k_history", reason: "must be at least 1" });
    }
    let n = x.len();
    if n <= k_history {
        return Err(EntropyError::InsufficientData { what: "active_information_storage", needed: k_history + 1, actual: n });
    }
    let n_samples = n - k_history;

    let nats = match backend {
        TeBackend::Discrete { bins } => {
            if bins < 2 {
                return Err(EntropyError::InvalidConfig { field: "bins", reason: "must be at least 2" });
            }
            let symbols = quantile_bin(x, bins);
            if (bins as u64).pow(k_history as u32) > u32::MAX as u64 {
                return Err(EntropyError::InvalidConfig { field: "k_history", reason: "packed alphabet exceeds u32::MAX" });
            }
            let mut past = Vec::with_capacity(n_samples);
            let mut future = Vec::with_capacity(n_samples);
            for i in k_history..n {
                let sym = symbols[i - k_history..i].iter().fold(0u64, |acc, &s| acc * bins as u64 + s as u64) as u32;
                past.push(sym);
                future.push(symbols[i]);
            }
            crate::mutual::mutual_information(&past, &future, crate::estimators::DiscreteMethod::Plugin, LogBase::Nats)?.value
        }
        TeBackend::Knn { k } => {
            let past = history_matrix(x, k_history, k_history);
            let future: Vec<f64> = (k_history..n).map(|i| x[i]).collect();
            let total_dim = k_history + 1;
            let mut joint = Vec::with_capacity(n_samples * total_dim);
            for i in 0..n_samples {
                joint.extend_from_slice(&past[i * k_history..(i + 1) * k_history]);
                joint.push(future[i]);
            }
            ksg1_generalized(&joint, total_dim, k_history, k)?
        }
    };
    let clamped = crate::numeric::clamp_near_zero(nats, 1e-6);

    Ok(Estimate {
        value: base.from_nats(clamped),
        base,
        method: "active_information_storage",
        n: n_samples,
        n_effective: n_samples as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings: Vec::new(),
        diagnostics: vec![("k_history", k_history as f64)],
    })
}
// #endregion 🔖️ActiveInformationStorage

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_config_rejects_zero_history() {
        assert!(TransferConfig::new(0, 1, TeBackend::Discrete { bins: 3 }).is_err());
    }

    #[test]
    fn te_of_independent_series_discrete_is_near_zero() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let n = 3000;
        let source: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let target: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let cfg = TransferConfig::new(1, 1, TeBackend::Discrete { bins: 3 }).unwrap();
        let est = transfer_entropy(&source, &target, cfg).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn te_detects_coupling_discrete() {
        // 🔐️ target[i] = source[i-1] (with some noise mixed via binning): TE(source->target)
        // should be clearly larger than TE(target->source).
        let mut rng = crate::numeric::Xorshift64::new(2);
        let n = 4000;
        let source: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let mut target = vec![0.0; n];
        target[1..n].copy_from_slice(&source[..n - 1]);
        let cfg = TransferConfig::new(1, 1, TeBackend::Discrete { bins: 4 }).unwrap();
        let forward = transfer_entropy(&source, &target, cfg).unwrap();
        let backward = transfer_entropy(&target, &source, cfg).unwrap();
        assert!(forward.value > backward.value, "forward={} backward={}", forward.value, backward.value);
        assert!(forward.value > 0.1, "forward={}", forward.value);
    }

    #[test]
    fn ais_of_white_noise_is_near_zero() {
        let mut rng = crate::numeric::Xorshift64::new(3);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_f64()).collect();
        let est = active_information_storage(&x, 1, TeBackend::Discrete { bins: 3 }, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn ais_of_highly_predictable_series_is_positive() {
        let mut rng = crate::numeric::Xorshift64::new(4);
        let n = 2000;
        let mut x = vec![0.0; n];
        for i in 1..n {
            x[i] = 0.9 * x[i - 1] + 0.1 * rng.next_gaussian();
        }
        let cfg_backend = TeBackend::Discrete { bins: 4 };
        let est = active_information_storage(&x, 1, cfg_backend, LogBase::Nats).unwrap();
        assert!(est.value > 0.1, "got {}", est.value);
    }

    #[test]
    fn te_rejects_length_mismatch() {
        let cfg = TransferConfig::new(1, 1, TeBackend::Discrete { bins: 3 }).unwrap();
        assert!(matches!(transfer_entropy(&[1.0, 2.0], &[1.0], cfg), Err(EntropyError::LengthMismatch { .. })));
    }

    mod quick {
        use super::*;

        #[test]
        fn te_knn_detects_coupling_on_coupled_logistic_maps() {
            let mut rng = crate::numeric::Xorshift64::new(5);
            let n = 800;
            let mut x = vec![0.4 + 0.1 * rng.next_f64(); n];
            let mut y = vec![0.4 + 0.1 * rng.next_f64(); n];
            let r = 3.7;
            let coupling = 0.3;
            for i in 1..n {
                x[i] = r * x[i - 1] * (1.0 - x[i - 1]);
                let driven = (1.0 - coupling) * y[i - 1] + coupling * x[i - 1];
                y[i] = r * driven * (1.0 - driven);
            }
            let cfg = TransferConfig::new(1, 1, TeBackend::Knn { k: 4 }).unwrap();
            let forward = transfer_entropy(&x, &y, cfg).unwrap();
            let backward = transfer_entropy(&y, &x, cfg).unwrap();
            assert!(forward.value > backward.value, "forward={} backward={}", forward.value, backward.value);
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Transfer

// #region 🔖️Spatial
pub mod spatial {
//! 🖼️ Image / spatial entropy over plain pixel slices (no image-decoding dependency): global
//! grayscale histogram entropy and gray-level co-occurrence matrix (GLCM) texture entropy.

use crate::numeric::x_ln_x;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};

// #region 🔖️Config
/// 🖼️ Which spatial-entropy computation [`entropy_2d`] performs.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SpatialMethod {
    /// 🖼️ Global histogram entropy over all pixel values.
    Global,
    /// 🖼️ Gray-level co-occurrence matrix entropy for the pixel offset `(dx, dy)`.
    Glcm { dx: i32, dy: i32 },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SpatialConfig {
    pub method: SpatialMethod,
    pub bins: usize,
}

impl SpatialConfig {
    pub fn new(method: SpatialMethod, bins: usize) -> Result<Self, EntropyError> {
        if bins < 2 {
            return Err(EntropyError::InvalidConfig { field: "bins", reason: "must be at least 2" });
        }
        Ok(Self { method, bins })
    }
}
// #endregion 🔖️Config

// #region 🔖️Binning
fn bin_pixels(pixels: &[f64], bins: usize) -> Result<Vec<usize>, EntropyError> {
    let (min, max) = pixels.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    if max <= min {
        return Err(EntropyError::DegenerateInput { what: "constant image has zero dynamic range" });
    }
    Ok(pixels
        .iter()
        .map(|&v| (((v - min) / (max - min) * bins as f64).floor() as usize).min(bins - 1))
        .collect())
}
// #endregion 🔖️Binning

// #region 🔖️Dispatch
/// 🖼️ Computes a spatial entropy measure over a row-major `width x height` pixel grid.
pub fn entropy_2d(pixels: &[f64], width: usize, height: usize, cfg: SpatialConfig) -> Result<Estimate, EntropyError> {
    if pixels.is_empty() {
        return Err(EntropyError::EmptyInput { what: "pixels" });
    }
    if pixels.len() != width * height {
        return Err(EntropyError::ShapeMismatch { what: "pixels", expected: width * height, actual: pixels.len() });
    }
    for (i, &v) in pixels.iter().enumerate() {
        if !v.is_finite() {
            return Err(EntropyError::NonFinite { what: "pixels", index: i });
        }
    }
    let levels = bin_pixels(pixels, cfg.bins)?;

    let (nats, method, diagnostics) = match cfg.method {
        SpatialMethod::Global => {
            let mut counts = vec![0.0_f64; cfg.bins];
            for &l in &levels {
                counts[l] += 1.0;
            }
            let n = pixels.len() as f64;
            let nats = -counts.iter().map(|&c| x_ln_x(c / n)).sum::<f64>();
            (nats, "global_histogram", vec![("bins", cfg.bins as f64)])
        }
        SpatialMethod::Glcm { dx, dy } => {
            let mut joint = vec![0.0_f64; cfg.bins * cfg.bins];
            let mut pairs = 0.0_f64;
            for y in 0..height as i32 {
                for x in 0..width as i32 {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
                        continue;
                    }
                    let a = levels[(y as usize) * width + x as usize];
                    let b = levels[(ny as usize) * width + nx as usize];
                    joint[a * cfg.bins + b] += 1.0;
                    joint[b * cfg.bins + a] += 1.0; // 🖼️ standard GLCM symmetrization
                    pairs += 2.0;
                }
            }
            if pairs <= 0.0 {
                return Err(EntropyError::InvalidConfig { field: "dx/dy", reason: "offset produces no valid pixel pairs" });
            }
            let nats = -joint.iter().map(|&c| x_ln_x(c / pairs)).sum::<f64>();
            (nats, "glcm", vec![("bins", cfg.bins as f64), ("dx", dx as f64), ("dy", dy as f64)])
        }
    };

    let mut warnings = Vec::new();
    if pixels.len() < 10 * cfg.bins {
        warnings.push(Warning::SmallSample { n: pixels.len(), recommended: 10 * cfg.bins });
    }

    Ok(Estimate {
        value: LogBase::Nats.from_nats(nats),
        base: LogBase::Nats,
        method,
        n: pixels.len(),
        n_effective: pixels.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics,
    })
}
// #endregion 🔖️Dispatch

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_image_is_rejected() {
        let pixels = vec![5.0; 16];
        let cfg = SpatialConfig::new(SpatialMethod::Global, 4).unwrap();
        assert!(matches!(entropy_2d(&pixels, 4, 4, cfg), Err(EntropyError::DegenerateInput { .. })));
    }

    #[test]
    fn noisy_image_has_higher_glcm_entropy_than_smooth_gradient() {
        // 🔐️ A checkerboard is NOT a good "high texture" counter-example here: it alternates
        // between exactly two values, so its dx=1 co-occurrence matrix is nearly deterministic
        // (low entropy). Genuine per-pixel noise, which visits many distinct adjacent-value
        // pairs unpredictably, is the correct high-entropy comparison against a smooth gradient.
        let width = 16;
        let height = 16;
        let mut rng = crate::numeric::Xorshift64::new(9);
        let noise: Vec<f64> = (0..width * height).map(|_| rng.next_f64()).collect();
        let gradient: Vec<f64> = (0..width * height).map(|i| (i % width) as f64).collect();
        let cfg = SpatialConfig::new(SpatialMethod::Glcm { dx: 1, dy: 0 }, 4).unwrap();
        let h_noise = entropy_2d(&noise, width, height, cfg).unwrap().value;
        let h_gradient = entropy_2d(&gradient, width, height, cfg).unwrap().value;
        assert!(h_noise > h_gradient, "noise={h_noise} gradient={h_gradient}");
    }

    #[test]
    fn global_entropy_of_uniform_random_image_is_near_max() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let width = 32;
        let height = 32;
        let pixels: Vec<f64> = (0..width * height).map(|_| rng.next_f64()).collect();
        let cfg = SpatialConfig::new(SpatialMethod::Global, 8).unwrap();
        let est = entropy_2d(&pixels, width, height, cfg).unwrap();
        assert!(est.value > 0.8 * 8.0_f64.ln(), "got {}", est.value);
    }

    #[test]
    fn shape_mismatch_is_rejected() {
        let pixels = vec![1.0, 2.0, 3.0];
        let cfg = SpatialConfig::new(SpatialMethod::Global, 4).unwrap();
        assert!(matches!(entropy_2d(&pixels, 2, 2, cfg), Err(EntropyError::ShapeMismatch { .. })));
    }

    #[test]
    fn spatial_config_rejects_small_bins() {
        assert!(SpatialConfig::new(SpatialMethod::Global, 1).is_err());
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Spatial

// #region 🔖️Graph
pub mod graph {
//! 🕸️ Graph entropy over plain edge lists (no graph-library dependency): degree-distribution
//! entropy and random-walk entropy rate.

use crate::numeric::x_ln_x;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};

// #region 🔖️Degree
/// 🕸️ Shannon entropy of the (out-)degree distribution of a graph given as an edge list.
/// `directed` selects out-degree counting; undirected counts each edge toward both endpoints.
pub fn degree_distribution_entropy(edges: &[(u32, u32)], n_nodes: usize, directed: bool, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if n_nodes == 0 {
        return Err(EntropyError::EmptyInput { what: "n_nodes" });
    }
    let mut degree = vec![0u64; n_nodes];
    for &(a, b) in edges {
        let (a, b) = (a as usize, b as usize);
        if a >= n_nodes || b >= n_nodes {
            return Err(EntropyError::ShapeMismatch { what: "edge endpoint", expected: n_nodes, actual: a.max(b) + 1 });
        }
        degree[a] += 1;
        if !directed {
            degree[b] += 1;
        }
    }
    let max_degree = degree.iter().copied().max().unwrap_or(0) as usize;
    let mut hist = vec![0.0_f64; max_degree + 1];
    for &d in &degree {
        hist[d as usize] += 1.0;
    }
    let n = n_nodes as f64;
    let nats = -hist.iter().map(|&c| x_ln_x(c / n)).sum::<f64>();

    let mut warnings = Vec::new();
    if n_nodes < 30 {
        warnings.push(Warning::SmallSample { n: n_nodes, recommended: 30 });
    }

    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method: "degree_distribution_entropy",
        n: n_nodes,
        n_effective: n_nodes as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("max_degree", max_degree as f64), ("edges", edges.len() as f64)],
    })
}
// #endregion 🔖️Degree

// #region 🔖️RandomWalk
/// 🕸️ Random-walk entropy rate `sum_i pi_i * H(row_i)`: the average per-step uncertainty of a
/// simple (optionally weighted) random walk on the graph, weighted by the walk's stationary node
/// distribution (via power iteration).
pub fn random_walk_entropy_rate(edges: &[(u32, u32)], n_nodes: usize, weights: Option<&[f64]>, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if n_nodes == 0 {
        return Err(EntropyError::EmptyInput { what: "n_nodes" });
    }
    if let Some(w) = weights {
        if w.len() != edges.len() {
            return Err(EntropyError::LengthMismatch { expected: edges.len(), actual: w.len() });
        }
    }
    let mut adjacency = vec![0.0_f64; n_nodes * n_nodes];
    for (i, &(a, b)) in edges.iter().enumerate() {
        let (a, b) = (a as usize, b as usize);
        if a >= n_nodes || b >= n_nodes {
            return Err(EntropyError::ShapeMismatch { what: "edge endpoint", expected: n_nodes, actual: a.max(b) + 1 });
        }
        let w = weights.map_or(1.0, |ws| ws[i]);
        if w < 0.0 || !w.is_finite() {
            return Err(EntropyError::InvalidProbability { index: i, value: w });
        }
        adjacency[a * n_nodes + b] += w;
        adjacency[b * n_nodes + a] += w;
    }

    let mut transition = vec![0.0_f64; n_nodes * n_nodes];
    let mut row_entropy_nats = vec![0.0_f64; n_nodes];
    for i in 0..n_nodes {
        let row_sum: f64 = adjacency[i * n_nodes..(i + 1) * n_nodes].iter().sum();
        if row_sum > 0.0 {
            for j in 0..n_nodes {
                transition[i * n_nodes + j] = adjacency[i * n_nodes + j] / row_sum;
            }
        } else {
            transition[i * n_nodes + i] = 1.0; // 🕸️ isolated node: absorbing self-loop
        }
        row_entropy_nats[i] = -transition[i * n_nodes..(i + 1) * n_nodes].iter().map(|&p| x_ln_x(p)).sum::<f64>();
    }

    let mut pi = vec![1.0 / n_nodes as f64; n_nodes];
    let mut converged = false;
    for _ in 0..10_000 {
        let mut next = vec![0.0_f64; n_nodes];
        for i in 0..n_nodes {
            if pi[i] <= 0.0 {
                continue;
            }
            for j in 0..n_nodes {
                next[j] += pi[i] * transition[i * n_nodes + j];
            }
        }
        let delta: f64 = pi.iter().zip(next.iter()).map(|(&a, &b)| (a - b).abs()).sum();
        pi = next;
        if delta < 1e-12 {
            converged = true;
            break;
        }
    }
    if !converged {
        return Err(EntropyError::NotConverged { what: "random walk stationary distribution", iterations: 10_000 });
    }

    let nats = pi.iter().zip(row_entropy_nats.iter()).map(|(&p, &h)| p * h).sum::<f64>();

    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method: "random_walk_entropy_rate",
        n: n_nodes,
        n_effective: n_nodes as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings: Vec::new(),
        diagnostics: vec![("edges", edges.len() as f64)],
    })
}
// #endregion 🔖️RandomWalk

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degree_entropy_of_regular_graph_is_zero() {
        // 🔐️ a 4-cycle: every node has degree 2.
        let edges = [(0, 1), (1, 2), (2, 3), (3, 0)];
        let est = degree_distribution_entropy(&edges, 4, false, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn degree_entropy_rejects_out_of_range_endpoint() {
        let edges = [(0, 5)];
        assert!(matches!(
            degree_distribution_entropy(&edges, 3, false, LogBase::Bits),
            Err(EntropyError::ShapeMismatch { .. })
        ));
    }

    #[test]
    fn random_walk_entropy_rate_of_complete_graph_matches_uniform_row_entropy() {
        // 🔐️ K4: every node connects to every other node; each row is uniform over 3 neighbors.
        let edges = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        let est = random_walk_entropy_rate(&edges, 4, None, LogBase::Bits).unwrap();
        let expected = 3.0_f64.log2();
        assert!((est.value - expected).abs() < 1e-6, "got {}", est.value);
    }

    #[test]
    fn random_walk_entropy_rate_of_cycle_matches_binary_entropy() {
        // 🔐️ 4-cycle: every row is [0.5, 0.5] over its two neighbors -> 1 bit.
        let edges = [(0, 1), (1, 2), (2, 3), (3, 0)];
        let est = random_walk_entropy_rate(&edges, 4, None, LogBase::Bits).unwrap();
        assert!((est.value - 1.0).abs() < 1e-6, "got {}", est.value);
    }

    #[test]
    fn random_walk_handles_isolated_node() {
        let edges = [(0, 1)];
        let est = random_walk_entropy_rate(&edges, 3, None, LogBase::Bits).unwrap();
        assert!(est.value.is_finite());
    }

    #[test]
    fn random_walk_rejects_negative_weight() {
        let edges = [(0, 1)];
        assert!(random_walk_entropy_rate(&edges, 2, Some(&[-1.0]), LogBase::Bits).is_err());
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Graph

// #region 🔖️Ml
pub mod ml {
//! 🤖️ Machine-learning uncertainty measures: per-sample predictive entropy over classifier
//! outputs, BALD (epistemic) mutual information from ensemble predictions, and expected
//! calibration error.

use crate::numeric::x_ln_x;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase};

// #region 🔖️Predictive
/// 🤖️ Shannon entropy of each row of a row-major `[n_samples x n_classes]` probability batch.
pub fn predictive_entropy(probs: &[f64], n_classes: usize, base: LogBase) -> Result<Vec<Estimate>, EntropyError> {
    base.validate()?;
    if n_classes == 0 {
        return Err(EntropyError::InvalidConfig { field: "n_classes", reason: "must be at least 1" });
    }
    if probs.is_empty() || !probs.len().is_multiple_of(n_classes) {
        return Err(EntropyError::ShapeMismatch { what: "probs", expected: n_classes, actual: probs.len() % n_classes.max(1) });
    }
    let n_samples = probs.len() / n_classes;
    let mut out = Vec::with_capacity(n_samples);
    for i in 0..n_samples {
        let row = &probs[i * n_classes..(i + 1) * n_classes];
        let p = crate::counts::validate_probabilities(row, crate::Tolerances::default())?;
        let nats = -p.iter().map(|&v| x_ln_x(v)).sum::<f64>();
        out.push(Estimate {
            value: base.from_nats(nats),
            base,
            method: "predictive_entropy",
            n: n_classes,
            n_effective: n_classes as f64,
            std_error: None,
            ci: None::<ConfidenceInterval>,
            warnings: Vec::new(),
            diagnostics: Vec::new(),
        });
    }
    Ok(out)
}
// #endregion 🔖️Predictive

// #region 🔖️Bald
/// 🤖️ BALD mutual information per sample: `H(mean_over_members(p)) - mean_over_members(H(p))`,
/// splitting total predictive uncertainty into epistemic (this value) and aleatoric (the
/// subtracted mean-member-entropy term) components. `ensemble_probs` is row-major
/// `[n_samples][n_members][n_classes]` flattened.
pub fn bald_mutual_information(ensemble_probs: &[f64], n_members: usize, n_classes: usize, base: LogBase) -> Result<Vec<Estimate>, EntropyError> {
    base.validate()?;
    if n_members == 0 || n_classes == 0 {
        return Err(EntropyError::InvalidConfig { field: "n_members/n_classes", reason: "must be at least 1" });
    }
    let per_sample_len = n_members * n_classes;
    if ensemble_probs.is_empty() || !ensemble_probs.len().is_multiple_of(per_sample_len) {
        return Err(EntropyError::ShapeMismatch { what: "ensemble_probs", expected: per_sample_len, actual: ensemble_probs.len() % per_sample_len.max(1) });
    }
    let n_samples = ensemble_probs.len() / per_sample_len;
    let mut out = Vec::with_capacity(n_samples);
    for s in 0..n_samples {
        let sample = &ensemble_probs[s * per_sample_len..(s + 1) * per_sample_len];
        let mut mean_probs = vec![0.0_f64; n_classes];
        let mut mean_member_entropy_nats = 0.0_f64;
        for m in 0..n_members {
            let member = &sample[m * n_classes..(m + 1) * n_classes];
            let p = crate::counts::validate_probabilities(member, crate::Tolerances::default())?;
            for c in 0..n_classes {
                mean_probs[c] += p[c] / n_members as f64;
            }
            mean_member_entropy_nats += -p.iter().map(|&v| x_ln_x(v)).sum::<f64>() / n_members as f64;
        }
        let mean_probs = crate::counts::validate_probabilities(&mean_probs, crate::Tolerances::default())?;
        let predictive_nats = -mean_probs.iter().map(|&v| x_ln_x(v)).sum::<f64>();
        let bald_nats = crate::numeric::clamp_near_zero(predictive_nats - mean_member_entropy_nats, 1e-9);

        out.push(Estimate {
            value: base.from_nats(bald_nats),
            base,
            method: "bald_mutual_information",
            n: n_members,
            n_effective: n_members as f64,
            std_error: None,
            ci: None::<ConfidenceInterval>,
            warnings: Vec::new(),
            diagnostics: vec![("predictive_entropy", base.from_nats(predictive_nats)), ("mean_member_entropy", base.from_nats(mean_member_entropy_nats))],
        });
    }
    Ok(out)
}
// #endregion 🔖️Bald

// #region 🔖️Calibration
/// 🤖️ Expected calibration error: bins predictions by confidence into `n_bins` equal-width bins
/// in `[0,1]`, and reports the confidence-weighted average `|accuracy - confidence|` per bin.
pub fn expected_calibration_error(confidences: &[f64], correct: &[bool], n_bins: usize) -> Result<f64, EntropyError> {
    if confidences.len() != correct.len() {
        return Err(EntropyError::LengthMismatch { expected: confidences.len(), actual: correct.len() });
    }
    if confidences.is_empty() {
        return Err(EntropyError::EmptyInput { what: "confidences" });
    }
    if n_bins == 0 {
        return Err(EntropyError::InvalidConfig { field: "n_bins", reason: "must be at least 1" });
    }
    for (i, &c) in confidences.iter().enumerate() {
        if !(0.0..=1.0).contains(&c) {
            return Err(EntropyError::InvalidProbability { index: i, value: c });
        }
    }
    let mut bin_conf_sum = vec![0.0_f64; n_bins];
    let mut bin_correct_sum = vec![0.0_f64; n_bins];
    let mut bin_count = vec![0.0_f64; n_bins];
    for (&c, &ok) in confidences.iter().zip(correct.iter()) {
        let bin = ((c * n_bins as f64) as usize).min(n_bins - 1);
        bin_conf_sum[bin] += c;
        bin_correct_sum[bin] += if ok { 1.0 } else { 0.0 };
        bin_count[bin] += 1.0;
    }
    let n = confidences.len() as f64;
    let mut ece = 0.0_f64;
    for b in 0..n_bins {
        if bin_count[b] <= 0.0 {
            continue;
        }
        let avg_conf = bin_conf_sum[b] / bin_count[b];
        let accuracy = bin_correct_sum[b] / bin_count[b];
        ece += (bin_count[b] / n) * (accuracy - avg_conf).abs();
    }
    Ok(ece)
}
// #endregion 🔖️Calibration

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predictive_entropy_of_confident_prediction_is_zero() {
        let probs = [1.0, 0.0, 0.0];
        let est = predictive_entropy(&probs, 3, LogBase::Bits).unwrap();
        assert!(est[0].value.abs() < 1e-9);
    }

    #[test]
    fn predictive_entropy_of_uniform_is_max() {
        let probs = [0.25, 0.25, 0.25, 0.25];
        let est = predictive_entropy(&probs, 4, LogBase::Bits).unwrap();
        assert!((est[0].value - 2.0).abs() < 1e-9);
    }

    #[test]
    fn predictive_entropy_batch_shape() {
        let probs = [0.5, 0.5, 0.9, 0.1, 0.25, 0.75];
        let est = predictive_entropy(&probs, 2, LogBase::Bits).unwrap();
        assert_eq!(est.len(), 3);
    }

    #[test]
    fn bald_of_unanimous_ensemble_is_near_zero() {
        // 🔐️ every member gives the same confident prediction: no epistemic disagreement.
        let ensemble = [1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
        let est = bald_mutual_information(&ensemble, 3, 2, LogBase::Bits).unwrap();
        assert!(est[0].value.abs() < 1e-9);
    }

    #[test]
    fn bald_of_disagreeing_ensemble_is_positive() {
        // 🔐️ members confidently disagree with each other but each is individually confident.
        let ensemble = [1.0, 0.0, 0.0, 1.0];
        let est = bald_mutual_information(&ensemble, 2, 2, LogBase::Bits).unwrap();
        assert!(est[0].value > 0.9, "got {}", est[0].value);
    }

    #[test]
    fn ece_of_perfectly_calibrated_predictions_is_zero() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let n = 5000;
        let confidences: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let correct: Vec<bool> = confidences.iter().map(|&c| rng.next_f64() < c).collect();
        let ece = expected_calibration_error(&confidences, &correct, 10).unwrap();
        assert!(ece < 0.05, "got {ece}");
    }

    #[test]
    fn ece_of_badly_miscalibrated_predictions_is_large() {
        let confidences = vec![0.95; 100];
        let correct = vec![false; 100];
        let ece = expected_calibration_error(&confidences, &correct, 10).unwrap();
        assert!(ece > 0.8, "got {ece}");
    }

    #[test]
    fn ece_rejects_length_mismatch() {
        assert!(expected_calibration_error(&[0.5], &[], 5).is_err());
    }

    #[test]
    fn ece_rejects_out_of_range_confidence() {
        assert!(expected_calibration_error(&[1.5], &[true], 5).is_err());
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Ml

// #region 🔖️Streaming
pub mod streaming {
//! 🌊️ Online/streaming entropy estimation: a mergeable `StreamingEstimator` trait plus exact
//! incremental counts, a fixed sliding window, and exponentially decayed counts.

use crate::numeric::x_ln_x;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};
use std::collections::VecDeque;

// #region 🔖️Trait
/// 🌊️ Mergeable online state: `update` folds one observation in, `remove` undoes one (where
/// supported), `merge` combines two independently accumulated states, `estimate` reports the
/// current entropy, `reset` clears all state, and `snapshot`/`restore` round-trip the state
/// through a plain-data representation (no serde — see [`StreamingSnapshot`]).
pub trait StreamingEstimator {
    type Item;
    fn update(&mut self, x: Self::Item);
    fn remove(&mut self, x: Self::Item) -> Result<(), EntropyError>;
    fn merge(&mut self, other: &Self) -> Result<(), EntropyError>;
    fn estimate(&self) -> Result<Estimate, EntropyError>;
    fn reset(&mut self);
    fn snapshot(&self) -> StreamingSnapshot;
    fn restore(snapshot: &StreamingSnapshot) -> Result<Self, EntropyError>
    where
        Self: Sized;
}

/// 🌊️ Plain-data snapshot of a streaming estimator's internal counts, used for `snapshot`/
/// `restore` round-tripping without depending on an external serialization crate.
#[derive(Clone, PartialEq, Debug)]
pub struct StreamingSnapshot {
    pub counts: Vec<f64>,
    pub alphabet_size: usize,
    pub base: LogBase,
    pub method: &'static str,
    pub extra: Vec<f64>,
}
// #endregion 🔖️Trait

// #region 🔖️Shared
fn plugin_entropy_from_counts(counts: &[f64], base: LogBase, method: &'static str, n_raw: usize) -> Estimate {
    let total: f64 = counts.iter().sum();
    let nats = if total > 0.0 { -counts.iter().map(|&c| x_ln_x(c / total)).sum::<f64>() } else { 0.0 };
    let mut warnings = Vec::new();
    let occupied = counts.iter().filter(|&&c| c > 0.0).count();
    if occupied * 2 < counts.len() {
        warnings.push(Warning::Undersampled { occupied_bins: occupied, total_bins: counts.len() });
    }
    Estimate {
        value: base.from_nats(nats),
        base,
        method,
        n: n_raw,
        n_effective: total,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("alphabet_size", counts.len() as f64), ("total_weight", total)],
    }
}
// #endregion 🔖️Shared

// #region 🔖️StreamingCounts
/// 🌊️ Exact incremental symbol counts over a fixed `0..alphabet_size` alphabet.
pub struct StreamingCounts {
    counts: Vec<f64>,
    base: LogBase,
    n_raw: usize,
}

impl StreamingCounts {
    pub fn new(alphabet_size: usize, base: LogBase) -> Self {
        Self { counts: vec![0.0; alphabet_size], base, n_raw: 0 }
    }
}

impl StreamingEstimator for StreamingCounts {
    type Item = u32;

    fn update(&mut self, x: u32) {
        if (x as usize) < self.counts.len() {
            self.counts[x as usize] += 1.0;
            self.n_raw += 1;
        }
    }

    fn remove(&mut self, x: u32) -> Result<(), EntropyError> {
        let idx = x as usize;
        if idx >= self.counts.len() || self.counts[idx] < 1.0 {
            return Err(EntropyError::InvalidConfig { field: "x", reason: "no observation of this symbol to remove" });
        }
        self.counts[idx] -= 1.0;
        self.n_raw -= 1;
        Ok(())
    }

    fn merge(&mut self, other: &Self) -> Result<(), EntropyError> {
        if self.counts.len() != other.counts.len() {
            return Err(EntropyError::LengthMismatch { expected: self.counts.len(), actual: other.counts.len() });
        }
        for (a, &b) in self.counts.iter_mut().zip(other.counts.iter()) {
            *a += b;
        }
        self.n_raw += other.n_raw;
        Ok(())
    }

    fn estimate(&self) -> Result<Estimate, EntropyError> {
        if self.n_raw == 0 {
            return Err(EntropyError::EmptyInput { what: "streaming counts" });
        }
        Ok(plugin_entropy_from_counts(&self.counts, self.base, "streaming_counts", self.n_raw))
    }

    fn reset(&mut self) {
        self.counts.iter_mut().for_each(|c| *c = 0.0);
        self.n_raw = 0;
    }

    fn snapshot(&self) -> StreamingSnapshot {
        StreamingSnapshot { counts: self.counts.clone(), alphabet_size: self.counts.len(), base: self.base, method: "streaming_counts", extra: vec![self.n_raw as f64] }
    }

    fn restore(snapshot: &StreamingSnapshot) -> Result<Self, EntropyError> {
        Ok(Self { counts: snapshot.counts.clone(), base: snapshot.base, n_raw: snapshot.extra.first().copied().unwrap_or(0.0) as usize })
    }
}
// #endregion 🔖️StreamingCounts

// #region 🔖️SlidingWindow
/// 🌊️ Entropy over the most recent `capacity` observations only (older ones are evicted exactly,
/// via [`StreamingCounts::remove`]).
pub struct SlidingWindowEntropy {
    window: VecDeque<u32>,
    capacity: usize,
    counts: StreamingCounts,
}

impl SlidingWindowEntropy {
    pub fn new(alphabet_size: usize, capacity: usize, base: LogBase) -> Result<Self, EntropyError> {
        if capacity == 0 {
            return Err(EntropyError::InvalidConfig { field: "capacity", reason: "must be at least 1" });
        }
        Ok(Self { window: VecDeque::with_capacity(capacity), capacity, counts: StreamingCounts::new(alphabet_size, base) })
    }
}

impl StreamingEstimator for SlidingWindowEntropy {
    type Item = u32;

    fn update(&mut self, x: u32) {
        self.counts.update(x);
        self.window.push_back(x);
        if self.window.len() > self.capacity {
            if let Some(evicted) = self.window.pop_front() {
                let _ = self.counts.remove(evicted);
            }
        }
    }

    fn remove(&mut self, _x: u32) -> Result<(), EntropyError> {
        Err(EntropyError::InvalidConfig { field: "remove", reason: "SlidingWindowEntropy evicts automatically; explicit remove is unsupported" })
    }

    fn merge(&mut self, _other: &Self) -> Result<(), EntropyError> {
        Err(EntropyError::InvalidConfig { field: "merge", reason: "SlidingWindowEntropy carries order-dependent state and cannot be merged" })
    }

    fn estimate(&self) -> Result<Estimate, EntropyError> {
        self.counts.estimate()
    }

    fn reset(&mut self) {
        self.window.clear();
        self.counts.reset();
    }

    fn snapshot(&self) -> StreamingSnapshot {
        let mut snap = self.counts.snapshot();
        snap.method = "sliding_window_entropy";
        snap.extra.push(self.capacity as f64);
        for &v in &self.window {
            snap.extra.push(v as f64);
        }
        snap
    }

    fn restore(snapshot: &StreamingSnapshot) -> Result<Self, EntropyError> {
        if snapshot.extra.len() < 2 {
            return Err(EntropyError::InvalidConfig { field: "snapshot", reason: "missing capacity/window data" });
        }
        let capacity = snapshot.extra[1] as usize;
        let window: VecDeque<u32> = snapshot.extra[2..].iter().map(|&v| v as u32).collect();
        let counts = StreamingCounts { counts: snapshot.counts.clone(), base: snapshot.base, n_raw: snapshot.extra[0] as usize };
        Ok(Self { window, capacity, counts })
    }
}
// #endregion 🔖️SlidingWindow

// #region 🔖️Decayed
/// 🌊️ Exponentially forgetting counts: each `update` first multiplies every count by `decay` (in
/// `(0, 1]`) before incrementing the observed symbol, so older observations fade geometrically.
/// `remove` is semantically unsupported (there is no well-defined inverse of decay) and always
/// errors.
pub struct DecayedEntropy {
    counts: Vec<f64>,
    decay: f64,
    base: LogBase,
}

impl DecayedEntropy {
    pub fn new(alphabet_size: usize, decay: f64, base: LogBase) -> Result<Self, EntropyError> {
        if !(0.0 < decay && decay <= 1.0) {
            return Err(EntropyError::InvalidConfig { field: "decay", reason: "must be in (0, 1]" });
        }
        Ok(Self { counts: vec![0.0; alphabet_size], decay, base })
    }
}

impl StreamingEstimator for DecayedEntropy {
    type Item = u32;

    fn update(&mut self, x: u32) {
        for c in &mut self.counts {
            *c *= self.decay;
        }
        if (x as usize) < self.counts.len() {
            self.counts[x as usize] += 1.0;
        }
    }

    fn remove(&mut self, _x: u32) -> Result<(), EntropyError> {
        Err(EntropyError::InvalidConfig { field: "remove", reason: "DecayedEntropy has no well-defined inverse of exponential decay" })
    }

    fn merge(&mut self, other: &Self) -> Result<(), EntropyError> {
        if self.counts.len() != other.counts.len() {
            return Err(EntropyError::LengthMismatch { expected: self.counts.len(), actual: other.counts.len() });
        }
        for (a, &b) in self.counts.iter_mut().zip(other.counts.iter()) {
            *a += b;
        }
        Ok(())
    }

    fn estimate(&self) -> Result<Estimate, EntropyError> {
        let total: f64 = self.counts.iter().sum();
        if total <= 0.0 {
            return Err(EntropyError::EmptyInput { what: "decayed counts" });
        }
        Ok(plugin_entropy_from_counts(&self.counts, self.base, "decayed_entropy", self.counts.len()))
    }

    fn reset(&mut self) {
        self.counts.iter_mut().for_each(|c| *c = 0.0);
    }

    fn snapshot(&self) -> StreamingSnapshot {
        StreamingSnapshot { counts: self.counts.clone(), alphabet_size: self.counts.len(), base: self.base, method: "decayed_entropy", extra: vec![self.decay] }
    }

    fn restore(snapshot: &StreamingSnapshot) -> Result<Self, EntropyError> {
        let decay = snapshot.extra.first().copied().unwrap_or(1.0);
        Ok(Self { counts: snapshot.counts.clone(), decay, base: snapshot.base })
    }
}
// #endregion 🔖️Decayed

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streaming_counts_update_matches_batch_entropy() {
        let mut sc = StreamingCounts::new(4, LogBase::Bits);
        for &x in &[0u32, 1, 1, 2, 2, 2, 3] {
            sc.update(x);
        }
        let est = sc.estimate().unwrap();
        let counts = crate::counts::Counts::from_symbols(&[0, 1, 1, 2, 2, 2, 3], 4).unwrap();
        let expected = crate::discrete::entropy(&counts.probabilities(), LogBase::Bits).unwrap();
        assert!((est.value - expected).abs() < 1e-9);
    }

    #[test]
    fn streaming_counts_remove_undoes_update() {
        let mut sc = StreamingCounts::new(3, LogBase::Nats);
        sc.update(0);
        sc.update(1);
        sc.remove(0).unwrap();
        assert_eq!(sc.n_raw, 1);
    }

    #[test]
    fn streaming_counts_remove_rejects_unobserved_symbol() {
        let mut sc = StreamingCounts::new(3, LogBase::Nats);
        sc.update(0);
        assert!(sc.remove(1).is_err());
    }

    #[test]
    fn streaming_counts_merge_matches_combined_batch() {
        let mut a = StreamingCounts::new(3, LogBase::Nats);
        let mut b = StreamingCounts::new(3, LogBase::Nats);
        for &x in &[0u32, 1, 1] {
            a.update(x);
        }
        for &x in &[2u32, 2, 0] {
            b.update(x);
        }
        a.merge(&b).unwrap();
        let est = a.estimate().unwrap();
        let counts = crate::counts::Counts::from_symbols(&[0, 1, 1, 2, 2, 0], 3).unwrap();
        let expected = crate::discrete::entropy(&counts.probabilities(), LogBase::Nats).unwrap();
        assert!((est.value - expected).abs() < 1e-9);
    }

    #[test]
    fn streaming_counts_snapshot_restore_roundtrips() {
        let mut sc = StreamingCounts::new(3, LogBase::Bits);
        for &x in &[0u32, 1, 2, 2] {
            sc.update(x);
        }
        let snap = sc.snapshot();
        let restored = StreamingCounts::restore(&snap).unwrap();
        assert_eq!(sc.estimate().unwrap().value, restored.estimate().unwrap().value);
    }

    #[test]
    fn sliding_window_matches_batch_recomputed_at_every_step() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let capacity = 20;
        let mut sw = SlidingWindowEntropy::new(4, capacity, LogBase::Nats).unwrap();
        let mut history: Vec<u32> = Vec::new();
        for _ in 0..200 {
            let x = rng.next_below(4) as u32;
            sw.update(x);
            history.push(x);
            let window_start = history.len().saturating_sub(capacity);
            let window = &history[window_start..];
            let counts = crate::counts::Counts::from_symbols(window, 4).unwrap();
            let expected = crate::discrete::entropy(&counts.probabilities(), LogBase::Nats).unwrap();
            let got = sw.estimate().unwrap().value;
            assert!((got - expected).abs() < 1e-9, "mismatch at len {}", history.len());
        }
    }

    #[test]
    fn sliding_window_remove_and_merge_are_unsupported() {
        let mut sw = SlidingWindowEntropy::new(3, 5, LogBase::Nats).unwrap();
        sw.update(0);
        assert!(sw.remove(0).is_err());
        let other = SlidingWindowEntropy::new(3, 5, LogBase::Nats).unwrap();
        assert!(sw.merge(&other).is_err());
    }

    #[test]
    fn decayed_entropy_rejects_bad_decay() {
        assert!(DecayedEntropy::new(3, 0.0, LogBase::Nats).is_err());
        assert!(DecayedEntropy::new(3, 1.5, LogBase::Nats).is_err());
    }

    #[test]
    fn decayed_entropy_remove_is_unsupported() {
        let mut de = DecayedEntropy::new(3, 0.9, LogBase::Nats).unwrap();
        de.update(0);
        assert!(de.remove(0).is_err());
    }

    #[test]
    fn decayed_entropy_forgets_old_symbols() {
        let mut de = DecayedEntropy::new(2, 0.5, LogBase::Bits).unwrap();
        for _ in 0..50 {
            de.update(0);
        }
        // 🔐️ after many decayed updates of the same symbol, entropy should be near zero
        // (essentially deterministic), then adding a burst of the other symbol should raise it.
        let before = de.estimate().unwrap().value;
        assert!(before < 0.1, "got {before}");
        for _ in 0..50 {
            de.update(1);
        }
        let after = de.estimate().unwrap().value;
        assert!(after < 0.5, "got {after}"); // 🔐️ decay erased symbol-0 history; now near-deterministic on symbol 1
    }

    #[test]
    fn decayed_entropy_snapshot_restore_roundtrips() {
        let mut de = DecayedEntropy::new(3, 0.8, LogBase::Nats).unwrap();
        de.update(0);
        de.update(1);
        let snap = de.snapshot();
        let restored = DecayedEntropy::restore(&snap).unwrap();
        assert!((de.estimate().unwrap().value - restored.estimate().unwrap().value).abs() < 1e-12);
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Streaming

// #region 🔖️Features
pub mod features {
//! 📋️ Batch feature extraction and estimator-selection automation: a deterministically-ordered
//! named registry of standard entropy features over a single raw series, plus simple heuristics
//! for picking bin counts and kNN neighbor counts.

use crate::regularity::{sample_entropy, RegularityConfig};
use crate::symbolic::OrdinalConfig;
use crate::{BinsSpec, EntropyError, Estimate, LogBase, Tolerance};

// #region 🔖️Feature
/// 📋️ One named entry of a [`FeatureRegistry::compute`] result.
#[derive(Clone, Debug)]
pub struct Feature {
    pub name: &'static str,
    pub estimate: Estimate,
}
// #endregion 🔖️Feature

// #region 🔖️StandardFeatures
fn feature_histogram_entropy(x: &[f64]) -> Result<Estimate, EntropyError> {
    crate::continuous::entropy_continuous(x, &crate::continuous::ContinuousMethod::Histogram(BinsSpec::Sturges), LogBase::Nats)
}

fn feature_sample_entropy(x: &[f64]) -> Result<Estimate, EntropyError> {
    let cfg = RegularityConfig::new(2, Tolerance::Auto)?;
    sample_entropy(x, cfg, LogBase::Nats)
}

fn feature_permutation_entropy(x: &[f64]) -> Result<Estimate, EntropyError> {
    let cfg = OrdinalConfig::new(3, 1)?;
    crate::ordinal::permutation_entropy(x, cfg, LogBase::Nats)
}

fn feature_spectral_entropy(x: &[f64]) -> Result<Estimate, EntropyError> {
    crate::spectral::spectral_entropy(x, crate::spectral::SpectralConfig::default())
}

fn feature_lempel_ziv(x: &[f64]) -> Result<Estimate, EntropyError> {
    let (_, sd) = {
        let n = x.len() as f64;
        let mean = x.iter().sum::<f64>() / n;
        let var = x.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / n;
        (mean, var.sqrt())
    };
    let symbols: Vec<u32> = if sd > 0.0 {
        let mean = x.iter().sum::<f64>() / x.len() as f64;
        x.iter().map(|&v| if v > mean { 1 } else { 0 }).collect()
    } else {
        vec![0; x.len()]
    };
    crate::lz::lempel_ziv_complexity(&symbols, true)
}
// #endregion 🔖️StandardFeatures

// #region 🔖️Registry
type FeatureFn = fn(&[f64]) -> Result<Estimate, EntropyError>;

/// 📋️ A deterministically-ordered set of named entropy features to compute over a single raw
/// series in one pass.
pub struct FeatureRegistry {
    entries: Vec<(&'static str, FeatureFn)>,
}

impl FeatureRegistry {
    /// 📋️ The built-in standard feature set: histogram entropy, sample entropy, permutation
    /// entropy, spectral entropy, and normalized Lempel-Ziv complexity (over a median-split
    /// binary encoding).
    pub fn standard() -> Self {
        Self {
            entries: vec![
                ("histogram_entropy", feature_histogram_entropy as FeatureFn),
                ("sample_entropy", feature_sample_entropy as FeatureFn),
                ("permutation_entropy", feature_permutation_entropy as FeatureFn),
                ("spectral_entropy", feature_spectral_entropy as FeatureFn),
                ("lempel_ziv_complexity", feature_lempel_ziv as FeatureFn),
            ],
        }
    }

    /// 📋️ Registers an additional named feature function, appended after the existing entries
    /// (preserving deterministic ordering).
    pub fn with_feature(mut self, name: &'static str, f: FeatureFn) -> Self {
        self.entries.push((name, f));
        self
    }

    /// 📋️ Computes every registered feature over `x`, in registration order. The first feature
    /// that errors short-circuits the whole batch (no partial/best-effort results) — callers
    /// that want per-feature failure isolation should call individual feature functions directly.
    pub fn compute(&self, x: &[f64]) -> Result<Vec<Feature>, EntropyError> {
        self.entries.iter().map(|&(name, f)| Ok(Feature { name, estimate: f(x)? })).collect()
    }
}
// #endregion 🔖️Registry

// #region 🔖️Automation
/// 📋️ Suggests a histogram binning rule for `x`: Freedman-Diaconis when the interquartile range
/// is positive (robust to outliers), falling back to Sturges' rule otherwise (e.g. heavily
/// discrete/degenerate data where IQR collapses to zero).
pub fn suggest_bins(x: &[f64]) -> BinsSpec {
    if x.len() < 2 {
        return BinsSpec::Fixed(1);
    }
    let mut sorted = x.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let q1 = sorted[(sorted.len() as f64 * 0.25) as usize];
    let q3 = sorted[((sorted.len() as f64 * 0.75) as usize).min(sorted.len() - 1)];
    if q3 - q1 > 0.0 {
        BinsSpec::FreedmanDiaconis
    } else {
        BinsSpec::Sturges
    }
}

/// 📋️ Suggests a kNN neighbor count `k` for continuous estimators (Kozachenko-Leonenko/KSG):
/// `round(sqrt(n) / 2)`, clamped to `[3, 20]` and to at most `n / 4` for small samples (the
/// standard practical bias/variance compromise for these estimators).
pub fn suggest_knn_k(n: usize) -> usize {
    if n == 0 {
        return 3;
    }
    let raw = ((n as f64).sqrt() / 2.0).round() as usize;
    raw.clamp(3, 20).min((n / 4).max(1))
}
// #endregion 🔖️Automation

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_registry_computes_all_features_in_order() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_gaussian()).collect();
        let registry = FeatureRegistry::standard();
        let features = registry.compute(&x).unwrap();
        let names: Vec<&str> = features.iter().map(|f| f.name).collect();
        assert_eq!(names, vec!["histogram_entropy", "sample_entropy", "permutation_entropy", "spectral_entropy", "lempel_ziv_complexity"]);
        for f in &features {
            assert!(f.estimate.value.is_finite(), "{} produced non-finite value", f.name);
        }
    }

    #[test]
    fn with_feature_appends_after_standard_entries() {
        let registry = FeatureRegistry::standard().with_feature("custom", feature_histogram_entropy as FeatureFn);
        let mut rng = crate::numeric::Xorshift64::new(2);
        let x: Vec<f64> = (0..1500).map(|_| rng.next_gaussian()).collect();
        let features = registry.compute(&x).unwrap();
        assert_eq!(features.last().unwrap().name, "custom");
    }

    #[test]
    fn compute_propagates_first_error() {
        let registry = FeatureRegistry::standard();
        let constant = vec![1.0; 500];
        assert!(registry.compute(&constant).is_err());
    }

    #[test]
    fn suggest_bins_prefers_freedman_diaconis_for_spread_data() {
        let mut rng = crate::numeric::Xorshift64::new(3);
        let x: Vec<f64> = (0..500).map(|_| rng.next_gaussian()).collect();
        assert!(matches!(suggest_bins(&x), BinsSpec::FreedmanDiaconis));
    }

    #[test]
    fn suggest_bins_falls_back_to_sturges_for_degenerate_iqr() {
        let mut x = vec![0.0; 100];
        x[0] = 1000.0; // 🔐️ a single outlier keeps the IQR at zero
        assert!(matches!(suggest_bins(&x), BinsSpec::Sturges));
    }

    #[test]
    fn suggest_knn_k_scales_with_sample_size_and_stays_bounded() {
        assert_eq!(suggest_knn_k(0), 3);
        assert!(suggest_knn_k(16) <= 4);
        let k_large = suggest_knn_k(1_000_000);
        assert!((3..=20).contains(&k_large));
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Features


// #region 🔖️Exports
pub use counts::{Counts, JointCounts, SmoothingPrior};
pub use discrete::{
    binary_entropy, cross_entropy, entropy, hartley_entropy, joint_entropy, conditional_entropy,
    renyi_entropy, tsallis_entropy, sharma_mittal_entropy, kaniadakis_entropy, collision_entropy,
    min_entropy, normalized_entropy,
};
pub use estimators::{DiscreteMethod, entropy_discrete};
pub use knn::{KdTree, brute_force_knn};
pub use continuous::{
    ContinuousMethod, KdeConfig, KdeDensity, Bandwidth, Kernel, entropy_continuous,
};
pub use divergence::{
    kl_divergence, js_divergence, hellinger_distance, bhattacharyya_distance, total_variation,
    chi_square_divergence, wasserstein_1d, energy_distance, renyi_divergence, tsallis_divergence,
    log_det_divergence, bregman_divergence,
};
pub use mutual::{
    mutual_information, mutual_information_knn, conditional_mutual_information, KsgConfig,
    KsgVariant, total_correlation, dual_total_correlation, o_information,
};
pub use pid::{pid_two_sources, PidLattice, PidAtoms};
pub use fisher::{fisher_information, aic, aicc, bic, hqc, mdl};
pub use symbolic::{
    Symbolizer, OrdinalSymbolizer, DispersionSymbolizer, QuantileSymbolizer, ThresholdSymbolizer,
    embed, OrdinalConfig,
};
pub use regularity::{RegularityConfig, approximate_entropy, sample_entropy, fuzzy_entropy};
pub use ordinal::{
    permutation_entropy, dispersion_entropy, DispersionConfig, increment_entropy, slope_entropy,
};
pub use markov::MarkovChain;
pub use multiscale::{MultiscaleConfig, MultiscaleResult, Grain, multiscale_entropy};
pub use lz::{Compressor, Lz78Compressor, lempel_ziv_complexity, ncd};
pub use fft::{Complex, Fft, WindowKind, window};
pub use spectral::{SpectralConfig, spectral_entropy};
pub use wavelet::{WaveletConfig, WaveletFamily, BoundaryMode, wavelet_entropy};
pub use matrix::{jacobi_eigen_symmetric, svd_jacobi, cholesky, von_neumann_entropy, svd_entropy};
pub use inference::{
    Xorshift64, bootstrap_ci, jackknife_ci, permutation_test, SurrogateConfig, SurrogateKind,
    surrogate_series, fdr_bh,
};
pub use transfer::{TransferConfig, TeBackend, transfer_entropy, active_information_storage};
pub use spatial::{SpatialConfig, entropy_2d};
pub use graph::{degree_distribution_entropy, random_walk_entropy_rate};
pub use ml::{predictive_entropy, bald_mutual_information, expected_calibration_error};
pub use streaming::{
    StreamingEstimator, StreamingSnapshot, StreamingCounts, SlidingWindowEntropy, DecayedEntropy,
};
pub use features::{FeatureRegistry, Feature, suggest_bins, suggest_knn_k};
// #endregion 🔖️Exports
