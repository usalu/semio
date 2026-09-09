use super::*;
use protocol::DiffCodec;

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
    let a = snapshot(Some("DOCTYPE html"), elem("html", vec![("lang", Some("en"))], vec![elem("p", vec![("id", Some("x")), ("disabled", None)], vec![])]));
    let b = snapshot(
        None,
        elem(
            "html",
            vec![("lang", Some("de")), ("data-x", None)],
            vec![
                elem("div", vec![], vec![HtmlNode::Text { text: "hi".into() }, HtmlNode::Comment { text: " c ".into() }]),
                HtmlNode::Element { name: "script".into(), attributes: vec![], children: vec![HtmlNode::RawText { parent_kind: RawTextKind::Script, text: "1+1;".into() }] },
            ],
        ),
    );
    let c = snapshot(None, HtmlNode::Text { text: "root-replaced".into() });

    let cases = vec![HtmlDiff::default(), HtmlDiff::between(&a, &b), HtmlDiff::between(&b, &a), HtmlDiff::between(&a, &c), HtmlDiff::between(&c, &a)];
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
