//! 🧱️ D-separation and conditional independence tests: the d-separation query, the implied independencies, the CI test trait and the Fisher z and G-squared tests. See [`super`].

use super::*;

// #region 🔖️DSeparation
/// 🚧️ One conditional-independence statement `x ⟂ y | z` implied by a graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiStatement {
    pub x: usize,
    pub y: usize,
    pub z: Vec<usize>,
}

/// 🚧️ Tests `x ⟂ y | z` in `dag` via the moralized-ancestral-graph criterion (Lauritzen et al.
/// 1990): restrict to the ancestral set of `x ∪ y ∪ z`, moralize, delete `z`, and check whether
/// `x` and `y` stay connected.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn d_separated(dag: &CausalDag, x: &[usize], y: &[usize], z: &[usize]) -> bool {
    let mut relevant: HashSet<usize> = HashSet::new();
    for &v in x.iter().chain(y).chain(z) {
        relevant.insert(v);
        relevant.extend(dag.ancestors(v));
    }
    // Moralize only the subgraph induced by the ancestral set: a co-parent marriage is only real
    // if the common child is itself ancestral — moralizing the full DAG first (as `moralize()`
    // does) would wrongly marry co-parents of children that fall outside the ancestral set.
    let mut moral_edges: BTreeSet<(usize, usize)> = BTreeSet::new();
    for &child in &relevant {
        let parents: Vec<usize> = dag.parents(child).iter().copied().filter(|p| relevant.contains(p)).collect();
        for &p in &parents {
            moral_edges.insert(norm_pair(p, child));
        }
        for i in 0..parents.len() {
            for j in (i + 1)..parents.len() {
                moral_edges.insert(norm_pair(parents[i], parents[j]));
            }
        }
    }
    let filtered_edges: Vec<(usize, usize)> = moral_edges.into_iter().filter(|&(a, b)| !z.contains(&a) && !z.contains(&b)).collect();
    let adj = semio_framework_graph_algorithms::adjacency(dag.n(), &filtered_edges, false);
    let labels = semio_framework_graph_algorithms::connected_components(&adj);
    x.iter().all(|&xi| y.iter().all(|&yi| z.contains(&xi) || z.contains(&yi) || labels[xi] != labels[yi]))
}

/// 📜️ Local-Markov implied CIs, emitted pairwise: for each `v`, `v ⟂ w | pa(v)` for every
/// non-descendant `w` that is not a parent of `v`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn implied_independencies(dag: &CausalDag) -> Vec<CiStatement> {
    let n = dag.n();
    let mut out = Vec::new();
    for v in 0..n {
        let parents = dag.parents(v).to_vec();
        let descendants: HashSet<usize> = dag.descendants(v).into_iter().collect();
        for w in 0..n {
            if w == v || descendants.contains(&w) || parents.contains(&w) {
                continue;
            }
            out.push(CiStatement { x: v, y: w, z: parents.clone() });
        }
    }
    out
}

/// 🔬️ Tests every implied CI against data — a graph-fit diagnostic ("model criticism"). Generic over
/// the test implementation (O1 — R11(a): a borrowed-reference parameter is trivially generic, no enum
/// needed even though the set of `CiTest` impls in this module is closed).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn test_implied_independencies<T: CiTest>(
    dag: &CausalDag,
    data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table,
    test: &T,
) -> Result<Vec<(CiStatement, semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::TestResult)>, CausalError> {
    implied_independencies(dag)
        .into_iter()
        .map(|stmt| {
            let result = test.test(data, stmt.x, stmt.y, &stmt.z)?;
            Ok((stmt, result))
        })
        .collect()
}
// #endregion 🔖️DSeparation

// #region 🔖️CiTest
/// 🔬️ A conditional-independence test on tabular data, indexed by table/DAG column.
pub trait CiTest {
    fn test(&self, data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, x: usize, y: usize, z: &[usize]) -> Result<semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::TestResult, CausalError>;
}

/// 🔬️ Fisher-z partial-correlation test for continuous data, precomputing the correlation matrix
/// once per dataset — PC-stable runs thousands of tests against the same table.
pub struct FisherZ {
    corr: semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD,
    pub(super) n: usize,
}

impl FisherZ {
    /// 🔬️ Precomputes the complete-case correlation matrix over `columns` (in the same order as
    /// the paired [`CausalDag`]'s variable indices).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn for_table(data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, columns: &[usize]) -> Result<Self, CausalError> {
        let (corr, n) = semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::correlation_from_table(data, columns)?;
        Ok(Self { corr, n })
    }
}

impl CiTest for FisherZ {
    fn test(&self, _data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, x: usize, y: usize, z: &[usize]) -> Result<semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::TestResult, CausalError> {
        Ok(semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::fisher_z_test(&self.corr, x, y, z, self.n)?)
    }
}

/// 🔬️ G² likelihood-ratio test for categorical data.
pub struct GSquared;

impl CiTest for GSquared {
    fn test(&self, data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, x: usize, y: usize, z: &[usize]) -> Result<semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::TestResult, CausalError> {
        let cat_x = data.categorical(x)?;
        let cat_y = data.categorical(y)?;
        let given_cols: Vec<&semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::CategoricalColumn> = z.iter().map(|&zi| data.categorical(zi)).collect::<Result<_, _>>()?;
        let given_codes: Vec<&[u32]> = given_cols.iter().map(|c| c.codes()).collect();
        let given_levels: Vec<usize> = given_cols.iter().map(|c| c.n_levels()).collect();
        Ok(semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::g2_ci_test(cat_x.codes(), cat_y.codes(), &given_codes, (cat_x.n_levels(), cat_y.n_levels(), &given_levels))?)
    }
}
// #endregion 🔖️CiTest
