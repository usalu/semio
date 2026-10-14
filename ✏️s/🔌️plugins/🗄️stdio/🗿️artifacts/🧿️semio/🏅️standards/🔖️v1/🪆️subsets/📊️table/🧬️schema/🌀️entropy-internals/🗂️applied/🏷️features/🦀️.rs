//! 📋️ Batch feature extraction and estimator-selection automation: a deterministically-ordered
//! named registry of standard entropy features over a single raw series, plus simple heuristics
//! for picking bin counts and kNN neighbor counts.

use semio_s_artifact_stdio_semio_table_series::schema::entropy_internals::regularity::sample_entropy;
use semio_s_artifact_stdio_semio_table_series::schema::entropy_internals::regularity::RegularityConfig;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::symbolic::OrdinalConfig;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::BinsSpec;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Estimate;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::LogBase;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Tolerance;

// #region 🔖️Feature
/// 📋️ One named entry of a [`FeatureRegistry::compute`] result.
#[derive(Clone, Debug)]
pub struct Feature {
    pub name: &'static str,
    pub estimate: Estimate,
}
// #endregion 🔖️Feature

// #region 🔖️StandardFeatures
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn feature_histogram_entropy(x: &[f64]) -> Result<Estimate, EntropyError> {
    semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::continuous::entropy_continuous(x, &semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::continuous::ContinuousMethod::Histogram(BinsSpec::Sturges), LogBase::Nats)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn feature_sample_entropy(x: &[f64]) -> Result<Estimate, EntropyError> {
    let cfg = RegularityConfig::new(2, Tolerance::Auto)?;
    sample_entropy(x, cfg, LogBase::Nats)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn feature_permutation_entropy(x: &[f64]) -> Result<Estimate, EntropyError> {
    let cfg = OrdinalConfig::new(3, 1)?;
    semio_s_artifact_stdio_semio_table_series::schema::entropy_internals::ordinal::permutation_entropy(x, cfg, LogBase::Nats)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn feature_spectral_entropy(x: &[f64]) -> Result<Estimate, EntropyError> {
    semio_s_artifact_stdio_semio_table_series::schema::entropy_internals::spectral::spectral_entropy(x, semio_s_artifact_stdio_semio_table_series::schema::entropy_internals::spectral::SpectralConfig::default())
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    semio_s_artifact_stdio_semio_table_series::schema::entropy_internals::lz::lempel_ziv_complexity(&symbols, true)
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn with_feature(mut self, name: &'static str, f: FeatureFn) -> Self {
        self.entries.push((name, f));
        self
    }

    /// 📋️ Computes every registered feature over `x`, in registration order. The first feature
    /// that errors short-circuits the whole batch (no partial/best-effort results) — callers
    /// that want per-feature failure isolation should call individual feature functions directly.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compute(&self, x: &[f64]) -> Result<Vec<Feature>, EntropyError> {
        self.entries.iter().map(|&(name, f)| Ok(Feature { name, estimate: f(x)? })).collect()
    }
}
// #endregion 🔖️Registry

// #region 🔖️Automation
/// 📋️ Suggests a histogram binning rule for `x`: Freedman-Diaconis when the interquartile range
/// is positive (robust to outliers), falling back to Sturges' rule otherwise (e.g. heavily
/// discrete/degenerate data where IQR collapses to zero).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
