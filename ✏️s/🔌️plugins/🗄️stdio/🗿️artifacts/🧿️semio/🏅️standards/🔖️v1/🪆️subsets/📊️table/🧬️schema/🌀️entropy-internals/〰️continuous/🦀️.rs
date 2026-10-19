//! 📈️ Differential (continuous) entropy estimators: histogram, Gaussian KDE (leave-one-out
//! plug-in), Kozachenko-Leonenko kNN, Vasicek/Correa m-spacing, and the Gaussian closed form.

use crate::schema::entropy_internals::knn::KdTree;
use crate::schema::entropy_internals::numeric::{digamma, log_sum_exp, neumaier_sum, x_ln_x};
use crate::schema::entropy_internals::{BinsSpec, ConfidenceInterval, EntropyError, Estimate, LogBase, Metric, Warning};

// #region 🔖️Shared
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn mean_and_sd(x: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let mean = neumaier_sum(x.iter().copied()) / n;
    let var = neumaier_sum(x.iter().map(|&v| (v - mean).powi(2))) / n;
    (mean, var.sqrt())
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
        let log_terms: Vec<f64> = self.data.iter().map(|&xi| self.kernel.log_density_contribution((x - xi) / self.h)).collect();
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
            let log_terms: Vec<f64> = (0..n).filter(|&j| j != i).map(|j| self.kernel.log_density_contribution((self.data[i] - self.data[j]) / self.h)).collect();
            let log_density = log_sum_exp(&log_terms) - (nf - 1.0).ln() - self.h.ln();
            sum_log_density += log_density;
        }
        let nats = -sum_log_density / nf;
        Ok(Estimate { value: base.from_nats(nats), base, method: "kde_loo", n, n_effective: nf, std_error: None, ci: None::<ConfidenceInterval>, warnings: Vec::new(), diagnostics: vec![("bandwidth", self.h)] })
    }
}
// #endregion 🔖️Kde

// #region 🔖️Histogram
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn histogram_entropy_nats(x: &[f64], bins: &BinsSpec) -> Result<(f64, usize), EntropyError> {
    let n = x.len() as f64;
    let (min, max) = x.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
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
                    let skew = if sd > 0.0 { neumaier_sum(x.iter().map(|&v| ((v - mean) / sd).powi(3))) / n } else { 0.0 };
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sorted_with_clamped(x: &[f64], i: i64) -> f64 {
    let n = x.len() as i64;
    let clamped = i.clamp(1, n);
    x[(clamped - 1) as usize]
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    Ok(Estimate { value: base.from_nats(nats), base, method: method_name(method), n, n_effective: n as f64, std_error: None, ci: None::<ConfidenceInterval>, warnings, diagnostics })
}
// #endregion 🔖️Dispatch

// #region 🔖️Tests
#[cfg(test)]
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
