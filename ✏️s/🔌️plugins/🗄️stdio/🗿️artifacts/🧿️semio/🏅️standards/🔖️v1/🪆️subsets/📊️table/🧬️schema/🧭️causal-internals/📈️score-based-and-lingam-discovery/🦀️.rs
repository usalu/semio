//! 📈️ Score based and LiNGAM discovery: the local and DAG BIC scores, DirectLiNGAM and greedy equivalence search. See [`super`].

use super::*;

// #region 🔖️Discovery
/// 📉️ Decomposable Gaussian BIC of one node given a candidate parent set (Chickering, JMLR 2002):
/// `-n/2 * ln(sigma^2) - (|parents|+1)/2 * ln(n)`, the per-node summand a score-based search adds/removes.
#[allow(clippy::needless_range_loop, reason = "row indexes both the MatD design matrix by (row, col) and the values slice — enumerate() would only remove the values index")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn local_bic(data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, node: usize, parents: &[usize]) -> Result<f64, CausalError> {
    let y = data.continuous(node)?;
    let n = y.len();
    let p = parents.len();
    if parents.is_empty() {
        let m = semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::mean(y)?;
        let ss: f64 = y.iter().map(|v| (v - m).powi(2)).sum();
        let sigma2 = (ss / n as f64).max(1e-12);
        return Ok(-0.5 * n as f64 * sigma2.ln() - 0.5 * (n as f64).ln());
    }
    let mut design = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD::zeros(n, p);
    for (col, &parent) in parents.iter().enumerate() {
        let values = data.continuous(parent)?;
        for row in 0..n {
            design.set(row, col, values[row]);
        }
    }
    let fit = semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::ols(&design, y, true)?;
    let ss_res: f64 = fit.residuals.iter().map(|r| r * r).sum();
    let sigma2 = (ss_res / n as f64).max(1e-12);
    Ok(-0.5 * n as f64 * sigma2.ln() - 0.5 * (p as f64 + 1.0) * (n as f64).ln())
}

/// 📉️ Sum of local BICs over every node of `dag` given its own parent set.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dag_bic(data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, dag: &CausalDag) -> Result<f64, CausalError> {
    (0..dag.n()).map(|v| local_bic(data, v, dag.parents(v))).sum()
}

/// 📦️ DirectLiNGAM output: the recovered causal order and the pruned weighted adjacency (`weights[child][parent]`).
pub struct LingamResult {
    pub order: Vec<usize>,
    pub weights: semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD,
    pub dag: CausalDag,
}

