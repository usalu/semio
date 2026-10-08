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

//#endregion absorb_law

//#region validation
#[semio_framework_async_macros::async_test]
async fn a_removal_of_a_page_the_base_does_not_have_is_refused() {
    let base = one(612.0, 792.0, "a");
    let diff = PdfDiff { pages: Some(PdfPagesDiff { removed: vec![7], ..Default::default() }) };
    assert!(protocol::apply_diff(&diff, &base).is_err());
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

//#endregion field_sweep

//#region diff_codec_text_binary_roundtrip_law
/// 🧪️ `protocol::DiffCodec` LAW — exercises a modified page, an inserted page, a removed page
/// and the fully-empty diff, both text (`print_diff`/`parse_diff`) and binary
/// (`encode_diff`/`decode_diff`) sides.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    use protocol::{DiffBinary,DiffCodec,DiffText};
    let resized = PdfPageDiff { width: Some(300.5), height: Some(400.25), text: Some("changed text".into()), ..Default::default() };
    let cases = vec![
        PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: resized }], ..Default::default() }) },
        PdfDiff { pages: Some(PdfPagesDiff { added: vec![PdfPageAdded { index: 1, page: page(1.0, 2.0, "added (with parens) and a comma,") }], ..Default::default() }) },
        PdfDiff { pages: Some(PdfPagesDiff { removed: vec![1], ..Default::default() }) },
        PdfDiff::default(),
    ];
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
