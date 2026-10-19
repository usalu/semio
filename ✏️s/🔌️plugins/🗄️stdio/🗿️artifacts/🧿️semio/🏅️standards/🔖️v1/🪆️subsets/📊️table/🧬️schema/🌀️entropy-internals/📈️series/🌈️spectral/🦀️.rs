//! 📶️ Welch-method spectral entropy: segment a real time series, window and FFT each segment,
//! average the one-sided power spectra into a periodogram, and treat the normalized periodogram
//! as a probability distribution over frequency bins whose Shannon entropy summarizes how
//! concentrated (tonal) vs. spread (noise-like) the signal's power is across frequency. See
//! Welch, P. (1967), "The use of fast Fourier transform for the estimation of power spectra."

use crate::schema::entropy_internals::fft::{window, Complex, Fft, WindowKind};
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::ConfidenceInterval;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Estimate;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::LogBase;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Warning;

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

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn validate_overlap(overlap: f64) -> Result<(), EntropyError> {
    if !overlap.is_finite() || !(0.0..1.0).contains(&overlap) {
        return Err(EntropyError::InvalidConfig { field: "overlap", reason: "must be a finite fraction in [0, 1)" });
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
        return Err(EntropyError::InvalidConfig { field: "segment_len", reason: "must not exceed input length" });
    }

    let hop = (((segment_len as f64) * (1.0 - cfg.overlap)).round().max(1.0)) as usize;
    let win = window(cfg.window, segment_len);
    let plan = Fft::new(segment_len);
    let n_bins = segment_len / 2 + 1;

    let mut power_sum = vec![0.0_f64; n_bins];
    let mut n_segments = 0usize;
    let mut start = 0usize;
    while start + segment_len <= x.len() {
        let segment: Vec<Complex> = x[start..start + segment_len].iter().zip(win.iter()).map(|(&xv, &wv)| Complex::new(xv * wv, 0.0)).collect();
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
                return Err(EntropyError::InvalidConfig { field: "band", reason: "must satisfy 0 <= lo < hi <= 0.5" });
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

    let entropy_nats = semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::discrete::entropy(&p, LogBase::Nats)?;
    let value = if cfg.normalize {
        if bins <= 1 {
            0.0
        } else {
            entropy_nats / (bins as f64).ln()
        }
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
        diagnostics: vec![("segments", n_segments as f64), ("segment_len", segment_len as f64), ("bins", bins as f64)],
    })
}
// #endregion 🔖️Welch

// #region 🔖️Tests
#[cfg(test)]
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
