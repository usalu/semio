//! 🧮️ Index-based graph algorithms: traversal, ordering, cycles, components, shortest paths.

use std::collections::HashMap;

// #region 🔖️Adjacency
/// 🧮️ Compact adjacency built once per query batch.
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`, so no `#[value(...)]` rename is needed: field names are the wire shape.
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
pub struct Adjacency {
    n: usize,
    out: Vec<Vec<usize>>,
    inc: Vec<Vec<usize>>,
}

impl Adjacency {
    pub fn node_count(&self) -> usize {
        self.n
    }
    pub fn out_neighbors(&self, i: usize) -> &[usize] {
        &self.out[i]
    }
    pub fn in_neighbors(&self, i: usize) -> &[usize] {
        &self.inc[i]
    }
}

/// 🧮️ Builds adjacency lists from index edges; `directed` controls whether reverse edges are also recorded as out-edges.
pub fn adjacency(node_count: usize, edges: &[(usize, usize)], directed: bool) -> Adjacency {
    let mut out = vec![Vec::new(); node_count];
    let mut inc = vec![Vec::new(); node_count];
    for &(a, b) in edges {
        if a >= node_count || b >= node_count {
            continue;
        }
        out[a].push(b);
        inc[b].push(a);
        if !directed {
            out[b].push(a);
            inc[a].push(b);
        }
    }
    Adjacency { n: node_count, out, inc }
}
// #endregion 🔖️Adjacency

// #region 🔖️IdIndex
/// 🔤️ Deterministic string-id <-> index bridge (ids sorted for reproducible ordering).
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`, so no `#[value(...)]` rename is needed: field names are the wire shape.
#[derive(Clone, Debug, Default, value_derive::ToValue, value_derive::FromValue)]
pub struct IdIndex {
    ids: Vec<String>,
    index: HashMap<String, usize>,
}

impl IdIndex {
    pub fn from_ids<'a>(ids: impl Iterator<Item = &'a str>) -> Self {
        let mut sorted: Vec<String> = ids.map(|s| s.to_string()).collect();
        sorted.sort();
        sorted.dedup();
        let index = sorted.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
        Self { ids: sorted, index }
    }

    pub fn from_edges<'a>(edges: impl Iterator<Item = (&'a str, &'a str)>) -> Self {
        let mut all: Vec<String> = Vec::new();
        for (a, b) in edges {
            all.push(a.to_string());
            all.push(b.to_string());
        }
        Self::from_ids(all.iter().map(|s| s.as_str()))
    }

    pub fn index_of(&self, id: &str) -> Option<usize> {
        self.index.get(id).copied()
    }

    pub fn id_of(&self, index: usize) -> Option<&str> {
        self.ids.get(index).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    fn edges_to_indices(&self, edges: &[(String, String)]) -> Vec<(usize, usize)> {
        // 🔀️ Rewritten from `.filter_map(..)` — the closure was sync and could not `.await` the
        // per-id `index_of` lookups (R10 residue shape #1).
        let mut out = Vec::with_capacity(edges.len());
        for (a, b) in edges {
            if let (Some(ia), Some(ib)) = (self.index_of(a), self.index_of(b)) {
                out.push((ia, ib));
            }
        }
        out
    }
}
// #endregion 🔖️IdIndex

// #region 🔖️Traversal
/// 🌊️ Breadth-first visitation order from the given seeds.
fn bfs_order(adj: &Adjacency, seeds: &[usize]) -> Vec<usize> {
    let mut visited = vec![false; adj.n];
    let mut order = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    for &s in seeds {
        if s < adj.n && !visited[s] {
            visited[s] = true;
            queue.push_back(s);
        }
    }
    while let Some(u) = queue.pop_front() {
        order.push(u);
        for &v in &adj.out[u] {
            if !visited[v] {
                visited[v] = true;
                queue.push_back(v);
            }
        }
    }
    order
}

/// 📏️ Unweighted BFS distance from a single seed to every reachable node.
pub fn bfs_distances(adj: &Adjacency, seed: usize) -> Vec<Option<u32>> {
    let mut dist = vec![None; adj.n];
    if seed >= adj.n {
        return dist;
    }
    dist[seed] = Some(0);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(seed);
    while let Some(u) = queue.pop_front() {
        let du = dist[u].expect("every queued node was assigned a distance before being pushed");
        for &v in &adj.out[u] {
            if dist[v].is_none() {
                dist[v] = Some(du + 1);
                queue.push_back(v);
            }
        }
    }
    dist
}

