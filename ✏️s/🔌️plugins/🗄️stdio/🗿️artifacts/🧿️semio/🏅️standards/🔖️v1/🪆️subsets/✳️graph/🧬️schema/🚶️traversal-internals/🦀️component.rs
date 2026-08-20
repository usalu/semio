//! 🌊️ Breadth-first and depth-first traversal families over the generic `GraphView` traits.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use graph_core::{EdgeId, EdgeRef, GraphView, NodeId};

// #region 🔖️Shared
/// 🔗️ Picks the first (lowest `EdgeId`) edge connecting `u` to `v`; safe to `expect` because every caller only invokes this for a pair already reported by `out_neighbors`/`neighbors`, which guarantees at least one connecting edge exists.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn any_edge<G: GraphView>(graph: &G, u: NodeId, v: NodeId) -> EdgeRef {
    graph.edges_between(u, v).next().expect("out_neighbors/neighbors only report pairs with a connecting edge")
}

/// 🔌️ Every edge incident to `node`: out-edges only for directed views (matching successor-direction traversal), all touching edges for undirected views. Parallel edges (multigraphs) each appear once per their `EdgeId`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn incident_edges<G: GraphView>(graph: &G, node: NodeId) -> Vec<EdgeRef> {
    if graph.is_directed() {
        graph.out_neighbors(node).flat_map(|nbr| graph.edges_between(node, nbr)).collect()
    } else {
        graph.neighbors(node).flat_map(|nbr| graph.edges_between(node, nbr)).collect()
    }
}
// #endregion 🔖️Shared

// #region 🔖️Bfs
/// 🧭️ Generic breadth-first edge traversal driven by a caller-supplied neighbor-ordering hook (NetworkX `generic_bfs_edges`); every other BFS function in this crate is built on top of this one.
pub async fn generic_bfs_edges<G: GraphView, F: Fn(&G, NodeId) -> Vec<NodeId>>(graph: &G, source: NodeId, neighbor_fn: F) -> Vec<EdgeRef> {
    let mut result = Vec::new();
    if !graph.contains_node(source) {
        return result;
    }
    let mut visited: BTreeSet<NodeId> = BTreeSet::new();
    visited.insert(source);
    let mut queue: VecDeque<NodeId> = VecDeque::new();
    queue.push_back(source);
    while let Some(node) = queue.pop_front() {
        for nbr in neighbor_fn(graph, node) {
            if visited.insert(nbr) {
                result.push(any_edge(graph, node, nbr));
                queue.push_back(nbr);
            }
        }
    }
    result
}

/// 🔁️ Breadth-first tree edges from `source`, in discovery order (NetworkX `bfs_edges`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bfs_edges<G: GraphView>(graph: &G, source: NodeId) -> Vec<EdgeRef> {
    generic_bfs_edges(graph, source, |g: &G, n: NodeId| -> Vec<NodeId> { g.out_neighbors(n).collect() })
}

/// 🌳️ Breadth-first spanning tree edges from `source` (NetworkX `bfs_tree`); NetworkX returns a tree graph, this crate returns the same edge list as `bfs_edges` — callers build a graph from it if a materialized tree is needed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bfs_tree<G: GraphView>(graph: &G, source: NodeId) -> Vec<EdgeRef> {
    bfs_edges(graph, source)
}

/// 🧱️ Multi-source breadth-first layering: `layers[0]` is the (deduplicated, existing) `sources`, `layers[k]` is every node first reached at distance `k` (NetworkX `bfs_layers`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bfs_layers<G: GraphView>(graph: &G, sources: &[NodeId]) -> Vec<Vec<NodeId>> {
    let mut layers = Vec::new();
    let mut visited: BTreeSet<NodeId> = BTreeSet::new();
    let mut frontier: BTreeSet<NodeId> = sources.iter().copied().filter(|&n| graph.contains_node(n)).collect();
    if frontier.is_empty() {
        return layers;
    }
    visited.extend(frontier.iter().copied());
    layers.push(frontier.iter().copied().collect());
    loop {
        let mut next: BTreeSet<NodeId> = BTreeSet::new();
        for &node in &frontier {
            for nbr in graph.out_neighbors(node) {
                if visited.insert(nbr) {
                    next.insert(nbr);
                }
            }
        }
        if next.is_empty() {
            break;
        }
        layers.push(next.iter().copied().collect());
        frontier = next;
    }
    layers
}

