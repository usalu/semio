
use super::*;
use crate::standards::v1::subsets::text::schema::snapshot::{STDIO_SEMIOTEXT_DOCUMENT_SCHEMA, SemioTextMarkKind};
use protocol::DiffCodec;

#[semio_framework_async_macros::async_test]
async fn apply_replaces_runs_wholesale() {
    let base = SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: vec![SemioTextRun { language: "en".into(), content: "a".into(), marks: vec![] }] };
    let diff = SemioTextDiff { runs: Some(SemioTextRunList { values: vec![SemioTextRun { language: "en".into(), content: "b".into(), marks: vec![] }] }) };
    let next = diff.apply(&base).expect("apply must succeed for a well-formed fixture");
    assert_eq!(next.runs[0].content, "b");
}

#[semio_framework_async_macros::async_test]
async fn absorb_last_write_wins() {
    let mut d1 = SemioTextDiff { runs: Some(SemioTextRunList { values: vec![SemioTextRun { language: "en".into(), content: "a".into(), marks: vec![] }] }) };
    let d2 = SemioTextDiff { runs: Some(SemioTextRunList { values: vec![SemioTextRun { language: "en".into(), content: "b".into(), marks: vec![] }] }) };
    d1.absorb(d2.clone());
    assert_eq!(d1, d2);
}

#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioTextDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioTextDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}

#[semio_framework_async_macros::async_test]
async fn mark_kind_helper_smoke() {
    assert_eq!(dec_mark_kind("l").unwrap(), SemioTextMarkKind::Link);
}
