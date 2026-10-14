//! 🧪️ Statistical inference on top of any entropy/information statistic: resampling confidence
//! intervals (bootstrap, jackknife), permutation hypothesis tests, surrogate-data generation for
//! null-model construction, and multiple-comparisons correction. Every source of randomness here
//! is an explicit `u64` seed fed through [`Xorshift64`] — never wall-clock time — so a full
//! surrogate/permutation batch is exactly reproducible from one seed.

use crate::schema::entropy_internals::fft::{Complex, Fft};
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::inverse_normal_cdf;
pub use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::Xorshift64;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::ConfidenceInterval;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;

// #region 🔖️ConfidenceIntervals
/// 🧪️ Linear-interpolated percentile of an already-sorted slice (`p` in `[0, 1]`), interpolating
/// between the two nearest order statistics when `p * (n - 1)` falls between integer indices.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn bootstrap_ci(data: &[f64], statistic: impl Fn(&[f64]) -> f64, n_bootstrap: usize, level: f64, seed: u64) -> Result<ConfidenceInterval, EntropyError> {
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn jackknife_ci(data: &[f64], statistic: impl Fn(&[f64]) -> f64, level: f64) -> Result<ConfidenceInterval, EntropyError> {
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn permutation_test(x: &[f64], y: &[f64], statistic: impl Fn(&[f64], &[f64]) -> f64, n_permutations: usize, seed: u64) -> Result<f64, EntropyError> {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn circular_shift_surrogate(x: &[f64], rng: &mut Xorshift64) -> Vec<f64> {
    let n = x.len();
    let k = 1 + rng.next_below(n - 1);
    (0..n).map(|i| x[(i + k) % n]).collect()
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn randomize_phases(spectrum: &mut [Complex], rng: &mut Xorshift64) {
    let n = spectrum.len();
    let half = (n - 1) / 2;
    for k in 1..=half {
        let rotation = Complex::from_polar(1.0, rng.next_f64() * 2.0 * core::f64::consts::PI);
        spectrum[k] = spectrum[k] * rotation;
        spectrum[n - k] = spectrum[k].conj();
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
        let adjusted_spectrum: Vec<Complex> = ranked_spectrum.iter().zip(original_magnitude.iter()).map(|(c, &magnitude)| Complex::from_polar(magnitude, c.arg())).collect();
        current = fft.inverse(&adjusted_spectrum).iter().map(|c| c.re).collect();
    }
    rank_adjusted
}

/// 🧪️ Generates `cfg.count` surrogate series of the same length as `x`, per `cfg.kind`. Uses a
/// single [`Xorshift64`] seeded from `cfg.seed` and advanced across every generated surrogate
/// (never reseeded per-surrogate) so the whole batch is reproducible from one seed.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
