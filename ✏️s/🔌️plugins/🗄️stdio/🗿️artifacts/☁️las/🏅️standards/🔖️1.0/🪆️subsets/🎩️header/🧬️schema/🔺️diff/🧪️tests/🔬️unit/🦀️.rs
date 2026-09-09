use super::*;

#[semio_framework_async_macros::async_test]
async fn invalid_collection_targets_are_rejected_before_mutation() {
    let base = LasSnapshot::default();
    let diff = LasDiff { vlrs: Some(LasVlrsDiff { removed: vec![0], ..Default::default() }), ..Default::default() };
    let error = diff.apply(&base).expect_err("missing VLR target must be rejected");
    assert_eq!(error.code, "invalid-remove-index");
    assert_eq!(error.target, vec!["vlrs", "0"]);
    assert_eq!(base, LasSnapshot::default());
}

/// 🧪️ `DiffCodec` round-trip law for the hand-rolled `LasDiff` text/binary grammar —
/// exercises every header scalar, both `LasPointDiff` tri-states (`gps_time`/`rgb`, both
/// `Some(None)` and `Some(Some(_))` transitions), and both collection triples (`vlrs`/`points`,
/// `removed`/`modified`/`added`) simultaneously via `demo_diff_cases()`'s real `between()`
/// results — the single source of truth also reused by `⚙️engine/🦀️.rs`'s
/// `diff_grammar_conformance_law`/`protocol_walk_law`.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = LasDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = LasDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
