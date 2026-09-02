//! quick-xml-svg-codec — standalone SVG 1.1 declaration/doctype/element-tree codec on top of
//! `quick-xml` 0.42's own generic streaming XML reader/writer (`quick_xml::Reader`/`Writer`).
//! Zero dependencies beyond `quick-xml` itself (see this crate's own Cargo.toml — its own
//! `[workspace]`, isolated from the repository's root workspace and Cargo.lock).
//!
//! This binary is the READER-oracle counterpart the repository's own SVG 1.1/base oracle
//! registration (`../../🔣️oracle.json`, oracle id `quick-xml-svg-1-1-mutate-reader`) needs.
//! It is DELIBERATELY independent of, and shares no code with, this subset's own
//! `🦀️oracle.rs` (the `cross-semio-implementation` oracle that COMPUTES what a
//! mutation should produce): every type, parser and writer below is a fresh implementation built
//! directly against quick-xml's own event API, never imported from that module. It is also
//! independent of the sibling 📰xml 1.0 `✳️base` subset's own `quick-xml`-backed oracle-probe
//! crate — same underlying library, disjoint code, per this subset's own header docstring.
//!
//! quick-xml has NO knowledge of SVG semantics: it has never heard of `viewBox` or `transform`
//! grammar. So unlike `component.rs`'s own decomposed numeric geometry projection, THIS reader
//! projects `viewBox`/`transform` as opaque attribute STRING values, same as every other
//! attribute. That is still an honest, witnessing comparison — a `viewBox` or `transform` value
//! that changed is a string that differs — it is simply not a SEMANTIC one. See this crate's own
//! sibling `🔬️probes/📜️script.ts` header and `../../🔣️oracle.json`'s
//! `svg-1-1-quick-xml-reader-v1` comparisonProfile for exactly this scoping.
//!
//! Three subcommands:
//!   build   <recipe-id> <out-dir>   — writes <out-dir>/<recipe-id>/before.svg [and after.svg]
//!   project <path-to-svg>           — decodes a real SVG/XML file and prints a typed JSON
//!                                     projection on stdout (declaration, doctype, and the full
//!                                     element/text/cdata/comment/pi tree with attributes in
//!                                     SOURCE order — the profile-level name-sort, if any, is the
//!                                     caller's job, never this binary's)
//!   list-recipes                    — prints every declared recipe id, one per line
//!
//! Every recipe's BEFORE and AFTER document is authored directly as typed Rust values below —
//! never by executing this repository's own `SvgMutation`/`SvgDiff` dispatch — then handed whole
//! to `quick_xml::Writer` to become real bytes. Attribute order is ALWAYS an explicit `Vec` built
//! in a fixed literal order (never a `HashMap`), so `build` is byte-reproducible: `quick_xml`'s
//! writer preserves insertion order verbatim, and re-running `build` for the same recipe-id
//! iterates the exact same `Vec` in the exact same order every time.

use quick_xml::events::{BytesCData, BytesDecl, BytesPI, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use std::borrow::Cow;
use std::env;
use std::fs;
use std::path::Path;

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq)]
enum QNode {
    Element { name: String, attrs: Vec<(String, String)>, children: Vec<QNode> },
    Text(String),
    CData(String),
    Comment(String),
    Pi { target: String, data: String },
}

#[derive(Clone, Debug, PartialEq)]
struct QDecl {
    version: String,
    encoding: Option<String>,
    standalone: Option<bool>,
}

#[derive(Clone, Debug, Default)]
struct QDoc {
    declaration: Option<QDecl>,
    doctype: Option<String>,
    root: Option<QNode>,
}
//#endregion 🔖️Types

//#region 🔖️NodePath
/// 🧭 Child-index chain from the root element — mirrors the production `NodePath` addressing
/// scheme (`crate::artifacts::svg::schema::snapshot::NodePath`), reimplemented here against
/// `QDoc` rather than imported. `path == []` addresses the root itself.
fn q_node_at_mut<'a>(doc: &'a mut QDoc, path: &[usize]) -> &'a mut QNode {
    let mut node = doc.root.as_mut().expect("recipe document has no root element");
    for &index in path {
        match node {
            QNode::Element { children, .. } => node = children.get_mut(index).unwrap_or_else(|| panic!("child index {index} out of range")),
            _ => panic!("path descends into a non-element node"),
        }
    }
    node
}

