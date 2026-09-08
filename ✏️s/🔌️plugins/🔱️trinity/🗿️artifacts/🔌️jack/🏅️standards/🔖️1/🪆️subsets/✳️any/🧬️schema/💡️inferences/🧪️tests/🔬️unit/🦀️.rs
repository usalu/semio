
use super::*;
use crate::{Edge, Node, Port, PortDirection, PropertyBag};
use protocol::Inference;

//#region 🧸️Fixtures
fn node(id: &str) -> Node {
    Node {
        id: id.into(),
        kind: "Piece".into(),
        name: id.into(),
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
        properties: PropertyBag::new(),
        ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }, Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
    }
}

fn edge(id: &str, source: &str, target: &str) -> Edge {
    Edge { id: id.into(), kind: "Connection".into(), source: source.into(), target: target.into(), properties: PropertyBag::new() }
}

fn chain_snapshot() -> JackSnapshot {
    JackSnapshot::with_content(
        "trinity.graph".into(),
        "chain".into(),
        None,
        Default::default(),
        Default::default(),
        vec![node("root"), node("mid"), node("leaf")],
        vec![edge("e1", "root@out", "mid@in"), edge("e2", "mid@out", "leaf@in")],
        Some("root".into()),
    )
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = chain_snapshot();
    assert_eq!(JackInference::infer(&snapshot), JackInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(JackInference::infer(&JackSnapshot::default()), JackInference::default());
}

#[semio_framework_async_macros::async_test]
async fn inference_matches_compute_topology_directly() {
    let snapshot = chain_snapshot();
    let inferred = JackInference::infer(&snapshot);
    assert_eq!(inferred.topology, compute_topology(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_matches_compute_flat_position_directly() {
    let snapshot = chain_snapshot();
    let inferred = JackInference::infer(&snapshot);
    assert_eq!(inferred.flat_position, compute_flat_position(&snapshot));
}
//#endregion 🧪️InferenceLaws