/// ⬅️ Maps every non-source node reached from `source` to its breadth-first parent (NetworkX `bfs_predecessors`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bfs_predecessors<G: GraphView>(graph: &G, source: NodeId) -> BTreeMap<NodeId, NodeId> {
    let mut preds = BTreeMap::new();
    for edge in bfs_edges(graph, source) {
        preds.insert(edge.v, edge.u);
    }
    preds
}

/// ➡️ Maps every node with breadth-first tree children to the list of those children, in discovery order (NetworkX `bfs_successors`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bfs_successors<G: GraphView>(graph: &G, source: NodeId) -> BTreeMap<NodeId, Vec<NodeId>> {
    let mut succs: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();
    for edge in bfs_edges(graph, source) {
        succs.entry(edge.u).or_default().push(edge.v);
    }
    succs
}

/// 📏️ Every node whose shortest-path distance from `source` is exactly `distance` (NetworkX `descendants_at_distance`); `distance == 0` yields `{source}`, and an unreachable `distance` yields the empty set.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn descendants_at_distance<G: GraphView>(graph: &G, source: NodeId, distance: usize) -> BTreeSet<NodeId> {
    let mut current: BTreeSet<NodeId> = BTreeSet::new();
    if !graph.contains_node(source) {
        return current;
    }
    current.insert(source);
    let mut visited = current.clone();
    for _ in 0..distance {
        let mut next = BTreeSet::new();
        for &node in &current {
            for nbr in graph.out_neighbors(node) {
                if visited.insert(nbr) {
                    next.insert(nbr);
                }
            }
        }
        current = next;
        if current.is_empty() {
            break;
        }
    }
    current
}
// #endregion 🔖️Bfs

// #region 🔖️Dfs
/// 🪆️ Depth-first tree edges from `source`, iterative (explicit stack, no recursion) so traversal depth is bounded only by heap memory (NetworkX `dfs_edges`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dfs_edges<G: GraphView>(graph: &G, source: NodeId) -> Vec<EdgeRef> {
    let mut result = Vec::new();
    if !graph.contains_node(source) {
        return result;
    }
    let mut visited: BTreeSet<NodeId> = BTreeSet::new();
    visited.insert(source);
    let mut stack: Vec<(NodeId, Vec<NodeId>, usize)> = vec![(source, graph.out_neighbors(source).collect(), 0)];
    while !stack.is_empty() {
        let i = stack.len() - 1;
        let node = stack[i].0;
        if stack[i].2 < stack[i].1.len() {
            let nbr = stack[i].1[stack[i].2];
            stack[i].2 += 1;
            if visited.insert(nbr) {
                result.push(any_edge(graph, node, nbr));
                stack.push((nbr, graph.out_neighbors(nbr).collect(), 0));
            }
        } else {
            stack.pop();
        }
    }
    result
}

/// 🌲️ Depth-first spanning tree edges from `source` (NetworkX `dfs_tree`); same simplification as `bfs_tree` — returns the edge list, not a materialized tree graph.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dfs_tree<G: GraphView>(graph: &G, source: NodeId) -> Vec<EdgeRef> {
    dfs_edges(graph, source)
}

/// 🔼️ Nodes in depth-first preorder (parent emitted before its subtree) starting at `source` (NetworkX `dfs_preorder_nodes`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dfs_preorder_nodes<G: GraphView>(graph: &G, source: NodeId) -> Vec<NodeId> {
    if !graph.contains_node(source) {
        return Vec::new();
    }
    let mut nodes = vec![source];
    nodes.extend(dfs_edges(graph, source).into_iter().map(|e| e.v));
    nodes
}

