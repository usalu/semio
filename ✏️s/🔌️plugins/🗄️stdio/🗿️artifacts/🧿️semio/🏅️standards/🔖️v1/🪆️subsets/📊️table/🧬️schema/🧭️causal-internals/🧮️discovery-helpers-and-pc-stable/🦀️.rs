//! 🧮️ Discovery helpers and PC-stable: the standardisation, covariance and combination helpers, the PC options and result, PC-stable and the v-structure orientation. See [`super`].

use super::*;

// #region 🔖️Internal
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn standardize(values: &[f64]) -> Vec<f64> {
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let var = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
    let sd = var.sqrt().max(1e-12);
    values.iter().map(|v| (v - mean) / sd).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn covariance_pop(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let ma = a.iter().sum::<f64>() / n;
    let mb = b.iter().sum::<f64>() / n;
    a.iter().zip(b).map(|(x, y)| (x - ma) * (y - mb)).sum::<f64>() / n
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn variance_pop(a: &[f64]) -> f64 {
    covariance_pop(a, a)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn correlation_pop(a: &[f64], b: &[f64]) -> f64 {
    let sa = variance_pop(a).sqrt().max(1e-12);
    let sb = variance_pop(b).sqrt().max(1e-12);
    covariance_pop(a, b) / (sa * sb)
}

/// 🧮️ All `k`-element subsets of `items`, in lexicographic index order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn combinations(items: &[usize], k: usize) -> Vec<Vec<usize>> {
    if k == 0 {
        return vec![Vec::new()];
    }
    if items.len() < k {
        return Vec::new();
    }
    let mut out = Vec::new();
    for i in 0..=(items.len() - k) {
        for mut rest in combinations(&items[(i + 1)..], k - 1) {
            rest.insert(0, items[i]);
            out.push(rest);
        }
    }
    out
}
// #endregion 🔖️Internal

// #region 🔖️Discovery
/// ⚙️ PC-stable options: significance threshold and the largest conditioning-set size searched.
#[derive(Clone, Copy, Debug)]
pub struct PcOptions {
    pub alpha: f64,
    pub max_cond_size: usize,
}

impl Default for PcOptions {
    fn default() -> Self {
        Self { alpha: 0.05, max_cond_size: 3 }
    }
}

/// 📦️ PC-stable output: the resulting CPDAG plus every separating set found while pruning the skeleton.
pub struct PcResult {
    pub cpdag: Cpdag,
    pub sepsets: HashMap<(usize, usize), Vec<usize>>,
}

/// 🔎️ PC-stable (Colombo & Maathuis, JMLR 2014): level-wise skeleton pruning using adjacency sets
/// frozen at the start of each level (making the result independent of variable ordering), then
/// v-structure orientation and Meek-rule closure.
#[allow(clippy::needless_range_loop, reason = "x indexes both adj_snapshot and, via the inner loop, cpdag/removals by the same variable id — enumerate() over one Vec wouldn't simplify the rest")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pc_stable<T: CiTest>(data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, test: &T, opts: PcOptions) -> Result<PcResult, CausalError> {
    let n = data.n_cols();
    let mut cpdag = Cpdag::complete(data.names().to_vec());
    let mut sepsets: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for level in 0..=opts.max_cond_size {
        let adj_snapshot: Vec<Vec<usize>> = (0..n).map(|v| cpdag.adjacent(v)).collect();
        let mut removals: Vec<(usize, usize, Vec<usize>)> = Vec::new();
        for x in 0..n {
            for &y in &adj_snapshot[x] {
                if y <= x || !cpdag.has_edge(x, y) {
                    continue;
                }
                let neighbors: Vec<usize> = adj_snapshot[x].iter().copied().filter(|&z| z != y).collect();
                if neighbors.len() < level {
                    continue;
                }
                for subset in combinations(&neighbors, level) {
                    let result = test.test(data, x, y, &subset)?;
                    if result.p_value > opts.alpha {
                        removals.push((x, y, subset));
                        break;
                    }
                }
            }
        }
        for (x, y, z) in removals {
            cpdag.remove_edge(x, y);
            sepsets.insert(norm_pair(x, y), z);
        }
    }
    orient_v_structures(&mut cpdag, &sepsets);
    apply_meek_rules(&mut cpdag);
    Ok(PcResult { cpdag, sepsets })
}

/// 🧩️ Orients unshielded colliders `a -> c <- b` for every non-adjacent pair `a, b` sharing
/// neighbor `c` where `c` is absent from `a, b`'s separating set (Spirtes, Glymour & Scheines 2000).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn orient_v_structures(cpdag: &mut Cpdag, sepsets: &HashMap<(usize, usize), Vec<usize>>) {
    let n = cpdag.n();
    let mut to_orient = Vec::new();
    for c in 0..n {
        let neighbors = cpdag.adjacent(c);
        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                let (a, b) = (neighbors[i], neighbors[j]);
                if cpdag.has_edge(a, b) {
                    continue;
                }
                let c_in_sepset = sepsets.get(&norm_pair(a, b)).is_some_and(|s| s.contains(&c));
                if !c_in_sepset && cpdag.is_undirected(a, c) && cpdag.is_undirected(b, c) {
                    to_orient.push((a, c));
                    to_orient.push((b, c));
                }
            }
        }
    }
    for (a, b) in to_orient {
        if cpdag.is_undirected(a, b) {
            cpdag.orient(a, b);
        }
    }
}
// #endregion 🔖️Discovery
