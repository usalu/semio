
use super::*;
use crate::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, PortRef, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn node(id: &str) -> FlowNode {
    FlowNode { id: id.into(), kind: "task".into(), label: id.into(), params: Vec::new(), position: Default::default() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn edge(id: &str, from_node: &str, to_node: &str) -> FlowEdge {
    FlowEdge { id: id.into(), from: PortRef { node: from_node.into(), port: "out".into() }, to: PortRef { node: to_node.into(), port: "in".into() }, kind: "data".into() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn chain_snapshot() -> SemioFlowSnapshot {
    // root -e1- mid -e2- leaf: a 3-node chain.
    SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes: vec![node("root"), node("mid"), node("leaf")], edges: vec![edge("e1", "root", "mid"), edge("e2", "mid", "leaf")] }
}

#[semio_framework_async_macros::async_test]
async fn chain_is_cycle_free_with_increasing_depth() {
    let topology = compute_semio_flow_topology(&chain_snapshot());
    assert!(topology.cycle_free);
    assert_eq!(topology.node_count, 3);
    assert_eq!(topology.topo_order, vec!["root".to_string(), "mid".to_string(), "leaf".to_string()]);
    assert_eq!(topology.depth.get("root"), Some(&0));
    assert_eq!(topology.depth.get("mid"), Some(&1));
    assert_eq!(topology.depth.get("leaf"), Some(&2));
}

#[semio_framework_async_macros::async_test]
async fn a_cycle_is_reported_as_not_cycle_free() {
    let mut snapshot = chain_snapshot();
    snapshot.edges.push(edge("e3", "leaf", "root"));
    let topology = compute_semio_flow_topology(&snapshot);
    assert!(!topology.cycle_free);
    assert!(topology.topo_order.is_empty(), "every node in the 3-cycle has nonzero indegree");
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = chain_snapshot();
    assert_eq!(compute_semio_flow_topology(&snapshot), compute_semio_flow_topology(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_flow_topology(&SemioFlowSnapshot::default()), SemioFlowTopology::default());
}
