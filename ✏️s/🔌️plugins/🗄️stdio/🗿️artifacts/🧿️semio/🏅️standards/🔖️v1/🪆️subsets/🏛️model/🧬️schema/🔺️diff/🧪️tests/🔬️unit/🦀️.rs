use super::*;

/// 🧪️ diff_codec_text_binary_roundtrip_law: hand-rolled `DiffCodec` text+binary round trip.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let d = demo_diff_cases().swap_remove(1);

    let printed = d.print_diff();
    assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
    let parsed = SemioModelDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
    assert_eq!(parsed, d);

    let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
    let decoded = SemioModelDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
    assert_eq!(decoded, d);

    // Empty diff also round-trips (the common "no change" case every artifact's codec hits).
    let empty = SemioModelDiff::default();
    assert_eq!(empty.print_diff(), "");
    assert_eq!(SemioModelDiff::parse_diff("").unwrap(), empty);
}