/// 🏷️ Update-in-place when present (so an untouched attribute keeps its source position), append
/// when new — same shape as production `set_element_attr`/this subset's own oracle `q_set_attr`.
fn q_set_attr(node: &mut QNode, name: &str, value: &str) {
    if let QNode::Element { attrs, .. } = node {
        match attrs.iter_mut().find(|(key, _)| key == name) {
            Some(entry) => entry.1 = value.to_string(),
            None => attrs.push((name.to_string(), value.to_string())),
        }
    }
}
//#endregion 🔖️NodePath

//#region 🔖️Parse
/// 📥️ Builds a [`QDoc`] from real SVG/XML bytes via `quick_xml::Reader`'s zero-copy event
/// stream. Same entity-handling shape as this subset's own oracle `parse_svg` (independent
/// reimplementation, not shared code): `Event::Text` bodies arrive with entity/character
/// references split into separate `Event::GeneralRef` events under quick-xml 0.42, so runs of
/// `Text`+`GeneralRef` between two structural events are coalesced into ONE `QNode::Text`.
fn parse_svg(bytes: &[u8]) -> QDoc {
    let text = std::str::from_utf8(bytes).expect("svg source is not UTF-8");
    let mut reader = Reader::from_str(text);
    let mut doc = QDoc::default();
    let mut stack: Vec<(String, Vec<(String, String)>, Vec<QNode>)> = Vec::new();
    let mut text_buf = String::new();

    fn flush(text_buf: &mut String, stack: &mut [(String, Vec<(String, String)>, Vec<QNode>)]) {
        if text_buf.is_empty() {
            return;
        }
        if let Some((_, _, children)) = stack.last_mut() {
            children.push(QNode::Text(std::mem::take(text_buf)));
        } else {
            text_buf.clear();
        }
    }

    fn attach(stack: &mut Vec<(String, Vec<(String, String)>, Vec<QNode>)>, doc: &mut QDoc, node: QNode) {
        if let Some((_, _, children)) = stack.last_mut() {
            children.push(node);
        } else if doc.root.is_none() {
            doc.root = Some(node);
        }
    }

    fn read_attrs(start: &BytesStart) -> Vec<(String, String)> {
        let mut attrs = Vec::new();
        for attr in start.attributes() {
            let attr = attr.expect("read one xml attribute");
            let value = attr.unescape_value().expect("unescape attribute value").into_owned();
            attrs.push((attr.key.as_ref().to_string(), value));
        }
        attrs
    }

    loop {
        match reader.read_event().expect("read one xml event") {
            Event::Decl(decl) => {
                let version = decl.version().expect("read xml decl version").into_owned();
                let encoding = decl.encoding().transpose().expect("read xml decl encoding").map(Cow::into_owned);
                let standalone = decl.standalone().transpose().expect("read xml decl standalone").map(|c| c.as_ref() == "yes");
                doc.declaration = Some(QDecl { version, encoding, standalone });
            }
            // 📜️ `BytesText::as_ref()` (`AsRef<str>`) — quick-xml 0.42's DocType event carries the
            // raw content between `<!DOCTYPE` and `>` verbatim, never entity-decoded.
            Event::DocType(raw) => doc.doctype = Some(raw.as_ref().to_string()),
            Event::PI(pi) => {
                flush(&mut text_buf, &mut stack);
                attach(&mut stack, &mut doc, QNode::Pi { target: pi.target().to_string(), data: pi.content().to_string() });
            }
            Event::Comment(text) => {
                flush(&mut text_buf, &mut stack);
                attach(&mut stack, &mut doc, QNode::Comment(text.as_ref().to_string()));
            }
            Event::CData(text) => {
                flush(&mut text_buf, &mut stack);
                if let Some((_, _, children)) = stack.last_mut() {
                    children.push(QNode::CData(text.as_ref().to_string()));
                }
            }
            // 📜️ No `BytesText::unescape()` exists in quick-xml 0.42 — entity/character references
            // arrive as SEPARATE `Event::GeneralRef` events (see below), so a bare `Text` event's own
            // content is already the final, un-escaped run and is taken as-is.
            Event::Text(text) => text_buf.push_str(text.as_ref()),
            Event::GeneralRef(reference) => match reference.resolve_char_ref().expect("resolve character reference") {
                Some(ch) => text_buf.push(ch),
                None => match quick_xml::escape::resolve_predefined_entity(reference.as_ref()) {
                    Some(resolved) => text_buf.push_str(resolved),
                    None => panic!("svg source references unknown entity &{};", reference.as_ref()),
                },
            },
            Event::Start(start) => {
                flush(&mut text_buf, &mut stack);
                let attrs = read_attrs(&start);
                stack.push((start.name().as_ref().to_string(), attrs, Vec::new()));
            }
            Event::Empty(start) => {
                flush(&mut text_buf, &mut stack);
                let attrs = read_attrs(&start);
                attach(&mut stack, &mut doc, QNode::Element { name: start.name().as_ref().to_string(), attrs, children: Vec::new() });
            }
            Event::End(_) => {
                flush(&mut text_buf, &mut stack);
                let (name, attrs, children) = stack.pop().expect("svg source: unmatched closing tag");
                attach(&mut stack, &mut doc, QNode::Element { name, attrs, children });
            }
            Event::Eof => {
                flush(&mut text_buf, &mut stack);
                break;
            }
        }
    }
    assert!(doc.root.is_some(), "svg document requires root element");
    doc
}
//#endregion 🔖️Parse

