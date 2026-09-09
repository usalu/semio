use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated_snapshot() -> SemioCadSnapshot {
    SemioCadSnapshot {
        schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: vec![CadLayer { name: "0".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true }],
        blocks: vec![CadBlock {
            name: "door".into(),
            base_point: SemioPoint2 { x: 0.0, y: 0.0 },
            entities: vec![CadEntityRecord { handle: "b1".into(), layer: "0".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 1.0, y: 0.0 } } }],
        }],
        entities: vec![
            CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 2.0, y: 2.0 }, radius: 1.5 } },
            CadEntityRecord { handle: "h2".into(), layer: "0".into(), entity: CadEntity::Insert { block_name: "door".into(), insertion_point: SemioPoint2 { x: 5.0, y: 5.0 }, scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 90.0 } },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = SemioCadSnapshot::default();
    let bytes = <SemioCadSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioCadSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioCadSnapshot::default();
    let text = <SemioCadSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

/// 🧪️ Law 5 — `codec_retention_law`: decode(encode(x)) == x on a fully populated snapshot
/// (layers/blocks/nested-block-entities/top-level entities incl. `Insert`), both facets.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = populated_snapshot();
    let bytes = <SemioCadSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let via_pack = <SemioCadSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode pack");
    assert_eq!(via_pack, snap);

    let text = <SemioCadSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let via_dsl = <SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse dsl");
    assert_eq!(via_dsl, snap);
}

/// 🧪️ Every `CadEntity` variant (all 9) round-trips through both the pack binary and the dsl
/// text codec — the demo fixture used by the fixture-honesty conformance law.
#[semio_framework_async_macros::async_test]
async fn demo_snapshot_round_trips_pack_and_dsl() {
    let demo = demo_cad_snapshot();
    let packed = <SemioCadSnapshot as store::ArtifactPack>::encode_pack(&demo);
    assert_eq!(<SemioCadSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode"), demo);
    let text = <SemioCadSnapshot as store::ArtifactDsl>::print_dsl(&demo);
    assert_eq!(<SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), demo);
}
