
use super::*;
use crate::standards::v1::subsets::flow::schema::snapshot::STDIO_SEMIOFLOW_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio() -> SemioFlowSnapshot {
    SemioFlowSnapshot {
        schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(),
        nodes: vec![
            FlowNode { id: "n1".into(), kind: "source".into(), label: "Source".into(), params: vec![FlowParam { key: "count".into(), value: "3".into() }], position: SemioPoint2 { x: 0.0, y: 0.0 } },
            FlowNode { id: "n2".into(), kind: "sink".into(), label: "Sink".into(), params: Vec::new(), position: SemioPoint2 { x: 100.0, y: 50.0 } },
        ],
        edges: vec![FlowEdge { id: "e1".into(), from: PortRef { node: "n1".into(), port: "out".into() }, to: PortRef { node: "n2".into(), port: "in".into() }, kind: "data".into() }],
    }
}

#[semio_framework_async_macros::async_test]
async fn maps_nodes_and_edges_to_json() {
    let json = semio_framework_plugin::resolve_ready(SemioFlowToJson::serialize(&sample_semio())).expect("serialize");
    let root = match &json.value {
        JsonValue::Object { members } => members,
        other => panic!("expected object, got {other:?}"),
    };
    let nodes = root.iter().find(|m| m.key == "nodes").expect("nodes member");
    match &nodes.value {
        JsonValue::Array { items } => assert_eq!(items.len(), 2),
        other => panic!("expected array, got {other:?}"),
    }
}

/// 🔁️ Full round trip through THIS pair alone: serialize then encode/decode through the real
/// `JsonSnapshot::ArtifactPack` byte codec and re-parse — proves the JSON shape this leaf emits
/// is not just structurally right but genuinely re-parseable RFC8259 text.
#[semio_framework_async_macros::async_test]
async fn serialized_json_round_trips_through_the_real_json_text_codec() {
    let json1 = semio_framework_plugin::resolve_ready(SemioFlowToJson::serialize(&sample_semio())).expect("serialize");
    let text = semio_s_artifact_stdio_json::schema::snapshot::write_json_text(&json1.value);
    let reparsed = semio_s_artifact_stdio_json::schema::snapshot::parse_json_text(&text).expect("re-parse emitted json text");
    assert_eq!(reparsed, json1.value);
}
