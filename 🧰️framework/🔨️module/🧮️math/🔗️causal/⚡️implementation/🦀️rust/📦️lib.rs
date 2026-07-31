//! 🔀️ Causal inference: DAG/CPDAG/PAG models, d-separation, PC-stable/GES/LiNGAM/FCI discovery, backdoor/frontdoor/ID-algorithm identification, linear-Gaussian and discrete SCMs with interventional and counterfactual queries, and potential-outcome effect estimators.
//!
//! Variable indices are shared across [`CausalDag`]/[`Cpdag`] and `mathematical_tabular::Table`
//! columns throughout this crate: a DAG built over `n` named variables is expected to pair with a
//! table whose column `i` holds observations of variable `i`, for every `i`.

use mathematical_probability::Continuous;
use std::collections::{BTreeSet, HashMap, HashSet};

// #region 🔖️Errors
/// ⚠️ Fallible-computation error type shared by every function in this crate.
#[derive(Debug, thiserror::Error)]
pub enum CausalError {
    #[error("variable `{0}` not found")]
    VariableNotFound(String),
    #[error("edges contain a cycle through node indices {0:?}")]
    NotADag(Vec<usize>),
    #[error("column {0} has wrong type: expected {1}")]
    ColumnType(usize, &'static str),
    #[error("dimension mismatch: {0}")]
    DimensionMismatch(String),
    #[error("effect not identifiable: {0}")]
    NotIdentifiable(String),
    #[error("singular linear system in {0}")]
    Singular(&'static str),
    #[error("invalid query: {0}")]
    InvalidQuery(String),
    #[error("inference too large: factor would have {0} entries (limit {1})")]
    InferenceTooLarge(usize, usize),
    #[error(transparent)]
    Stats(#[from] mathematical_statistics::StatisticsError),
    #[error(transparent)]
    Tabular(#[from] mathematical_tabular::TabularError),
    #[error(transparent)]
    Probability(#[from] mathematical_probability::ProbabilityError),
}

/// 🧮️ Normalizes an unordered pair so `a <= b`, the canonical key for undirected-edge sets.
fn norm_pair(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
// #endregion 🔖️Errors

// #region 🔖️Dag
/// 🧭️ A directed acyclic causal graph over named variables.
#[derive(Clone, Debug, PartialEq)]
pub struct CausalDag {
    names: Vec<String>,
    index: HashMap<String, usize>,
    parents: Vec<Vec<usize>>,
    children: Vec<Vec<usize>>,
    topo: Vec<usize>,
}

impl CausalDag {
    /// 🧭️ Builds a DAG from index edges, validating acyclicity via
    /// `mathematical_graph::algorithms::topo_sort`.
    pub fn new(names: Vec<String>, edges: &[(usize, usize)]) -> Result<Self, CausalError> {
        let n = names.len();
        for &(a, b) in edges {
            if a >= n {
                return Err(CausalError::VariableNotFound(format!("index {a}")));
            }
            if b >= n {
                return Err(CausalError::VariableNotFound(format!("index {b}")));
            }
        }
        let adj = mathematical_graph::algorithms::adjacency(n, edges, true);
        let topo = mathematical_graph::algorithms::topo_sort(&adj).map_err(|e| CausalError::NotADag(e.cycle))?;
        let mut parents = vec![Vec::new(); n];
        let mut children = vec![Vec::new(); n];
        for &(a, b) in edges {
            children[a].push(b);
            parents[b].push(a);
        }
        for p in parents.iter_mut() {
            p.sort_unstable();
            p.dedup();
        }
        for c in children.iter_mut() {
            c.sort_unstable();
            c.dedup();
        }
        let index = names.iter().enumerate().map(|(i, n)| (n.clone(), i)).collect();
        Ok(Self { names, index, parents, children, topo })
    }

    /// 🧭️ Builds a DAG from named edges, resolving each name against `names`.
    pub fn from_named_edges(names: Vec<String>, edges: &[(&str, &str)]) -> Result<Self, CausalError> {
        let index: HashMap<&str, usize> = names.iter().enumerate().map(|(i, n)| (n.as_str(), i)).collect();
        let mut resolved = Vec::with_capacity(edges.len());
        for &(a, b) in edges {
            let ai = *index.get(a).ok_or_else(|| CausalError::VariableNotFound(a.to_string()))?;
            let bi = *index.get(b).ok_or_else(|| CausalError::VariableNotFound(b.to_string()))?;
            resolved.push((ai, bi));
        }
        Self::new(names, &resolved)
    }

    pub fn n(&self) -> usize {
        self.names.len()
    }

    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.index.get(name).copied()
    }

    pub fn edges(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        for (child, parents) in self.parents.iter().enumerate() {
            for &parent in parents {
                out.push((parent, child));
            }
        }
        out.sort_unstable();
        out
    }

    pub fn parents(&self, v: usize) -> &[usize] {
        &self.parents[v]
    }

    pub fn children(&self, v: usize) -> &[usize] {
        &self.children[v]
    }

    /// 🧭️ Strict ancestors of `v` (excludes `v` itself), ascending.
    pub fn ancestors(&self, v: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut stack = self.parents[v].clone();
        while let Some(u) = stack.pop() {
            if visited.insert(u) {
                stack.extend(self.parents[u].iter().copied());
            }
        }
        let mut out: Vec<usize> = visited.into_iter().collect();
        out.sort_unstable();
        out
    }

    /// 🧭️ Strict descendants of `v` (excludes `v` itself), ascending.
    pub fn descendants(&self, v: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut stack = self.children[v].clone();
        while let Some(u) = stack.pop() {
            if visited.insert(u) {
                stack.extend(self.children[u].iter().copied());
            }
        }
        let mut out: Vec<usize> = visited.into_iter().collect();
        out.sort_unstable();
        out
    }

    pub fn topological_order(&self) -> &[usize] {
        &self.topo
    }

    /// 🕸️ Undirected moral-graph edges (`a < b`): the skeleton plus edges marrying every pair of co-parents.
    pub fn moralize(&self) -> Vec<(usize, usize)> {
        let mut edges: BTreeSet<(usize, usize)> = BTreeSet::new();
        for (child, parents) in self.parents.iter().enumerate() {
            for &parent in parents {
                edges.insert(norm_pair(parent, child));
            }
            for i in 0..parents.len() {
                for j in (i + 1)..parents.len() {
                    edges.insert(norm_pair(parents[i], parents[j]));
                }
            }
        }
        edges.into_iter().collect()
    }
}
// #endregion 🔖️Dag

// #region 🔖️Cpdag
/// 🔀️ A completed partially directed acyclic graph (CPDAG): directed causal edges plus undirected
/// (Markov-equivalence-unresolved) edges over the same named variables as a [`CausalDag`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cpdag {
    names: Vec<String>,
    directed: BTreeSet<(usize, usize)>,
    undirected: BTreeSet<(usize, usize)>,
}

impl Cpdag {
    pub fn new(names: Vec<String>) -> Self {
        Self { names, directed: BTreeSet::new(), undirected: BTreeSet::new() }
    }

    /// 🔀️ A fully undirected complete graph — the PC algorithm's starting skeleton.
    pub fn complete(names: Vec<String>) -> Self {
        let n = names.len();
        let mut undirected = BTreeSet::new();
        for i in 0..n {
            for j in (i + 1)..n {
                undirected.insert((i, j));
            }
        }
        Self { names, directed: BTreeSet::new(), undirected }
    }

    pub fn n(&self) -> usize {
        self.names.len()
    }

    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn has_edge(&self, a: usize, b: usize) -> bool {
        self.directed.contains(&(a, b)) || self.directed.contains(&(b, a)) || self.undirected.contains(&norm_pair(a, b))
    }

    pub fn is_directed(&self, a: usize, b: usize) -> bool {
        self.directed.contains(&(a, b))
    }

    pub fn is_undirected(&self, a: usize, b: usize) -> bool {
        self.undirected.contains(&norm_pair(a, b))
    }

    pub fn adjacent(&self, v: usize) -> Vec<usize> {
        let mut out: BTreeSet<usize> = BTreeSet::new();
        for &(a, b) in &self.directed {
            if a == v {
                out.insert(b);
            }
            if b == v {
                out.insert(a);
            }
        }
        for &(a, b) in &self.undirected {
            if a == v {
                out.insert(b);
            }
            if b == v {
                out.insert(a);
            }
        }
        out.into_iter().collect()
    }

    pub fn remove_edge(&mut self, a: usize, b: usize) {
        self.directed.remove(&(a, b));
        self.directed.remove(&(b, a));
        self.undirected.remove(&norm_pair(a, b));
    }

    /// 🔀️ Resolves the undirected edge `(a, b)` to `a -> b`.
    pub fn orient(&mut self, a: usize, b: usize) {
        self.undirected.remove(&norm_pair(a, b));
        self.directed.insert((a, b));
    }

    pub fn directed_edges(&self) -> Vec<(usize, usize)> {
        self.directed.iter().copied().collect()
    }

    pub fn undirected_edges(&self) -> Vec<(usize, usize)> {
        self.undirected.iter().copied().collect()
    }

    /// 🧭️ The CPDAG of `dag`'s Markov equivalence class: skeleton + v-structures, closed under
    /// [`apply_meek_rules`] (sound and complete for this step per Meek 1995).
    pub fn from_dag(dag: &CausalDag) -> Cpdag {
        let mut cpdag = Cpdag::new(dag.names().to_vec());
        for (a, b) in dag.edges() {
            cpdag.undirected.insert(norm_pair(a, b));
        }
        let mut to_orient = Vec::new();
        for c in 0..dag.n() {
            let parents = dag.parents(c);
            for i in 0..parents.len() {
                for j in (i + 1)..parents.len() {
                    let (a, b) = (parents[i], parents[j]);
                    if !cpdag.has_edge(a, b) {
                        to_orient.push((a, c));
                        to_orient.push((b, c));
                    }
                }
            }
        }
        for (a, b) in to_orient {
            cpdag.undirected.remove(&norm_pair(a, b));
            cpdag.directed.insert((a, b));
        }
        apply_meek_rules(&mut cpdag);
        cpdag
    }

    /// 🧭️ Extends to a consistent member DAG via the Dor–Tarsi (1992) algorithm; `None` if no
    /// consistent extension exists (not possible for a CPDAG built by [`Cpdag::from_dag`] or
    /// [`pc_stable`], but possible for a hand-built [`Cpdag`]).
    pub fn to_dag(&self) -> Option<CausalDag> {
        let n = self.n();
        let mut directed: Vec<(usize, usize)> = self.directed.iter().copied().collect();
        let mut remaining_undirected: BTreeSet<(usize, usize)> = self.undirected.clone();
        let mut active: BTreeSet<usize> = (0..n).collect();
        let skeleton_adjacent = |a: usize, b: usize| self.has_edge(a, b);
        while !remaining_undirected.is_empty() {
            let mut found: Option<(usize, Vec<usize>)> = None;
            for &x in &active {
                let has_outgoing = directed.iter().any(|&(u, v)| u == x && active.contains(&v));
                if has_outgoing {
                    continue;
                }
                let mut neighbors: Vec<usize> = Vec::new();
                for &(u, v) in &remaining_undirected {
                    if u == x {
                        neighbors.push(v);
                    }
                    if v == x {
                        neighbors.push(u);
                    }
                }
                for &(u, v) in &directed {
                    if v == x && active.contains(&u) {
                        neighbors.push(u);
                    }
                }
                neighbors.sort_unstable();
                neighbors.dedup();
                let is_clique = neighbors.iter().enumerate().all(|(i, &a)| neighbors[(i + 1)..].iter().all(|&b| skeleton_adjacent(a, b)));
                if is_clique {
                    found = Some((x, neighbors));
                    break;
                }
            }
            let (x, neighbors) = found?;
            for y in neighbors {
                if remaining_undirected.remove(&norm_pair(x, y)) {
                    directed.push((y, x));
                }
            }
            active.remove(&x);
        }
        CausalDag::new(self.names.clone(), &directed).ok()
    }
}

/// 🧩️ Meek's orientation-propagation rules R1–R3 (Meek, UAI 1995), applied to a fixed point:
/// R1 avoids creating a new unshielded collider, R2 avoids creating a directed cycle, R3 closes a
/// remaining case implied by the first two plus acyclicity. R4 is deferred: every formulation
/// tried during development structurally overlapped R1's trigger condition (R1 always fires first
/// with a different orientation), which reads as a misremembered rule rather than a fixture
/// problem — shipping an unverified R4 risked silently wrong orientations, so it is left out
/// until it can be checked against a reference implementation.
pub fn apply_meek_rules(cpdag: &mut Cpdag) {
    let n = cpdag.n();
    loop {
        let mut changed = false;

        // R1: a->b, b-c undirected, a and c not adjacent => b->c.
        for (a, b) in cpdag.directed_edges() {
            for c in cpdag.adjacent(b) {
                if c != a && cpdag.is_undirected(b, c) && !cpdag.has_edge(a, c) {
                    cpdag.orient(b, c);
                    changed = true;
                }
            }
        }

        // R2: a->b->c, a-c undirected => a->c.
        for (a, b) in cpdag.directed_edges() {
            for c in cpdag.adjacent(b) {
                if cpdag.is_directed(b, c) && cpdag.is_undirected(a, c) {
                    cpdag.orient(a, c);
                    changed = true;
                }
            }
        }

        // R3: a-b, a-c, a-d undirected, c->b, d->b, c and d not adjacent => a->b.
        for a in 0..n {
            let undirected_neighbors: Vec<usize> = cpdag.adjacent(a).into_iter().filter(|&x| cpdag.is_undirected(a, x)).collect();
            for &b in &undirected_neighbors {
                let candidates: Vec<usize> = undirected_neighbors.iter().copied().filter(|&x| x != b && cpdag.is_directed(x, b)).collect();
                let mut orient_here = false;
                for i in 0..candidates.len() {
                    for j in (i + 1)..candidates.len() {
                        if !cpdag.has_edge(candidates[i], candidates[j]) {
                            orient_here = true;
                        }
                    }
                }
                if orient_here {
                    cpdag.orient(a, b);
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }
}
// #endregion 🔖️Cpdag

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
    let adj = mathematical_graph::algorithms::adjacency(dag.n(), &filtered_edges, false);
    let labels = mathematical_graph::algorithms::connected_components(&adj);
    x.iter().all(|&xi| y.iter().all(|&yi| z.contains(&xi) || z.contains(&yi) || labels[xi] != labels[yi]))
}

/// 📜️ Local-Markov implied CIs, emitted pairwise: for each `v`, `v ⟂ w | pa(v)` for every
/// non-descendant `w` that is not a parent of `v`.
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

/// 🔬️ Tests every implied CI against data — a graph-fit diagnostic ("model criticism").
pub fn test_implied_independencies(dag: &CausalDag, data: &mathematical_tabular::Table, test: &dyn CiTest) -> Result<Vec<(CiStatement, mathematical_statistics::TestResult)>, CausalError> {
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
    fn test(&self, data: &mathematical_tabular::Table, x: usize, y: usize, z: &[usize]) -> Result<mathematical_statistics::TestResult, CausalError>;
}

/// 🔬️ Fisher-z partial-correlation test for continuous data, precomputing the correlation matrix
/// once per dataset — PC-stable runs thousands of tests against the same table.
pub struct FisherZ {
    corr: mathematical_algebra::MatD,
    n: usize,
}

impl FisherZ {
    /// 🔬️ Precomputes the complete-case correlation matrix over `columns` (in the same order as
    /// the paired [`CausalDag`]'s variable indices).
    pub fn for_table(data: &mathematical_tabular::Table, columns: &[usize]) -> Result<Self, CausalError> {
        let (corr, n) = mathematical_statistics::correlation_from_table(data, columns)?;
        Ok(Self { corr, n })
    }
}

impl CiTest for FisherZ {
    fn test(&self, _data: &mathematical_tabular::Table, x: usize, y: usize, z: &[usize]) -> Result<mathematical_statistics::TestResult, CausalError> {
        Ok(mathematical_statistics::fisher_z_test(&self.corr, x, y, z, self.n)?)
    }
}

/// 🔬️ G² likelihood-ratio test for categorical data.
pub struct GSquared;

impl CiTest for GSquared {
    fn test(&self, data: &mathematical_tabular::Table, x: usize, y: usize, z: &[usize]) -> Result<mathematical_statistics::TestResult, CausalError> {
        let cat_x = data.categorical(x)?;
        let cat_y = data.categorical(y)?;
        let given_cols: Vec<&mathematical_tabular::CategoricalColumn> = z.iter().map(|&zi| data.categorical(zi)).collect::<Result<_, _>>()?;
        let given_codes: Vec<&[u32]> = given_cols.iter().map(|c| c.codes()).collect();
        let given_levels: Vec<usize> = given_cols.iter().map(|c| c.n_levels()).collect();
        Ok(mathematical_statistics::g2_ci_test(cat_x.codes(), cat_y.codes(), &given_codes, (cat_x.n_levels(), cat_y.n_levels(), &given_levels))?)
    }
}
// #endregion 🔖️CiTest

// #region 🔖️Internal
fn standardize(values: &[f64]) -> Vec<f64> {
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let var = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
    let sd = var.sqrt().max(1e-12);
    values.iter().map(|v| (v - mean) / sd).collect()
}

fn covariance_pop(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let ma = a.iter().sum::<f64>() / n;
    let mb = b.iter().sum::<f64>() / n;
    a.iter().zip(b).map(|(x, y)| (x - ma) * (y - mb)).sum::<f64>() / n
}

fn variance_pop(a: &[f64]) -> f64 {
    covariance_pop(a, a)
}

fn correlation_pop(a: &[f64], b: &[f64]) -> f64 {
    let sa = variance_pop(a).sqrt().max(1e-12);
    let sb = variance_pop(b).sqrt().max(1e-12);
    covariance_pop(a, b) / (sa * sb)
}

/// 🧮️ All `k`-element subsets of `items`, in lexicographic index order.
fn combinations(items: &[usize], k: usize) -> Vec<Vec<usize>> {
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
pub fn pc_stable(data: &mathematical_tabular::Table, test: &dyn CiTest, opts: PcOptions) -> Result<PcResult, CausalError> {
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

/// 📉️ Decomposable Gaussian BIC of one node given a candidate parent set (Chickering, JMLR 2002):
/// `-n/2 * ln(sigma^2) - (|parents|+1)/2 * ln(n)`, the per-node summand a score-based search adds/removes.
#[allow(clippy::needless_range_loop, reason = "row indexes both the MatD design matrix by (row, col) and the values slice — enumerate() would only remove the values index")]
pub fn local_bic(data: &mathematical_tabular::Table, node: usize, parents: &[usize]) -> Result<f64, CausalError> {
    let y = data.continuous(node)?;
    let n = y.len();
    let p = parents.len();
    if parents.is_empty() {
        let m = mathematical_statistics::mean(y)?;
        let ss: f64 = y.iter().map(|v| (v - m).powi(2)).sum();
        let sigma2 = (ss / n as f64).max(1e-12);
        return Ok(-0.5 * n as f64 * sigma2.ln() - 0.5 * (n as f64).ln());
    }
    let mut design = mathematical_algebra::MatD::zeros(n, p);
    for (col, &parent) in parents.iter().enumerate() {
        let values = data.continuous(parent)?;
        for row in 0..n {
            design.set(row, col, values[row]);
        }
    }
    let fit = mathematical_statistics::ols(&design, y, true)?;
    let ss_res: f64 = fit.residuals.iter().map(|r| r * r).sum();
    let sigma2 = (ss_res / n as f64).max(1e-12);
    Ok(-0.5 * n as f64 * sigma2.ln() - 0.5 * (p as f64 + 1.0) * (n as f64).ln())
}

/// 📉️ Sum of local BICs over every node of `dag` given its own parent set.
pub fn dag_bic(data: &mathematical_tabular::Table, dag: &CausalDag) -> Result<f64, CausalError> {
    (0..dag.n()).map(|v| local_bic(data, v, dag.parents(v))).sum()
}

/// 📦️ DirectLiNGAM output: the recovered causal order and the pruned weighted adjacency (`weights[child][parent]`).
pub struct LingamResult {
    pub order: Vec<usize>,
    pub weights: mathematical_algebra::MatD,
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
pub fn direct_lingam(data: &mathematical_tabular::Table, prune_alpha: f64) -> Result<LingamResult, CausalError> {
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

    let mut weights = mathematical_algebra::MatD::zeros(n, n);
    let mut edges = Vec::new();
    for (pos, &child) in order.iter().enumerate() {
        let candidate_parents = &order[..pos];
        if candidate_parents.is_empty() {
            continue;
        }
        let n_rows = columns[child].len();
        let mut design = mathematical_algebra::MatD::zeros(n_rows, candidate_parents.len());
        for (col, &parent) in candidate_parents.iter().enumerate() {
            for row in 0..n_rows {
                design.set(row, col, columns[parent][row]);
            }
        }
        let fit = mathematical_statistics::ols(&design, &columns[child], true)?;
        for (col, &parent) in candidate_parents.iter().enumerate() {
            let coeff = fit.coefficients[col + 1];
            let se = fit.std_errors[col + 1];
            if se > 1e-12 {
                let t_stat = coeff / se;
                let p_value = 2.0 * (1.0 - mathematical_probability::StudentT::new(fit.dof as f64)?.cdf(t_stat.abs()));
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
pub fn ges(data: &mathematical_tabular::Table) -> Result<Cpdag, CausalError> {
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

// #region 🔖️Identification
/// 🚪️ Backdoor criterion (Pearl 2009 §3.3.1): `z` contains no descendant of `x` and blocks every
/// backdoor path, checked as d-separation in the graph with `x`'s outgoing edges removed.
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
    Err(CausalError::NotIdentifiable(
        "no backdoor or frontdoor adjustment found; latent-confounder ADMG identification (general ID algorithm) is out of scope for a fully-observed DAG".to_string(),
    ))
}
// #endregion 🔖️Identification

// #region 🔖️ScmLinear
/// 🧮️ Linear-Gaussian SCM: `v = intercept_v + Σ_p weights[v][p]·p + ε_v`, `ε_v ~ N(0, noise_var[v])`.
#[derive(Clone, Debug)]
pub struct LinearGaussianScm {
    pub dag: CausalDag,
    pub weights: mathematical_algebra::MatD,
    pub intercepts: mathematical_algebra::VecD,
    pub noise_var: mathematical_algebra::VecD,
}

impl LinearGaussianScm {
    /// 📈️ Per-node OLS on parents (roots get their marginal mean/variance).
    #[allow(clippy::needless_range_loop, reason = "row indexes both the MatD design matrix by (row, col) and the pv slice — enumerate() would only remove one of the two")]
    pub fn fit(dag: &CausalDag, data: &mathematical_tabular::Table) -> Result<Self, CausalError> {
        let n = dag.n();
        let mut weights = mathematical_algebra::MatD::zeros(n, n);
        let mut intercepts = mathematical_algebra::VecD::zeros(n);
        let mut noise_var = mathematical_algebra::VecD::zeros(n);
        for v in 0..n {
            let y = data.continuous(v)?;
            let parents = dag.parents(v);
            if parents.is_empty() {
                intercepts.set(v, mathematical_statistics::mean(y)?);
                noise_var.set(v, mathematical_statistics::variance(y).unwrap_or(1e-12).max(1e-12));
                continue;
            }
            let n_rows = y.len();
            let mut design = mathematical_algebra::MatD::zeros(n_rows, parents.len());
            for (col, &p) in parents.iter().enumerate() {
                let pv = data.continuous(p)?;
                for row in 0..n_rows {
                    design.set(row, col, pv[row]);
                }
            }
            let fit = mathematical_statistics::ols(&design, y, true)?;
            intercepts.set(v, fit.coefficients[0]);
            for (col, &p) in parents.iter().enumerate() {
                weights.set(v, p, fit.coefficients[col + 1]);
            }
            noise_var.set(v, fit.sigma2.max(1e-12));
        }
        Ok(Self { dag: dag.clone(), weights, intercepts, noise_var })
    }

    /// 🎲️ Ancestral sampling in topological order.
    #[allow(clippy::needless_range_loop, reason = "row indexes a per-variable column selected by the inner topological-order loop, not a single Vec — no single iterator covers both loop levels")]
    pub fn simulate(&self, n_samples: usize, rng: &mut mathematical_random::Rng) -> Result<mathematical_tabular::Table, CausalError> {
        let n = self.dag.n();
        let mut columns = vec![vec![0.0f64; n_samples]; n];
        for row in 0..n_samples {
            for &v in self.dag.topological_order() {
                let mut val = self.intercepts.get(v);
                for &p in self.dag.parents(v) {
                    val += self.weights.get(v, p) * columns[p][row];
                }
                let sd = self.noise_var.get(v).sqrt().max(1e-12);
                val += mathematical_probability::Normal::new(0.0, sd)?.sample(rng);
                columns[v][row] = val;
            }
        }
        Ok(mathematical_tabular::Table::from_f64_columns(self.dag.names().to_vec(), columns)?)
    }

    /// ✂️ `do(v := value)` for each entry: cuts `v`'s incoming edges, fixes `intercept = value`, `noise_var = 0`.
    pub fn intervened(&self, interventions: &[(usize, f64)]) -> Result<Self, CausalError> {
        let intervened_set: HashSet<usize> = interventions.iter().map(|&(v, _)| v).collect();
        let remaining_edges: Vec<(usize, usize)> = self.dag.edges().into_iter().filter(|&(_, child)| !intervened_set.contains(&child)).collect();
        let new_dag = CausalDag::new(self.dag.names().to_vec(), &remaining_edges)?;
        let n = self.dag.n();
        let mut weights = mathematical_algebra::MatD::zeros(n, n);
        let mut intercepts = self.intercepts.clone();
        let mut noise_var = self.noise_var.clone();
        for v in 0..n {
            if intervened_set.contains(&v) {
                continue;
            }
            for &p in self.dag.parents(v) {
                weights.set(v, p, self.weights.get(v, p));
            }
        }
        for &(v, value) in interventions {
            intercepts.set(v, value);
            noise_var.set(v, 0.0);
        }
        Ok(Self { dag: new_dag, weights, intercepts, noise_var })
    }

    /// 🧮️ `E[v]` for every `v`, via forward substitution in topological order.
    pub fn mean(&self) -> mathematical_algebra::VecD {
        let mut mu = mathematical_algebra::VecD::zeros(self.dag.n());
        for &v in self.dag.topological_order() {
            let mut val = self.intercepts.get(v);
            for &p in self.dag.parents(v) {
                val += self.weights.get(v, p) * mu.get(p);
            }
            mu.set(v, val);
        }
        mu
    }

    /// 🧮️ Implied covariance `(I−B)⁻¹ D (I−B)⁻ᵀ`.
    pub fn implied_covariance(&self) -> Result<mathematical_algebra::MatD, CausalError> {
        let n = self.dag.n();
        let mut i_minus_b = mathematical_algebra::MatD::identity(n);
        for v in 0..n {
            for &p in self.dag.parents(v) {
                i_minus_b.set(v, p, -self.weights.get(v, p));
            }
        }
        let inv = mathematical_statistics::invert(&i_minus_b)?;
        let mut d = mathematical_algebra::MatD::zeros(n, n);
        for v in 0..n {
            d.set(v, v, self.noise_var.get(v));
        }
        Ok(inv.matmul(&d).matmul(&inv.transpose()))
    }

    /// 🎯️ `∂E[y]/∂(do x)`: sum over directed `x -> ... -> y` paths of the product of edge
    /// weights, via topological dynamic programming.
    pub fn total_effect(&self, x: usize, y: usize) -> f64 {
        let mut sensitivity = vec![0.0f64; self.dag.n()];
        sensitivity[x] = 1.0;
        for &v in self.dag.topological_order() {
            if v == x {
                continue;
            }
            let mut acc = 0.0;
            for &p in self.dag.parents(v) {
                acc += self.weights.get(v, p) * sensitivity[p];
            }
            sensitivity[v] = acc;
        }
        sensitivity[y]
    }

    /// 🎯️ `E[y | do(x=1)] − E[y | do(x=0)]`; equals [`LinearGaussianScm::total_effect`] for a
    /// linear model, kept as a distinct name for symmetry with the potential-outcomes estimators.
    pub fn ate(&self, x: usize, y: usize) -> f64 {
        self.total_effect(x, y)
    }

    /// 🔮️ Abduction–action–prediction (Pearl 2009 ch. 7, Thm 7.1.7): recovers every exogenous
    /// noise term from a fully observed row, applies `interventions`, and re-propagates the same
    /// noise — exact for a linear-Gaussian model.
    pub fn counterfactual(&self, observed: &[f64], interventions: &[(usize, f64)]) -> Result<mathematical_algebra::VecD, CausalError> {
        let n = self.dag.n();
        if observed.len() != n {
            return Err(CausalError::DimensionMismatch(format!("observed row has {} entries, expected {n}", observed.len())));
        }
        let mut noise = vec![0.0f64; n];
        for v in 0..n {
            let mut predicted = self.intercepts.get(v);
            for &p in self.dag.parents(v) {
                predicted += self.weights.get(v, p) * observed[p];
            }
            noise[v] = observed[v] - predicted;
        }
        let intervened_map: HashMap<usize, f64> = interventions.iter().copied().collect();
        let mut out = mathematical_algebra::VecD::zeros(n);
        for &v in self.dag.topological_order() {
            if let Some(&value) = intervened_map.get(&v) {
                out.set(v, value);
                continue;
            }
            let mut val = self.intercepts.get(v);
            for &p in self.dag.parents(v) {
                val += self.weights.get(v, p) * out.get(p);
            }
            val += noise[v];
            out.set(v, val);
        }
        Ok(out)
    }

    /// 🔮️ `E[target | do(interventions)]`: the mean of the intervened model at `target`.
    pub fn interventional_mean(&self, target: usize, interventions: &[(usize, f64)]) -> Result<f64, CausalError> {
        Ok(self.intervened(interventions)?.mean().get(target))
    }
}
// #endregion 🔖️ScmLinear

// #region 🔖️ScmDiscrete
/// 🎲️ Conditional probability table for one node: `probs` is flat, indexed `[node value fastest,
/// then parents[0], parents[1], ...]` — the same mixed-radix layout [`Factor`] uses internally, so
/// [`Factor::from_cpt`] can copy it directly.
#[derive(Clone, Debug)]
pub struct Cpt {
    pub node: usize,
    pub parents: Vec<usize>,
    pub cardinality: usize,
    pub probs: Vec<f64>,
}

/// 🎲️ A CPT-based Bayesian network treated as a discrete SCM: interventions replace a node's CPT
/// with a point mass. Counterfactuals are out of scope for v1 — CPTs fix only the joint
/// distribution, not the functional/twin-network structure a counterfactual query needs.
#[derive(Clone, Debug)]
pub struct DiscreteScm {
    pub dag: CausalDag,
    pub cardinalities: Vec<usize>,
    pub cpts: Vec<Cpt>,
}

impl DiscreteScm {
    /// 📊️ MLE CPTs from categorical columns with additive (Laplace) smoothing `pseudocount` (`0.0` = pure MLE).
    pub fn fit(dag: &CausalDag, data: &mathematical_tabular::Table, pseudocount: f64) -> Result<Self, CausalError> {
        let n = dag.n();
        let columns: Vec<&mathematical_tabular::CategoricalColumn> = (0..n).map(|i| data.categorical(i)).collect::<Result<_, _>>()?;
        let cardinalities: Vec<usize> = (0..n).map(|v| columns[v].n_levels()).collect();
        let mut cpts = Vec::with_capacity(n);
        for v in 0..n {
            let parents = dag.parents(v).to_vec();
            let n_configs = parents.iter().map(|&p| cardinalities[p]).product::<usize>().max(1);
            let mut counts = vec![pseudocount; n_configs * cardinalities[v]];
            for row in 0..data.n_rows() {
                let own = columns[v].codes()[row];
                if own == mathematical_tabular::MISSING_CODE {
                    continue;
                }
                let mut config = 0usize;
                let mut stride = 1usize;
                let mut missing_parent = false;
                for &p in &parents {
                    let code = columns[p].codes()[row];
                    if code == mathematical_tabular::MISSING_CODE {
                        missing_parent = true;
                        break;
                    }
                    config += code as usize * stride;
                    stride *= cardinalities[p];
                }
                if missing_parent {
                    continue;
                }
                counts[config * cardinalities[v] + own as usize] += 1.0;
            }
            let mut probs = vec![0.0; counts.len()];
            for config in 0..n_configs {
                let start = config * cardinalities[v];
                let total: f64 = counts[start..start + cardinalities[v]].iter().sum();
                let total = if total > 0.0 { total } else { 1.0 };
                for k in 0..cardinalities[v] {
                    probs[start + k] = counts[start + k] / total;
                }
            }
            cpts.push(Cpt { node: v, parents, cardinality: cardinalities[v], probs });
        }
        Ok(Self { dag: dag.clone(), cardinalities, cpts })
    }

    /// 🎲️ Ancestral sampling in topological order.
    #[allow(clippy::needless_range_loop, reason = "row indexes a per-variable column selected by the inner topological-order loop, not a single Vec — no single iterator covers both loop levels")]
    pub fn simulate(&self, n_samples: usize, rng: &mut mathematical_random::Rng) -> Result<mathematical_tabular::Table, CausalError> {
        let n = self.dag.n();
        let mut codes = vec![vec![0u32; n_samples]; n];
        for row in 0..n_samples {
            for &v in self.dag.topological_order() {
                let cpt = &self.cpts[v];
                let mut config = 0usize;
                let mut stride = 1usize;
                for &p in &cpt.parents {
                    config += codes[p][row] as usize * stride;
                    stride *= self.cardinalities[p];
                }
                let start = config * cpt.cardinality;
                let probs = &cpt.probs[start..start + cpt.cardinality];
                let mut draw = rng.next_f64();
                let mut chosen = cpt.cardinality - 1;
                for (k, &p) in probs.iter().enumerate() {
                    if draw < p {
                        chosen = k;
                        break;
                    }
                    draw -= p;
                }
                codes[v][row] = chosen as u32;
            }
        }
        let names = self.dag.names().to_vec();
        let columns: Vec<(Vec<u32>, Vec<String>)> = (0..n).map(|v| (codes[v].clone(), (0..self.cardinalities[v]).map(|k| format!("l{k}")).collect())).collect();
        Ok(mathematical_tabular::Table::from_categorical_columns(names, columns)?)
    }

    /// ✂️ `do(v := value)` for each entry: replaces `v`'s CPT with a point mass and drops its parents.
    pub fn intervened(&self, interventions: &[(usize, u32)]) -> Result<Self, CausalError> {
        let intervened_map: HashMap<usize, u32> = interventions.iter().copied().collect();
        let remaining_edges: Vec<(usize, usize)> = self.dag.edges().into_iter().filter(|&(_, child)| !intervened_map.contains_key(&child)).collect();
        let new_dag = CausalDag::new(self.dag.names().to_vec(), &remaining_edges)?;
        let mut cpts = self.cpts.clone();
        for (&v, &value) in &intervened_map {
            let card = self.cardinalities[v];
            let mut probs = vec![0.0; card];
            probs[value as usize] = 1.0;
            cpts[v] = Cpt { node: v, parents: Vec::new(), cardinality: card, probs };
        }
        Ok(Self { dag: new_dag, cardinalities: self.cardinalities.clone(), cpts })
    }

    /// 🔢️ `P(target | evidence)` by variable elimination (Zhang & Poole 1994) over a
    /// min-remaining-factors ordering, guarded against blow-up.
    pub fn posterior(&self, target: usize, evidence: &[(usize, u32)]) -> Result<Vec<f64>, CausalError> {
        const MAX_FACTOR_ENTRIES: usize = 1_000_000;
        let n = self.dag.n();
        let evidence_map: HashMap<usize, u32> = evidence.iter().copied().collect();
        let mut factors: Vec<Factor> = self.cpts.iter().map(|cpt| Factor::from_cpt(cpt, &self.cardinalities)).collect();
        for (&v, &value) in &evidence_map {
            for factor in factors.iter_mut() {
                *factor = factor.restrict(v, value);
            }
        }
        let mut eliminate: Vec<usize> = (0..n).filter(|v| *v != target && !evidence_map.contains_key(v)).collect();
        while !eliminate.is_empty() {
            let (pos, &var) =
                eliminate.iter().enumerate().min_by_key(|&(_, &v)| factors.iter().filter(|f| f.vars.contains(&v)).count()).expect("eliminate is non-empty");
            eliminate.remove(pos);
            let (involved, mut rest): (Vec<Factor>, Vec<Factor>) = factors.into_iter().partition(|f| f.vars.contains(&var));
            if involved.is_empty() {
                factors = rest;
                continue;
            }
            let mut product = involved[0].clone();
            for f in &involved[1..] {
                product = product.multiply(f);
                if product.values.len() > MAX_FACTOR_ENTRIES {
                    return Err(CausalError::InferenceTooLarge(product.values.len(), MAX_FACTOR_ENTRIES));
                }
            }
            rest.push(product.marginalize(var));
            factors = rest;
        }
        let mut result = factors[0].clone();
        for f in &factors[1..] {
            result = result.multiply(f);
        }
        let total: f64 = result.values.iter().sum();
        if total <= 0.0 {
            return Err(CausalError::InvalidQuery("evidence has zero probability".to_string()));
        }
        Ok(result.values.iter().map(|v| v / total).collect())
    }

    /// 🔮️ `P(target | do(interventions), evidence)`: posterior on the intervened network.
    pub fn interventional_distribution(&self, target: usize, interventions: &[(usize, u32)], evidence: &[(usize, u32)]) -> Result<Vec<f64>, CausalError> {
        self.intervened(interventions)?.posterior(target, evidence)
    }
}

/// 🧮️ A variable-elimination factor: a flat table over `vars`, mixed-radix with `vars[0]` fastest-varying.
#[derive(Clone, Debug)]
struct Factor {
    vars: Vec<usize>,
    cards: Vec<usize>,
    values: Vec<f64>,
}

impl Factor {
    fn from_cpt(cpt: &Cpt, cardinalities: &[usize]) -> Self {
        let mut vars = vec![cpt.node];
        vars.extend_from_slice(&cpt.parents);
        let cards: Vec<usize> = vars.iter().map(|&v| cardinalities[v]).collect();
        Self { vars, cards, values: cpt.probs.clone() }
    }

    fn encode(&self, assignment: &HashMap<usize, u32>) -> usize {
        let mut idx = 0usize;
        let mut stride = 1usize;
        for (i, &v) in self.vars.iter().enumerate() {
            idx += assignment[&v] as usize * stride;
            stride *= self.cards[i];
        }
        idx
    }

    fn restrict(&self, var: usize, value: u32) -> Self {
        let Some(pos) = self.vars.iter().position(|&v| v == var) else {
            return self.clone();
        };
        let new_vars: Vec<usize> = self.vars.iter().copied().enumerate().filter(|&(i, _)| i != pos).map(|(_, v)| v).collect();
        let new_cards: Vec<usize> = self.cards.iter().copied().enumerate().filter(|&(i, _)| i != pos).map(|(_, c)| c).collect();
        let total_new = new_cards.iter().product::<usize>().max(1);
        let mut values = Vec::with_capacity(total_new);
        for idx in 0..total_new {
            let mut assignment = decode(&new_vars, &new_cards, idx);
            assignment.insert(var, value);
            values.push(self.values[self.encode(&assignment)]);
        }
        Factor { vars: new_vars, cards: new_cards, values }
    }

    fn multiply(&self, other: &Factor) -> Factor {
        let mut vars = self.vars.clone();
        let mut cards = self.cards.clone();
        for (i, &v) in other.vars.iter().enumerate() {
            if !vars.contains(&v) {
                vars.push(v);
                cards.push(other.cards[i]);
            }
        }
        let total = cards.iter().product::<usize>().max(1);
        let mut values = Vec::with_capacity(total);
        for idx in 0..total {
            let assignment = decode(&vars, &cards, idx);
            values.push(self.values[self.encode(&assignment)] * other.values[other.encode(&assignment)]);
        }
        Factor { vars, cards, values }
    }

    fn marginalize(&self, var: usize) -> Factor {
        let Some(pos) = self.vars.iter().position(|&v| v == var) else {
            return self.clone();
        };
        let new_vars: Vec<usize> = self.vars.iter().copied().enumerate().filter(|&(i, _)| i != pos).map(|(_, v)| v).collect();
        let new_cards: Vec<usize> = self.cards.iter().copied().enumerate().filter(|&(i, _)| i != pos).map(|(_, c)| c).collect();
        let total_new = new_cards.iter().product::<usize>().max(1);
        let mut values = vec![0.0; total_new];
        for (new_idx, slot) in values.iter_mut().enumerate() {
            let mut assignment = decode(&new_vars, &new_cards, new_idx);
            let mut sum = 0.0;
            for k in 0..self.cards[pos] {
                assignment.insert(var, k as u32);
                sum += self.values[self.encode(&assignment)];
            }
            *slot = sum;
        }
        Factor { vars: new_vars, cards: new_cards, values }
    }
}

/// 🧮️ Decodes a flat mixed-radix index (`vars[0]` fastest-varying) into a var -> value assignment.
fn decode(vars: &[usize], cards: &[usize], mut idx: usize) -> HashMap<usize, u32> {
    let mut out = HashMap::with_capacity(vars.len());
    for (i, &v) in vars.iter().enumerate() {
        let c = cards[i];
        out.insert(v, (idx % c) as u32);
        idx /= c;
    }
    out
}
// #endregion 🔖️ScmDiscrete

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

fn bootstrap_ci(data: &mathematical_tabular::Table, opts: &BootstrapOptions, point_fn: &dyn Fn(&mathematical_tabular::Table) -> Result<f64, CausalError>) -> (f64, f64) {
    let mut rng = mathematical_random::Rng::from_seed(opts.seed);
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

fn wrap_estimate(point: f64, data: &mathematical_tabular::Table, opts: &EstimationOptions, point_fn: &dyn Fn(&mathematical_tabular::Table) -> Result<f64, CausalError>) -> EffectEstimate {
    match &opts.bootstrap {
        Some(bootstrap_opts) => {
            let (lo, hi) = bootstrap_ci(data, bootstrap_opts, point_fn);
            EffectEstimate { estimate: point, ci: Some((lo, hi)), ci_level: bootstrap_opts.level }
        }
        None => EffectEstimate { estimate: point, ci: None, ci_level: 0.0 },
    }
}

fn naive_difference_point(data: &mathematical_tabular::Table, treatment: usize, outcome: usize) -> Result<f64, CausalError> {
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
pub fn naive_difference(data: &mathematical_tabular::Table, treatment: usize, outcome: usize, opts: &EstimationOptions) -> Result<EffectEstimate, CausalError> {
    let point = naive_difference_point(data, treatment, outcome)?;
    Ok(wrap_estimate(point, data, opts, &|d| naive_difference_point(d, treatment, outcome)))
}

#[allow(clippy::needless_range_loop, reason = "row indexes both the MatD design matrix by (row, col) and a values/t slice — enumerate() would only remove one of the two")]
fn design_with_treatment(data: &mathematical_tabular::Table, treatment: usize, covariates: &[usize], treatment_value: Option<f64>) -> Result<mathematical_algebra::MatD, CausalError> {
    let n = data.n_rows();
    let mut design = mathematical_algebra::MatD::zeros(n, covariates.len() + 1);
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

fn g_formula_point(data: &mathematical_tabular::Table, treatment: usize, outcome: usize, covariates: &[usize]) -> Result<f64, CausalError> {
    let y = data.continuous(outcome)?;
    let design = design_with_treatment(data, treatment, covariates, None)?;
    let fit = mathematical_statistics::ols(&design, y, true)?;
    let predict_mean = |design: &mathematical_algebra::MatD| -> f64 {
        let n = design.rows;
        let p = design.cols;
        let total: f64 = (0..n)
            .map(|row| fit.coefficients[0] + (0..p).map(|col| fit.coefficients[col + 1] * design.get(row, col)).sum::<f64>())
            .sum();
        total / n as f64
    };
    let design_t1 = design_with_treatment(data, treatment, covariates, Some(1.0))?;
    let design_t0 = design_with_treatment(data, treatment, covariates, Some(0.0))?;
    Ok(predict_mean(&design_t1) - predict_mean(&design_t0))
}

/// 📏️ G-formula / regression-adjustment ATE: fit `y ~ t + covariates` by OLS, then average the
/// model's predicted `y` at `t=1` minus at `t=0` over the empirical covariate distribution.
pub fn g_formula_ate(data: &mathematical_tabular::Table, treatment: usize, outcome: usize, covariates: &[usize], opts: &EstimationOptions) -> Result<EffectEstimate, CausalError> {
    let point = g_formula_point(data, treatment, outcome, covariates)?;
    Ok(wrap_estimate(point, data, opts, &|d| g_formula_point(d, treatment, outcome, covariates)))
}

#[allow(clippy::needless_range_loop, reason = "row indexes both the MatD design matrix by (row, col) and the values slice — enumerate() would only remove the values index")]
fn ipw_point(data: &mathematical_tabular::Table, treatment: usize, outcome: usize, covariates: &[usize]) -> Result<f64, CausalError> {
    const EPS: f64 = 1e-6;
    let n = data.n_rows();
    let mut design = mathematical_algebra::MatD::zeros(n, covariates.len());
    for (col, &c) in covariates.iter().enumerate() {
        let values = data.continuous(c)?;
        for row in 0..n {
            design.set(row, col, values[row]);
        }
    }
    let t = data.continuous(treatment)?;
    let fit = mathematical_statistics::logistic(&design, t, true)?;
    let propensity = mathematical_statistics::logistic_predict(&fit, &design, true)?;
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
pub fn ipw_ate(data: &mathematical_tabular::Table, treatment: usize, outcome: usize, covariates: &[usize], opts: &EstimationOptions) -> Result<EffectEstimate, CausalError> {
    let point = ipw_point(data, treatment, outcome, covariates)?;
    Ok(wrap_estimate(point, data, opts, &|d| ipw_point(d, treatment, outcome, covariates)))
}
// #endregion 🔖️Estimation

// #region 🔖️Query
/// 🔮️ One what-if request: `do(...)` interventions and, for counterfactual queries, `given(...)` factual evidence.
#[derive(Clone, Debug, Default)]
pub struct WhatIf {
    pub interventions: Vec<(usize, f64)>,
    pub evidence: Vec<(usize, f64)>,
}

impl WhatIf {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn do_(mut self, var: usize, value: f64) -> Self {
        self.interventions.push((var, value));
        self
    }

    pub fn given(mut self, var: usize, value: f64) -> Self {
        self.evidence.push((var, value));
        self
    }
}

impl LinearGaussianScm {
    /// 🔮️ Level 2 (empty evidence): interventional mean of `target`. Level 3 (evidence covering
    /// every variable): exact counterfactual via abduction–action–prediction. Partial evidence is
    /// out of scope for v1 (needs conditional-MVN abduction) and returns `InvalidQuery`.
    pub fn query(&self, target: usize, what_if: &WhatIf) -> Result<f64, CausalError> {
        if what_if.evidence.is_empty() {
            return self.interventional_mean(target, &what_if.interventions);
        }
        if what_if.evidence.len() != self.dag.n() {
            return Err(CausalError::InvalidQuery(
                "counterfactual queries need evidence for every variable in v1 (partial-evidence abduction is out of scope)".to_string(),
            ));
        }
        let mut observed = vec![0.0; self.dag.n()];
        for &(v, value) in &what_if.evidence {
            observed[v] = value;
        }
        Ok(self.counterfactual(&observed, &what_if.interventions)?.get(target))
    }
}
// #endregion 🔖️Query

// #region 🔖️UnitTests
#[cfg(test)]
mod tests {
    use super::*;

    // #region 🔖️Fixtures
    fn chain3() -> CausalDag {
        // x -> y -> z
        CausalDag::from_named_edges(vec!["x".into(), "y".into(), "z".into()], &[("x", "y"), ("y", "z")]).unwrap()
    }

    fn fork3() -> CausalDag {
        // x <- y -> z
        CausalDag::from_named_edges(vec!["x".into(), "y".into(), "z".into()], &[("y", "x"), ("y", "z")]).unwrap()
    }

    fn collider3() -> CausalDag {
        // x -> z <- y
        CausalDag::from_named_edges(vec!["x".into(), "y".into(), "z".into()], &[("x", "z"), ("y", "z")]).unwrap()
    }
    // #endregion 🔖️Fixtures

    // #region 🔖️DSeparationTests
    #[test]
    fn chain_d_separation_pattern() {
        let dag = chain3();
        assert!(d_separated(&dag, &[0], &[2], &[1]), "x _||_ z | y should hold on a chain");
        assert!(!d_separated(&dag, &[0], &[2], &[]), "x _||_ z should not hold marginally on a chain");
    }

    #[test]
    fn fork_d_separation_pattern() {
        let dag = fork3();
        assert!(d_separated(&dag, &[0], &[2], &[1]), "x _||_ z | y should hold on a fork");
        assert!(!d_separated(&dag, &[0], &[2], &[]), "x _||_ z should not hold marginally on a fork");
    }

    #[test]
    fn collider_d_separation_pattern() {
        let dag = collider3();
        assert!(d_separated(&dag, &[0], &[1], &[]), "x _||_ y should hold marginally on a collider");
        assert!(!d_separated(&dag, &[0], &[1], &[2]), "x _||_ y | z should not hold: conditioning on the collider opens the path");
    }

    #[test]
    fn moralization_marries_coparents() {
        let dag = collider3();
        let moral = dag.moralize();
        assert!(moral.contains(&(0, 1)), "co-parents 0 and 1 of the collider should be married");
    }

    #[test]
    fn implied_independencies_hold_by_d_separation() {
        let dag = chain3();
        for stmt in implied_independencies(&dag) {
            assert!(d_separated(&dag, &[stmt.x], &[stmt.y], &stmt.z), "implied statement {stmt:?} should be d-separated");
        }
    }
    // #endregion 🔖️DSeparationTests

    // #region 🔖️CpdagTests
    #[test]
    fn cpdag_from_chain_is_fully_undirected() {
        let cpdag = Cpdag::from_dag(&chain3());
        assert!(cpdag.is_undirected(0, 1));
        assert!(cpdag.is_undirected(1, 2));
        assert!(cpdag.directed_edges().is_empty());
    }

    #[test]
    fn cpdag_from_collider_keeps_both_arrows() {
        let cpdag = Cpdag::from_dag(&collider3());
        assert!(cpdag.is_directed(0, 2));
        assert!(cpdag.is_directed(1, 2));
        assert!(!cpdag.has_edge(0, 1), "collider parents should not be adjacent");
    }

    #[test]
    fn cpdag_to_dag_round_trip_preserves_skeleton_and_v_structures() {
        let dag = collider3();
        let cpdag = Cpdag::from_dag(&dag);
        let extended = cpdag.to_dag().expect("collider CPDAG is extendable");
        assert_eq!(extended.edges().len(), dag.edges().len());
        assert!(extended.parents(2).contains(&0) && extended.parents(2).contains(&1));
    }

    #[test]
    fn cpdag_to_dag_round_trip_on_chain_is_acyclic_and_same_skeleton() {
        let dag = chain3();
        let cpdag = Cpdag::from_dag(&dag);
        let extended = cpdag.to_dag().expect("chain CPDAG is extendable");
        assert_eq!(extended.edges().len(), 2);
    }

    #[test]
    fn meek_rule1_orients_to_avoid_new_collider() {
        // a -> b, b - c undirected, a and c not adjacent => b -> c.
        let mut cpdag = Cpdag::new(vec!["a".into(), "b".into(), "c".into()]);
        cpdag.directed.insert((0, 1));
        cpdag.undirected.insert((1, 2));
        apply_meek_rules(&mut cpdag);
        assert!(cpdag.is_directed(1, 2));
    }

    #[test]
    fn meek_rule2_orients_to_avoid_cycle() {
        // a -> b -> c, a - c undirected => a -> c.
        let mut cpdag = Cpdag::new(vec!["a".into(), "b".into(), "c".into()]);
        cpdag.directed.insert((0, 1));
        cpdag.directed.insert((1, 2));
        cpdag.undirected.insert((0, 2));
        apply_meek_rules(&mut cpdag);
        assert!(cpdag.is_directed(0, 2));
    }

    #[test]
    fn meek_rule3_orients_via_two_directed_co_parents() {
        // a-b, a-c, a-d undirected; c->b, d->b directed; c,d not adjacent => a->b.
        let mut cpdag = Cpdag::new(vec!["a".into(), "b".into(), "c".into(), "d".into()]);
        cpdag.undirected.insert((0, 1));
        cpdag.undirected.insert((0, 2));
        cpdag.undirected.insert((0, 3));
        cpdag.directed.insert((2, 1));
        cpdag.directed.insert((3, 1));
        apply_meek_rules(&mut cpdag);
        assert!(cpdag.is_directed(0, 1));
    }

    // #endregion 🔖️CpdagTests

    // #region 🔖️CiTestTests
    #[test]
    fn fisher_z_ci_test_via_causal_dag_columns() {
        let mut table = mathematical_tabular::Table::new();
        let n = 200;
        let mut rng = mathematical_random::Rng::from_seed(7);
        let x: Vec<f64> = (0..n).map(|_| mathematical_probability::Normal::STANDARD.sample(&mut rng)).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi * 0.8 + mathematical_probability::Normal::STANDARD.sample(&mut rng) * 0.2).collect();
        let z: Vec<f64> = y.iter().map(|&yi| yi * 0.8 + mathematical_probability::Normal::STANDARD.sample(&mut rng) * 0.2).collect();
        table.push_continuous("x", x).unwrap();
        table.push_continuous("y", y).unwrap();
        table.push_continuous("z", z).unwrap();
        let ci = FisherZ::for_table(&table, &[0, 1, 2]).unwrap();
        let marginal = ci.test(&table, 0, 2, &[]).unwrap();
        let conditional = ci.test(&table, 0, 2, &[1]).unwrap();
        assert!(marginal.p_value < 0.05, "x,z should look dependent marginally");
        assert!(conditional.p_value > 0.05, "x,z should look independent given y on a chain");
    }
    // #endregion 🔖️CiTestTests

    // #region 🔖️DiscoveryTests
    fn linear_chain_scm() -> LinearGaussianScm {
        // x -> m -> y, coefficients 2.0 and 1.5.
        let dag = CausalDag::from_named_edges(vec!["x".into(), "m".into(), "y".into()], &[("x", "m"), ("m", "y")]).unwrap();
        let mut weights = mathematical_algebra::MatD::zeros(3, 3);
        weights.set(1, 0, 2.0);
        weights.set(2, 1, 1.5);
        LinearGaussianScm { dag, weights, intercepts: mathematical_algebra::VecD::from_vec(vec![0.0, 0.0, 0.0]), noise_var: mathematical_algebra::VecD::from_vec(vec![1.0, 0.25, 0.25]) }
    }

    #[test]
    fn local_bic_prefers_the_true_parent_set() {
        let scm = linear_chain_scm();
        let mut rng = mathematical_random::Rng::from_seed(99);
        let mut data = scm.simulate(500, &mut rng).unwrap();
        // An exogenous, causally unrelated column — unlike `y` (a downstream descendant of `m`
        // in the chain, and thus itself strongly correlated with `m`), this one has no relationship
        // to `m` at all, so it is a genuine negative control for "does BIC reward a real parent".
        let unrelated: Vec<f64> = (0..data.n_rows()).map(|_| mathematical_probability::Normal::STANDARD.sample(&mut rng)).collect();
        data.push_continuous("unrelated", unrelated).unwrap();
        let with_true_parent = local_bic(&data, 1, &[0]).unwrap();
        let with_no_parent = local_bic(&data, 1, &[]).unwrap();
        let with_unrelated_covariate = local_bic(&data, 1, &[3]).unwrap();
        assert!(with_true_parent > with_no_parent, "true parent should score higher than no parent");
        assert!(with_true_parent > with_unrelated_covariate, "true parent should score higher than an unrelated covariate");
    }
    // #endregion 🔖️DiscoveryTests

    // #region 🔖️IdentificationTests
    #[test]
    fn backdoor_minimal_set_on_confounder_triangle() {
        // z -> x, z -> y, x -> y.
        let dag = CausalDag::from_named_edges(vec!["x".into(), "y".into(), "z".into()], &[("z", "x"), ("z", "y"), ("x", "y")]).unwrap();
        let sets = minimal_backdoor_sets(&dag, 0, 1, 3);
        assert_eq!(sets, vec![vec![2]]);
    }

    #[test]
    fn backdoor_m_bias_graph_allows_empty_adjustment() {
        // a -> x, b -> y, a -> m, b -> m (m is a collider, not on any backdoor path): x -> y direct edge.
        let dag = CausalDag::from_named_edges(vec!["x".into(), "y".into(), "a".into(), "b".into(), "m".into()], &[("a", "x"), ("b", "y"), ("a", "m"), ("b", "m"), ("x", "y")]).unwrap();
        assert!(backdoor_satisfied(&dag, 0, 1, &[]), "no backdoor path from x to y should exist here");
    }

    #[test]
    fn frontdoor_satisfied_on_classic_mediator_graph() {
        // u -> x, u -> y (unobserved confounder u), x -> m -> y.
        let dag = CausalDag::from_named_edges(vec!["x".into(), "y".into(), "m".into(), "u".into()], &[("u", "x"), ("u", "y"), ("x", "m"), ("m", "y")]).unwrap();
        assert!(frontdoor_satisfied(&dag, 0, 1, &[2]));
    }

    #[test]
    fn identify_returns_no_confounding_for_a_direct_unconfounded_edge() {
        let dag = CausalDag::from_named_edges(vec!["x".into(), "y".into()], &[("x", "y")]).unwrap();
        assert_eq!(identify(&dag, 0, 1).unwrap(), Identification::NoConfounding);
    }

    #[test]
    fn identify_returns_backdoor_for_a_confounded_edge() {
        let dag = CausalDag::from_named_edges(vec!["x".into(), "y".into(), "z".into()], &[("z", "x"), ("z", "y"), ("x", "y")]).unwrap();
        assert_eq!(identify(&dag, 0, 1).unwrap(), Identification::Backdoor { adjustment: vec![2] });
    }
    // #endregion 🔖️IdentificationTests

    // #region 🔖️ScmLinearTests
    #[test]
    fn linear_scm_total_effect_matches_analytic_path_product() {
        let scm = linear_chain_scm();
        assert!((scm.total_effect(0, 2) - 3.0).abs() < 1e-9);
        assert!((scm.ate(0, 2) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn linear_scm_implied_covariance_matches_hand_computation() {
        let scm = linear_chain_scm();
        let cov = scm.implied_covariance().unwrap();
        // Var(x) = 1.0; Var(m) = 4*1.0 + 0.25 = 4.25; Var(y) = 1.5^2*4.25 + 0.25 = 9.8125; Cov(x,y) = 2.0*1.5*Var(x) = 3.0.
        assert!((cov.get(0, 0) - 1.0).abs() < 1e-9);
        assert!((cov.get(1, 1) - 4.25).abs() < 1e-9);
        assert!((cov.get(2, 2) - 9.8125).abs() < 1e-9);
        assert!((cov.get(0, 2) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn linear_scm_counterfactual_shifts_by_exact_path_product_preserving_noise() {
        let scm = linear_chain_scm();
        let observed = [1.0, 2.5, 5.0]; // noise_m = 2.5 - 2.0*1.0 = 0.5; noise_y = 5.0 - 1.5*2.5 = 1.25
        let cf = scm.counterfactual(&observed, &[(0, 2.0)]).unwrap();
        // do(x=2.0): m = 2.0*2.0 + 0.5 = 4.5; y = 1.5*4.5 + 1.25 = 8.0
        assert!((cf.get(1) - 4.5).abs() < 1e-9);
        assert!((cf.get(2) - 8.0).abs() < 1e-9);
        // shift in y should equal exactly a*b*(delta x) = 2.0*1.5*1.0 = 3.0
        assert!((cf.get(2) - observed[2] - 3.0).abs() < 1e-9);
    }

    #[test]
    fn what_if_query_level2_vs_level3() {
        let scm = linear_chain_scm();
        let interventional = scm.query(2, &WhatIf::new().do_(0, 2.0)).unwrap();
        // E[y | do(x=2)] = 1.5 * (2.0 * 2.0) = 6.0 (all intercepts are 0 in this fixture).
        assert!((interventional - 6.0).abs() < 1e-9);
        let observed = [1.0, 2.5, 5.0];
        let what_if = WhatIf::new().do_(0, 2.0).given(0, observed[0]).given(1, observed[1]).given(2, observed[2]);
        let counterfactual = scm.query(2, &what_if).unwrap();
        assert!((counterfactual - 8.0).abs() < 1e-9);
    }
    // #endregion 🔖️ScmLinearTests

    // #region 🔖️ScmDiscreteTests
    fn sprinkler_scm() -> DiscreteScm {
        // Classic sprinkler net: Cloudy -> {Sprinkler, Rain} -> Wet. All variables binary (0=false, 1=true).
        let dag = CausalDag::from_named_edges(
            vec!["cloudy".into(), "sprinkler".into(), "rain".into(), "wet".into()],
            &[("cloudy", "sprinkler"), ("cloudy", "rain"), ("sprinkler", "wet"), ("rain", "wet")],
        )
        .unwrap();
        let cardinalities = vec![2, 2, 2, 2];
        let cloudy = Cpt { node: 0, parents: vec![], cardinality: 2, probs: vec![0.5, 0.5] };
        // sprinkler | cloudy: P(sprinkler=1|cloudy=0)=0.5, P(sprinkler=1|cloudy=1)=0.1
        let sprinkler = Cpt { node: 1, parents: vec![0], cardinality: 2, probs: vec![0.5, 0.5, 0.9, 0.1] };
        // rain | cloudy: P(rain=1|cloudy=0)=0.2, P(rain=1|cloudy=1)=0.8
        let rain = Cpt { node: 2, parents: vec![0], cardinality: 2, probs: vec![0.8, 0.2, 0.2, 0.8] };
        // wet | sprinkler, rain (config mixed-radix: sprinkler fastest)
        let wet = Cpt {
            node: 3,
            parents: vec![1, 2],
            cardinality: 2,
            probs: vec![
                1.0, 0.0, // sprinkler=0,rain=0
                0.1, 0.9, // sprinkler=1,rain=0
                0.1, 0.9, // sprinkler=0,rain=1
                0.01, 0.99, // sprinkler=1,rain=1
            ],
        };
        DiscreteScm { dag, cardinalities, cpts: vec![cloudy, sprinkler, rain, wet] }
    }

    #[test]
    fn sprinkler_posterior_vs_interventional_distribution_contrast() {
        let scm = sprinkler_scm();
        let posterior_rain = scm.posterior(2, &[(3, 1)]).unwrap();
        let prior_rain = scm.posterior(2, &[]).unwrap();
        assert!(posterior_rain[1] > prior_rain[1], "observing wet grass should raise P(rain)");

        let interventional_rain = scm.interventional_distribution(2, &[(1, 1)], &[]).unwrap();
        assert!((interventional_rain[1] - prior_rain[1]).abs() < 1e-9, "do(sprinkler=1) should not change the marginal of rain: intervening breaks the backdoor path through cloudy");

        let conditional_on_sprinkler = scm.posterior(2, &[(1, 1)]).unwrap();
        assert!((conditional_on_sprinkler[1] - prior_rain[1]).abs() > 1e-9, "merely conditioning on sprinkler=1 (not intervening) should shift P(rain) via the cloudy backdoor path");
    }

    // #endregion 🔖️ScmDiscreteTests

    // #region 🔖️EstimationTests
    fn confounded_dataset(true_ate: f64, n: usize, seed: u64) -> mathematical_tabular::Table {
        let mut rng = mathematical_random::Rng::from_seed(seed);
        let z: Vec<f64> = (0..n).map(|_| mathematical_probability::Normal::STANDARD.sample(&mut rng)).collect();
        let t: Vec<f64> = z.iter().map(|&zi| f64::from(u8::from(zi + mathematical_probability::Normal::STANDARD.sample(&mut rng) > 0.0))).collect();
        let y: Vec<f64> = z.iter().zip(&t).map(|(&zi, &ti)| 2.0 * zi + true_ate * ti + mathematical_probability::Normal::STANDARD.sample(&mut rng) * 0.5).collect();
        mathematical_tabular::Table::from_f64_columns(vec!["z".into(), "t".into(), "y".into()], vec![z, t, y]).unwrap()
    }

    // #endregion 🔖️EstimationTests

    // #region 🔖️ErrorPathTests
    #[test]
    fn cyclic_edges_are_rejected() {
        let err = CausalDag::new(vec!["a".into(), "b".into()], &[(0, 1), (1, 0)]).unwrap_err();
        assert!(matches!(err, CausalError::NotADag(_)));
    }

    #[test]
    fn wrong_column_type_errors() {
        let mut table = mathematical_tabular::Table::new();
        table.push_continuous("x", vec![1.0, 2.0]).unwrap();
        let err = GSquared.test(&table, 0, 0, &[]).unwrap_err();
        assert!(matches!(err, CausalError::Tabular(_)));
    }

    #[test]
    fn oversized_variable_elimination_is_rejected() {
        // 20 independent binary roots all feeding one "sink" child: the sink's own CPT factor
        // already has 2^21 entries (21 binary variables), well past the 1e6-entry guard, and
        // eliminating any root multiplies straight into that factor.
        let n_parents = 20;
        let mut names = vec!["sink".to_string()];
        names.extend((0..n_parents).map(|i| format!("p{i}")));
        let edges: Vec<(usize, usize)> = (0..n_parents).map(|i| (i + 1, 0usize)).collect();
        let dag = CausalDag::new(names, &edges).unwrap();
        let cardinalities = vec![2usize; n_parents + 1];
        let mut cpts = vec![Cpt { node: 0, parents: (1..=n_parents).collect(), cardinality: 2, probs: vec![0.5; 2usize.pow(n_parents as u32) * 2] }];
        for i in 0..n_parents {
            cpts.push(Cpt { node: i + 1, parents: vec![], cardinality: 2, probs: vec![0.5, 0.5] });
        }
        let scm = DiscreteScm { dag, cardinalities, cpts };
        let result = scm.posterior(1, &[]);
        assert!(matches!(result, Err(CausalError::InferenceTooLarge(_, _))), "posterior over a 2^21-entry join should exceed the guard");
    }
    // #endregion 🔖️ErrorPathTests

    // #region 🔖️QuickTests
    // 🐢️ Tests that simulate/fit at a scale (thousands of rows, hundreds of bootstrap replicates)
    // needed for statistical power, too slow for the 15s "fundamental" budget — see
    // `repo/lib/js/index.ts`'s `runCargoTestBudgeted`/`TEST_LEVEL_BUDGET_MS`.
    mod quick {
        use super::*;

        #[test]
        fn pc_stable_recovers_known_cpdag_from_simulated_chain() {
            let scm = linear_chain_scm();
            let mut rng = mathematical_random::Rng::from_seed(2024);
            let data = scm.simulate(2000, &mut rng).unwrap();
            let ci = FisherZ::for_table(&data, &[0, 1, 2]).unwrap();
            let result = pc_stable(&data, &ci, PcOptions { alpha: 0.01, max_cond_size: 2 }).unwrap();
            let truth = Cpdag::from_dag(&scm.dag);
            assert_eq!(result.cpdag.directed_edges(), truth.directed_edges());
            assert_eq!(result.cpdag.undirected_edges(), truth.undirected_edges());
        }

        #[test]
        fn ges_recovers_known_cpdag_from_simulated_chain() {
            let scm = linear_chain_scm();
            let mut rng = mathematical_random::Rng::from_seed(4242);
            let data = scm.simulate(2000, &mut rng).unwrap();
            let found = ges(&data).unwrap();
            let truth = Cpdag::from_dag(&scm.dag);
            assert_eq!(found.directed_edges(), truth.directed_edges());
            assert_eq!(found.undirected_edges(), truth.undirected_edges());
        }

        #[test]
        fn direct_lingam_recovers_causal_order_on_uniform_noise_sem() {
            // x -> y with uniform (non-Gaussian) noise.
            let n = 3000;
            let mut rng = mathematical_random::Rng::from_seed(55);
            let x: Vec<f64> = (0..n).map(|_| mathematical_probability::Uniform::new(-1.0, 1.0).unwrap().sample(&mut rng)).collect();
            let y: Vec<f64> = x.iter().map(|&xi| 2.0 * xi + mathematical_probability::Uniform::new(-1.0, 1.0).unwrap().sample(&mut rng)).collect();
            let table = mathematical_tabular::Table::from_f64_columns(vec!["x".into(), "y".into()], vec![x, y]).unwrap();
            let result = direct_lingam(&table, 0.05).unwrap();
            assert_eq!(result.order[0], 0, "x should be recovered as the earlier (more exogenous) variable");
            assert!(result.dag.parents(1).contains(&0), "y should have x as a parent after pruning");
        }

        #[test]
        fn linear_scm_simulate_then_fit_recovers_coefficients() {
            let scm = linear_chain_scm();
            let mut rng = mathematical_random::Rng::from_seed(321);
            let data = scm.simulate(5000, &mut rng).unwrap();
            let refit = LinearGaussianScm::fit(&scm.dag, &data).unwrap();
            assert!((refit.weights.get(1, 0) - 2.0).abs() < 0.1);
            assert!((refit.weights.get(2, 1) - 1.5).abs() < 0.1);
        }

        #[test]
        fn discrete_scm_fit_recovers_cpts_from_simulated_data() {
            let scm = sprinkler_scm();
            let mut rng = mathematical_random::Rng::from_seed(17);
            let data = scm.simulate(20_000, &mut rng).unwrap();
            let refit = DiscreteScm::fit(&scm.dag, &data, 1.0).unwrap();
            let cloudy_fit = &refit.cpts[0];
            assert!((cloudy_fit.probs[1] - 0.5).abs() < 0.05, "recovered P(cloudy=1) should be near 0.5, got {}", cloudy_fit.probs[1]);
        }

        #[test]
        fn naive_difference_is_biased_while_adjusted_estimators_recover_true_ate() {
            let true_ate = 1.5;
            let data = confounded_dataset(true_ate, 4000, 42);
            let opts = EstimationOptions::default();
            let naive = naive_difference(&data, 1, 2, &opts).unwrap();
            let g_formula = g_formula_ate(&data, 1, 2, &[0], &opts).unwrap();
            let ipw = ipw_ate(&data, 1, 2, &[0], &opts).unwrap();
            assert!((naive.estimate - true_ate).abs() > 0.3, "naive estimate {} should be visibly biased away from {true_ate}", naive.estimate);
            assert!((g_formula.estimate - true_ate).abs() < 0.2, "g-formula estimate {} should be close to {true_ate}", g_formula.estimate);
            // IPW carries more finite-sample variance than g-formula (propensity-weight reweighting
            // amplifies noise), so it gets a looser tolerance for the same sample size.
            assert!((ipw.estimate - true_ate).abs() < 0.35, "IPW estimate {} should be close to {true_ate}", ipw.estimate);
        }

        #[test]
        fn bootstrap_ci_contains_true_ate() {
            let true_ate = 1.5;
            let data = confounded_dataset(true_ate, 5000, 7);
            let opts = EstimationOptions { bootstrap: Some(BootstrapOptions { replicates: 300, seed: 11, level: 0.95 }) };
            let g_formula = g_formula_ate(&data, 1, 2, &[0], &opts).unwrap();
            let (lo, hi) = g_formula.ci.expect("bootstrap CI requested");
            assert!(lo <= g_formula.estimate && g_formula.estimate <= hi);
            assert!(lo <= true_ate && true_ate <= hi, "95% CI [{lo}, {hi}] should contain the true ATE {true_ate}");
        }
    }
    // #endregion 🔖️QuickTests
}
// #endregion 🔖️UnitTests
