
use super::*;
use protocol::DiffCodec;

/// 🧪️ `SlideShapeDiff::Replace` coverage: a shape-KIND change at the same slide/shape index
/// (never reachable through any single mutation variant — only through a real structural
/// `between()` on two full snapshots, e.g. `SetSnapshot`) must fall back to whole-shape
/// replacement, round-trip through the hand-rolled `DiffCodec`, and apply/inverse correctly.
#[semio_framework_async_macros::async_test]
async fn shape_kind_change_produces_replace_and_round_trips() {
    let mut a = snapshot_a();
    a.slides[0].shapes = vec![SlideShape::TextBox { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 1.0, height: 1.0 }, blocks: vec![DocBlock::paragraph("was text")] }];
    let mut b = a.clone();
    b.slides[0].shapes = vec![SlideShape::Placeholder { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 1.0, height: 1.0 }, kind: PlaceholderKind::Footer }];

    let diff = SemioPresentationDiff::between(&a, &b);
    let shapes_diff = diff.slides.as_ref().unwrap().modified[0].diff.shapes.as_ref().expect("shapes diff present");
    assert_eq!(shapes_diff.modified.len(), 1);
    assert!(matches!(&shapes_diff.modified[0].diff, SlideShapeDiff::Replace { .. }), "expected Replace for a shape-kind change, got {:?}", shapes_diff.modified[0].diff);

    assert_eq!(MutationDiff::apply(&diff, &a).expect("apply must succeed for a well-formed fixture"), b);
    let inv = DiffAlgebra::inverse(&diff, &a);
    assert_eq!(MutationDiff::apply(&inv, &b).expect("apply must succeed for a well-formed fixture"), a);

    let printed = diff.print_diff();
    let parsed = SemioPresentationDiff::parse_diff(&printed).expect("parse_diff");
    assert_eq!(parsed, diff, "Replace round-trip through the hand-rolled grammar failed (printed {printed:?})");
}

/// 🧪️ `DiffCodec` round-trip law over the hand-rolled `SemioPresentationDiff` grammar —
/// exercises masters/layouts (named-keyed removed/modified/added), slides (index-keyed
/// removed/modified/added incl. nested shape + `DocBlock`-reuse changes), and the `layout_id`
/// tri-state, in both directions plus the empty/self cases.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = snapshot_a();
    let b = snapshot_b();
    let cases = vec![SemioPresentationDiff::default(), SemioPresentationDiff::between(&a, &b), SemioPresentationDiff::between(&b, &a), SemioPresentationDiff::between(&a, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioPresentationDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioPresentationDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }

    // Field sweep confirmation: every collection flavor + the tri-state actually exercised.
    let diff_ab = SemioPresentationDiff::between(&a, &b);
    let masters = diff_ab.masters.as_ref().expect("masters diff present");
    assert!(!masters.removed.is_empty() && !masters.modified.is_empty() && !masters.added.is_empty(), "masters: not every flavor exercised");
    let layouts = diff_ab.layouts.as_ref().expect("layouts diff present");
    assert!(!layouts.modified.is_empty(), "layouts: master_id modify not exercised");
    let slides = diff_ab.slides.as_ref().expect("slides diff present");
    assert!(!slides.removed.is_empty(), "slides: removed not exercised");
    assert_eq!(slides.modified.len(), 1);
    let slide_diff = &slides.modified[0].diff;
    assert_eq!(slide_diff.layout_id, Some(Some("layout1".to_string())), "layout_id tri-state Some(Some(_)) not exercised");
    let shapes_diff = slide_diff.shapes.as_ref().expect("shapes diff present");
    assert!(!shapes_diff.modified.is_empty() && !shapes_diff.added.is_empty(), "shapes: modified/added not exercised");
    assert!(slide_diff.notes.as_ref().expect("notes diff present").added.len() > 0, "notes: added not exercised");
}
