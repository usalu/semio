use super::*;
use crate::standards::v1::subsets::base::schema::triples::IndexedTripleDiff;
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
use protocol::{DiffBinary,DiffCodec,DiffText};

fn one_node() -> SemioGraphSnapshot {
    SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: vec![SemioGraphNode { id: GraphNodeId::new("a"), ..Default::default() }], edges: vec![] }
}

fn relabel(label: &str) -> SemioGraphDiff {
    use crate::standards::v1::subsets::base::schema::triples::IndexModified;
    SemioGraphDiff { nodes: Some(IndexedTripleDiff { modified: vec![IndexModified { index: 0, diff: SemioGraphNodeDiff { label: Some(label.into()), ..Default::default() } }], ..Default::default() }), edges: None }
}

#[semio_framework_async_macros::async_test]
async fn apply_touches_only_named_node_fields() {
    let base = one_node();
    let next = protocol::apply_diff(&relabel("x"), &base).expect("apply must succeed for a well-formed fixture");
    assert_eq!(next.nodes[0].label, "x");
    assert_eq!(next.nodes[0].id, GraphNodeId::new("a"), "untouched node fields must be preserved");
}

#[semio_framework_async_macros::async_test]
async fn absorb_equals_sequential_apply() {
    let base = one_node();
    let (first, second) = (relabel("x"), relabel("y"));
    let mut absorbed = first.clone();
    absorbed.absorb(second.clone());
    let sequential = protocol::apply_diff(&second, &protocol::apply_diff(&first, &base).unwrap()).unwrap();
    assert_eq!(protocol::apply_diff(&absorbed, &base).unwrap(), sequential);
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_base() {
    let base = one_node();
    let diff = relabel("x");
    let next = protocol::apply_diff(&diff, &base).unwrap();
    let inverse = protocol::command::DiffAlgebra::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff(&inverse, &next).unwrap(), base);
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
    assert_eq!(crate::standards::v1::subsets::graph::io::text::snapshot::dec_edge_id("").unwrap(), GraphEdgeId::new(""));
}
