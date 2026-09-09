use super::*;
use crate::STDIO_PDF_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn page(width: f64, height: f64, text: &str) -> PageDoc {
    PageDoc { width, height, text: text.into() }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snap(pages: Vec<PageDoc>) -> PdfSnapshot {
    PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn one(width: f64, height: f64, text: &str) -> PdfSnapshot {
    snap(vec![page(width, height, text)])
}

//#region between_roundtrip_law
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = one(612.0, 792.0, "hello");
    let b = one(300.0, 400.0, "world");
    assert_eq!(PdfDiff::between(&a, &b).apply(&a).unwrap(), b);
    assert_eq!(PdfDiff::between(&b, &a).apply(&b).unwrap(), a);
    assert!(PdfDiff::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law_across_a_growing_and_shrinking_page_tree() {
    let one_page = snap(vec![page(612.0, 792.0, "a")]);
    let three_pages = snap(vec![page(612.0, 792.0, "a"), page(595.276, 841.89, "b"), page(200.0, 300.0, "")]);
    assert_eq!(PdfDiff::between(&one_page, &three_pages).apply(&one_page).unwrap(), three_pages);
    assert_eq!(PdfDiff::between(&three_pages, &one_page).apply(&three_pages).unwrap(), one_page);
}
//#endregion between_roundtrip_law

//#region inverse_law
#[semio_framework_async_macros::async_test]
async fn inverse_law_diff_level() {
    let a = snap(vec![page(612.0, 792.0, "hello"), page(10.0, 20.0, "second")]);
    let b = snap(vec![page(300.0, 400.0, "world")]);
    let diff = PdfDiff::between(&a, &b);
    let mid = diff.apply(&a).unwrap();
    assert_eq!(mid, b);
    assert_eq!(diff.inverse(&a).apply(&mid).unwrap(), a);
}
//#endregion inverse_law

//#region absorb_law
#[semio_framework_async_macros::async_test]
async fn absorb_law_sequential_composition() {
    let s0 = one(612.0, 792.0, "a");
    let s1 = one(300.0, 792.0, "a");
    let s2 = one(300.0, 400.0, "b");
    let d1 = PdfDiff::between(&s0, &s1);
    let d2 = PdfDiff::between(&s1, &s2);
    let sequential = d2.apply(&d1.apply(&s0).unwrap()).unwrap();
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&s0).unwrap(), sequential);
    assert_eq!(sequential, s2);
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_sequential_composition_over_page_insertion_and_removal() {
    let s0 = snap(vec![page(1.0, 1.0, "a"), page(2.0, 2.0, "b")]);
    let s1 = snap(vec![page(1.0, 1.0, "a"), page(3.0, 3.0, "c"), page(2.0, 2.0, "b")]);
    let s2 = snap(vec![page(1.0, 1.0, "a"), page(3.0, 3.0, "c!")]);
    let d1 = PdfDiff::between(&s0, &s1);
    let d2 = PdfDiff::between(&s1, &s2);
    let sequential = d2.apply(&d1.apply(&s0).unwrap()).unwrap();
    assert_eq!(sequential, s2);
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&s0).unwrap(), sequential);
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_associativity() {
    let s0 = one(1.0, 1.0, "a");
    let s1 = one(2.0, 1.0, "a");
    let s2 = one(2.0, 2.0, "b");
    let s3 = one(3.0, 2.0, "c");
    let d1 = PdfDiff::between(&s0, &s1);
    let d2 = PdfDiff::between(&s1, &s2);
    let d3 = PdfDiff::between(&s2, &s3);
    let mut left = d1.clone();
    left.absorb(d2.clone());
    left.absorb(d3.clone());
    let mut right_tail = d2.clone();
    right_tail.absorb(d3.clone());
    let mut right = d1.clone();
    right.absorb(right_tail);
    assert_eq!(left.apply(&s0).unwrap(), s3);
    assert_eq!(right.apply(&s0).unwrap(), s3);
    assert_eq!(left, right);
}
//#endregion absorb_law

//#region validation
#[semio_framework_async_macros::async_test]
async fn a_removal_of_a_page_the_base_does_not_have_is_refused() {
    let base = one(612.0, 792.0, "a");
    let diff = PdfDiff { pages: Some(PdfPagesDiff { removed: vec![7], ..Default::default() }) };
    assert!(diff.apply(&base).is_err());
}
//#endregion validation

//#region field_sweep
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> PdfSnapshot {
    one(612.0, 792.0, "base text")
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> PdfSnapshot {
    one(300.5, 400.25, "changed text")
}

#[semio_framework_async_macros::async_test]
async fn field_sweep_between_roundtrips_both_directions() {
    let (a, b) = (sweep_a(), sweep_b());
    assert_eq!(PdfDiff::between(&a, &b).apply(&a).unwrap(), b);
    assert_eq!(PdfDiff::between(&b, &a).apply(&b).unwrap(), a);
    assert!(PdfDiff::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn field_sweep_every_field_present_in_diff() {
    let (a, b) = (sweep_a(), sweep_b());
    let diff = PdfDiff::between(&a, &b).pages.expect("the page lane moved");
    let page = &diff.modified.first().expect("page 0 is modified").diff;
    assert_eq!(page.width, Some(300.5));
    assert_eq!(page.height, Some(400.25));
    assert_eq!(page.text, Some("changed text".to_string()));
}
//#endregion field_sweep

//#region diff_codec_text_binary_roundtrip_law
/// 🧪️ `protocol::DiffCodec` LAW — exercises a modified page, an inserted page, a removed page
/// and the fully-empty diff, both text (`print_diff`/`parse_diff`) and binary
/// (`encode_diff`/`decode_diff`) sides.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    use protocol::DiffCodec;
    let (a, b) = (sweep_a(), sweep_b());
    let grown = snap(vec![page(612.0, 792.0, "base text"), page(1.0, 2.0, "added (with parens) and a comma,")]);
    let cases = vec![PdfDiff::between(&a, &b), PdfDiff::between(&a, &grown), PdfDiff::between(&grown, &a), PdfDiff::between(&a, &a)];
    for diff in cases {
        let printed = diff.print_diff();
        assert!(!printed.contains('\n'), "print_diff must not contain a newline: {printed:?}");
        let parsed = PdfDiff::parse_diff(&printed).expect("parse_diff must accept its own print_diff output");
        assert_eq!(parsed, diff, "parse_diff(print_diff(d)) must equal d");

        let encoded = diff.encode_diff().expect("encode_diff must succeed");
        let decoded = PdfDiff::decode_diff(&encoded).expect("decode_diff must accept its own encode_diff output");
        assert_eq!(decoded, diff, "decode_diff(encode_diff(d)) must equal d");
    }
}
//#endregion diff_codec_text_binary_roundtrip_law
