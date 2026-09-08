
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample() -> SemioFlowSnapshot {
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
async fn json_pack_round_trips() {
    let snap = sample();
    let bytes = <SemioFlowSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioFlowSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = sample();
    let text = <SemioFlowSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioFlowSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_no_nodes_or_edges() {
    let snap = SemioFlowSnapshot::default();
    assert!(snap.nodes.is_empty());
    assert!(snap.edges.is_empty());
}
