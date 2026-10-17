//! 📊️ Effect estimation: the effect estimate and bootstrap options, the naive difference, the g-formula and inverse probability weighting estimators. See [`super`].

use super::*;

// #region 🔖️Estimation
/// 📏️ A point estimate with an optional bootstrap percentile confidence interval.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EffectEstimate {
    pub estimate: f64,
    pub ci: Option<(f64, f64)>,
    pub ci_level: f64,
}

/// ⚙️ Bootstrap resampling options: replicate count, seed, and the two-sided confidence level.
#[derive(Clone, Copy, Debug)]
pub struct BootstrapOptions {
    pub replicates: usize,
    pub seed: u64,
    pub level: f64,
}

/// ⚙️ Common estimation options; treatment/outcome columns are continuous, treatment coded 0/1.
#[derive(Clone, Debug, Default)]
pub struct EstimationOptions {
    pub bootstrap: Option<BootstrapOptions>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bootstrap_ci(
    data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table,
    opts: &BootstrapOptions,
    point_fn: &dyn Fn(&semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table) -> Result<f64, CausalError>,
) -> (f64, f64) {
    let mut rng = semio_framework_geometry::random::Rng::from_seed(opts.seed);
    let n = data.n_rows();
    let mut estimates = Vec::with_capacity(opts.replicates);
    for _ in 0..opts.replicates {
        let indices: Vec<usize> = (0..n).map(|_| rng.next_range(0, n as u64) as usize).collect();
        if let Ok(resampled) = data.select_rows(&indices) {
            if let Ok(estimate) = point_fn(&resampled) {
                estimates.push(estimate);
            }
        }
    }
    if estimates.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    estimates.sort_by(|a, b| a.partial_cmp(b).expect("bootstrap estimates are finite"));
    let alpha = 1.0 - opts.level;
    let lo_idx = (((alpha / 2.0) * estimates.len() as f64) as usize).min(estimates.len() - 1);
    let hi_idx = (((1.0 - alpha / 2.0) * estimates.len() as f64) as usize).min(estimates.len() - 1);
    (estimates[lo_idx], estimates[hi_idx])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_estimate(
    point: f64,
    data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table,
    opts: &EstimationOptions,
    point_fn: &dyn Fn(&semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table) -> Result<f64, CausalError>,
) -> EffectEstimate {
    match &opts.bootstrap {
        Some(bootstrap_opts) => {
            let (lo, hi) = bootstrap_ci(data, bootstrap_opts, point_fn);
            EffectEstimate { estimate: point, ci: Some((lo, hi)), ci_level: bootstrap_opts.level }
        }
        None => EffectEstimate { estimate: point, ci: None, ci_level: 0.0 },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn naive_difference_point(data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, treatment: usize, outcome: usize) -> Result<f64, CausalError> {
    let t = data.continuous(treatment)?;
    let y = data.continuous(outcome)?;
    let (mut sum1, mut n1, mut sum0, mut n0) = (0.0, 0usize, 0.0, 0usize);
    for (&ti, &yi) in t.iter().zip(y) {
        if ti > 0.5 {
            sum1 += yi;
            n1 += 1;
        } else {
            sum0 += yi;
            n0 += 1;
        }
    }
    if n1 == 0 || n0 == 0 {
        return Err(CausalError::InvalidQuery("naive_difference needs both treated and control rows".to_string()));
    }
    Ok(sum1 / n1 as f64 - sum0 / n0 as f64)
}

/// 📏️ Level-1 baseline: `E[y|t=1] − E[y|t=0]` — biased under confounding, useful as a comparison point.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn naive_difference(data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, treatment: usize, outcome: usize, opts: &EstimationOptions) -> Result<EffectEstimate, CausalError> {
    let point = naive_difference_point(data, treatment, outcome)?;
    Ok(wrap_estimate(point, data, opts, &|d| naive_difference_point(d, treatment, outcome)))
}

#[allow(clippy::needless_range_loop, reason = "row indexes both the MatD design matrix by (row, col) and a values/t slice — enumerate() would only remove one of the two")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn design_with_treatment(
    data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table,
    treatment: usize,
    covariates: &[usize],
    treatment_value: Option<f64>,
) -> Result<semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD, CausalError> {
    let n = data.n_rows();
    let mut design = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD::zeros(n, covariates.len() + 1);
    let t = data.continuous(treatment)?;
    for row in 0..n {
        design.set(row, 0, treatment_value.unwrap_or(t[row]));
    }
    for (col, &c) in covariates.iter().enumerate() {
        let values = data.continuous(c)?;
        for row in 0..n {
            design.set(row, col + 1, values[row]);
        }
    }
    Ok(design)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn g_formula_point(data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, treatment: usize, outcome: usize, covariates: &[usize]) -> Result<f64, CausalError> {
    let y = data.continuous(outcome)?;
    let design = design_with_treatment(data, treatment, covariates, None)?;
    let fit = semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::ols(&design, y, true)?;
    let predict_mean = |design: &semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD| -> f64 {
        let n = design.rows;
        let p = design.cols;
        let total: f64 = (0..n).map(|row| fit.coefficients[0] + (0..p).map(|col| fit.coefficients[col + 1] * design.get(row, col)).sum::<f64>()).sum();
        total / n as f64
    };
    let design_t1 = design_with_treatment(data, treatment, covariates, Some(1.0))?;
    let design_t0 = design_with_treatment(data, treatment, covariates, Some(0.0))?;
    Ok(predict_mean(&design_t1) - predict_mean(&design_t0))
}

/// 📏️ G-formula / regression-adjustment ATE: fit `y ~ t + covariates` by OLS, then average the
/// model's predicted `y` at `t=1` minus at `t=0` over the empirical covariate distribution.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn g_formula_ate(data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, treatment: usize, outcome: usize, covariates: &[usize], opts: &EstimationOptions) -> Result<EffectEstimate, CausalError> {
    let point = g_formula_point(data, treatment, outcome, covariates)?;
    Ok(wrap_estimate(point, data, opts, &|d| g_formula_point(d, treatment, outcome, covariates)))
}

#[allow(clippy::needless_range_loop, reason = "row indexes both the MatD design matrix by (row, col) and the values slice — enumerate() would only remove the values index")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ipw_point(data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, treatment: usize, outcome: usize, covariates: &[usize]) -> Result<f64, CausalError> {
    const EPS: f64 = 1e-6;
    let n = data.n_rows();
    let mut design = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD::zeros(n, covariates.len());
    for (col, &c) in covariates.iter().enumerate() {
        let values = data.continuous(c)?;
        for row in 0..n {
            design.set(row, col, values[row]);
        }
    }
    let t = data.continuous(treatment)?;
    let fit = semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::logistic(&design, t, true)?;
    let propensity = semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::logistic_predict(&fit, &design, true)?;
    let y = data.continuous(outcome)?;
    let (mut num1, mut den1, mut num0, mut den0) = (0.0, 0.0, 0.0, 0.0);
    for row in 0..n {
        let e = propensity[row].clamp(EPS, 1.0 - EPS);
        if t[row] > 0.5 {
            num1 += y[row] / e;
            den1 += 1.0 / e;
        } else {
            num0 += y[row] / (1.0 - e);
            den0 += 1.0 / (1.0 - e);
        }
    }
    if den1 <= 0.0 || den0 <= 0.0 {
        return Err(CausalError::InvalidQuery("ipw_ate needs both treated and control rows".to_string()));
    }
    Ok(num1 / den1 - num0 / den0)
}

/// 📏️ Inverse-probability weighting ATE: logistic propensity model, Hájek (normalized-weight)
/// estimator, propensities clipped to `[1e-6, 1-1e-6]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ipw_ate(data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, treatment: usize, outcome: usize, covariates: &[usize], opts: &EstimationOptions) -> Result<EffectEstimate, CausalError> {
    let point = ipw_point(data, treatment, outcome, covariates)?;
    Ok(wrap_estimate(point, data, opts, &|d| ipw_point(d, treatment, outcome, covariates)))
}
// #endregion 🔖️Estimation