//#region 🔖️Write
/// 📤️ Re-serializes a [`QDoc`] via `quick_xml::Writer`. Attribute order is exactly the source
/// `Vec`'s own order — `quick_xml`'s writer never reorders — which is what makes `build`
/// byte-reproducible (see this file's own header). An empty element is always written
/// `Event::Empty`: self-closing vs explicit empty open/close is never distinguished on READ
/// either (both parse to zero children), so there is no source form to preserve.
fn write_node(writer: &mut Writer<Vec<u8>>, node: &QNode) {
    match node {
        QNode::Text(text) => writer.write_event(Event::Text(BytesText::new(text))).expect("write text event"),
        QNode::CData(text) => writer.write_event(Event::CData(BytesCData::new(text.as_str()))).expect("write cdata event"),
        QNode::Comment(text) => writer.write_event(Event::Comment(BytesText::from_escaped(text.as_str()))).expect("write comment event"),
        QNode::Pi { target, data } => {
            let content = if data.is_empty() { target.clone() } else { format!("{target} {data}") };
            writer.write_event(Event::PI(BytesPI::new(content))).expect("write pi event")
        }
        QNode::Element { name, attrs, children } => {
            let mut start = BytesStart::new(name.as_str());
            for (key, value) in attrs {
                start.push_attribute((key.as_str(), value.as_str()));
            }
            if children.is_empty() {
                writer.write_event(Event::Empty(start)).expect("write empty element event");
                return;
            }
            let end = start.to_end().into_owned();
            writer.write_event(Event::Start(start)).expect("write start element event");
            for child in children {
                write_node(writer, child);
            }
            writer.write_event(Event::End(end)).expect("write end element event");
        }
    }
}

fn write_svg(doc: &QDoc) -> Vec<u8> {
    let mut writer = Writer::new(Vec::new());
    if let Some(decl) = &doc.declaration {
        let standalone = decl.standalone.map(|value| if value { "yes" } else { "no" });
        writer.write_event(Event::Decl(BytesDecl::new(&decl.version, decl.encoding.as_deref(), standalone))).expect("write xml declaration");
    }
    if let Some(raw) = &doc.doctype {
        writer.write_event(Event::DocType(BytesText::from_escaped(raw.as_str()))).expect("write doctype");
    }
    if let Some(root) = &doc.root {
        write_node(&mut writer, root);
    }
    writer.into_inner()
}
//#endregion 🔖️Write

//#region 🔖️Json
fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn node_json(node: &QNode) -> String {
    match node {
        QNode::Text(text) => format!("{{\"kind\":\"text\",\"text\":{}}}", json_str(text)),
        QNode::CData(text) => format!("{{\"kind\":\"cdata\",\"text\":{}}}", json_str(text)),
        QNode::Comment(text) => format!("{{\"kind\":\"comment\",\"text\":{}}}", json_str(text)),
        QNode::Pi { target, data } => format!("{{\"kind\":\"pi\",\"target\":{},\"data\":{}}}", json_str(target), json_str(data)),
        QNode::Element { name, attrs, children } => {
            let attrs_json: Vec<String> = attrs.iter().map(|(key, value)| format!("{{\"name\":{},\"value\":{}}}", json_str(key), json_str(value))).collect();
            let children_json: Vec<String> = children.iter().map(node_json).collect();
            format!("{{\"kind\":\"element\",\"name\":{},\"attrs\":[{}],\"children\":[{}]}}", json_str(name), attrs_json.join(","), children_json.join(","))
        }
    }
}

