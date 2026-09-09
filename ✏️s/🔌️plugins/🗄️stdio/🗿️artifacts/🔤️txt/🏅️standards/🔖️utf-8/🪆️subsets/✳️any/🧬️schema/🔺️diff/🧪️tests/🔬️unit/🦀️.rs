use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn lines(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_before_matches_canonical_shape() {
    // Insert("f") at 2, then Remove(0) — canonical case: {removed:[0], added:[(1,f)]}.
    let d1 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: 2, text: "f".into() }] }), ..Default::default() };
    let d2 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![0], modified: vec![], added: vec![] }), ..Default::default() };
    let mut merged = d1.clone();
    merged.absorb(d2.clone());
    let ld = merged.lines.clone().expect("lines diff present");
    assert_eq!(ld.removed, vec![0]);
    assert_eq!(ld.added, vec![TxtLineAdded { index: 1, text: "f".into() }]);
    assert!(ld.modified.is_empty());

    let base = TxtSnapshot { lines: lines(&["a", "b", "c", "d"]), ..Default::default() };
    let sequential = {
        let mid = d1.apply(&base).unwrap();
        d2.apply(&mid).unwrap()
    };
    assert_eq!(merged.apply(&base).unwrap(), sequential);
}

#[semio_framework_async_macros::async_test]
async fn insert_insert_same_index_both_survive() {
    let d1 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: 2, text: "f".into() }] }), ..Default::default() };
    let d2 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: 2, text: "g".into() }] }), ..Default::default() };
    let mut merged = d1.clone();
    merged.absorb(d2.clone());
    let base = TxtSnapshot { lines: lines(&["a", "b", "c", "d"]), ..Default::default() };
    let sequential = {
        let mid = d1.apply(&base).unwrap();
        d2.apply(&mid).unwrap()
    };
    assert_eq!(merged.apply(&base).unwrap(), sequential);
    assert!(sequential.lines.contains(&"f".to_string()) && sequential.lines.contains(&"g".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn add_then_set_field_patches_into_added() {
    let d1 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: 1, text: "f".into() }] }), ..Default::default() };
    let d2 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![TxtLineModified { index: 1, text: "v".into() }], added: vec![] }), ..Default::default() };
    let mut merged = d1.clone();
    merged.absorb(d2.clone());
    let ld = merged.lines.clone().expect("lines diff present");
    assert!(ld.modified.is_empty(), "patched value should live in the added entry, not a separate modified entry");
    assert_eq!(ld.added, vec![TxtLineAdded { index: 1, text: "v".into() }]);

    let base = TxtSnapshot { lines: lines(&["a", "b", "c"]), ..Default::default() };
    let sequential = {
        let mid = d1.apply(&base).unwrap();
        d2.apply(&mid).unwrap()
    };
    assert_eq!(merged.apply(&base).unwrap(), sequential);
}

#[semio_framework_async_macros::async_test]
async fn modify_then_remove_drops_the_modify() {
    let d1 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![TxtLineModified { index: 0, text: "m".into() }], added: vec![] }), ..Default::default() };
    let d2 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![0], modified: vec![], added: vec![] }), ..Default::default() };
    let mut merged = d1.clone();
    merged.absorb(d2.clone());
    let ld = merged.lines.clone().expect("lines diff present");
    assert_eq!(ld.removed, vec![0]);
    assert!(ld.modified.is_empty());

    let base = TxtSnapshot { lines: lines(&["a", "b"]), ..Default::default() };
    let sequential = {
        let mid = d1.apply(&base).unwrap();
        d2.apply(&mid).unwrap()
    };
    assert_eq!(merged.apply(&base).unwrap(), sequential);
}

#[semio_framework_async_macros::async_test]
async fn absorb_associative_over_a_triple() {
    let base = TxtSnapshot { lines: lines(&["a", "b", "c"]), ..Default::default() };
    let d1 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![1], modified: vec![], added: vec![] }), ..Default::default() };
    let d2 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: 0, text: "x".into() }] }), ..Default::default() };
    let d3 = TxtDiff { trailing_newline: Some(true), ..Default::default() };

    let mut left = d1.clone();
    left.absorb(d2.clone());
    left.absorb(d3.clone());

    let mut mid = d2.clone();
    mid.absorb(d3.clone());
    let mut right = d1.clone();
    right.absorb(mid);

    assert_eq!(left.apply(&base).unwrap(), right.apply(&base).unwrap());
    let sequential = {
        let s1 = d1.apply(&base).unwrap();
        let s2 = d2.apply(&s1).unwrap();
        d3.apply(&s2).unwrap()
    };
    assert_eq!(left.apply(&base).unwrap(), sequential);
}

