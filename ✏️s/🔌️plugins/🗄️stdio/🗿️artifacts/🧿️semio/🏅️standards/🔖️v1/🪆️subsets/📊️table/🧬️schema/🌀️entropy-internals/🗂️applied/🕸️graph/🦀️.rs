//! 🕸️ Graph entropy over plain edge lists (no graph-library dependency): degree-distribution
//! entropy and random-walk entropy rate.

use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::x_ln_x;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::ConfidenceInterval;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Estimate;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::LogBase;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Warning;

// #region 🔖️Degree
/// 🕸️ Shannon entropy of the (out-)degree distribution of a graph given as an edge list.
/// `directed` selects out-degree counting; undirected counts each edge toward both endpoints.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn degree_distribution_entropy(edges: &[(u32, u32)], n_nodes: usize, directed: bool, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if n_nodes == 0 {
        return Err(EntropyError::EmptyInput { what: "n_nodes" });
    }
    let mut degree = vec![0u64; n_nodes];
    for &(a, b) in edges {
        let (a, b) = (a as usize, b as usize);
        if a >= n_nodes || b >= n_nodes {
            return Err(EntropyError::ShapeMismatch { what: "edge endpoint", expected: n_nodes, actual: a.max(b) + 1 });
        }
        degree[a] += 1;
        if !directed {
            degree[b] += 1;
        }
    }
    let max_degree = degree.iter().copied().max().unwrap_or(0) as usize;
    let mut hist = vec![0.0_f64; max_degree + 1];
    for &d in &degree {
        hist[d as usize] += 1.0;
    }
    let n = n_nodes as f64;
    let nats = -hist.iter().map(|&c| x_ln_x(c / n)).sum::<f64>();

    let mut warnings = Vec::new();
    if n_nodes < 30 {
        warnings.push(Warning::SmallSample { n: n_nodes, recommended: 30 });
    }

    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method: "degree_distribution_entropy",
        n: n_nodes,
        n_effective: n_nodes as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("max_degree", max_degree as f64), ("edges", edges.len() as f64)],
    })
}
// #endregion 🔖️Degree

// #region 🔖️RandomWalk
/// 🕸️ Random-walk entropy rate `sum_i pi_i * H(row_i)`: the average per-step uncertainty of a
/// simple (optionally weighted) random walk on the graph, weighted by the walk's stationary node
/// distribution (via power iteration).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn random_walk_entropy_rate(edges: &[(u32, u32)], n_nodes: usize, weights: Option<&[f64]>, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if n_nodes == 0 {
        return Err(EntropyError::EmptyInput { what: "n_nodes" });
    }
    if let Some(w) = weights {
        if w.len() != edges.len() {
            return Err(EntropyError::LengthMismatch { expected: edges.len(), actual: w.len() });
        }
    }
    let mut adjacency = vec![0.0_f64; n_nodes * n_nodes];
    for (i, &(a, b)) in edges.iter().enumerate() {
        let (a, b) = (a as usize, b as usize);
        if a >= n_nodes || b >= n_nodes {
            return Err(EntropyError::ShapeMismatch { what: "edge endpoint", expected: n_nodes, actual: a.max(b) + 1 });
        }
        let w = weights.map_or(1.0, |ws| ws[i]);
        if w < 0.0 || !w.is_finite() {
            return Err(EntropyError::InvalidProbability { index: i, value: w });
        }
        adjacency[a * n_nodes + b] += w;
        adjacency[b * n_nodes + a] += w;
    }

    let mut transition = vec![0.0_f64; n_nodes * n_nodes];
    let mut row_entropy_nats = vec![0.0_f64; n_nodes];
    for i in 0..n_nodes {
        let row_sum: f64 = adjacency[i * n_nodes..(i + 1) * n_nodes].iter().sum();
        if row_sum > 0.0 {
            for j in 0..n_nodes {
                transition[i * n_nodes + j] = adjacency[i * n_nodes + j] / row_sum;
            }
        } else {
            transition[i * n_nodes + i] = 1.0;
        }
        row_entropy_nats[i] = -transition[i * n_nodes..(i + 1) * n_nodes].iter().map(|&p| x_ln_x(p)).sum::<f64>();
    }

    let mut pi = vec![1.0 / n_nodes as f64; n_nodes];
    let mut converged = false;
    for _ in 0..10_000 {
        let mut next = vec![0.0_f64; n_nodes];
        for i in 0..n_nodes {
            if pi[i] <= 0.0 {
                continue;
            }
            for j in 0..n_nodes {
                next[j] += pi[i] * transition[i * n_nodes + j];
            }
        }
        let delta: f64 = pi.iter().zip(next.iter()).map(|(&a, &b)| (a - b).abs()).sum();
        pi = next;
        if delta < 1e-12 {
            converged = true;
            break;
        }
    }
    if !converged {
        return Err(EntropyError::NotConverged { what: "random walk stationary distribution", iterations: 10_000 });
    }

    let nats = pi.iter().zip(row_entropy_nats.iter()).map(|(&p, &h)| p * h).sum::<f64>();

    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method: "random_walk_entropy_rate",
        n: n_nodes,
        n_effective: n_nodes as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings: Vec::new(),
        diagnostics: vec![("edges", edges.len() as f64)],
    })
}
// #endregion 🔖️RandomWalk

// #region 🔖️Tests
#[cfg(test)]
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
