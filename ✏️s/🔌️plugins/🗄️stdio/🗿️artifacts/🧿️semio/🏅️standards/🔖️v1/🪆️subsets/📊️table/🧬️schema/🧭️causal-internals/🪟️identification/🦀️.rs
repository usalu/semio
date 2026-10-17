//! 🪟️ Identification: the backdoor and frontdoor criteria, the minimal backdoor sets and the ID-algorithm identification result. See [`super`].

use super::*;

// #region 🔖️Identification
/// 🚪️ Backdoor criterion (Pearl 2009 §3.3.1): `z` contains no descendant of `x` and blocks every
/// backdoor path, checked as d-separation in the graph with `x`'s outgoing edges removed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn backdoor_satisfied(dag: &CausalDag, x: usize, y: usize, z: &[usize]) -> bool {
    let descendants_x: HashSet<usize> = dag.descendants(x).into_iter().collect();
    if z.iter().any(|zi| descendants_x.contains(zi)) {
        return false;
    }
    let pruned_edges: Vec<(usize, usize)> = dag.edges().into_iter().filter(|&(a, _)| a != x).collect();
    let Ok(pruned) = CausalDag::new(dag.names().to_vec(), &pruned_edges) else {
        return false;
    };
    d_separated(&pruned, &[x], &[y], z)
}

/// 🚪️ Every inclusion-minimal backdoor set up to `max_size`, searched over
/// `An(x) ∪ An(y) \ ({x, y} ∪ De(x))`, smallest sets first so a superset of an already-found
/// minimal set is never re-checked.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn minimal_backdoor_sets(dag: &CausalDag, x: usize, y: usize, max_size: usize) -> Vec<Vec<usize>> {
    let mut candidates: BTreeSet<usize> = BTreeSet::new();
    candidates.extend(dag.ancestors(x));
    candidates.extend(dag.ancestors(y));
    let descendants_x: HashSet<usize> = dag.descendants(x).into_iter().collect();
    candidates.remove(&x);
    candidates.remove(&y);
    candidates.retain(|c| !descendants_x.contains(c));
    let pool: Vec<usize> = candidates.into_iter().collect();

    let mut found: Vec<Vec<usize>> = Vec::new();
    for size in 0..=max_size.min(pool.len()) {
        for subset in combinations(&pool, size) {
            if found.iter().any(|existing| existing.iter().all(|v| subset.contains(v))) {
                continue;
            }
            if backdoor_satisfied(dag, x, y, &subset) {
                found.push(subset);
            }
        }
    }
    found
}

/// 🚪️ Frontdoor criterion for mediator set `m` (Pearl 2009 §3.4): `m` intercepts every directed
/// `x -> y` path, there is no unblocked backdoor path `x -> ... -> m`, and every backdoor path
/// `m -> ... -> y` is blocked by `x`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn frontdoor_satisfied(dag: &CausalDag, x: usize, y: usize, m: &[usize]) -> bool {
    let m_set: HashSet<usize> = m.iter().copied().collect();
    if m.is_empty() || m_set.contains(&x) || m_set.contains(&y) {
        return false;
    }
    let edges_without_m: Vec<(usize, usize)> = dag.edges().into_iter().filter(|&(a, b)| !m_set.contains(&a) && !m_set.contains(&b)).collect();
    if let Ok(pruned) = CausalDag::new(dag.names().to_vec(), &edges_without_m) {
        if pruned.descendants(x).contains(&y) {
            return false;
        }
    }
    for &mi in m {
        if !dag.descendants(x).contains(&mi) || !dag.descendants(mi).contains(&y) {
            return false;
        }
        if !backdoor_satisfied(dag, x, mi, &[]) {
            return false;
        }
    }
    let edges_without_m_out: Vec<(usize, usize)> = dag.edges().into_iter().filter(|&(a, _)| !m_set.contains(&a)).collect();
    let Ok(pruned_out) = CausalDag::new(dag.names().to_vec(), &edges_without_m_out) else {
        return false;
    };
    d_separated(&pruned_out, m, &[y], &[x])
}

/// 🧾️ How an interventional query was identified — drives which estimator is applicable.
#[derive(Clone, Debug, PartialEq)]
pub enum Identification {
    NoConfounding,
    Backdoor { adjustment: Vec<usize> },
    Frontdoor { mediators: Vec<usize> },
}

/// 🧾️ Finds an identification strategy for `P(y | do(x))` on a fully-observed DAG, checked in
/// order of estimator simplicity: no adjustment needed, then a minimal backdoor set, then a
/// frontdoor mediator set. On a fully-observed DAG some backdoor set (e.g. `x`'s own parents)
/// always exists, so this always succeeds; latent-confounder identification via the general
/// Shpitser–Pearl ID algorithm needs an ADMG (bidirected-edge) graph representation that
/// [`CausalDag`] does not model in v1, and is out of scope here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn identify(dag: &CausalDag, x: usize, y: usize) -> Result<Identification, CausalError> {
    if backdoor_satisfied(dag, x, y, &[]) {
        return Ok(Identification::NoConfounding);
    }
    let sets = minimal_backdoor_sets(dag, x, y, dag.n());
    if let Some(adjustment) = sets.into_iter().min_by_key(Vec::len) {
        return Ok(Identification::Backdoor { adjustment });
    }
    let other_vars: Vec<usize> = (0..dag.n()).filter(|&v| v != x && v != y).collect();
    for m_size in 1..=other_vars.len() {
        for mediators in combinations(&other_vars, m_size) {
            if frontdoor_satisfied(dag, x, y, &mediators) {
                return Ok(Identification::Frontdoor { mediators });
            }
        }
    }
    Err(CausalError::NotIdentifiable("no backdoor or frontdoor adjustment found; latent-confounder ADMG identification (general ID algorithm) is out of scope for a fully-observed DAG".to_string()))
}
// #endregion 🔖️Identification
