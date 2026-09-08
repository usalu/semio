
use super::*;
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
use protocol::DiffCodec;

#[semio_framework_async_macros::async_test]
async fn apply_replaces_nodes_and_edges_wholesale() {
    let base = SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: vec![SemioGraphNode { id: GraphNodeId::new("a"), ..Default::default() }], edges: vec![] };
    let diff = SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: vec![SemioGraphNode { id: GraphNodeId::new("b"), ..Default::default() }] }), edges: None };
    let next = diff.apply(&base).expect("apply must succeed for a well-formed fixture");
    assert_eq!(next.nodes[0].id, GraphNodeId::new("b"));
}

#[semio_framework_async_macros::async_test]
async fn absorb_last_write_wins() {
    let mut d1 = SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: vec![SemioGraphNode { id: GraphNodeId::new("a"), ..Default::default() }] }), edges: None };
    let d2 = SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: vec![SemioGraphNode { id: GraphNodeId::new("b"), ..Default::default() }] }), edges: None };
    d1.absorb(d2.clone());
    assert_eq!(d1, d2);
}

#[semio_framework_async_macros::async_test]
async fn diff_codec_graph_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioGraphDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioGraphDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}

#[semio_framework_async_macros::async_test]
async fn empty_diff_prints_empty_string() {
    assert_eq!(SemioGraphDiff::default().print_diff(), "");
}

#[semio_framework_async_macros::async_test]
async fn edge_id_helper_smoke() {
    assert_eq!(dec_edge_id("").unwrap(), GraphEdgeId::new(""));
}