fn doc_json(doc: &QDoc) -> String {
    let declaration = match &doc.declaration {
        Some(decl) => format!(
            "{{\"present\":true,\"version\":{},\"encoding\":{},\"standalone\":{}}}",
            json_str(&decl.version),
            decl.encoding.as_deref().map(json_str).unwrap_or_else(|| "null".to_string()),
            decl.standalone.map(|b| b.to_string()).unwrap_or_else(|| "null".to_string())
        ),
        None => "{\"present\":false}".to_string(),
    };
    let doctype = match &doc.doctype {
        Some(raw) => format!("{{\"present\":true,\"raw\":{}}}", json_str(raw)),
        None => "{\"present\":false}".to_string(),
    };
    let root = doc.root.as_ref().map(node_json).unwrap_or_else(|| "null".to_string());
    format!("{{\"declaration\":{declaration},\"doctype\":{doctype},\"root\":{root}}}")
}
//#endregion 🔖️Json

//#region 🔖️BaseDocument
/// 🧬 The shared starting document every recipe clones from — an SVG 1.1 declaration + doctype,
/// root `<svg>` with a `viewBox`, one `<g transform=…>` holding a `<rect>` and a `<text>` child —
/// big enough to exercise all 9 declared mutation kinds meaningfully (an element to rename, an
/// attribute to add, a text node to edit, a child to insert/remove, root `viewBox` and `<g>`
/// `transform` to rewrite).
fn base_doc() -> QDoc {
    let rect = QNode::Element {
        name: "rect".into(),
        attrs: vec![("id".into(), "rect1".into()), ("x".into(), "0".into()), ("y".into(), "0".into()), ("width".into(), "10".into()), ("height".into(), "10".into())],
        children: vec![],
    };
    let text = QNode::Element { name: "text".into(), attrs: vec![("id".into(), "text1".into())], children: vec![QNode::Text("Hello".into())] };
    let group = QNode::Element { name: "g".into(), attrs: vec![("id".into(), "group1".into()), ("transform".into(), "translate(10,20)".into())], children: vec![rect, text] };
    let root = QNode::Element {
        name: "svg".into(),
        attrs: vec![("xmlns".into(), "http://www.w3.org/2000/svg".into()), ("viewBox".into(), "0 0 100 100".into()), ("width".into(), "100".into()), ("height".into(), "100".into())],
        children: vec![group],
    };
    QDoc {
        declaration: Some(QDecl { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: None }),
        // 📜️ No leading space: `quick_xml::Writer` inserts `"<!DOCTYPE "` (trailing space already
        // included) around this content, and the reader hands the content back the same way — see
        // this crate's own round-trip test.
        doctype: Some("svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\"".into()),
        root: Some(root),
    }
}
//#endregion 🔖️BaseDocument

//#region 🔖️Recipes
/// 🧪 One recipe: BEFORE always, AFTER always (every one of the 9 declared kinds is registered
/// `outcomes: ["applied"]` only in `../../🔣️oracle.json` — production's own `diff()` for every
/// `SvgMutation` leaf unconditionally returns `MutationOutcome::new(..)`, never `empty`/`error`/
/// `fatal`, per each leaf's own `../../🧬️schema/🧬️mutations/✏️<kind>/🦀️.rs` — so there is no
/// `-rejected-*` recipe to author here). Every AFTER state below touches EXACTLY the field the
/// real `SvgMutation` leaf's own `diff()` touches for that kind — never more.
fn recipe(id: &str) -> Option<(QDoc, QDoc)> {
    let base = base_doc();
    match id {
        "set-declaration-applied" => {
            let mut after = base.clone();
            after.declaration = Some(QDecl { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(false) });
            Some((base, after))
        }
        "set-doctype-applied" => {
            let mut after = base.clone();
            after.doctype = Some("svg SYSTEM \"svg11-flat.dtd\"".into());
            Some((base, after))
        }
        // 🧬 InsertElement{parent:[0], index:2, node:<circle>} — appended as the 3rd child of <g>.
        "insert-element-applied" => {
            let mut after = base.clone();
            let g = q_node_at_mut(&mut after, &[0]);
            if let QNode::Element { children, .. } = g {
                children.push(QNode::Element { name: "circle".into(), attrs: vec![("id".into(), "circle1".into()), ("cx".into(), "5".into()), ("cy".into(), "5".into()), ("r".into(), "2".into())], children: vec![] });
            }
            Some((base, after))
        }
        // 🧬 RemoveElement{parent:[0], index:1} — drops the <text> element (index 1 of <g>'s children).
        "remove-element-applied" => {
            let mut after = base.clone();
            let g = q_node_at_mut(&mut after, &[0]);
            if let QNode::Element { children, .. } = g {
                children.remove(1);
            }
            Some((base, after))
        }
        // 🧬 SetElementName{path:[0,0], name:"ellipse"} — renames <rect> in place.
        "set-element-name-applied" => {
            let mut after = base.clone();
            if let QNode::Element { name, .. } = q_node_at_mut(&mut after, &[0, 0]) {
                *name = "ellipse".into();
            }
            Some((base, after))
        }
        // 🧬 SetAttribute{path:[0,0], name:"fill", value:Some("red")} — new attribute, appended.
        "set-attribute-applied" => {
            let mut after = base.clone();
            q_set_attr(q_node_at_mut(&mut after, &[0, 0]), "fill", "red");
            Some((base, after))
        }
        // 🧬 SetText{path:[0,1,0], text:"World"} — the text node inside <text>.
        "set-text-applied" => {
            let mut after = base.clone();
            if let QNode::Text(text) = q_node_at_mut(&mut after, &[0, 1, 0]) {
                *text = "World".into();
            }
            Some((base, after))
        }
        // 🧬 SetViewBox{path:[], viewBox:Some([0,0,200,200])} — root's own viewBox.
        "set-view-box-applied" => {
            let mut after = base.clone();
            q_set_attr(q_node_at_mut(&mut after, &[]), "viewBox", "0 0 200 200");
            Some((base, after))
        }
        // 🧬 SetTransform{path:[0], transform:Some([Translate{30,40}, Scale{2}])} — <g>'s own transform.
        "set-transform-applied" => {
            let mut after = base.clone();
            q_set_attr(q_node_at_mut(&mut after, &[0]), "transform", "translate(30,40) scale(2)");
            Some((base, after))
        }
        _ => None,
    }
}

