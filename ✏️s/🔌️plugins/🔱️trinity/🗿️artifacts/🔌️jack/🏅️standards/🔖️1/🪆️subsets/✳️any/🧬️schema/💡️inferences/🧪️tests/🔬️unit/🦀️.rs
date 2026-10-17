use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
#[allow(unused_imports)]
use ::semio_framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::any::schema::content::JackWorkingScene;
use crate::standards::v1::subsets::any::schema::inferences::*;
use crate::standards::v1::subsets::any::schema::inferences::flat_position::compute_flat_position;
use crate::standards::v1::subsets::any::schema::inferences::topology::compute_topology;
use crate::standards::v1::subsets::any::schema::graph::{Edge, Node, Port};
use semio_framework_graph_manifest::{PortDirection, PropertyBag};
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
    JackSnapshot::with_content::<crate::test_support::FixtureComposition>("trinity.graph".into(), "chain".into(), None, Default::default(), Default::default(), JackWorkingScene { nodes: vec![node("root"), node("mid"), node("leaf")], edges: vec![edge("e1", "root@out", "mid@in"), edge("e2", "mid@out", "leaf@in")] }, Some("root".into()))
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = chain_snapshot();
    assert_eq!(JackInference::infer(&snapshot).expect("valid materialized inference fixture"), JackInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(JackInference::infer(&<crate::test_support::FixtureComposition as crate::standards::v1::subsets::any::schema::composition::JackComposition>::initial_snapshot()).expect("valid materialized inference fixture"), JackInference::default());
}

#[semio_framework_async_macros::async_test]
async fn inference_matches_compute_topology_directly() {
    let snapshot = chain_snapshot();
    let inferred = JackInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inferred.topology, compute_topology(&snapshot).expect("valid retained child inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_matches_compute_flat_position_directly() {
    let snapshot = chain_snapshot();
    let inferred = JackInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inferred.flat_position, compute_flat_position(&snapshot).expect("valid retained child inference fixture"));
}
//#endregion 🧪️InferenceLaws

#[test]
fn inference_default_law(){
 let projection=<crate::test_support::FixtureComposition as crate::standards::v1::subsets::any::schema::composition::JackComposition>::initial_snapshot();
 assert_eq!(JackInference::infer(&projection).expect("retained fixture"),JackInference::default());
 crate::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(projection).expect("fixture close");
}

#[test]
fn inference_determinism_law(){
 let projection=<crate::test_support::FixtureComposition as crate::standards::v1::subsets::any::schema::composition::JackComposition>::initial_snapshot();
 assert_eq!(JackInference::infer(&projection).expect("retained fixture"),JackInference::infer(&projection).expect("retained fixture"));
 crate::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(projection).expect("fixture close");
}
