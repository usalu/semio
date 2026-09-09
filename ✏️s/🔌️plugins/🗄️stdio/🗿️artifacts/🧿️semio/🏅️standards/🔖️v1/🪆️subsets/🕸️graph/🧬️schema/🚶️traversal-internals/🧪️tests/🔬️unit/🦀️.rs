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
