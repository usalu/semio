//! 📏️ Probability divergences and distances: KL/JS/Rényi/Tsallis families, classical distances
//! (Hellinger, Bhattacharyya, total variation, chi-square), empirical Wasserstein-1D and energy
//! distance over raw samples, and a closure-based Bregman divergence for arbitrary convex `phi`.

use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::counts::validate_probabilities;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::neumaier_sum;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::LogBase;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Tolerances;

// #region 🔖️Shared
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn reverse_kl_divergence(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    kl_divergence(q, p, base)
}

/// 📏️ Jeffreys divergence, the symmetrized KL: `D(p||q) + D(q||p)`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn jeffreys_divergence(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    let a = kl_divergence(p, q, base)?;
    let b = kl_divergence(q, p, base)?;
    Ok(a + b)
}
// #endregion 🔖️KlFamily

// #region 🔖️JensenFamily
/// 📏️ Jensen-Shannon divergence: `0.5*D(p||m) + 0.5*D(q||m)` with `m = 0.5*(p+q)`. Always finite
/// and bounded by `ln(2)` in nats regardless of support overlap.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn js_divergence(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    let (p, q) = validate_pair(p, q)?;
    let m: Vec<f64> = p.iter().zip(q.iter()).map(|(&pi, &qi)| 0.5 * (pi + qi)).collect();
    let d_pm = kl_divergence(&p, &m, LogBase::Nats)?;
    let d_qm = kl_divergence(&q, &m, LogBase::Nats)?;
    Ok(base.from_nats(0.5 * d_pm + 0.5 * d_qm))
}

/// 📏️ Jensen-Shannon distance, the square root of [`js_divergence`] in nats (a true metric).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn js_distance(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    Ok(js_divergence(p, q, LogBase::Nats)?.sqrt())
}

/// 📏️ Weighted Jensen-Shannon divergence with mixture weight `pi_p` for `p` (`pi_q = 1 - pi_p`).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn hellinger_distance(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let (p, q) = validate_pair(p, q)?;
    let sum = neumaier_sum(p.iter().zip(q.iter()).map(|(&pi, &qi)| (pi.sqrt() - qi.sqrt()).powi(2)));
    Ok((0.5 * sum).max(0.0).sqrt())
}

/// 📏️ Bhattacharyya coefficient `BC(p,q) = sum sqrt(p_i q_i)`, in `[0, 1]`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn bhattacharyya_coefficient(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let (p, q) = validate_pair(p, q)?;
    Ok(neumaier_sum(p.iter().zip(q.iter()).map(|(&pi, &qi)| (pi * qi).sqrt())).clamp(0.0, 1.0))
}

/// 📏️ Bhattacharyya distance `-ln(BC(p,q))`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn bhattacharyya_distance(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let bc = bhattacharyya_coefficient(p, q)?;
    if bc <= 0.0 {
        return Ok(f64::INFINITY);
    }
    Ok(-bc.ln())
}

/// 📏️ Total variation distance: `0.5 * sum |p_i - q_i|`, bounded in `[0, 1]`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn total_variation(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let (p, q) = validate_pair(p, q)?;
    Ok(0.5 * neumaier_sum(p.iter().zip(q.iter()).map(|(&pi, &qi)| (pi - qi).abs())))
}

/// 📏️ Pearson chi-square divergence: `sum (p_i - q_i)^2 / q_i`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn neyman_chi_square_divergence(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    chi_square_divergence(q, p)
}
// #endregion 🔖️ClassicalDistances

// #region 🔖️EmpiricalDistances
/// 📏️ Empirical 1-D Wasserstein (earth-mover) distance between two raw sample sets, computed as
/// the area between their empirical CDFs (`integral |F_x(t) - F_y(t)| dt`), which is exact and
/// requires no binning.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn log_det_divergence(cov_p: &[f64], cov_q: &[f64], n: usize) -> Result<f64, EntropyError> {
    let ld_p = crate::schema::entropy_internals::matrix::log_det(cov_p, n)?;
    let ld_q = crate::schema::entropy_internals::matrix::log_det(cov_q, n)?;
    let l_q = crate::schema::entropy_internals::matrix::cholesky(cov_q, n)?;
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
