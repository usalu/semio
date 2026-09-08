
use super::*;
use protocol::DiffCodec;

/// 🧪️ `DiffCodec` text/binary round-trip laws over every `demo_diff_cases()` fixture (`#region
/// 🔖️DemoCases` above) — the empty diff, a single-collection sparse diff, and the rich case
/// exercising every collection triple (name-keyed AND index-keyed) simultaneously, plus the
/// `Replace` (kind-change) branch of `DxfEntityDiff` and a NON-`Replace` kind-specific patch,
/// plus a nested block-level `entities` sub-diff (the SAME `DxfEntitiesDiff` machinery reused
/// at two tree depths) — shared with `⚙️engine/🦀️.rs`'s own conformance laws.
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must never contain a newline, for {d:?}");
        let parsed = DxfDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e:?}, for {d:?}"));
        assert_eq!(parsed, d, "parse_diff(print_diff(d)) == d");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e:?}, for {d:?}"));
        let decoded = DxfDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e:?}, for {d:?}"));
        assert_eq!(decoded, d, "decode_diff(encode_diff(d)) == d");

        let printed2 = d.print_diff();
        assert_eq!(printed, printed2, "print_diff must be deterministic, for {d:?}");
    }

    assert!(DxfDiff::default().print_diff().is_empty());
    assert_eq!(DxfDiff::parse_diff("").expect("parse empty"), DxfDiff::default());
}
