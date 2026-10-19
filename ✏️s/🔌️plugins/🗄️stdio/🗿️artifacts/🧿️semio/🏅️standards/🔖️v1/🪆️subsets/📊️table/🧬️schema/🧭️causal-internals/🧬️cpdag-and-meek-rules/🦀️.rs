//! 🧬️ CPDAG model: the completed partially directed graph, its orientation queries and the Meek orientation rules. See [`super`].

use super::*;

// #region 🔖️Cpdag
/// 🔀️ A completed partially directed acyclic graph (CPDAG): directed causal edges plus undirected
/// (Markov-equivalence-unresolved) edges over the same named variables as a [`CausalDag`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cpdag {
    pub(super) names: Vec<String>,
    pub(super) directed: BTreeSet<(usize, usize)>,
    pub(super) undirected: BTreeSet<(usize, usize)>,
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_meek_rules(cpdag: &mut Cpdag) {
    let n = cpdag.n();
    loop {
        let mut changed = false;

        for (a, b) in cpdag.directed_edges() {
            for c in cpdag.adjacent(b) {
                if c != a && cpdag.is_undirected(b, c) && !cpdag.has_edge(a, c) {
                    cpdag.orient(b, c);
                    changed = true;
                }
            }
        }

        for (a, b) in cpdag.directed_edges() {
            for c in cpdag.adjacent(b) {
                if cpdag.is_directed(b, c) && cpdag.is_undirected(a, c) {
                    cpdag.orient(a, c);
                    changed = true;
                }
            }
        }

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
