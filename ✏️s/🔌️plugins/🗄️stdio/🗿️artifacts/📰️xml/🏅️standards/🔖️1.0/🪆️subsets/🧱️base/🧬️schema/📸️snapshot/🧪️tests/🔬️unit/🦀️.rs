use super::*;

/// 🧭 Collapses only insignificant spacing outside quoted identifiers in a raw DOCTYPE payload.
fn normalize_reference_doctype_payload(bytes: &[u8]) -> String {
    let source = std::str::from_utf8(bytes).expect("UTF-8 doctype payload");
    let mut normalized = String::new();
    let mut quote = None;
    let mut pending_space = false;
    for character in source.trim().chars() {
        if let Some(delimiter) = quote {
            normalized.push(character);
            if character == delimiter {
                quote = None;
            }
        } else if character == '"' || character == '\'' {
            if pending_space && !normalized.is_empty() {
                normalized.push(' ');
            }
            pending_space = false;
            quote = Some(character);
            normalized.push(character);
        } else if character.is_whitespace() {
            pending_space = true;
        } else {
            if pending_space && !normalized.is_empty() {
                normalized.push(' ');
            }
            pending_space = false;
            normalized.push(character);
        }
    }
    normalized
}

/// 🧭 Reads document boundary order and content with the independent quick-xml parser.
fn reference_document_boundaries(source: &str) -> Vec<(String, String)> {
    use quick_xml::{events::Event, Reader};
    let mut reader = Reader::from_str(source);
    let mut depth = 0usize;
    let mut boundaries = Vec::new();
    loop {
        match reader.read_event().expect("independent XML boundary reader") {
            Event::Start(node) => {
                if depth == 0 {
                    boundaries.push(("root".into(), String::from_utf8(node.name().as_ref().to_vec()).unwrap()));
                }
                depth += 1;
            }
            Event::Empty(node) if depth == 0 => boundaries.push(("root".into(), String::from_utf8(node.name().as_ref().to_vec()).unwrap())),
            Event::End(_) => depth -= 1,
            Event::DocType(node) => boundaries.push(("doctype".into(), normalize_reference_doctype_payload(node.as_ref()))),
            Event::Comment(node) if depth == 0 => boundaries.push(("comment".into(), String::from_utf8(node.as_ref().to_vec()).unwrap())),
            Event::PI(node) if depth == 0 => boundaries.push(("processingInstruction".into(), String::from_utf8(node.as_ref().to_vec()).unwrap())),
            Event::Eof => break,
            _ => {}
        }
    }
    boundaries
}

fn boundary_kind(node: &XmlNode) -> &'static str {
    match node {
        XmlNode::Comment { .. } => "comment",
        XmlNode::ProcessingInstruction { .. } => "processingInstruction",
        XmlNode::Element { .. } => "element",
        XmlNode::Text { .. } => "text",
        XmlNode::CData { .. } => "cData",
    }
}

#[test]
fn neutral_document_boundary_fixture_round_trips_all_doctype_positions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🧭️document-boundaries/🔣️.json")).expect("neutral XML document-boundary fixture");
    for case in fixture["valid"].as_array().expect("valid cases") {
        let source = case["source"].as_str().expect("source");
        let doc = xml_document_from_text(source).unwrap_or_else(|error| panic!("{}: {error}", case["id"]));
        let doctype = doc.doctype.as_ref().expect("doctype");
        assert_eq!(doctype.prolog_position, case["doctypePosition"].as_u64().expect("doctype position"), "{}", case["id"]);
        assert_eq!(doc.prolog.iter().map(boundary_kind).collect::<Vec<_>>(), case["prologKinds"].as_array().expect("prolog kinds").iter().map(|value| value.as_str().expect("kind")).collect::<Vec<_>>(), "{}", case["id"]);
        assert_eq!(doc.epilog.iter().map(boundary_kind).collect::<Vec<_>>(), case["epilogKinds"].as_array().expect("epilog kinds").iter().map(|value| value.as_str().expect("kind")).collect::<Vec<_>>(), "{}", case["id"]);
        let exported = xml_document_to_text_checked(&doc).expect("valid authored document");
        assert_eq!(reference_document_boundaries(&exported), reference_document_boundaries(source), "{} independent boundary ordering and content", case["id"]);
        let reopened = xml_document_from_text(&exported).expect("exported XML reopens");
        assert_eq!(reopened, doc, "{}", case["id"]);
    }
}

#[test]
fn neutral_document_boundary_fixture_rejects_invalid_sources_and_authored_state() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🧭️document-boundaries/🔣️.json")).expect("neutral XML document-boundary fixture");
    for case in fixture["invalid"].as_array().expect("invalid cases") {
        let error = xml_document_from_text(case["source"].as_str().expect("source")).expect_err("invalid XML boundary source");
        assert_eq!(error, case["error"].as_str().expect("error"), "{}", case["id"]);
    }

    let invalid_position = XmlDocument {
        prolog: vec![XmlNode::Comment { text: "before".into() }],
        doctype: Some(XmlDoctype { prolog_position: 2, name: "root".into(), ..Default::default() }),
        root: Some(XmlNode::Element { name: "root".into(), attrs: Vec::new(), children: Vec::new() }),
        ..Default::default()
    };
    assert_eq!(xml_document_to_text_checked(&invalid_position).expect_err("invalid position"), fixture["invalidAuthored"][0]["error"].as_str().expect("position error"));

    let invalid_epilog = XmlDocument { epilog: vec![XmlNode::Text { text: "not misc".into() }], ..Default::default() };
    assert_eq!(xml_document_to_text_checked(&invalid_epilog).expect_err("invalid epilog"), fixture["invalidAuthored"][1]["error"].as_str().expect("epilog error"));
}

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
