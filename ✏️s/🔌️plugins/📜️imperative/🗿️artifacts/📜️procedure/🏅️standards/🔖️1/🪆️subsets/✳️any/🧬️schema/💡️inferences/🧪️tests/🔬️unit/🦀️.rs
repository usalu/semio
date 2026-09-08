
use super::*;
use crate::{Path, Step};
use protocol::Inference;
use std::collections::BTreeMap;

fn chain_snapshot() -> ProcedureSnapshot {
    let path = Path { steps: vec![Step { id: "a".into(), kind: "noop".into(), params: Default::default(), bodies: BTreeMap::new() }, Step { id: "b".into(), kind: "noop".into(), params: Default::default(), bodies: BTreeMap::new() }] };
    crate::procedure_snapshot_with_content("procedure.document", &path, &BTreeMap::new())
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = chain_snapshot();
    assert_eq!(ProcedureInference::infer(&snapshot), ProcedureInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(ProcedureInference::infer(&ProcedureSnapshot::default()), ProcedureInference::default());
}

#[semio_framework_async_macros::async_test]
async fn topology_counts_every_step_exactly_once() {
    let snapshot = chain_snapshot();
    let inferred = ProcedureInference::infer(&snapshot);
    assert_eq!(inferred.topology.node_count, 2);
    assert_eq!(inferred.topology.topo_order, vec!["a".to_string(), "b".to_string()]);
    assert!(inferred.topology.cycle_free);
}
