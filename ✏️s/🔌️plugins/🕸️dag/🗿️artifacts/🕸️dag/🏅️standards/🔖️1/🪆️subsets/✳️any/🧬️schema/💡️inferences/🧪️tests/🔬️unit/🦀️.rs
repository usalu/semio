use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = DagSnapshot::default();
    assert_eq!(DagInference::infer(&snapshot).expect("valid inference fixture"), DagInference::infer(&snapshot).expect("valid inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(DagInference::infer(&DagSnapshot::default()).expect("valid inference fixture"), DagInference::default());
}

/// 🌳️ The topology of the derivable demo content counts every node once and is cycle-free; the empty graph infers empty.
#[semio_framework_async_macros::async_test]
async fn topology_counts_every_node_exactly_once() {
    let snapshot = DagSnapshot::default();
    let inferred = DagInference::infer(&snapshot).expect("valid inference fixture");
    let node_count = crate::examples::demo::scene().nodes.len();
    assert_eq!(inferred.topology.node_count as usize, node_count);
    assert_eq!(inferred.topology.topo_order.len(), node_count);
    assert!(inferred.topology.cycle_free);
    assert_eq!(DagInference::infer(&crate::empty_snapshot()).expect("empty inference").topology.node_count, 0);
}
