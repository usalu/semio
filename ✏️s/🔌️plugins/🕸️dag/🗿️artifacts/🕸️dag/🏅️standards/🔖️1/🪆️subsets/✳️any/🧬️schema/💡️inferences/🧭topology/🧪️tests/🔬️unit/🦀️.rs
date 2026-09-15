use super::*;

fn node(id: &str) -> DagNodeSpec {
    DagNodeSpec { id: id.into(), ..Default::default() }
}

fn edge(id: &str, source: &str, target: &str) -> DagHostDocumentEdge {
    DagHostDocumentEdge { id: id.into(), source: source.into(), target: target.into(), ..Default::default() }
}

#[semio_framework_async_macros::async_test]
async fn linear_chain_orders_roots_before_leaves_with_increasing_depth() {
    let nodes = vec![node("a"), node("b"), node("c")];
    let edges = vec![edge("e1", "a", "b"), edge("e2", "b", "c")];
    let topology = compute_dag_topology(&nodes, &edges);
    assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    assert_eq!(topology.depth.get("a"), Some(&0));
    assert_eq!(topology.depth.get("b"), Some(&1));
    assert_eq!(topology.depth.get("c"), Some(&2));
    assert!(topology.cycle_free);
    assert_eq!(topology.node_count, 3);
}

#[semio_framework_async_macros::async_test]
async fn a_cycle_is_reported_as_not_cycle_free_but_still_totals_every_node() {
    let nodes = vec![node("a"), node("b")];
    let edges = vec![edge("e1", "a", "b"), edge("e2", "b", "a")];
    let topology = compute_dag_topology(&nodes, &edges);
    assert!(!topology.cycle_free);
    assert_eq!(topology.topo_order.len(), 2);
    assert_eq!(topology.node_count, 2);
}

#[semio_framework_async_macros::async_test]
async fn dangling_edge_endpoints_are_ignored() {
    let nodes = vec![node("a")];
    let edges = vec![edge("e1", "a", "missing")];
    let topology = compute_dag_topology(&nodes, &edges);
    assert!(topology.cycle_free);
    assert_eq!(topology.topo_order, vec!["a".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn port_endpoints_resolve_to_their_nodes() {
    let nodes = vec![node("a"), node("b"), node("c")];
    let edges = vec![edge("e1", "a@out", "b@in"), edge("e2", "b@out", "c@value")];
    let topology = compute_dag_topology(&nodes, &edges);
    assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    assert_eq!(topology.depth.get("c"), Some(&2));
}

/// ⏱️ The inference is recomputed on every read, so its whole-graph cost must stay below the interactive ceiling on a
/// large layered DAG (best of the fixture's attempts); above it the inference would need a stepped run.
#[semio_framework_async_macros::async_test]
async fn topology_of_a_large_layered_dag_stays_below_the_interactive_ceiling() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⏱️cost.json")).expect("cost law parses");
    let (layers, width) = (law["layers"].as_u64().expect("layers") as usize, law["width"].as_u64().expect("width") as usize);
    let id = |layer: usize, column: usize| format!("n{layer}-{column}");
    let nodes: Vec<DagNodeSpec> = (0..layers).flat_map(|layer| (0..width).map(move |column| (layer, column))).map(|(layer, column)| node(&id(layer, column))).collect();
    let edges: Vec<DagHostDocumentEdge> = (0..layers - 1)
        .flat_map(|layer| (0..width).flat_map(move |column| [column, (column + 1) % width].into_iter().map(move |next| (layer, column, next))))
        .map(|(layer, column, next)| edge(&format!("e{layer}-{column}-{next}"), &format!("{}@out", id(layer, column)), &format!("{}@in", id(layer + 1, next))))
        .collect();
    let mut best = u128::MAX;
    for _ in 0..law["attempts"].as_u64().expect("attempts") {
        let started = std::time::Instant::now();
        let topology = compute_dag_topology(&nodes, &edges);
        best = best.min(started.elapsed().as_micros());
        assert_eq!(topology.depth.get(&id(layers - 1, 0)), Some(&((layers - 1) as u32)));
    }
    let ids: Vec<String> = { let mut ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect(); ids.sort(); ids };
    let at = |endpoint: &str| ids.binary_search(&endpoint.split_once('@').map_or(endpoint, |(node, _)| node).to_string()).expect("known node") as u32;
    let oracle = semio_framework_graph_layout_run::testing::layout_run_longest_path_layers(ids.len(), &edges.iter().map(|edge| (at(&edge.source), at(&edge.target))).collect::<Vec<_>>()).expect("acyclic");
    let topology = compute_dag_topology(&nodes, &edges);
    assert!(ids.iter().zip(&oracle).all(|(id, layer)| topology.depth.get(id) == Some(layer)), "every depth equals the petgraph longest-path layer");
    assert!(best < u128::from(law["ceilingUs"].as_u64().expect("ceiling")), "topology took {best} µs");
}
