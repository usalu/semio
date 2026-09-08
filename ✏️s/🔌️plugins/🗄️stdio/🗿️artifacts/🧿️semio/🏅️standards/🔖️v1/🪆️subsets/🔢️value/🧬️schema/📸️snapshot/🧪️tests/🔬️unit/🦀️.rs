
use super::*;

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = demo_semio_value_snapshot();
    let bytes = <SemioValueSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioValueSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = demo_semio_value_snapshot();
    let text = <SemioValueSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioValueSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

/// 🧪️ codec_retention_law: decode(encode(x)) == x, with an explicit lexeme-fidelity assertion
/// (an arbitrary-precision int lexeme and a trailing-zero float lexeme — both would silently
/// corrupt if either variant were ever routed through `i64`/`f64`) plus the `Ref`/graph shape
/// surviving intact.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law_preserves_lexemes_bytes_and_graph_shape() {
    let snap = SemioValueSnapshot {
        schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
        root: SemioValue::List {
            items: vec![SemioValue::Int { lexeme: "9007199254740993".into() }, SemioValue::Float { lexeme: "1.2300".into() }, SemioValue::Bytes { value: (0..=255u8).collect() }, SemioValue::Ref { id: ValueId::new("root-child") }],
        },
        nodes: vec![SemioValueNode { id: ValueId::new("root-child"), value: SemioValue::Str { value: "leaf".into() } }],
    };
    let bytes = <SemioValueSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioValueSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
    match &back.root {
        SemioValue::List { items } => {
            assert_eq!(items[0], SemioValue::Int { lexeme: "9007199254740993".into() }, "int lexeme must survive verbatim");
            assert_eq!(items[1], SemioValue::Float { lexeme: "1.2300".into() }, "float lexeme (incl. trailing zero) must survive verbatim");
            assert_eq!(items[2], SemioValue::Bytes { value: (0..=255u8).collect() });
        }
        other => panic!("expected list root, got {other:?}"),
    }
    assert_eq!(back.nodes.len(), 1);
    assert_eq!(back.nodes[0].id, ValueId::new("root-child"));
}
