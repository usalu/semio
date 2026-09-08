
use super::*;

//#region 🔖️EscapeAttrRoundTrip
/// 🧪 Direct proof that [`xml_escape_attr`] escapes all three characters attribute-value
/// normalization (XML 1.0 §3.3.3) would otherwise silently fold to a single space on the next
/// parse.
#[test]
fn xml_escape_attr_escapes_tab_newline_and_carriage_return() {
    assert_eq!(xml_escape_attr("\t\n\r"), "&#9;&#10;&#13;");
    assert_eq!(xml_escape_attr("a\tb\nc\rd"), "a&#9;b&#10;c&#13;d");
}

/// 🧪 [`xml_escape_text`] is deliberately narrower: only `\r` needs re-escaping (line-break
/// normalization per §2.11) -- a literal tab or `\n` is legal text content and must round-trip
/// untouched.
#[test]
fn xml_escape_text_only_escapes_carriage_return() {
    assert_eq!(xml_escape_text("\t\n\r"), "\t\n&#13;");
}

/// 🧪 Decoding `&#9;`/`&#10;`/`&#13;` inside an attribute value, then re-encoding the document,
/// must re-emit the SAME character references -- writing the raw byte instead would silently
/// change the value's meaning on the next parse (attribute-value normalization folds a literal
/// tab/newline/CR to a single space, but a character reference is exempt from that step).
#[test]
fn attribute_value_decode_then_encode_round_trips_control_characters() {
    let source = "<root a=\"x&#9;y&#10;z&#13;w\"/>";
    let doc = xml_document_from_text(source).expect("valid document");
    let value = match doc.root.as_ref().expect("root") {
        XmlNode::Element { attrs, .. } => attrs[0].value.clone(),
        _ => panic!("expected element root"),
    };
    assert_eq!(value, "x\ty\nz\rw", "decode must yield the literal control characters");

    let re_encoded = xml_document_to_text(&doc);
    assert!(re_encoded.contains("x&#9;y&#10;z&#13;w"), "re-encode must re-escape all three as character references, got: {re_encoded}");
    assert!(!re_encoded.contains("x\ty"), "re-encode must not leave a literal tab in the attribute value");
    assert!(!re_encoded.contains("y\nz"), "re-encode must not leave a literal newline in the attribute value");
    assert!(!re_encoded.contains("z\rw"), "re-encode must not leave a literal carriage return in the attribute value");

    let reparsed = xml_document_from_text(&re_encoded).expect("valid re-encoded document");
    let reparsed_value = match reparsed.root.as_ref().expect("root") {
        XmlNode::Element { attrs, .. } => attrs[0].value.clone(),
        _ => panic!("expected element root"),
    };
    assert_eq!(reparsed_value, value, "decode -> encode -> decode must be a fixed point");
}
//#endregion 🔖️EscapeAttrRoundTrip

//#region 🔖️RealFixtureRoundTrip
/// 📎 The real, committed QR-code SVG fixture (svg subset's `qr-code.svg`) -- its `<image>`
/// element carries a real ~7.3 KB base64 `xlink:href`, folded across lines with 95 literal
/// `&#10;` character references. This is the exact real-world input that exposed the missing
/// re-escape (see the Wave 7 ticket finding on `xml_escape_attr`). The `xml` and `svg` subsets
/// share this codec, so this is a genuine regression case, not a synthetic one.
const REAL_QR_CODE_SVG: &str = include_str!("../../../../../../../../../🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧫️fixtures/🔳️qr-code.svg");

// 🚫️async: E1 pure test helper (file verified I/O-free) — see R9
fn find_xlink_href(node: &XmlNode) -> Option<&str> {
    match node {
        XmlNode::Element { name, attrs, children } => {
            if name == "image" {
                if let Some(attr) = attrs.iter().find(|a| a.name == "xlink:href") {
                    return Some(attr.value.as_str());
                }
            }
            children.iter().find_map(find_xlink_href)
        }
        _ => None,
    }
}

#[test]
fn real_svg_xlink_href_survives_decode_encode_decode() {
    let doc = xml_document_from_text(REAL_QR_CODE_SVG).expect("real qr-code.svg parses");
    let original_href = find_xlink_href(doc.root.as_ref().expect("root")).expect("real <image> xlink:href").to_string();
    assert!(original_href.contains('\n'), "decoded value must contain the literal newlines the &#10; refs decoded to");
    assert_eq!(original_href.chars().count(), 7301, "matches the ticket's documented real xlink:href length");

    let re_encoded = xml_document_to_text(&doc);
    let reparsed = xml_document_from_text(&re_encoded).expect("re-encoded document parses");
    let reparsed_href = find_xlink_href(reparsed.root.as_ref().expect("root")).expect("re-encoded <image> xlink:href");
    assert_eq!(reparsed_href, original_href, "decode -> encode -> decode must preserve the xlink:href byte-for-byte");

    let escaped_newlines = re_encoded.matches("&#10;").count();
    assert!(escaped_newlines >= 95, "re-encode must re-escape every decoded newline as &#10;, found {escaped_newlines}");
}
//#endregion 🔖️RealFixtureRoundTrip
