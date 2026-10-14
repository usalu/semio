//! 🎼️ Symbol-based time-series entropies built on `symbolic.rs`: permutation entropy (Bandt-
//! Pompe), dispersion entropy, increment entropy, and slope entropy — each reduces a real-valued
//! series to a finite alphabet, then reports the Shannon entropy of the resulting symbol
//! distribution.

use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::checked_state_count;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::neumaier_sum;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::x_ln_x;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::symbolic::DispersionSymbolizer;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::symbolic::OrdinalConfig;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::symbolic::OrdinalSymbolizer;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::symbolic::Symbolizer;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::ConfidenceInterval;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Estimate;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::LogBase;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Warning;

// #region 🔖️Shared
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    Ok(Estimate { value: base.from_nats(nats), base, method, n: symbols.len(), n_effective: symbols.len() as f64, std_error: None, ci: None::<ConfidenceInterval>, warnings, diagnostics })
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_sd(x: &[f64]) -> f64 {
    let n = x.len() as f64;
    let mean = neumaier_sum(x.iter().copied()) / n;
    (neumaier_sum(x.iter().map(|&v| (v - mean).powi(2))) / n).sqrt()
}
// #endregion 🔖️Shared

// #region 🔖️Permutation
/// 🎼️ Bandt-Pompe permutation entropy: Shannon entropy of the ordinal-pattern distribution
/// produced by [`OrdinalSymbolizer`].
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn dispersion_entropy(x: &[f64], cfg: DispersionConfig, base: LogBase) -> Result<Estimate, EntropyError> {
    let symbolizer = DispersionSymbolizer { classes: cfg.classes, dim: cfg.dim, tau: cfg.tau };
    let symbols = symbolizer.symbolize(x)?;
    symbol_distribution_entropy(&symbols, symbolizer.alphabet_size(), base, "dispersion_entropy", vec![("classes", cfg.classes as f64), ("dim", cfg.dim as f64), ("tau", cfg.tau as f64)])
}
// #endregion 🔖️Dispersion

// #region 🔖️Increment
/// 🎼️ Increment entropy: each successive difference `x[i+1] - x[i]` is encoded as a signed
/// magnitude symbol in `0..(2*levels+1)` (`levels` per-side magnitude buckets sized by quantiles
/// of `|increment| / sd(increments)`, symbol `levels` reserved for an exact-zero increment),
/// `word_length` consecutive increment-symbols are packed via mixed-radix into one word, and the
/// Shannon entropy of the resulting word distribution is reported.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    let total = checked_state_count(&vec![alphabet; word_length]).ok_or(EntropyError::InvalidConfig { field: "word_length", reason: "resulting alphabet overflows" })?;
    if total > u32::MAX as u128 {
        return Err(EntropyError::InvalidConfig { field: "word_length", reason: "resulting alphabet exceeds u32::MAX" });
    }
    let words: Vec<u32> = symbols.windows(word_length).map(|w| w.iter().fold(0u64, |acc, &s| acc * alphabet as u64 + s as u64) as u32).collect();

    symbol_distribution_entropy(&words, total as usize, base, "increment_entropy", vec![("word_length", word_length as f64), ("levels", levels as f64)])
}
// #endregion 🔖️Increment

// #region 🔖️Slope
/// 🎼️ Slope entropy: each successive slope `atan2(x[i+1] - x[i], 1)` (in radians) is classified
/// into one of 5 symbols by two angular thresholds `(gamma1, gamma2)` with `0 < gamma1 < gamma2 <
/// pi/2`: symbol `0`/`4` for a steep negative/positive slope beyond `gamma2`, `1`/`3` for a
/// shallow negative/positive slope in `(gamma1, gamma2]`, and `2` for a near-flat slope within
/// `gamma1`. `dim` consecutive slope-symbols are packed via mixed-radix (base 5) into one word.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
