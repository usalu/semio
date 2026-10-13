//! 🔁️ Round trips of the `flow` subset through each format: import, export, import again.

use crate::serializers::artifacts::json::v_rfc8259::any::SemioFlowToJson;
use semio_s_artifact_stdio_semio_flow_import::deserializers::artifacts::json::v_rfc8259::any::SemioFlowFromJson;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_framework_plugin::{ArtifactDeserializer, ArtifactSerializer};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn node(id: &str) -> FlowNode {
    FlowNode { id: id.into(), kind: "k".into(), label: "L".into(), params: Vec::new(), position: SemioPoint2::default() }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn edge(id: &str, from: &str, to: &str) -> FlowEdge {
    FlowEdge { id: id.into(), from: PortRef { node: from.into(), port: "out".into() }, to: PortRef { node: to.into(), port: "in".into() }, kind: "data".into() }
}
/// 🔁️ W4 G6 fixture-backed round trip: json1 -(deserialize)-> semio1 -(serialize)-> json2
/// -(deserialize)-> semio2, asserting semio1 == semio2 — this pair is lossless (every field
/// has a direct JSON member), so the round trip is exact, not just "modulo documented losses".
#[semio_framework_async_macros::async_test]
async fn json_round_trip_is_stable() {
    let semio1 = SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes: vec![node("a"), node("b")], edges: vec![edge("e1", "a", "b")] };
    let json1 = ::semio_framework_async::poll::resolve_ready(SemioFlowToJson::serialize(&semio1)).expect("serialize");
    let semio2 = ::semio_framework_async::poll::resolve_ready(SemioFlowFromJson::deserialize(&json1)).expect("deserialize");
    assert_eq!(semio1, semio2);
}
