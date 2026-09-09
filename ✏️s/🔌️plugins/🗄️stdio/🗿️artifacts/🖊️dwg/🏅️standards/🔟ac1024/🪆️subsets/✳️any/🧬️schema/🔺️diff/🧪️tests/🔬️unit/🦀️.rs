use super::*;

/// 🧪️ Logical diff text and binary codecs retain every field.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    use protocol::DiffCodec;
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = DwgDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?} (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff({d:?}) failed: {e}"));
        let decoded = DwgDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
    }
}
