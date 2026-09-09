use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn sample_json() -> JsonSnapshot {
    let text = r#"{"nodes":[{"id":"n1","kind":"source","label":"Source","params":[{"key":"count","value":"3"}],"position":{"x":0,"y":0}},{"id":"n2","kind":"sink","label":"Sink","params":[],"position":{"x":100,"y":50}}],"edges":[{"id":"e1","from":{"node":"n1","port":"out"},"to":{"node":"n2","port":"in"},"kind":"data"}]}"#;
    JsonSnapshot { schema: semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA.into(), value: semio_s_artifact_stdio_json::schema::snapshot::parse_json_text(text).expect("valid json fixture") }
}

#[semio_framework_async_macros::async_test]
async fn maps_nodes_and_edges() {
    let semio = semio_framework_plugin::resolve_ready(SemioFlowFromJson::deserialize(&sample_json())).expect("deserialize");
    assert_eq!(semio.nodes.len(), 2);
    assert_eq!(semio.edges.len(), 1);
    assert_eq!(semio.nodes[0].id, "n1");
    assert_eq!(semio.nodes[0].params[0].key, "count");
    assert_eq!(semio.nodes[1].position.x, 100.0);
    assert_eq!(semio.edges[0].from.node, "n1");
    assert_eq!(semio.edges[0].to.port, "in");
}

#[semio_framework_async_macros::async_test]
async fn missing_required_member_is_a_real_error() {
    let bad = JsonSnapshot { schema: semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA.into(), value: semio_s_artifact_stdio_json::schema::snapshot::parse_json_text("{}").unwrap() };
    assert!(semio_framework_plugin::resolve_ready(SemioFlowFromJson::deserialize(&bad)).is_err());
}
