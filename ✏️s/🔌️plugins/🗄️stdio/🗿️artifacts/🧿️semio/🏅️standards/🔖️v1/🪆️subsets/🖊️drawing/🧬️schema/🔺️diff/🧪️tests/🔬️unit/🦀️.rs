
use super::*;
use protocol::DiffCodec;

#[semio_framework_async_macros::async_test]
async fn field_sweep_every_field_and_every_collection_shape() {
    let a = sweep_a();
    let b = sweep_b();
    let d = SemioDrawingDiff::between(&a, &b);

    // Every top-level facet changed.
    assert!(d.canvas.is_some());
    assert!(d.styles.is_some());
    assert!(d.layers.is_some());

    let canvas = d.canvas.as_ref().unwrap();
    assert!(canvas.width.is_some() && canvas.height.is_some());
    assert_eq!(canvas.background, Some(None)); // tri-state clear

    let styles = d.styles.as_ref().unwrap();
    assert_eq!(styles.removed, vec!["gone".to_string()]);
    assert_eq!(styles.modified.len(), 1);
    assert_eq!(styles.added.len(), 1);

    // `layers` (3 base -> 2 other): positional pairwise-then-tail gives removed (base tail,
    // `l1b`) + modified (indices 0 and 1) -- `added` is structurally empty here (see the
    // sweep doc comment); `children` below covers the `added` case instead.
    let layers = d.layers.as_ref().unwrap();
    assert_eq!(layers.removed, vec![2usize]);
    assert_eq!(layers.modified.len(), 2);
    assert!(layers.added.is_empty());
    let layer0_diff = &layers.modified[0].diff;
    assert!(layer0_diff.name.is_some() && layer0_diff.visible.is_some() && layer0_diff.root.is_some());
    let DrawNodeDiff::Group(group_diff) = layer0_diff.root.as_ref().unwrap() else { panic!("expected group diff") };
    assert!(group_diff.transform.is_some());
    // `Group.children` (2 base -> 3 other): positional pairwise-then-tail gives modified
    // (index 0, the Path) + added (index 2, the nested Group) -- `removed` is structurally
    // empty here, completing the `layers`/`children` removed+added split the doc comment
    // describes.
    let children = group_diff.children.as_ref().unwrap();
    assert!(children.removed.is_empty());
    assert!(!children.modified.is_empty(), "expected a modified child (the Path)");
    assert!(!children.added.is_empty(), "expected an added child (the nested Group)");
    let DrawNodeDiff::Path(path_diff) = &children.modified[0].diff else { panic!("expected path diff") };
    assert!(path_diff.segments.is_some());
    assert_eq!(path_diff.style, Some(None)); // tri-state clear on a node-level style ref

    assert_eq!(d.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
    assert_eq!(<SemioDrawingDiff as DiffAlgebra<SemioDrawingSnapshot>>::between(&b, &a).apply(&b).expect("apply must succeed for a well-formed fixture"), a);
    assert!(<SemioDrawingDiff as DiffAlgebra<SemioDrawingSnapshot>>::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn inverse_law_round_trips() {
    let a = sweep_a();
    let b = sweep_b();
    let d = SemioDrawingDiff::between(&a, &b);
    let inv = d.inverse(&a);
    assert_eq!(inv.apply(&d.apply(&a).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture"), a);
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_composes_two_sequential_diffs() {
    let a = sweep_a();
    let mid = sweep_b();
    let mut after = sweep_b();
    after.canvas.width = 999.0;
    after.styles.push(DrawStyle { name: "third".into(), fill: None, stroke: None, stroke_width: None, opacity: None });

    let mut d1 = SemioDrawingDiff::between(&a, &mid);
    let d2 = SemioDrawingDiff::between(&mid, &after);
    let applied_before_absorb = d1.apply(&a).expect("apply must succeed for a well-formed fixture");
    d1.absorb(d2.clone());
    assert_eq!(d1.apply(&a).expect("apply must succeed for a well-formed fixture"), d2.apply(&applied_before_absorb).expect("apply must succeed for a well-formed fixture"));
    assert_eq!(d1.apply(&a).expect("apply must succeed for a well-formed fixture"), after);
}

#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_remove_annihilates_the_add() {
    // 📐️ Canonical correctness case (schema-design.md): Insert(2)+Remove(index-of-that-insert)
    // must annihilate the add entirely, never leave a dangling modified/removed entry.
    let base: Vec<DrawStyle> = vec![DrawStyle { name: "a".into(), fill: None, stroke: None, stroke_width: None, opacity: None }];
    let d1: NamedTripleDiff<String, DrawStyleDiff, DrawStyle> = NamedTripleDiff { removed: vec![], modified: vec![], added: vec![DrawStyle { name: "b".into(), fill: None, stroke: None, stroke_width: None, opacity: None }] };
    let d2: NamedTripleDiff<String, DrawStyleDiff, DrawStyle> = NamedTripleDiff { removed: vec!["b".to_string()], modified: vec![], added: vec![] };
    let absorbed = absorb_named(d1, d2, |a, b| absorb_style_diff(&a, &b), apply_style_diff, |s: &DrawStyle| s.name.clone());
    assert!(absorbed.added.is_empty());
    assert!(absorbed.removed.is_empty());
    let applied = apply_named(&base, &absorbed, |s| &s.name, apply_style_diff);
    assert_eq!(applied, base);
}

#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    let c = SemioDrawingSnapshot::default();
    let cases = vec![SemioDrawingDiff::default(), SemioDrawingDiff::between(&a, &b), SemioDrawingDiff::between(&b, &a), SemioDrawingDiff::between(&a, &c), SemioDrawingDiff::between(&c, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioDrawingDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioDrawingDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
