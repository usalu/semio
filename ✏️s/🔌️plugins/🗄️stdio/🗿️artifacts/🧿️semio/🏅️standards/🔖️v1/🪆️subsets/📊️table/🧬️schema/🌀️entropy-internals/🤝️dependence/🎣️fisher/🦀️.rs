//! 📉️ Fisher information and information-criterion model-selection scores. Fisher information is
//! recovered numerically from a supplied log-likelihood via a central second difference; the
//! criteria (AIC/AICc/BIC/HQC/MDL) are closed-form penalized-likelihood scores taking an
//! already-computed `ln_L` and parameter/sample counts — no external crate, no `Estimate` wrapper
//! since these are exact-given-inputs, not estimated-from-samples quantities.

use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;

// #region 🔖️FisherInformation
/// 📉️ Numerical (observed) Fisher information via a central second-difference of the supplied
/// log-likelihood function at `theta`: `-(L(theta+h) - 2*L(theta) + L(theta-h)) / h^2`, with a
/// fixed step `h = 1e-4 * theta.abs().max(1.0)` (scale-aware so it works across parameter
/// magnitudes).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn aic(log_likelihood: f64, num_params: usize) -> f64 {
    2.0 * num_params as f64 - 2.0 * log_likelihood
}

/// 📉️ Corrected AIC for small sample sizes: `aic + (2*k*(k+1)) / (n - k - 1)`. Errors with
/// [`EntropyError::InvalidConfig`] if `n <= k + 1` (undefined).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn aicc(log_likelihood: f64, num_params: usize, n: usize) -> Result<f64, EntropyError> {
    let k = num_params as f64;
    let n = n as f64;
    if n <= k + 1.0 {
        return Err(EntropyError::InvalidConfig { field: "n", reason: "AICc is undefined when n <= num_params + 1" });
    }
    Ok(aic(log_likelihood, num_params) + (2.0 * k * (k + 1.0)) / (n - k - 1.0))
}

/// 📉️ Bayesian information criterion: `k * ln(n) - 2*ln_L`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn bic(log_likelihood: f64, num_params: usize, n: usize) -> f64 {
    num_params as f64 * (n as f64).ln() - 2.0 * log_likelihood
}

/// 📉️ Hannan-Quinn criterion: `2*k*ln(ln(n)) - 2*ln_L`. For `n <= e` (so `ln(ln(n))` is undefined
/// or non-positive) the penalty term is treated as `0.0` rather than producing NaN/negative
/// infinity — a small-`n` edge case, not a claim that HQC is well-defined there.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn hqc(log_likelihood: f64, num_params: usize, n: usize) -> f64 {
    let penalty = if (n as f64) <= core::f64::consts::E { 0.0 } else { 2.0 * num_params as f64 * (n as f64).ln().ln() };
    penalty - 2.0 * log_likelihood
}

/// 📉️ A simple two-part-code minimum-description-length approximation:
/// `-ln_L + 0.5 * k * ln(n)` (equivalent to BIC/2 in the log-likelihood term's convention).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn mdl(log_likelihood: f64, num_params: usize, n: usize) -> f64 {
    -log_likelihood + 0.5 * num_params as f64 * (n as f64).ln()
}
// #endregion 🔖️Criteria

// #region 🔖️Tests
#[cfg(test)]
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