// #endregion 🔖️Traversal

// #region 🔖️Ordering
/// ⚠️ A cycle was found where a DAG was required; `cycle` lists the node indices on the cycle.
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`, so no `#[value(...)]` rename is needed: field names are the wire shape.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub struct CycleError {
    pub cycle: Vec<usize>,
}

/// 🔢️ Kahn's algorithm topological sort; index-ascending tie-break for determinism.
pub fn topo_sort(adj: &Adjacency) -> Result<Vec<usize>, CycleError> {
    let mut in_deg = vec![0usize; adj.n];
    for list in &adj.out {
        for &v in list {
            in_deg[v] += 1;
        }
    }
    let mut heap = std::collections::BinaryHeap::new();
    for (i, &deg) in in_deg.iter().enumerate() {
        if deg == 0 {
            heap.push(std::cmp::Reverse(i));
        }
    }
    let mut order = Vec::with_capacity(adj.n);
    while let Some(std::cmp::Reverse(u)) = heap.pop() {
        order.push(u);
        for &v in &adj.out[u] {
            in_deg[v] -= 1;
            if in_deg[v] == 0 {
                heap.push(std::cmp::Reverse(v));
            }
        }
    }
    if order.len() == adj.n {
        Ok(order)
    } else {
        let remaining: Vec<usize> = (0..adj.n).filter(|&i| in_deg[i] > 0).collect();
        Err(CycleError { cycle: find_cycle_among(adj, &remaining).unwrap_or(remaining) })
    }
}

/// 🪜️ Topological levels: each level contains nodes whose dependencies are all in earlier levels.
pub fn topo_levels(adj: &Adjacency) -> Result<Vec<Vec<usize>>, CycleError> {
    let mut in_deg = vec![0usize; adj.n];
    for list in &adj.out {
        for &v in list {
            in_deg[v] += 1;
        }
    }
    let mut levels = Vec::new();
    let mut remaining = in_deg.clone();
    let mut placed = vec![false; adj.n];
    let mut placed_count = 0;
    loop {
        let mut frontier: Vec<usize> = (0..adj.n).filter(|&i| !placed[i] && remaining[i] == 0).collect();
        if frontier.is_empty() {
            break;
        }
        frontier.sort_unstable();
        for &u in &frontier {
            placed[u] = true;
            placed_count += 1;
        }
        for &u in &frontier {
            for &v in &adj.out[u] {
                remaining[v] -= 1;
            }
        }
        levels.push(frontier);
    }
    if placed_count == adj.n {
        Ok(levels)
    } else {
        let unplaced: Vec<usize> = (0..adj.n).filter(|&i| !placed[i]).collect();
        Err(CycleError { cycle: find_cycle_among(adj, &unplaced).unwrap_or(unplaced) })
    }
}

// #endregion 🔖️Ordering

// #region 🔖️Cycles
/// 🔎️ Whether `to` is reachable from `from` following out-edges.
fn is_reachable(adj: &Adjacency, from: usize, to: usize) -> bool {
    if from == to {
        return true;
    }
    bfs_order(adj, &[from]).contains(&to)
}

/// ➕️ Whether adding an edge `source -> target` would create a cycle (i.e. `target` can already reach `source`).
fn would_create_cycle(adj: &Adjacency, source: usize, target: usize) -> bool {
    source == target || is_reachable(adj, target, source)
}

/// ➕️ String-id convenience: whether adding `source -> target` to `existing` directed edges would create a cycle.
pub fn would_create_cycle_ids(existing: &[(String, String)], source: &str, target: &str) -> bool {
    if source == target {
        return true;
    }
    let index = IdIndex::from_edges(existing.iter().map(|(a, b)| (a.as_str(), b.as_str())));
    let (Some(s), Some(t)) = (index.index_of(source), index.index_of(target)) else {
        return false;
    };
    let edges_idx = index.edges_to_indices(existing);
    let adj = adjacency(index.len(), &edges_idx, true);
    would_create_cycle(&adj, s, t)
}

