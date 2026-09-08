
use super::*;
use protocol::DiffCodec;

/// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `XmlDiff` grammar — exercises the
/// recursive enum tree (`Element`/`Text`/`Replace` `XmlNodeDiff` variants), both top-level
/// tri-states, attribute add/remove/modify, and nested child add/remove/modify. Reuses
/// `demo_diff_cases()` (the single prolog of truth also consumed by
/// `⚙️engine/🦀️.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`).
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = XmlDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = XmlDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
