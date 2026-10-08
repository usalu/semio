use super::*;
use protocol::{DiffBinary,DiffCodec,DiffText};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn elem(name: &str, attrs: Vec<(&str, Option<&str>)>, children: Vec<HtmlNode>) -> HtmlNode {
    HtmlNode::Element { name: name.to_string(), attributes: attrs.into_iter().map(|(n, v)| HtmlAttr { name: n.to_string(), value: v.map(|s| s.to_string()) }).collect(), children }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot(doctype: Option<&str>, root: HtmlNode) -> HtmlSnapshot {
    HtmlSnapshot { schema: crate::standards::v5::subsets::any::schema::snapshot::STDIO_HTML_DOCUMENT_SCHEMA.into(), doctype: doctype.map(|s| s.to_string()), root }
}

/// 🧪️ diff_codec_text_binary_roundtrip_law: exercises the recursive enum tree (`Element`/
/// `Text`/`Comment`/`RawText`/`Replace` `HtmlNodeDiff` variants), the top-level tri-state, and
/// nested attribute/child add/remove/modify.
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    let cases = demo_diff_cases();
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = HtmlDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = HtmlDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
