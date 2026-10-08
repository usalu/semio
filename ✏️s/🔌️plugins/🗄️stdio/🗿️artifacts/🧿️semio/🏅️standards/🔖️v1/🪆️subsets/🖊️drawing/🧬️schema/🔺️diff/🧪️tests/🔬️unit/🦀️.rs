use crate::standards::v1::subsets::drawing::schema::diff::transform;
use super::*;
use protocol::{DiffBinary,DiffCodec,DiffText};

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
    let cases = demo_diff_cases();
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
