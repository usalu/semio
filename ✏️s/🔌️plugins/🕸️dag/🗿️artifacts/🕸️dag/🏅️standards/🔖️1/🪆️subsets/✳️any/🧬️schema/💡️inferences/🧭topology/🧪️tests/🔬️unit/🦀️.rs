
use super::*;

fn node(id: &str) -> DagNodeSpec {
    DagNodeSpec { id: id.into(), ..Default::default() }
}

fn edge(id: &str, source: &str, target: &str) -> DagFixtureEdge {
    DagFixtureEdge { id: id.into(), source: source.into(), target: target.into(), ..Default::default() }
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
