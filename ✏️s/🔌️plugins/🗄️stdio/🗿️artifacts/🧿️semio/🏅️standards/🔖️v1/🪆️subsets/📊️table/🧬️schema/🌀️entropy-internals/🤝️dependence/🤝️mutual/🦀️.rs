//! 🔗️ Mutual information family: discrete plug-in/bias-corrected MI and conditional MI, KSG-1/
//! KSG-2 continuous MI, and multivariate generalizations (total correlation, dual total
//! correlation, O-information).

use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::counts::Counts;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::counts::JointCounts;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::estimators::entropy_discrete;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::estimators::DiscreteMethod;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::knn::KdTree;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::checked_state_count;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::clamp_near_zero;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::digamma;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::ConfidenceInterval;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Estimate;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::LogBase;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Metric;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Warning;

// #region 🔖️Packing
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn counts_to_u64(raw: &[f64]) -> Vec<u64> {
    raw.iter().map(|&c| c.round().max(0.0) as u64).collect()
}

/// 🔗️ Packs several aligned symbol sequences into one joint symbol via mixed-radix encoding,
/// checked against `u32` overflow.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn pack_symbols(parts: &[&[u32]], sizes: &[usize]) -> Result<(Vec<u32>, usize), EntropyError> {
    let total = checked_state_count(sizes).ok_or(EntropyError::InvalidConfig { field: "sizes", reason: "joint alphabet size overflows u128" })?;
    if total > u32::MAX as u128 {
        return Err(EntropyError::InvalidConfig { field: "sizes", reason: "joint alphabet size exceeds u32::MAX" });
    }
    let n = parts[0].len();
    let mut combined = vec![0u32; n];
    for i in 0..n {
        let mut acc: u64 = 0;
        for (part, &size) in parts.iter().zip(sizes.iter()) {
            acc = acc * size as u64 + part[i] as u64;
        }
        combined[i] = acc as u32;
    }
    Ok((combined, total as usize))
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn plugin_entropy_nats(symbols: &[u32], alphabet: usize) -> Result<f64, EntropyError> {
    let counts = Counts::from_symbols(symbols, alphabet)?;
    let est = entropy_discrete(&counts_to_u64(counts.raw()), DiscreteMethod::Plugin, LogBase::Nats)?;
    Ok(est.value)
}
// #endregion 🔖️Packing

// #region 🔖️DiscreteMi
/// 🔗️ Discrete mutual information `I(X;Y) = H(X) + H(Y) - H(X,Y)`, all three terms estimated
/// with the same bias-correction `method` for internal consistency.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn mutual_information(x: &[u32], y: &[u32], method: DiscreteMethod, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if x.len() != y.len() {
        return Err(EntropyError::LengthMismatch { expected: x.len(), actual: y.len() });
    }
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    let x_size = *x.iter().max().unwrap() as usize + 1;
    let y_size = *y.iter().max().unwrap() as usize + 1;
    let joint = JointCounts::from_pairs(x, y, x_size, y_size)?;
    let marg_x = Counts::from_symbols(x, x_size)?;
    let marg_y = Counts::from_symbols(y, y_size)?;

    let h_xy = entropy_discrete(&counts_to_u64(joint.as_counts().raw()), method, LogBase::Nats)?;
    let h_x = entropy_discrete(&counts_to_u64(marg_x.raw()), method, LogBase::Nats)?;
    let h_y = entropy_discrete(&counts_to_u64(marg_y.raw()), method, LogBase::Nats)?;

    let nats = clamp_near_zero(h_x.value + h_y.value - h_xy.value, 1e-9);
    let mut warnings = h_x.warnings;
    warnings.extend(h_y.warnings);
    warnings.extend(h_xy.warnings);

    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method: "discrete_mi",
        n: x.len(),
        n_effective: x.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("x_alphabet", x_size as f64), ("y_alphabet", y_size as f64)],
    })
}

