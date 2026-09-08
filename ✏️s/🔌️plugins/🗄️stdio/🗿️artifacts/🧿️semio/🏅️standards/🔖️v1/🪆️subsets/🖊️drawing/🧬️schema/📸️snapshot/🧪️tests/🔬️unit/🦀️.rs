
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample() -> SemioDrawingSnapshot {
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: 100.0, height: 50.0, background: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
        styles: vec![DrawStyle { name: "s1".into(), fill: Some(SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(2.0), opacity: None }],
        layers: vec![DrawLayer {
            id: "l0".into(),
            name: "base".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![
                    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }, PathSegment::LineTo { to: SemioPoint2 { x: 10.0, y: 10.0 } }, PathSegment::Close], style: Some("s1".into()) },
                    DrawNode::Text { value: "hi".into(), at: SemioPoint2 { x: 5.0, y: 5.0 }, style: None },
                    DrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: 8.0, height: 8.0, mime: "image/png".into(), bytes: vec![1, 2, 3] },
                ],
            },
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = sample();
    let bytes = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = sample();
    let text = <SemioDrawingSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_round_trips() {
    let snap = SemioDrawingSnapshot::default();
    let bytes = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

/// 🧪️ Every `PathSegment`/`DrawNode` variant (incl. nested `Group.children` recursion) round-
/// trips through both the pack binary and the dsl text codec — the demo fixture used by the
/// fixture-honesty conformance law.
#[semio_framework_async_macros::async_test]
async fn demo_snapshot_round_trips_pack_and_dsl() {
    let demo = demo_drawing_snapshot();
    let packed = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&demo);
    assert_eq!(<SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode"), demo);
    let text = <SemioDrawingSnapshot as store::ArtifactDsl>::print_dsl(&demo);
    assert_eq!(<SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), demo);
}