/// 🔽️ Nodes in depth-first postorder (a node emitted only after its whole subtree finishes) starting at `source` (NetworkX `dfs_postorder_nodes`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dfs_postorder_nodes<G: GraphView>(graph: &G, source: NodeId) -> Vec<NodeId> {
    let mut postorder = Vec::new();
    if !graph.contains_node(source) {
        return postorder;
    }
    let mut visited: BTreeSet<NodeId> = BTreeSet::new();
    visited.insert(source);
    let mut stack: Vec<(NodeId, Vec<NodeId>, usize)> = vec![(source, graph.out_neighbors(source).collect(), 0)];
    while !stack.is_empty() {
        let i = stack.len() - 1;
        let node = stack[i].0;
        if stack[i].2 < stack[i].1.len() {
            let nbr = stack[i].1[stack[i].2];
            stack[i].2 += 1;
            if visited.insert(nbr) {
                stack.push((nbr, graph.out_neighbors(nbr).collect(), 0));
            }
        } else {
            postorder.push(node);
            stack.pop();
        }
    }
    postorder
}

/// 🏷️ Every edge encountered during a depth-first walk from `source`, labeled `true` for a tree (forward-discovery) edge and `false` for a non-tree edge (a back/cross/forward edge to an already-visited node) — NetworkX `dfs_labeled_edges` additionally emits synthetic start/finish markers per node; this crate omits those and keeps strictly one label per physically traversed edge. On undirected views the trivial mirror edge back to a node's own parent is not re-emitted (it is the same physical edge already labeled `true` on the way in); every other repeat, including self-loops and true back-edges in a cycle, is labeled `false`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dfs_labeled_edges<G: GraphView>(graph: &G, source: NodeId) -> Vec<(EdgeRef, bool)> {
    let mut result = Vec::new();
    if !graph.contains_node(source) {
        return result;
    }
    let mut visited: BTreeSet<NodeId> = BTreeSet::new();
    visited.insert(source);
    let directed = graph.is_directed();
    let mut stack: Vec<(NodeId, Vec<NodeId>, usize, Option<NodeId>)> = vec![(source, graph.out_neighbors(source).collect(), 0, None)];
    while !stack.is_empty() {
        let i = stack.len() - 1;
        let node = stack[i].0;
        let parent = stack[i].3;
        if stack[i].2 < stack[i].1.len() {
            let nbr = stack[i].1[stack[i].2];
            stack[i].2 += 1;
            let edge = any_edge(graph, node, nbr);
            if visited.insert(nbr) {
                result.push((edge, true));
                stack.push((nbr, graph.out_neighbors(nbr).collect(), 0, Some(node)));
            } else if directed || Some(nbr) != parent {
                result.push((edge, false));
            }
        } else {
            stack.pop();
        }
    }
    result
}

/// ⏪️ Maps every non-source node reached from `source` to its depth-first parent (NetworkX `dfs_predecessors`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dfs_predecessors<G: GraphView>(graph: &G, source: NodeId) -> BTreeMap<NodeId, NodeId> {
    let mut preds = BTreeMap::new();
    for edge in dfs_edges(graph, source) {
        preds.insert(edge.v, edge.u);
    }
    preds
}

/// ⏩️ Maps every node with depth-first tree children to the list of those children, in discovery order (NetworkX `dfs_successors`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dfs_successors<G: GraphView>(graph: &G, source: NodeId) -> BTreeMap<NodeId, Vec<NodeId>> {
    let mut succs: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();
    for edge in dfs_edges(graph, source) {
        succs.entry(edge.u).or_default().push(edge.v);
    }
    succs
}
// #endregion 🔖️Dfs

// #region 🔖️EdgeTraversal
/// 🕸️ Visits every edge reachable from `source` in breadth-first order, each edge exactly once by `EdgeId` — unlike `bfs_edges`, parallel edges in a multigraph are all visited, not just the first between a pair (NetworkX `edge_bfs`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn edge_bfs<G: GraphView>(graph: &G, source: NodeId) -> Vec<EdgeRef> {
    let mut result = Vec::new();
    if !graph.contains_node(source) {
        return result;
    }
    let mut visited_nodes: BTreeSet<NodeId> = BTreeSet::new();
    let mut visited_edges: BTreeSet<EdgeId> = BTreeSet::new();
    visited_nodes.insert(source);
    let mut queue: VecDeque<(NodeId, Vec<EdgeRef>, usize)> = VecDeque::new();
    queue.push_back((source, incident_edges(graph, source), 0));
    while !queue.is_empty() {
        let node = queue[0].0;
        if queue[0].2 < queue[0].1.len() {
            let edge = queue[0].1[queue[0].2];
            queue[0].2 += 1;
            if visited_edges.insert(edge.id) {
                result.push(edge);
                let other = if edge.u == node { edge.v } else { edge.u };
                if visited_nodes.insert(other) {
                    queue.push_back((other, incident_edges(graph, other), 0));
                }
            }
        } else {
            queue.pop_front();
        }
    }
    result
}