#[semio_framework_async_macros::async_test]
async fn between_roundtrip_synthetic() {
    let a = TxtSnapshot { lines: lines(&["a", "b", "c"]), trailing_newline: true, line_ending: LineEnding::Lf, ..Default::default() };
    let b = TxtSnapshot { lines: lines(&["a", "x", "c", "d"]), trailing_newline: false, line_ending: LineEnding::CrLf, ..Default::default() };
    assert_eq!(TxtDiff::between(&a, &b).apply(&a).unwrap(), b);
    assert_eq!(TxtDiff::between(&b, &a).apply(&b).unwrap(), a);
    assert!(TxtDiff::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diff_level_roundtrip() {
    let base = TxtSnapshot { lines: lines(&["a", "b"]), trailing_newline: false, line_ending: LineEnding::Lf, ..Default::default() };
    let d = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![0], modified: vec![], added: vec![TxtLineAdded { index: 0, text: "z".into() }] }), trailing_newline: Some(true), line_ending: Some(LineEnding::CrLf) };
    let next = d.apply(&base).unwrap();
    let inv = d.inverse(&base);
    assert_eq!(inv.apply(&next).unwrap(), base);
}

/// 🧪️ F6: `DiffCodec` round-trip laws (derived via `dsl::DslDiff`) — exercises the empty
/// diff, scalar-only changes, and every one of `TxtLinesDiff`'s `removed`/`modified`/`added`
/// sections populated simultaneously (a real `between()` result can only ever populate
/// `modified`+`removed` OR `modified`+`added`, per `TxtLinesDiff::between`'s own base-tail/
/// other-tail algorithm above -- so this test also directly constructs one diff exercising
/// all three sections at once, plus a genuine `between()` result for good measure).
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    use protocol::DiffCodec;
    let a = TxtSnapshot { lines: lines(&["a", "b", "c"]), trailing_newline: true, line_ending: LineEnding::Lf, ..Default::default() };
    let b = TxtSnapshot { lines: lines(&["a", "x", "c", "d"]), trailing_newline: false, line_ending: LineEnding::CrLf, ..Default::default() };
    let cases = vec![
        TxtDiff::default(),
        TxtDiff { trailing_newline: Some(true), line_ending: Some(LineEnding::CrLf), lines: None },
        TxtDiff {
            trailing_newline: Some(false),
            line_ending: Some(LineEnding::Lf),
            lines: Some(TxtLinesDiff { removed: vec![0, 2], modified: vec![TxtLineModified { index: 1, text: "changed".into() }], added: vec![TxtLineAdded { index: 0, text: "new-head".into() }, TxtLineAdded { index: 3, text: "new-tail".into() }] }),
        },
        TxtDiff::between(&a, &b),
    ];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = TxtDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?} (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff({d:?}) failed: {e}"));
        let decoded = TxtDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
    }
}

//#region 🔖️DiffGrammarConformanceLaw
/// 🧪️ P2-P3: `dsl::parse_grammar` + `dsl::Recognizer::compile` + `.recognize` against REAL
/// `print_diff` output -- the empty diff, a scalar-only diff, and a diff exercising every one
/// of `TxtLinesDiff`'s `removed`/`modified`/`added` sections at once (a real `between()`
/// result can only ever populate `modified`+`removed` OR `modified`+`added` at a time, per
/// `TxtLinesDiff::between`'s own base-tail/other-tail algorithm above, so this also directly
/// constructs one diff exercising all three sections simultaneously, plus a genuine
/// `between()` result for good measure -- same case list `diff_codec_text_binary_roundtrip_law`
/// already uses).
#[semio_framework_async_macros::async_test]
async fn diff_grammar_conformance_law() {
    use protocol::DiffCodec;
    let grammar_text = crate::schema::diff::text::COMPONENT_GRAMMAR_SEMIO;
    let grammar = dsl::parse_grammar(grammar_text).expect("parse diff grammar");
    let recognizer = dsl::Recognizer::compile(&grammar);

    let a = TxtSnapshot { lines: lines(&["a", "b", "c"]), trailing_newline: true, line_ending: LineEnding::Lf, ..Default::default() };
    let b = TxtSnapshot { lines: lines(&["a", "x", "c", "d"]), trailing_newline: false, line_ending: LineEnding::CrLf, ..Default::default() };
    let cases = vec![
        TxtDiff::default(),
        TxtDiff { trailing_newline: Some(true), line_ending: Some(LineEnding::CrLf), lines: None },
        TxtDiff {
            trailing_newline: Some(false),
            line_ending: Some(LineEnding::Lf),
            lines: Some(TxtLinesDiff { removed: vec![0, 2], modified: vec![TxtLineModified { index: 1, text: "changed".into() }], added: vec![TxtLineAdded { index: 0, text: "new-head".into() }, TxtLineAdded { index: 3, text: "new-tail".into() }] }),
        },
        TxtDiff::between(&a, &b),
    ];
    for d in cases {
        let printed = d.print_diff();
        let ok = recognizer.recognize(&printed).unwrap_or_else(|e| panic!("recognize({printed:?}) errored: {e:?}"));
        assert!(ok, "diff grammar must recognize real print_diff output {printed:?} for {d:?}");
    }
}
//#endregion 🔖️DiffGrammarConformanceLaw