const RECIPE_IDS: &[&str] = &[
    "set-declaration-applied",
    "set-doctype-applied",
    "insert-element-applied",
    "remove-element-applied",
    "set-element-name-applied",
    "set-attribute-applied",
    "set-text-applied",
    "set-view-box-applied",
    "set-transform-applied",
];
//#endregion 🔖️Recipes

//#region 🔖️Entry
fn cmd_build(id: &str, out_dir: &str) -> i32 {
    let Some((before, after)) = recipe(id) else {
        eprintln!("[quick-xml-svg-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));
        return 1;
    };
    let dir = Path::new(out_dir).join(id);
    fs::create_dir_all(&dir).expect("create fixture recipe directory");
    fs::write(dir.join("before.svg"), write_svg(&before)).expect("write before.svg");
    fs::write(dir.join("after.svg"), write_svg(&after)).expect("write after.svg");
    eprintln!("[quick-xml-svg-codec] {id}: before.svg + after.svg -> {}", dir.display());
    0
}

fn cmd_project(path: &str) -> i32 {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[quick-xml-svg-codec] cannot read {path}: {e}");
            return 1;
        }
    };
    let doc = parse_svg(&bytes);
    println!("{}", doc_json(&doc));
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("build") => {
            let (Some(id), Some(out_dir)) = (args.get(2), args.get(3)) else {
                eprintln!("usage: quick-xml-svg-codec build <recipe-id> <out-dir>");
                std::process::exit(2);
            };
            cmd_build(id, out_dir)
        }
        Some("project") => {
            let Some(path) = args.get(2) else {
                eprintln!("usage: quick-xml-svg-codec project <path-to-svg>");
                std::process::exit(2);
            };
            cmd_project(path)
        }
        Some("list-recipes") => {
            for id in RECIPE_IDS {
                println!("{id}");
            }
            0
        }
        _ => {
            eprintln!("usage: quick-xml-svg-codec build <recipe-id> <out-dir> | project <path-to-svg> | list-recipes");
            2
        }
    };
    std::process::exit(code);
}
//#endregion 🔖️Entry

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_declared_recipe_id_resolves() {
        for id in RECIPE_IDS {
            assert!(recipe(id).is_some(), "recipe {id} must resolve");
        }
    }

    #[test]
    fn encode_decode_round_trips_the_base_document() {
        let doc = base_doc();
        let bytes = write_svg(&doc);
        let back = parse_svg(&bytes);
        assert_eq!(back.declaration.as_ref().unwrap().version, "1.0");
        assert_eq!(back.doctype, doc.doctype);
        match (&doc.root, &back.root) {
            (Some(QNode::Element { children: a, .. }), Some(QNode::Element { children: b, .. })) => assert_eq!(a.len(), b.len()),
            _ => panic!("root must be an element on both sides"),
        }
    }

    #[test]
    fn each_recipe_after_differs_from_before() {
        for id in RECIPE_IDS {
            let (before, after) = recipe(id).unwrap();
            assert_ne!(write_svg(&before), write_svg(&after), "recipe {id} must actually change something");
        }
    }
}
//#endregion 🔖️Tests