fn find_cycle_among(adj: &Adjacency, candidates: &[usize]) -> Option<Vec<usize>> {
    let mut color = vec![0u8; adj.n];
    let mut path = Vec::new();
    fn dfs(u: usize, adj: &Adjacency, color: &mut [u8], path: &mut Vec<usize>) -> Option<Vec<usize>> {
        color[u] = 1;
        path.push(u);
        for &v in &adj.out[u] {
            if color[v] == 1 {
                let start = path.iter().position(|&x| x == v).expect("color[v] == 1 means v is currently on the open dfs path");
                return Some(path[start..].to_vec());
            }
            if color[v] == 0 {
                if let Some(cycle) = dfs(v, adj, color, path) {
                    return Some(cycle);
                }
            }
        }
        path.pop();
        color[u] = 2;
        None
    }
    for &start in candidates {
        if color[start] == 0 {
            if let Some(cycle) = dfs(start, adj, &mut color, &mut path) {
                return Some(cycle);
            }
        }
    }
    None
}

// #endregion 🔖️Cycles

// #region 🔖️Components
/// 🧮️ Union-find (disjoint-set) with path compression and union-by-rank.
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`, so no `#[value(...)]` rename is needed: field names are the wire shape.
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), rank: vec![0; n] }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
    }

    pub fn same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

/// 🧩️ Weak connected-component id per node (undirected reachability, works for directed adjacency too).
pub fn connected_components(adj: &Adjacency) -> Vec<usize> {
    let mut uf = UnionFind::new(adj.n);
    for u in 0..adj.n {
        for &v in &adj.out[u] {
            uf.union(u, v);
        }
    }
    let mut root_to_component: HashMap<usize, usize> = HashMap::new();
    let mut labels = vec![0usize; adj.n];
    for (u, label) in labels.iter_mut().enumerate() {
        let root = uf.find(u);
        let next_id = root_to_component.len();
        let id = *root_to_component.entry(root).or_insert(next_id);
        *label = id;
    }
    labels
}

/// 🧩️ Tarjan's strongly connected components; returned in reverse-topological order, nodes sorted within each.
pub fn strongly_connected_components(adj: &Adjacency) -> Vec<Vec<usize>> {
    struct State {
        index: Vec<Option<u32>>,
        lowlink: Vec<u32>,
        on_stack: Vec<bool>,
        stack: Vec<usize>,
        counter: u32,
        out: Vec<Vec<usize>>,
    }
    fn strongconnect(u: usize, adj: &Adjacency, st: &mut State) {
        st.index[u] = Some(st.counter);
        st.lowlink[u] = st.counter;
        st.counter += 1;
        st.stack.push(u);
        st.on_stack[u] = true;
        for &v in &adj.out[u] {
            if st.index[v].is_none() {
                strongconnect(v, adj, st);
                st.lowlink[u] = st.lowlink[u].min(st.lowlink[v]);
            } else if st.on_stack[v] {
                st.lowlink[u] = st.lowlink[u].min(st.index[v].expect("on_stack[v] implies index[v] was assigned when v was first visited"));
            }
        }
        if st.lowlink[u] == st.index[u].expect("index[u] was assigned at the start of this strongconnect call") {
            let mut component = Vec::new();
            loop {
                let w = st.stack.pop().expect("u is still on the tarjan stack until its own component is popped");
                st.on_stack[w] = false;
                component.push(w);
                if w == u {
                    break;
                }
            }
            component.sort_unstable();
            st.out.push(component);
        }
    }
    let mut st = State { index: vec![None; adj.n], lowlink: vec![0; adj.n], on_stack: vec![false; adj.n], stack: Vec::new(), counter: 0, out: Vec::new() };
    for u in 0..adj.n {
        if st.index[u].is_none() {
            strongconnect(u, adj, &mut st);
        }
    }
    st.out
}

// #endregion 🔖️Components

// #region 🔖️Paths
/// 📏️ Dijkstra shortest distances from `from` to every node, given non-negative edge weights parallel to adjacency out-edges.
pub fn dijkstra(adj: &Adjacency, weights: &HashMap<(usize, usize), f64>, from: usize) -> Vec<Option<f64>> {
    let mut dist = vec![None; adj.n];
    if from >= adj.n {
        return dist;
    }
    dist[from] = Some(0.0);
    let mut heap = std::collections::BinaryHeap::new();
    heap.push(std::cmp::Reverse(OrderedFloat(0.0, from)));
    while let Some(std::cmp::Reverse(OrderedFloat(d, u))) = heap.pop() {
        if dist[u].is_none_or(|cur| d > cur) {
            continue;
        }
        for &v in &adj.out[u] {
            let w = weights.get(&(u, v)).copied().unwrap_or(1.0);
            let nd = d + w;
            if dist[v].is_none_or(|cur| nd < cur) {
                dist[v] = Some(nd);
                heap.push(std::cmp::Reverse(OrderedFloat(nd, v)));
            }
        }
    }
    dist
}

