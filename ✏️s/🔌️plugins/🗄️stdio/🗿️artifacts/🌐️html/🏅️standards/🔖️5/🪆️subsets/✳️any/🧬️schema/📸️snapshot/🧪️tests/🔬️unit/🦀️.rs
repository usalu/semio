use super::*;

const FIXTURE: &str = include_str!("../../../../📚️examples/🎬️demo/🖼️assets/🧪️example/🌐️.html");

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn el(name: &str, attrs: Vec<HtmlAttr>, children: Vec<HtmlNode>) -> HtmlNode {
    HtmlNode::Element { name: name.into(), attributes: attrs, children }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn text(s: &str) -> HtmlNode {
    HtmlNode::Text { text: s.into() }
}

/// 🧪 Pins WHATWG §13.2.6.4.3 "before head": a whitespace-only character token before `<head>`
/// is IGNORED, so `<html>`'s first child is the `<head>` element itself. Without this the whole
/// path space inside `<html>` is shifted one place away from every conformant HTML5 DOM.
#[test]
fn whitespace_before_head_is_not_a_node() {
    let snap = parse_html_document("<html lang=\"de\">\n  <head></head>\n  <body></body>\n</html>").unwrap();
    let HtmlNode::Element { children, .. } = &snap.root else { panic!("root not element") };
    assert!(matches!(&children[0], HtmlNode::Element { name, .. } if name == "head"), "got {:?}", children[0]);
    assert!(matches!(&children[1], HtmlNode::Text { text } if text == "\n  "));
    assert!(matches!(&children[2], HtmlNode::Element { name, .. } if name == "body"), "got {:?}", children[2]);
    assert_eq!(children.len(), 3, "nothing may follow <body>: {children:?}");
}

/// 🧪 Pins WHATWG §13.2.6.4.20 "after body" and §13.2.6.4.22 "after after body": whitespace
/// between `</body>` and `</html>` AND after `</html>` is processed with the "in body" rules,
/// whose insertion point is still `body`, and merges onto `body`'s last text node rather than
/// becoming a child of `html` or being discarded.
#[test]
fn whitespace_after_body_belongs_to_body() {
    let snap = parse_html_document("<html><head></head><body><p>x</p>\n  </body>\n</html>\n").unwrap();
    let HtmlNode::Element { children, .. } = &snap.root else { panic!("root not element") };
    assert_eq!(children.len(), 2);
    let HtmlNode::Element { children: body, .. } = &children[1] else { panic!("body not element") };
    assert_eq!(body.len(), 2);
    assert!(matches!(&body[1], HtmlNode::Text { text } if text == "\n  \n\n"), "got {:?}", body[1]);
}

/// 🧪 Nothing may follow `</html>` on the way out: whatever came after it on the way in is
/// already inside `body`, so a courtesy trailing newline would be a NEW character the next
/// conformant read puts inside `body` again. `write` after `parse` is a true fixpoint.
#[test]
fn writing_never_emits_anything_after_the_root_element() {
    let source = "<!DOCTYPE html>\n<html><head></head><body><p>x</p>\n</body></html>";
    let printed = write_html_document(&parse_html_document(source).unwrap());
    assert_eq!(printed, source);
    assert_eq!(write_html_document(&parse_html_document(&printed).unwrap()), printed);
}

/// 🧪 The normalization is scoped to an `<html>` root: a fragment-shaped document keeps every
/// text node exactly where the source put it.
#[test]
fn non_html_root_keeps_its_whitespace_verbatim() {
    let snap = parse_html_document("<section>\n  <p>x</p>\n</section>").unwrap();
    let HtmlNode::Element { children, .. } = &snap.root else { panic!("root not element") };
    assert_eq!(children.len(), 3);
    assert!(matches!(&children[0], HtmlNode::Text { text } if text == "\n  "));
}

#[semio_framework_async_macros::async_test]
async fn parses_void_elements_without_children_or_close_tag() {
    let snap = parse_html_document("<html><br><img src=\"x.png\"></html>").unwrap();
    match &snap.root {
        HtmlNode::Element { children, .. } => {
            assert_eq!(children.len(), 2);
            assert!(matches!(&children[0], HtmlNode::Element { name, children, .. } if name == "br" && children.is_empty()));
            assert!(matches!(&children[1], HtmlNode::Element { name, children, .. } if name == "img" && children.is_empty()));
        }
        other => panic!("expected element root, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn parses_valueless_boolean_attribute() {
    let snap = parse_html_document("<html><p disabled>hi</p></html>").unwrap();
    match &snap.root {
        HtmlNode::Element { children, .. } => match &children[0] {
            HtmlNode::Element { attributes, .. } => {
                assert_eq!(attributes.len(), 1);
                assert_eq!(attributes[0].name, "disabled");
                assert_eq!(attributes[0].value, None);
            }
            other => panic!("expected element, got {other:?}"),
        },
        other => panic!("expected element root, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn parses_comment_and_script_style_rawtext() {
    let snap = parse_html_document("<html><!-- hi --><style>.a { color: red; }</style><script>if (1 < 2) { console.log(\"</not-a-tag>\"); }</script></html>").unwrap();
    match &snap.root {
        HtmlNode::Element { children, .. } => {
            assert!(matches!(&children[0], HtmlNode::Comment { text } if text == " hi "));
            match &children[1] {
                HtmlNode::Element { name, children, .. } => {
                    assert_eq!(name, "style");
                    assert!(matches!(&children[0], HtmlNode::RawText { parent_kind: RawTextKind::Style, text } if text == ".a { color: red; }"));
                }
                other => panic!("expected style element, got {other:?}"),
            }
            match &children[2] {
                HtmlNode::Element { name, children, .. } => {
                    assert_eq!(name, "script");
                    assert!(matches!(&children[0], HtmlNode::RawText { parent_kind: RawTextKind::Script, text } if text.contains("</not-a-tag>")));
                }
                other => panic!("expected script element, got {other:?}"),
            }
        }
        other => panic!("expected element root, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn rejects_mismatched_close_tag() {
    assert!(parse_html_document("<html><div></span></html>").is_err());
}

#[semio_framework_async_macros::async_test]
async fn rejects_self_closing_syntax_on_non_void_element() {
    assert!(parse_html_document("<html><div/></html>").is_err());
}

#[semio_framework_async_macros::async_test]
async fn accepts_self_closing_syntax_on_void_element() {
    let snap = parse_html_document("<html><br/></html>").unwrap();
    match &snap.root {
        HtmlNode::Element { children, .. } => assert!(matches!(&children[0], HtmlNode::Element { name, .. } if name == "br")),
        other => panic!("expected element root, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn decodes_and_encodes_small_entity_subset_only() {
    let snap = parse_html_document("<html><p>a &amp; b &lt;3 &#65; &#x42; &nbsp;</p></html>").unwrap();
    match &snap.root {
        HtmlNode::Element { children, .. } => match &children[0] {
            HtmlNode::Element { children, .. } => {
                assert!(matches!(&children[0], HtmlNode::Text { text } if text == "a & b <3 A B &nbsp;"));
            }
            other => panic!("expected p element, got {other:?}"),
        },
        other => panic!("expected element root, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn empty_default_snapshot_has_html_root() {
    let snap = HtmlSnapshot::default();
    assert!(matches!(&snap.root, HtmlNode::Element { name, .. } if name == "html"));
}

#[semio_framework_async_macros::async_test]
async fn nested_structure_round_trips_synthetic() {
    let snap = HtmlSnapshot {
        schema: STDIO_HTML_DOCUMENT_SCHEMA.into(),
        doctype: Some("DOCTYPE html".into()),
        root: el("html", vec![HtmlAttr::new("lang", "en")], vec![el("body", vec![], vec![el("p", vec![HtmlAttr::boolean("disabled")], vec![text("hi "), el("br", vec![], vec![]), text(" there")])])]),
    };
    let printed = write_html_document(&snap);
    let reparsed = parse_html_document(&printed).unwrap();
    assert_eq!(reparsed, snap);
}

//#region 🔖️CodecRetentionLaw
/// 🎯️ codec_retention_law: byte-preserving round trip of the real W0 fixture. Exact, not just
/// "documented-honest normalization" — the fixture's actual bytes follow this codec's canonical
/// top-level-whitespace convention (a single `\n` after the doctype, `<head>` opening
/// immediately after `<html …>` and `</html>` closing immediately after `</body>`, because
/// WHATWG tree construction gives whitespace in those two places to nothing and to `body`
/// respectively — see [`normalize_html_root_whitespace`]), and every attribute value is already
/// double-quoted, so `decode -> re-encode` matches the source byte-for-byte.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = parse_html_document(FIXTURE).expect("fixture parses");
    let re_encoded = write_html_document(&snap);
    assert_eq!(re_encoded, FIXTURE, "fixture must round-trip byte-for-byte");

    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <HtmlSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}
//#endregion 🔖️CodecRetentionLaw

#[semio_framework_async_macros::async_test]
async fn snapshot_dsl_and_pack_round_trip() {
    let snap = parse_html_document(FIXTURE).expect("fixture parses");
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <HtmlSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed, snap);
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <HtmlSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}
