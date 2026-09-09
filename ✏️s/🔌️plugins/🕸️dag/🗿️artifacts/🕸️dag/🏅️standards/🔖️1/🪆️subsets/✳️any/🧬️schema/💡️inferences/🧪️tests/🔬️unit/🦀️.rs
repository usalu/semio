use super::*;
use crate::{DagFixtureEdge, DagNodeSpec};
use protocol::Inference;

fn chain_snapshot() -> DagSnapshot {
    let a = DagNodeSpec { id: "a".into(), ..Default::default() };
    let b = DagNodeSpec { id: "b".into(), ..Default::default() };
    let edges = vec![DagFixtureEdge { id: "e1".into(), source: "a".into(), target: "b".into(), ..Default::default() }];
    let content = crate::dag_content_child_with_owner(vec![a, b], edges);
    DagSnapshot { schema: "dag.dag".into(), content }
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = chain_snapshot();
    assert_eq!(DagInference::infer(&snapshot), DagInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(DagInference::infer(&DagSnapshot::default()), DagInference::default());
}

#[semio_framework_async_macros::async_test]
async fn topology_counts_every_node_exactly_once() {
    let snapshot = chain_snapshot();
    let inferred = DagInference::infer(&snapshot);
    let node_count = snapshot.nodes().len();
    assert_eq!(inferred.topology.node_count as usize, node_count);
    assert_eq!(inferred.topology.topo_order.len(), node_count);
    assert!(inferred.topology.cycle_free);
}
