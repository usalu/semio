use super::*;
use crate::{Path, Step};
use protocol::Inference;
use std::collections::BTreeMap;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = crate::schema::default_snapshot();
    assert_eq!(ProcedureInference::infer(&snapshot).expect("valid inference fixture"), ProcedureInference::infer(&snapshot).expect("valid inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(ProcedureInference::infer(&ProcedureSnapshot::default()).expect("valid inference fixture"), ProcedureInference::default());
}

/// 🧭️ The demo's derivable program counts every step exactly once, in program order.
#[semio_framework_async_macros::async_test]
async fn topology_counts_every_step_exactly_once() {
    let inferred = ProcedureInference::infer(&crate::schema::default_snapshot()).expect("valid inference fixture");
    assert_eq!(inferred.topology.node_count, 2);
    assert_eq!(inferred.topology.topo_order, vec!["step-1".to_string(), "step-2".to_string()]);
    assert!(inferred.topology.cycle_free);
}

/// 🌱️ A parent whose children are not derivable has no child view here, so it infers the empty topology instead of a
/// fabricated one (design §20.15: content lives only in the member stores).
#[semio_framework_async_macros::async_test]
async fn a_non_derivable_parent_infers_the_empty_topology() {
    let path = Path { steps: vec![Step { id: "a".into(), kind: "noop".into(), params: Default::default(), bodies: BTreeMap::new() }] };
    let snapshot = crate::procedure_snapshot_naming(&path, &BTreeMap::new());
    assert_eq!(ProcedureInference::infer(&snapshot).expect("valid inference fixture"), ProcedureInference::default());
}