/// 🕳️ Visits every edge reachable from `source` in depth-first order, each edge exactly once by `EdgeId` (NetworkX `edge_dfs`); same multigraph-aware semantics as `edge_bfs`, but LIFO exploration order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn edge_dfs<G: GraphView>(graph: &G, source: NodeId) -> Vec<EdgeRef> {
    let mut result = Vec::new();
    if !graph.contains_node(source) {
        return result;
    }
    let mut visited_nodes: BTreeSet<NodeId> = BTreeSet::new();
    let mut visited_edges: BTreeSet<EdgeId> = BTreeSet::new();
    visited_nodes.insert(source);
    let mut stack: Vec<(NodeId, Vec<EdgeRef>, usize)> = vec![(source, incident_edges(graph, source), 0)];
    while !stack.is_empty() {
        let i = stack.len() - 1;
        let node = stack[i].0;
        if stack[i].2 < stack[i].1.len() {
            let edge = stack[i].1[stack[i].2];
            stack[i].2 += 1;
            if visited_edges.insert(edge.id) {
                result.push(edge);
                let other = if edge.u == node { edge.v } else { edge.u };
                if visited_nodes.insert(other) {
                    stack.push((other, incident_edges(graph, other), 0));
                }
            }
        } else {
            stack.pop();
        }
    }
    result
}
// #endregion 🔖️EdgeTraversal