/// 🔗️ Discrete conditional mutual information `I(X;Y|Z) = H(X,Z) + H(Y,Z) - H(X,Y,Z) - H(Z)`,
/// estimated with the maximum-likelihood plug-in on packed joint symbols.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn conditional_mutual_information(x: &[u32], y: &[u32], z: &[u32], base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if x.len() != y.len() || y.len() != z.len() {
        return Err(EntropyError::LengthMismatch { expected: x.len(), actual: y.len().min(z.len()) });
    }
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    let xs = *x.iter().max().unwrap() as usize + 1;
    let ys = *y.iter().max().unwrap() as usize + 1;
    let zs = *z.iter().max().unwrap() as usize + 1;

    let (xz, xz_size) = pack_symbols(&[x, z], &[xs, zs])?;
    let (yz, yz_size) = pack_symbols(&[y, z], &[ys, zs])?;
    let (xyz, xyz_size) = pack_symbols(&[x, y, z], &[xs, ys, zs])?;

    let h_xz = plugin_entropy_nats(&xz, xz_size)?;
    let h_yz = plugin_entropy_nats(&yz, yz_size)?;
    let h_xyz = plugin_entropy_nats(&xyz, xyz_size)?;
    let h_z = plugin_entropy_nats(z, zs)?;

    let nats = clamp_near_zero(h_xz + h_yz - h_xyz - h_z, 1e-9);
    Ok(Estimate { value: base.from_nats(nats), base, method: "discrete_cmi_plugin", n: x.len(), n_effective: x.len() as f64, std_error: None, ci: None::<ConfidenceInterval>, warnings: Vec::new(), diagnostics: vec![("z_alphabet", zs as f64)] })
}
// #endregion 🔖️DiscreteMi

// #region 🔖️Ksg
/// 🔗️ Which Kraskov-Stögbauer-Grassberger estimator variant [`mutual_information_knn`] uses.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum KsgVariant {
    #[default]
    Ksg1,
    Ksg2,
}

/// 🔗️ Configuration for [`mutual_information_knn`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct KsgConfig {
    pub k: usize,
    pub metric: Metric,
    pub variant: KsgVariant,
}

impl KsgConfig {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(k: usize, variant: KsgVariant) -> Result<Self, EntropyError> {
        if k == 0 {
            return Err(EntropyError::InvalidConfig { field: "k", reason: "must be at least 1" });
        }
        Ok(Self { k, metric: Metric::Chebyshev, variant })
    }
}

/// 🔗️ Continuous mutual information via the Kraskov-Stögbauer-Grassberger kNN estimator.
/// Digamma-based and therefore computed (and returned) in nats; call [`Estimate::in_base`] on
/// the result to convert.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn mutual_information_knn(x: &[f64], y: &[f64], cfg: KsgConfig) -> Result<Estimate, EntropyError> {
    if x.len() != y.len() {
        return Err(EntropyError::LengthMismatch { expected: x.len(), actual: y.len() });
    }
    let n = x.len();
    if cfg.k >= n {
        return Err(EntropyError::InvalidConfig { field: "k", reason: "must be less than the sample size" });
    }
    let joint: Vec<f64> = x.iter().zip(y.iter()).flat_map(|(&xi, &yi)| [xi, yi]).collect();
    let tree = KdTree::build(&joint, 2)?;

    let mut sum_digamma_nx = 0.0_f64;
    let mut sum_digamma_ny = 0.0_f64;
    for i in 0..n {
        let query = [x[i], y[i]];
        let neighbors = tree.k_nearest(&query, cfg.k, Metric::Chebyshev, Some(i));
        match cfg.variant {
            KsgVariant::Ksg1 => {
                let eps = neighbors.last().map_or(0.0, |&(_, d)| d);
                let nx = (0..n).filter(|&j| j != i && (x[j] - x[i]).abs() < eps).count();
                let ny = (0..n).filter(|&j| j != i && (y[j] - y[i]).abs() < eps).count();
                sum_digamma_nx += digamma(nx as f64 + 1.0);
                sum_digamma_ny += digamma(ny as f64 + 1.0);
            }
            KsgVariant::Ksg2 => {
                let eps_x = neighbors.iter().map(|&(j, _)| (x[j] - x[i]).abs()).fold(0.0_f64, f64::max);
                let eps_y = neighbors.iter().map(|&(j, _)| (y[j] - y[i]).abs()).fold(0.0_f64, f64::max);
                let nx = (0..n).filter(|&j| j != i && (x[j] - x[i]).abs() <= eps_x).count();
                let ny = (0..n).filter(|&j| j != i && (y[j] - y[i]).abs() <= eps_y).count();
                sum_digamma_nx += digamma(nx.max(1) as f64);
                sum_digamma_ny += digamma(ny.max(1) as f64);
            }
        }
    }

    let nats = match cfg.variant {
        KsgVariant::Ksg1 => digamma(cfg.k as f64) - (sum_digamma_nx + sum_digamma_ny) / n as f64 + digamma(n as f64),
        KsgVariant::Ksg2 => digamma(cfg.k as f64) - 1.0 / cfg.k as f64 - (sum_digamma_nx + sum_digamma_ny) / n as f64 + digamma(n as f64),
    };
    let clamped = clamp_near_zero(nats, 1e-6);

    let mut warnings = Vec::new();
    if n < 10 * cfg.k {
        warnings.push(Warning::SmallSample { n, recommended: 10 * cfg.k });
    }

    Ok(Estimate {
        value: clamped,
        base: LogBase::Nats,
        method: match cfg.variant {
            KsgVariant::Ksg1 => "ksg1",
            KsgVariant::Ksg2 => "ksg2",
        },
        n,
        n_effective: n as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("k", cfg.k as f64)],
    })
}
// #endregion 🔖️Ksg

