use super::*;
use protocol::{DiffBinary,DiffCodec,DiffText};

/// 🧪️ `DiffCodec` round-trip law over the hand-rolled `SemioPresentationDiff` grammar —
/// exercises masters/layouts (named-keyed removed/modified/added), slides (index-keyed
/// removed/modified/added incl. nested shape + `DocBlock`-reuse changes), and the `layout_id`
/// tri-state, in both directions plus the empty/self cases.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let cases = demo_diff_cases();
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioPresentationDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioPresentationDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }

}
