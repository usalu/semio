use crate::standards::v1::subsets::text::io::text::diff::dec_mark_kind;
use super::*;
use crate::standards::v1::subsets::base::schema::triples::IndexedTripleDiff;
use crate::standards::v1::subsets::text::schema::snapshot::{SemioTextMarkKind, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};
use protocol::{DiffBinary,DiffCodec,DiffText};

fn one_run(content: &str) -> SemioTextSnapshot {
    SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: vec![SemioTextRun { language: "en".into(), content: content.into(), marks: vec![] }] }
}

fn modify_run(index: usize, diff: SemioTextRunDiff) -> SemioTextDiff {
    use crate::standards::v1::subsets::base::schema::triples::IndexModified;
    SemioTextDiff { runs: Some(IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() }) }
}

#[semio_framework_async_macros::async_test]
async fn apply_touches_only_named_run_fields() {
    let base = one_run("a");
    let diff = modify_run(0, SemioTextRunDiff { content: Some("b".into()), ..Default::default() });
    let next = protocol::apply_diff(&diff, &base).expect("apply must succeed for a well-formed fixture");
    assert_eq!(next.runs[0].content, "b");
    assert_eq!(next.runs[0].language, "en", "untouched run fields must be preserved");
}

#[semio_framework_async_macros::async_test]
async fn absorb_equals_sequential_apply() {
    let base = one_run("a");
    let first = modify_run(0, SemioTextRunDiff { content: Some("b".into()), ..Default::default() });
    let second = modify_run(0, SemioTextRunDiff { language: Some("de".into()), content: Some("c".into()), ..Default::default() });
    let mut absorbed = first.clone();
    absorbed.absorb(second.clone());
    let sequential = protocol::apply_diff(&second, &protocol::apply_diff(&first, &base).unwrap()).unwrap();
    assert_eq!(protocol::apply_diff(&absorbed, &base).unwrap(), sequential);
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_base() {
    let base = one_run("a");
    let diff = modify_run(0, SemioTextRunDiff { content: Some("b".into()), ..Default::default() });
    let next = protocol::apply_diff(&diff, &base).unwrap();
    let inverse = protocol::command::DiffAlgebra::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff(&inverse, &next).unwrap(), base);
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