// #region 🔖️Multivariate
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn multivariate_joint_and_marginal_entropies(vars: &[&[u32]], sizes: &[usize]) -> Result<(f64, Vec<f64>), EntropyError> {
    if vars.is_empty() {
        return Err(EntropyError::EmptyInput { what: "vars" });
    }
    let n = vars[0].len();
    for v in vars {
        if v.len() != n {
            return Err(EntropyError::LengthMismatch { expected: n, actual: v.len() });
        }
    }
    let (joint, joint_size) = pack_symbols(vars, sizes)?;
    let h_joint = plugin_entropy_nats(&joint, joint_size)?;
    let marginals: Result<Vec<f64>, EntropyError> = vars.iter().zip(sizes.iter()).map(|(&v, &s)| plugin_entropy_nats(v, s)).collect();
    Ok((h_joint, marginals?))
}

/// 🔗️ Total correlation (multi-information) `sum H(X_i) - H(X_1,...,X_n)`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn total_correlation(vars: &[&[u32]], sizes: &[usize], base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    let (h_joint, marginals) = multivariate_joint_and_marginal_entropies(vars, sizes)?;
    let nats = clamp_near_zero(marginals.iter().sum::<f64>() - h_joint, 1e-9);
    Ok(multivariate_estimate(nats, base, "total_correlation", vars[0].len()))
}

/// 🔗️ Dual total correlation (binding information): `sum_i H(X_{-i}) - (n-1) * H(X_1,...,X_n)`,
/// where `X_{-i}` is the joint of all variables except `i`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn dual_total_correlation(vars: &[&[u32]], sizes: &[usize], base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    let n_vars = vars.len();
    if n_vars < 2 {
        return Err(EntropyError::InvalidConfig { field: "vars", reason: "dual total correlation needs at least 2 variables" });
    }
    let (h_joint, _) = multivariate_joint_and_marginal_entropies(vars, sizes)?;
    let mut sum_rest = 0.0_f64;
    for i in 0..n_vars {
        let rest_vars: Vec<&[u32]> = vars.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, &v)| v).collect();
        let rest_sizes: Vec<usize> = sizes.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, &s)| s).collect();
        let (h_rest, _) = multivariate_joint_and_marginal_entropies(&rest_vars, &rest_sizes)?;
        sum_rest += h_rest;
    }
    let nats = clamp_near_zero(sum_rest - (n_vars as f64 - 1.0) * h_joint, 1e-9);
    Ok(multivariate_estimate(nats, base, "dual_total_correlation", vars[0].len()))
}

/// 🔗️ O-information: `total_correlation - dual_total_correlation`. Positive values indicate
/// redundancy-dominated interactions, negative values synergy-dominated ones.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn o_information(vars: &[&[u32]], sizes: &[usize], base: LogBase) -> Result<Estimate, EntropyError> {
    let tc = total_correlation(vars, sizes, LogBase::Nats)?;
    let dtc = dual_total_correlation(vars, sizes, LogBase::Nats)?;
    let nats = tc.value - dtc.value;
    Ok(multivariate_estimate(nats, base, "o_information", vars[0].len()))
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn multivariate_estimate(nats: f64, base: LogBase, method: &'static str, n: usize) -> Estimate {
    Estimate { value: base.from_nats(nats), base, method, n, n_effective: n as f64, std_error: None, ci: None::<ConfidenceInterval>, warnings: Vec::new(), diagnostics: Vec::new() }
}
// #endregion 🔖️Multivariate

// #region 🔖️Tests
#[cfg(test)]
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
