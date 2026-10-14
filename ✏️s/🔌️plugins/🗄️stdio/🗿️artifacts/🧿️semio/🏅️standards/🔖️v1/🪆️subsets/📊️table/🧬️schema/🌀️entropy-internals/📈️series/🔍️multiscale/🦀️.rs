//! 📶️ Multiscale entropy: coarse-grain a series at increasing scales and report a chosen
//! regularity/ordinal entropy at each scale, summarized by a complexity index (mean entropy
//! across valid scales).

use crate::schema::entropy_internals::ordinal::{dispersion_entropy, permutation_entropy, DispersionConfig};
use crate::schema::entropy_internals::regularity::{fuzzy_entropy, sample_entropy, RegularityConfig};
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::symbolic::OrdinalConfig;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Estimate;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::LogBase;

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

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
