//! 🔁️ Regularity/complexity measures over a single scalar time series: Approximate Entropy
//! (ApEn), Sample Entropy (SampEn), and Fuzzy Entropy (FuzzyEn). All three compare
//! time-delay-embedded template vectors (via [`crate::schema::entropy_internals::symbolic::embed`]) under a Chebyshev
//! tolerance radius `r` and differ only in how "matching" is counted and whether self-matches
//! are included — see each function's docstring for the exact convention.

use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::neumaier_sum;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::ConfidenceInterval;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Estimate;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::LogBase;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Tolerance;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Warning;

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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(m: usize, r: Tolerance) -> Result<Self, EntropyError> {
        if m < 1 {
            return Err(EntropyError::InvalidConfig { field: "m", reason: "embedding dimension must be at least 1" });
        }
        Ok(Self { m, r })
    }
}

/// 🔁️ Sample standard deviation (`n - 1` denominator). Rejects a degenerate (constant, or
/// too-short-to-vary) series since it can never anchor a meaningful proportional tolerance.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn chebyshev(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&x, &y)| (x - y).abs()).fold(0.0_f64, f64::max)
}

/// 🔁️ Template minus its own mean (the Chen et al. fuzzy-entropy de-trending step).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn demean(t: &[f64]) -> Vec<f64> {
    let mean = neumaier_sum(t.iter().copied()) / t.len() as f64;
    t.iter().map(|&v| v - mean).collect()
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn small_sample_warning(n: usize) -> Option<Warning> {
    (n < 100).then_some(Warning::SmallSample { n, recommended: 100 })
}
// #endregion 🔖️Distance

// #region 🔖️ApproximateEntropy
/// 🔁️ `Phi(dim) = mean_i(ln(C_i))` where `C_i` counts matches INCLUDING the self-match `j == i` —
/// the defining (if statistically awkward) ApEn convention that SampEn below deliberately drops.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn approximate_entropy(x: &[f64], cfg: RegularityConfig, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if x.len() < cfg.m + 2 {
        return Err(EntropyError::InsufficientData { what: "approximate_entropy", needed: cfg.m + 2, actual: x.len() });
    }
    let r = resolve_tolerance(x, cfg.r)?;
    let templates_m = semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::symbolic::embed(x, cfg.m, 1)?;
    let templates_m1 = semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::symbolic::embed(x, cfg.m + 1, 1)?;
    let apen_nats = apen_phi(&templates_m, r) - apen_phi(&templates_m1, r);

    let mut warnings = Vec::new();
    warnings.extend(small_sample_warning(x.len()));

    Ok(Estimate { value: base.from_nats(apen_nats), base, method: "approximate_entropy", n: x.len(), n_effective: x.len() as f64, std_error: None, ci: None::<ConfidenceInterval>, warnings, diagnostics: vec![("m", cfg.m as f64), ("r", r)] })
}
// #endregion 🔖️ApproximateEntropy

// #region 🔖️SharedEmbedding
/// 🔁️ A pair of `(length-m templates, length-(m+1) templates)` over the same shared index range.
type TemplatePair = (Vec<Vec<f64>>, Vec<Vec<f64>>);

/// 🔁️ Embeds at `m + 1` first to fix the valid start-index range, then embeds at `m` and
/// truncates to that SAME range, so SampEn/FuzzyEn compare the two lengths over identical
/// windows rather than the (larger) index range `m`-only embedding would otherwise allow.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn shared_templates(x: &[f64], m: usize) -> Result<TemplatePair, EntropyError> {
    let templates_m1 = semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::symbolic::embed(x, m + 1, 1)?;
    let k = templates_m1.len();
    let mut templates_m = semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::symbolic::embed(x, m, 1)?;
    templates_m.truncate(k);
    Ok((templates_m, templates_m1))
}
// #endregion 🔖️SharedEmbedding

// #region 🔖️SampleEntropy
/// 🔁️ Sample Entropy (Richman & Moorman 2000): `SampEn = -ln(A / B)`, where `B` counts
/// length-`m` matches and `A` counts length-`(m + 1)` matches among the SAME index pairs
/// `i < j` (self-matches structurally excluded, unlike ApEn). `B == 0` is undefined; `A == 0`
/// is reported as `+infinity` with a diagnostic warning rather than erroring.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    Ok(Estimate { value: base.from_nats(sampen_nats), base, method: "sample_entropy", n: x.len(), n_effective: x.len() as f64, std_error: None, ci: None::<ConfidenceInterval>, warnings, diagnostics: vec![("m", cfg.m as f64), ("r", r)] })
}
// #endregion 🔖️SampleEntropy

// #region 🔖️FuzzyEntropy
/// 🔁️ `Phi(dim) = (1 / (K*(K-1))) * sum_i sum_{j != i} mu(Chebyshev(T_i - mean(T_i), T_j -
/// mean(T_j)))` with Gaussian membership `mu(d) = exp(-(d/r)^2)`, replacing SampEn/ApEn's hard
/// `<= r` indicator with a smooth one (Chen et al. 2007).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    Ok(Estimate { value: base.from_nats(fuzzyen_nats), base, method: "fuzzy_entropy", n: x.len(), n_effective: x.len() as f64, std_error: None, ci: None::<ConfidenceInterval>, warnings, diagnostics: vec![("m", cfg.m as f64), ("r", r)] })
}
// #endregion 🔖️FuzzyEntropy

// #region 🔖️Tests
#[cfg(test)]
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
