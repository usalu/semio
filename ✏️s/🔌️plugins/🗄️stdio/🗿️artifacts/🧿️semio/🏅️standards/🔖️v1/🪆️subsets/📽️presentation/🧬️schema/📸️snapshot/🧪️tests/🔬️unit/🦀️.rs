
use super::*;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips() {
    let snap = SemioPresentationSnapshot::default();
    let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioPresentationSnapshot::default();
    let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn demo_snapshot_pack_and_dsl_round_trip() {
    let snap = demo_semio_presentation_snapshot();
    let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);

    let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back_text = <SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}

/// 🧪️ Non-empty structural round trip: masters/layouts/slides all populated, exercising every
/// shape kind + the document-block reuse.
#[semio_framework_async_macros::async_test]
async fn pack_round_trips_populated_structure() {
    let snap = SemioPresentationSnapshot {
        schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(),
        masters: vec![SlideMaster { id: "master1".into(), shapes: vec![SlideShape::Placeholder { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 100.0, height: 20.0 }, kind: PlaceholderKind::Title }] }],
        layouts: vec![SlideLayout { id: "layout1".into(), master_id: "master1".into(), shapes: Vec::new() }],
        slides: vec![Slide {
            id: "slide1".into(),
            layout_id: Some("layout1".into()),
            shapes: vec![
                SlideShape::TextBox { frame: SlideFrame { origin: SemioPoint2 { x: 1.0, y: 2.0 }, width: 50.0, height: 10.0 }, blocks: vec![DocBlock::paragraph("x")] },
                SlideShape::Picture { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 10.0, height: 10.0 }, image: SlidePictureImage { asset_id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] } },
                SlideShape::Table { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 30.0, height: 30.0 }, rows: vec![SlideTableRow { cells: vec![SlideTableCell { blocks: vec![DocBlock::paragraph("x")] }] }] },
            ],
            notes: vec![DocBlock::paragraph("x")],
        }],
    };
    let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}