// #region 🔖️Beam
/// 📡️ Breadth-first traversal that keeps only the `width` highest-`value` nodes of each newly discovered layer before expanding further (NetworkX `bfs_beam_edges` instead prunes per-node successor lists to the top `width` by value; this crate prunes per-layer across the whole frontier, which is the semantics specified for this crate — documented deviation).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bfs_beam_edges<G: GraphView>(graph: &G, source: NodeId, width: usize, value: impl Fn(NodeId) -> f64) -> Vec<EdgeRef> {
    let mut result = Vec::new();
    if !graph.contains_node(source) {
        return result;
    }
    let mut visited: BTreeSet<NodeId> = BTreeSet::new();
    visited.insert(source);
    let mut frontier: Vec<NodeId> = vec![source];
    while !frontier.is_empty() {
        let mut discovered: BTreeMap<NodeId, NodeId> = BTreeMap::new();
        for &node in &frontier {
            for nbr in graph.out_neighbors(node) {
                if !visited.contains(&nbr) {
                    discovered.entry(nbr).or_insert(node);
                }
            }
        }
        if discovered.is_empty() {
            break;
        }
        let mut valued: Vec<(f64, NodeId)> = discovered.keys().map(|&n| (value(n), n)).collect();
        valued.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal).then(a.1.cmp(&b.1)));
        valued.truncate(width);
        let mut chosen: Vec<NodeId> = valued.into_iter().map(|(_, n)| n).collect();
        chosen.sort_unstable();
        let mut next_frontier = Vec::new();
        for child in chosen {
            let parent = discovered[&child];
            result.push(any_edge(graph, parent, child));
            visited.insert(child);
            next_frontier.push(child);
        }
        frontier = next_frontier;
    }
    result
}
// #endregion 🔖️Beam

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use graph_core::{Directed, Normal, Ported, Storage, Undirected};

    // #subregion Fixtures
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn chain() -> (Storage<Normal, Directed>, Vec<NodeId>) {
        let mut g = Storage::<Normal, Directed>::new();
        let nodes: Vec<NodeId> = (0..4).map(|_| g.add_node()).collect();
        g.add_edge(nodes[0], nodes[1]);
        g.add_edge(nodes[1], nodes[2]);
        g.add_edge(nodes[2], nodes[3]);
        (g, nodes)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn tree() -> (Storage<Normal, Directed>, Vec<NodeId>) {
        let mut g = Storage::<Normal, Directed>::new();
        let nodes: Vec<NodeId> = (0..6).map(|_| g.add_node()).collect();
        g.add_edge(nodes[0], nodes[1]);
        g.add_edge(nodes[0], nodes[2]);
        g.add_edge(nodes[1], nodes[3]);
        g.add_edge(nodes[1], nodes[4]);
        g.add_edge(nodes[2], nodes[5]);
        (g, nodes)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn cycle() -> (Storage<Normal, Directed>, Vec<NodeId>) {
        let mut g = Storage::<Normal, Directed>::new();
        let nodes: Vec<NodeId> = (0..3).map(|_| g.add_node()).collect();
        g.add_edge(nodes[0], nodes[1]);
        g.add_edge(nodes[1], nodes[2]);
        g.add_edge(nodes[2], nodes[0]);
        (g, nodes)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn disconnected() -> (Storage<Normal, Undirected>, Vec<NodeId>) {
        let mut g = Storage::<Normal, Undirected>::new();
        let nodes: Vec<NodeId> = (0..4).map(|_| g.add_node()).collect();
        g.add_edge(nodes[0], nodes[1]);
        g.add_edge(nodes[2], nodes[3]);
        (g, nodes)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn self_loop() -> (Storage<Normal, Directed>, Vec<NodeId>) {
        let mut g = Storage::<Normal, Directed>::new();
        let nodes: Vec<NodeId> = (0..2).map(|_| g.add_node()).collect();
        g.add_edge(nodes[0], nodes[0]);
        g.add_edge(nodes[0], nodes[1]);
        (g, nodes)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn multigraph() -> (Storage<Ported, Directed>, Vec<NodeId>) {
        let mut g = Storage::<Ported, Directed>::new();
        let n0 = g.add_node();
        let n1 = g.add_node();
        let h0a = g.add_handle(n0).expect("n0 exists");
        let h0b = g.add_handle(n0).expect("n0 exists");
        let h1 = g.add_handle(n1).expect("n1 exists");
        g.add_edge(h0a, h1);
        g.add_edge(h0b, h1);
        (g, vec![n0, n1])
    }
    // #endsubregion

    // #subregion Bfs
    #[semio_framework_async_macros::async_test]
    async fn bfs_edges_chain_is_linear() {
        let (g, n) = chain();
        let edges = bfs_edges(&g, n[0]);
        let pairs: Vec<(NodeId, NodeId)> = edges.iter().map(|e| (e.u, e.v)).collect();
        assert_eq!(pairs, vec![(n[0], n[1]), (n[1], n[2]), (n[2], n[3])]);
    }

    #[semio_framework_async_macros::async_test]
    async fn bfs_tree_matches_bfs_edges() {
        let (g, n) = tree();
        assert_eq!(bfs_tree(&g, n[0]), bfs_edges(&g, n[0]));
    }

    #[semio_framework_async_macros::async_test]
    async fn bfs_edges_on_cycle_terminates_and_covers_all_nodes() {
        let (g, n) = cycle();
        let edges = bfs_edges(&g, n[0]);
        assert_eq!(edges.len(), 2);
        let visited: BTreeSet<NodeId> = edges.iter().flat_map(|e| [e.u, e.v]).collect();
        assert_eq!(visited, n.iter().copied().collect::<BTreeSet<_>>());
    }

    #[semio_framework_async_macros::async_test]
    async fn bfs_layers_multi_source() {
        let (g, n) = tree();
        let layers = bfs_layers(&g, &[n[1], n[2]]);
        assert_eq!(layers[0], vec![n[1], n[2]]);
        assert_eq!(layers[1], vec![n[3], n[4], n[5]]);
    }

    #[semio_framework_async_macros::async_test]
    async fn bfs_predecessors_and_successors_on_tree() {
        let (g, n) = tree();
        let preds = bfs_predecessors(&g, n[0]);
        assert_eq!(preds.get(&n[3]), Some(&n[1]));
        assert_eq!(preds.get(&n[5]), Some(&n[2]));
        let succs = bfs_successors(&g, n[0]);
        assert_eq!(succs.get(&n[0]), Some(&vec![n[1], n[2]]));
        assert_eq!(succs.get(&n[1]), Some(&vec![n[3], n[4]]));
    }

    #[semio_framework_async_macros::async_test]
    async fn descendants_at_distance_on_tree() {
        let (g, n) = tree();
        assert_eq!(descendants_at_distance(&g, n[0], 0), BTreeSet::from([n[0]]));
        assert_eq!(descendants_at_distance(&g, n[0], 1), BTreeSet::from([n[1], n[2]]));
        assert_eq!(descendants_at_distance(&g, n[0], 2), BTreeSet::from([n[3], n[4], n[5]]));
        assert_eq!(descendants_at_distance(&g, n[0], 9), BTreeSet::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn generic_bfs_edges_honors_custom_neighbor_order() {
        let (g, n) = tree();
        let edges = generic_bfs_edges(&g, n[0], |graph: &Storage<Normal, Directed>, node: NodeId| -> Vec<NodeId> {
            let mut ns: Vec<NodeId> = graph.out_neighbors(node).collect();
            ns.reverse();
            ns
        });
        assert_eq!(edges.len(), 5);
        assert_eq!(edges[0].v, n[2]);
        assert_eq!(edges[1].v, n[1]);
    }

    #[semio_framework_async_macros::async_test]
    async fn bfs_on_disconnected_stays_in_source_component() {
        let (g, n) = disconnected();
        let edges = bfs_edges(&g, n[0]);
        let visited: BTreeSet<NodeId> = edges.iter().flat_map(|e| [e.u, e.v]).chain([n[0]]).collect();
        assert!(!visited.contains(&n[2]));
        assert!(!visited.contains(&n[3]));
    }

    #[semio_framework_async_macros::async_test]
    async fn bfs_edges_handles_self_loop_without_looping() {
        let (g, n) = self_loop();
        let edges = bfs_edges(&g, n[0]);
        let pairs: Vec<(NodeId, NodeId)> = edges.iter().map(|e| (e.u, e.v)).collect();
        assert_eq!(pairs, vec![(n[0], n[1])]);
    }

    #[semio_framework_async_macros::async_test]
    async fn bfs_edges_on_missing_source_is_empty() {
        let (g, _n) = chain();
        assert!(bfs_edges(&g, 999).is_empty());
    }
    // #endsubregion

    // #subregion Dfs
    #[semio_framework_async_macros::async_test]
    async fn dfs_edges_chain_is_linear() {
        let (g, n) = chain();
        let edges = dfs_edges(&g, n[0]);
        let pairs: Vec<(NodeId, NodeId)> = edges.iter().map(|e| (e.u, e.v)).collect();
        assert_eq!(pairs, vec![(n[0], n[1]), (n[1], n[2]), (n[2], n[3])]);
    }

    #[semio_framework_async_macros::async_test]
    async fn dfs_tree_matches_dfs_edges() {
        let (g, n) = tree();
        assert_eq!(dfs_tree(&g, n[0]), dfs_edges(&g, n[0]));
    }

    #[semio_framework_async_macros::async_test]
    async fn dfs_preorder_and_postorder_on_tree() {
        let (g, n) = tree();
        assert_eq!(dfs_preorder_nodes(&g, n[0]), vec![n[0], n[1], n[3], n[4], n[2], n[5]]);
        assert_eq!(dfs_postorder_nodes(&g, n[0]), vec![n[3], n[4], n[1], n[5], n[2], n[0]]);
    }

    #[semio_framework_async_macros::async_test]
    async fn dfs_on_cycle_terminates_and_labels_back_edge() {
        let (g, n) = cycle();
        let preorder = dfs_preorder_nodes(&g, n[0]);
        assert_eq!(preorder, vec![n[0], n[1], n[2]]);
        let labeled = dfs_labeled_edges(&g, n[0]);
        assert_eq!(labeled.len(), 3);
        let nontree: Vec<_> = labeled.iter().filter(|(_, forward)| !forward).collect();
        assert_eq!(nontree.len(), 1);
        assert_eq!(nontree[0].0.u, n[2]);
        assert_eq!(nontree[0].0.v, n[0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn dfs_labeled_edges_undirected_skips_trivial_parent_mirror() {
        let (g, n) = disconnected();
        let labeled = dfs_labeled_edges(&g, n[0]);
        assert_eq!(labeled, vec![(any_edge(&g, n[0], n[1]), true)]);
    }

    #[semio_framework_async_macros::async_test]
    async fn dfs_predecessors_and_successors_on_tree() {
        let (g, n) = tree();
        let preds = dfs_predecessors(&g, n[0]);
        assert_eq!(preds.get(&n[3]), Some(&n[1]));
        let succs = dfs_successors(&g, n[0]);
        assert_eq!(succs.get(&n[0]), Some(&vec![n[1], n[2]]));
    }

    #[semio_framework_async_macros::async_test]
    async fn dfs_on_disconnected_stays_in_source_component() {
        let (g, n) = disconnected();
        let nodes = dfs_preorder_nodes(&g, n[0]);
        assert_eq!(nodes, vec![n[0], n[1]]);
    }

    #[semio_framework_async_macros::async_test]
    async fn dfs_edges_handles_self_loop_without_looping() {
        let (g, n) = self_loop();
        let edges = dfs_edges(&g, n[0]);
        let pairs: Vec<(NodeId, NodeId)> = edges.iter().map(|e| (e.u, e.v)).collect();
        assert_eq!(pairs, vec![(n[0], n[1])]);
    }
    // #endsubregion

    // #subregion EdgeTraversal
    #[semio_framework_async_macros::async_test]
    async fn edge_bfs_visits_every_parallel_edge() {
        let (g, n) = multigraph();
        let plain = bfs_edges(&g, n[0]);
        assert_eq!(plain.len(), 1);
        let all = edge_bfs(&g, n[0]);
        assert_eq!(all.len(), 2);
        let ids: BTreeSet<EdgeId> = all.iter().map(|e| e.id).collect();
        assert_eq!(ids.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn edge_dfs_visits_every_parallel_edge() {
        let (g, n) = multigraph();
        let all = edge_dfs(&g, n[0]);
        assert_eq!(all.len(), 2);
        let ids: BTreeSet<EdgeId> = all.iter().map(|e| e.id).collect();
        assert_eq!(ids.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn edge_bfs_on_cycle_visits_each_edge_once() {
        let (g, _n) = cycle();
        let edges = edge_bfs(&g, 0);
        assert_eq!(edges.len(), 3);
        let ids: BTreeSet<EdgeId> = edges.iter().map(|e| e.id).collect();
        assert_eq!(ids.len(), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn edge_bfs_handles_self_loop_without_looping() {
        let (g, n) = self_loop();
        let edges = edge_bfs(&g, n[0]);
        assert_eq!(edges.len(), 2);
    }
    // #endsubregion

    // #subregion Beam
    #[semio_framework_async_macros::async_test]
    async fn bfs_beam_edges_keeps_only_top_width_per_layer() {
        let (g, n) = tree();
        let value = |node: NodeId| -> f64 {
            if node == n[1] {
                10.0
            } else if node == n[2] {
                1.0
            } else {
                0.0
            }
        };
        let edges = bfs_beam_edges(&g, n[0], 1, value);
        let visited: BTreeSet<NodeId> = edges.iter().flat_map(|e| [e.u, e.v]).collect();
        assert!(visited.contains(&n[1]));
        assert!(!visited.contains(&n[2]));
        assert!(!visited.contains(&n[5]));
    }

    #[semio_framework_async_macros::async_test]
    async fn bfs_beam_edges_width_zero_terminates_immediately() {
        let (g, n) = tree();
        let edges = bfs_beam_edges(&g, n[0], 0, |_: NodeId| -> f64 { 0.0 });
        assert!(edges.is_empty());
    }
    // #endsubregion
}
// #endregion 🔖️Tests