// 🧬️ ToValue/FromValue coverage deliberately SKIPPED (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01): private (module-scoped, non-`pub`), a Dijkstra priority-queue sort key never crossing
// any wire/DSL boundary, and a 2-field tuple struct — `#[value(transparent)]` only covers a
// single-field newtype, and a multi-field tuple struct's payload shape has no `#[value(...)]`
// equivalent at all (`semio-framework-value-derive` docs: "tuple variants with more than one
// unnamed field" are deliberately unsupported, same rule for tuple structs).
#[derive(Clone, Copy, Debug, PartialEq)]
struct OrderedFloat(f64, usize);
impl Eq for OrderedFloat {}
impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal).then(self.1.cmp(&other.1))
    }
}

// #endregion 🔖️Paths

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
    // an `async fn` directly (std has no executor for it), so every async test body in this
    // module runs through this instead. Sound because this crate performs no real I/O: every
    // future here resolves on its first poll, so a single poll (never a spin-park loop) is
    // enough — panics loudly if that invariant is ever violated rather than hanging.
    fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        fn noop(_: *const ()) {}
        fn clone_raw(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
        let raw = RawWaker::new(std::ptr::null(), &VTABLE);
        let waker = unsafe { Waker::from_raw(raw) };
        let mut cx = Context::from_waker(&waker);
        let mut fut = Box::pin(fut);
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => v,
            Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
        }
    }

    fn adj_from(n: usize, edges: &[(usize, usize)], directed: bool) -> Adjacency {
        adjacency(n, edges, directed)
    }

    #[test]
    fn bfs_order_visits_reachable_nodes_breadth_first() {
        block_on_test(async {
            let adj = adj_from(5, &[(0, 1), (0, 2), (1, 3), (2, 4)], true);
            let order = bfs_order(&adj, &[0]);
            assert_eq!(order, vec![0, 1, 2, 3, 4]);
        });
    }

    #[test]
    fn bfs_distances_unreachable_is_none() {
        block_on_test(async {
            let adj = adj_from(3, &[(0, 1)], true);
            let dist = bfs_distances(&adj, 0);
            assert_eq!(dist, vec![Some(0), Some(1), None]);
        });
    }

    #[test]
    fn topo_sort_orders_dependencies_before_dependents() {
        block_on_test(async {
            let adj = adj_from(4, &[(0, 1), (0, 2), (1, 3), (2, 3)], true);
            let order = topo_sort(&adj).expect("acyclic");
            let pos = |n: usize| order.iter().position(|&x| x == n).unwrap();
            assert!(pos(0) < pos(1));
            assert!(pos(1) < pos(3));
            assert!(pos(2) < pos(3));
        });
    }

    #[test]
    fn topo_sort_detects_cycle() {
        block_on_test(async {
            let adj = adj_from(3, &[(0, 1), (1, 2), (2, 0)], true);
            let err = topo_sort(&adj).unwrap_err();
            assert_eq!(err.cycle.len(), 3);
        });
    }

    #[test]
    fn topo_levels_groups_independent_nodes() {
        block_on_test(async {
            let adj = adj_from(4, &[(0, 2), (1, 2), (2, 3)], true);
            let levels = topo_levels(&adj).expect("acyclic");
            assert_eq!(levels[0], vec![0, 1]);
            assert_eq!(levels[1], vec![2]);
            assert_eq!(levels[2], vec![3]);
        });
    }

    #[test]
    fn would_create_cycle_detects_back_edge() {
        block_on_test(async {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            assert!(would_create_cycle(&adj, 2, 0));
            assert!(!would_create_cycle(&adj, 0, 2));
        });
    }

    #[test]
    fn would_create_cycle_ids_matches_index_version() {
        block_on_test(async {
            let existing = vec![("a".to_string(), "b".to_string()), ("b".to_string(), "c".to_string())];
            assert!(would_create_cycle_ids(&existing, "c", "a"));
            assert!(!would_create_cycle_ids(&existing, "a", "c"));
        });
    }

    #[test]
    fn connected_components_groups_weak_components() {
        block_on_test(async {
            let adj = adj_from(5, &[(0, 1), (1, 2), (3, 4)], true);
            let labels = connected_components(&adj);
            assert_eq!(labels[0], labels[1]);
            assert_eq!(labels[1], labels[2]);
            assert_eq!(labels[3], labels[4]);
            assert_ne!(labels[0], labels[3]);
        });
    }

    #[test]
    fn strongly_connected_components_finds_cycle_as_one_component() {
        block_on_test(async {
            let adj = adj_from(4, &[(0, 1), (1, 2), (2, 0), (2, 3)], true);
            let sccs = strongly_connected_components(&adj);
            let cyclic = sccs.iter().find(|c| c.contains(&0)).unwrap();
            assert_eq!(cyclic, &vec![0, 1, 2]);
            assert!(sccs.iter().any(|c| c == &vec![3]));
        });
    }

    #[test]
    fn union_find_unions_and_queries_sets() {
        block_on_test(async {
            let mut uf = UnionFind::new(4);
            uf.union(0, 1);
            uf.union(2, 3);
            assert!(uf.same_set(0, 1));
            assert!(!uf.same_set(0, 2));
        });
    }

    #[test]
    fn dijkstra_prefers_cheaper_longer_path() {
        block_on_test(async {
            let adj = adj_from(3, &[(0, 1), (1, 2), (0, 2)], true);
            let mut weights = HashMap::new();
            weights.insert((0, 1), 1.0);
            weights.insert((1, 2), 1.0);
            weights.insert((0, 2), 5.0);
            let dist = dijkstra(&adj, &weights, 0);
            assert_eq!(dist[2], Some(2.0));
        });
    }

    #[test]
    fn id_index_is_deterministic_and_sorted() {
        block_on_test(async {
            let edges = [("c".to_string(), "a".to_string()), ("a".to_string(), "b".to_string())];
            let index = IdIndex::from_edges(edges.iter().map(|(a, b)| (a.as_str(), b.as_str())));
            assert_eq!(index.id_of(0), Some("a"));
            assert_eq!(index.id_of(1), Some("b"));
            assert_eq!(index.id_of(2), Some("c"));
        });
    }

    #[test]
    fn id_index_from_ids_dedupes_and_reports_len() {
        block_on_test(async {
            let index = IdIndex::from_ids(["b", "a", "a"].into_iter());
            assert_eq!(index.len(), 2);
            assert!(!index.is_empty());
            assert_eq!(index.index_of("a"), Some(0));
            assert_eq!(index.index_of("z"), None);
            assert!(IdIndex::from_ids(std::iter::empty()).is_empty());
        });
    }

    #[test]
    fn adjacency_accessors_expose_node_count_and_neighbor_lists() {
        block_on_test(async {
            let adj = adj_from(3, &[(0, 1), (0, 2)], true);
            assert_eq!(adj.node_count(), 3);
            assert_eq!(adj.out_neighbors(0), &[1, 2]);
            assert_eq!(adj.in_neighbors(1), &[0]);
            assert!(adj.in_neighbors(0).is_empty());
        });
    }

    #[test]
    fn bfs_order_ignores_out_of_range_seeds() {
        block_on_test(async {
            let adj = adj_from(2, &[(0, 1)], true);
            assert_eq!(bfs_order(&adj, &[5]), Vec::<usize>::new());
        });
    }

    #[test]
    fn topo_levels_detects_cycle() {
        block_on_test(async {
            let adj = adj_from(3, &[(0, 1), (1, 2), (2, 0)], true);
            let err = topo_levels(&adj).unwrap_err();
            assert_eq!(err.cycle.len(), 3);
        });
    }

    #[test]
    fn would_create_cycle_self_loop_is_always_a_cycle() {
        block_on_test(async {
            let adj = adj_from(2, &[(0, 1)], true);
            assert!(would_create_cycle(&adj, 1, 1));
        });
    }

    #[test]
    fn is_reachable_from_equals_to_is_trivially_true() {
        block_on_test(async {
            let adj = adj_from(2, &[], true);
            assert!(is_reachable(&adj, 1, 1));
        });
    }

    #[test]
    fn dijkstra_unreachable_node_stays_none() {
        block_on_test(async {
            let adj = adj_from(3, &[(0, 1)], true);
            let dist = dijkstra(&adj, &HashMap::new(), 0);
            assert_eq!(dist, vec![Some(0.0), Some(1.0), None]);
        });
    }

    #[test]
    fn dijkstra_out_of_range_from_returns_empty() {
        block_on_test(async {
            let adj = adj_from(2, &[(0, 1)], true);
            assert_eq!(dijkstra(&adj, &HashMap::new(), 9), vec![None, None]);
        });
    }
}
// #endregion 🔖️Tests