/// 🔎️ DirectLiNGAM (Shimizu et al., JMLR 2011): repeatedly extracts the most-exogenous remaining
/// variable — the one whose linear-regression residuals against every other remaining variable are
/// least dependent on it — regresses it out, and recurses; final edges are pruned by an OLS
/// t-test at `prune_alpha` over each variable's full set of causally-earlier candidates.
///
/// Exogeneity is scored with a higher-moment (skewness-family) dependence proxy — `|corr(x_m, r_j^3)|`
/// summed over the residuals `r_j` of every other candidate regressed on `x_m` — rather than the
/// original paper's kernel-density mutual-information estimate; both exploit the same LiNGAM
/// identifiability condition (independent, non-Gaussian noise) to detect residual dependence.
#[allow(clippy::needless_range_loop, reason = "row indexes both the MatD design matrix by (row, col) and the columns[parent] slice — enumerate() would only remove one of the two")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn direct_lingam(data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, prune_alpha: f64) -> Result<LingamResult, CausalError> {
    let n = data.n_cols();
    let columns: Vec<Vec<f64>> = (0..n).map(|i| data.continuous(i).map(<[f64]>::to_vec)).collect::<Result<_, _>>()?;
    let mut working: HashMap<usize, Vec<f64>> = (0..n).map(|i| (i, standardize(&columns[i]))).collect();
    let mut remaining: Vec<usize> = (0..n).collect();
    let mut order: Vec<usize> = Vec::with_capacity(n);

    while remaining.len() > 1 {
        let mut best = remaining[0];
        let mut best_score = f64::INFINITY;
        for &m in &remaining {
            let xm = &working[&m];
            let mut score = 0.0;
            for &j in &remaining {
                if j == m {
                    continue;
                }
                let xj = &working[&j];
                let beta = covariance_pop(xm, xj) / variance_pop(xm).max(1e-12);
                let residual: Vec<f64> = xj.iter().zip(xm).map(|(&xjv, &xmv)| xjv - beta * xmv).collect();
                let r3: Vec<f64> = residual.iter().map(|r| r.powi(3)).collect();
                score += correlation_pop(xm, &r3).abs();
            }
            if score < best_score {
                best_score = score;
                best = m;
            }
        }
        order.push(best);
        let xm = working[&best].clone();
        for &j in &remaining {
            if j == best {
                continue;
            }
            let beta = covariance_pop(&xm, &working[&j]) / variance_pop(&xm).max(1e-12);
            let xj = working.get_mut(&j).expect("j is in `remaining` and thus in `working`");
            for (v, &xmv) in xj.iter_mut().zip(&xm) {
                *v -= beta * xmv;
            }
        }
        remaining.retain(|&v| v != best);
    }
    order.push(remaining[0]);

    let mut weights = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD::zeros(n, n);
    let mut edges = Vec::new();
    for (pos, &child) in order.iter().enumerate() {
        let candidate_parents = &order[..pos];
        if candidate_parents.is_empty() {
            continue;
        }
        let n_rows = columns[child].len();
        let mut design = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD::zeros(n_rows, candidate_parents.len());
        for (col, &parent) in candidate_parents.iter().enumerate() {
            for row in 0..n_rows {
                design.set(row, col, columns[parent][row]);
            }
        }
        let fit = semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::ols(&design, &columns[child], true)?;
        for (col, &parent) in candidate_parents.iter().enumerate() {
            let coeff = fit.coefficients[col + 1];
            let se = fit.std_errors[col + 1];
            if se > 1e-12 {
                let t_stat = coeff / se;
                let p_value = 2.0 * (1.0 - semio_s_artifact_stdio_semio_table_probability::schema::probability_internals::StudentT::new(fit.dof as f64)?.cdf(t_stat.abs()));
                if p_value < prune_alpha {
                    weights.set(child, parent, coeff);
                    edges.push((parent, child));
                }
            }
        }
    }
    let dag = CausalDag::new(data.names().to_vec(), &edges)?;
    Ok(LingamResult { order, weights, dag })
}

/// 🔎️ Score-based structure search using decomposable Gaussian BIC ([`dag_bic`]) — greedy
/// forward (add-best-acyclic-edge) then backward (remove-best-edge) hill-climbing directly on DAG
/// structure, in the spirit of Chickering's GES (JMLR 2002) but without its formal
/// equivalence-class insert/delete validity conditions; returns the CPDAG of the local optimum's
/// Markov equivalence class.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ges(data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table) -> Result<Cpdag, CausalError> {
    let n = data.n_cols();
    let names = data.names().to_vec();
    let mut edges: Vec<(usize, usize)> = Vec::new();

    loop {
        let baseline = CausalDag::new(names.clone(), &edges)?;
        let baseline_score = dag_bic(data, &baseline)?;
        let mut best: Option<((usize, usize), f64)> = None;
        for u in 0..n {
            for v in 0..n {
                if u == v || edges.contains(&(u, v)) {
                    continue;
                }
                let mut candidate = edges.clone();
                candidate.push((u, v));
                let Ok(dag) = CausalDag::new(names.clone(), &candidate) else { continue };
                let Ok(score) = dag_bic(data, &dag) else { continue };
                if best.as_ref().is_none_or(|&(_, b)| score > b) {
                    best = Some(((u, v), score));
                }
            }
        }
        match best {
            Some((edge, score)) if score > baseline_score + 1e-9 => edges.push(edge),
            _ => break,
        }
    }

    loop {
        let baseline = CausalDag::new(names.clone(), &edges)?;
        let baseline_score = dag_bic(data, &baseline)?;
        let mut best: Option<(usize, f64)> = None;
        for i in 0..edges.len() {
            let mut candidate = edges.clone();
            candidate.remove(i);
            let Ok(dag) = CausalDag::new(names.clone(), &candidate) else { continue };
            let Ok(score) = dag_bic(data, &dag) else { continue };
            if best.as_ref().is_none_or(|&(_, b)| score > b) {
                best = Some((i, score));
            }
        }
        match best {
            Some((i, score)) if score > baseline_score + 1e-9 => {
                edges.remove(i);
            }
            _ => break,
        }
    }

    let final_dag = CausalDag::new(names, &edges)?;
    Ok(Cpdag::from_dag(&final_dag))
}
// #endregion 🔖️Discovery
