use super::*;

/// 🧪️ F6/P2-FG3: `DiffCodec` round-trip laws for the hand-rolled `PlyDiff` text AND (now
/// real, no longer text-as-bytes) binary grammar — `demo_diff_cases()` exercises every
/// scalar field, the name-keyed `elements` triple in ALL THREE flavors (removed/modified/
/// added) simultaneously via a real `between()` result in both directions, the nested
/// index-keyed `rows` triple, the weak `properties` replace, and both `PlyProperty`/
/// `PlyValue` enum tag families (incl. `PlyValue::List`'s recursion).
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = PlyDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = PlyDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
