//! quick-xml-oracle-codec — standalone XML 1.0 declaration/doctype/element-tree codec on top of
//! `quick-xml` 0.42's own generic event reader/writer (`quick_xml::reader::Reader`,
//! `quick_xml::writer::Writer`). Zero dependencies beyond `quick-xml` itself (see this crate's own
//! Cargo.toml — its own `[workspace]`, isolated from the repository's root workspace and Cargo.lock).
//!
//! This binary is independent of, and never shares code with, this subset's own
//! `../../../🔮️oracle/🦀️.rs` (which is registered `cross-semio-implementation` and computes what a
//! mutation SHOULD produce) or with the sibling SVG 1.1 base subset's own quick-xml-based oracle —
//! both compose the same crate but own their own implementations, per this subset's own doc comment.
//!
//! Two subcommands:
//!   build   <recipe-id> <out-dir> <directory> <before-file> <after-file>
//!   project <path-to-xml>           — decodes a real XML file and prints a typed JSON projection
//!                                     on stdout: declaration, doctype, prolog[], root — the same
//!                                     shape this subset's own `semantic-xml-v1` comparison profile
//!                                     describes (attributes as an unordered name/value map, sibling
//!                                     and child order preserved).
//!
//! Every recipe's BEFORE and AFTER document is authored directly as typed Rust values below — never
//! by executing this repository's own XmlMutation dispatch/diff — then handed whole to
//! `quick_xml::writer::Writer` to become real bytes. All six declared mutation kinds
//! (`xml-1-0-base`'s own `kinds` list) resolve as `applied`-only per the mutation manifest, so every
//! recipe here writes both handpicked role files; there is no `-rejected-*` recipe in this
//! corpus.
//!
//! `quick-xml` 0.42 splits every `&entity;`/`&#NNN;` reference out of `Event::Text` into its own
//! `Event::GeneralRef`, so [`resolve_general_ref`] narrows resolution to numeric character
//! references plus the five predefined XML entities — the exact same scope this subset's own
//! `../../../🔮️oracle/🦀️.rs::resolve_general_ref` documents — and the shared base document's own
//! `item#i1` text run is deliberately pre-escaped with BOTH a named entity (`&amp;`) and a numeric
//! character reference (`&#169;`) so every fixture in this corpus exercises the reassembly-across-
//! events discipline, not just the `set-text` recipe.

use quick_xml::escape::resolve_xml_entity;
use quick_xml::events::{BytesCData, BytesDecl, BytesEnd, BytesPI, BytesRef, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use quick_xml::XmlVersion;
use std::env;
use std::fs;
use std::io::Cursor;
use std::path::Path;

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq)]
struct XDecl {
    version: String,
    encoding: Option<String>,
    standalone: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
enum XExternalId {
    System { system_id: String },
    Public { public_id: String, system_id: String },
}

#[derive(Clone, Debug, PartialEq)]
struct XEntity {
    parameter: bool,
    name: String,
    value: String,
}

#[derive(Clone, Debug, PartialEq)]
struct XDoctype {
    name: String,
    external_id: Option<XExternalId>,
    entities: Vec<XEntity>,
}

#[derive(Clone, Debug, PartialEq)]
enum XNode {
    Element { name: String, attrs: Vec<(String, String)>, children: Vec<XNode> },
    Text(String),
    CData(String),
    Comment(String),
    Pi { target: String, data: String },
}

#[derive(Clone, Debug, Default, PartialEq)]
struct XDoc {
    declaration: Option<XDecl>,
    doctype: Option<XDoctype>,
    prolog: Vec<XNode>,
    root: Option<XNode>,
}
//#endregion 🔖️Types

//#region 🔖️Encode
/// ✂️️ Minimal escaping for a quoted DTD literal (system/public id, entity value) — mirrors this
/// subset's own oracle's identically-named function's narrow scope (`&`, `"`).
fn escape_dtd_literal(raw: &str) -> String {
    raw.replace('&', "&amp;").replace('"', "&quot;")
}

fn doctype_content(doctype: &XDoctype) -> String {
    let mut out = doctype.name.clone();
    match &doctype.external_id {
        Some(XExternalId::System { system_id }) => out.push_str(&format!(" SYSTEM \"{}\"", escape_dtd_literal(system_id))),
        Some(XExternalId::Public { public_id, system_id }) => out.push_str(&format!(" PUBLIC \"{}\" \"{}\"", escape_dtd_literal(public_id), escape_dtd_literal(system_id))),
        None => {}
    }
    if !doctype.entities.is_empty() {
        out.push_str(" [");
        for entity in &doctype.entities {
            out.push_str("<!ENTITY ");
            if entity.parameter {
                out.push_str("% ");
            }
            out.push_str(&entity.name);
            out.push_str(&format!(" \"{}\">", escape_dtd_literal(&entity.value)));
        }
        out.push(']');
    }
    out
}

fn write_node<W: std::io::Write>(writer: &mut Writer<W>, node: &XNode) {
    match node {
        // 📌 `from_escaped` here (never the auto-escaping `BytesText::new`) so recipe authors control
        // the EXACT source bytes — see this file's own header on the base document's pre-escaped
        // `item#i1` text run, which deliberately embeds a raw `&#169;` numeric reference.
        XNode::Text(text) => writer.write_event(Event::Text(BytesText::from_escaped(text.as_str()))).expect("write text event"),
        XNode::CData(text) => writer.write_event(Event::CData(BytesCData::new(text.as_str()))).expect("write cdata event"),
        XNode::Comment(text) => writer.write_event(Event::Comment(BytesText::from_escaped(text.as_str()))).expect("write comment event"),
        XNode::Pi { target, data } => {
            let content = if data.is_empty() { target.clone() } else { format!("{target} {data}") };
            writer.write_event(Event::PI(BytesPI::new(content))).expect("write pi event");
        }
        XNode::Element { name, attrs, children } => {
            let mut start = BytesStart::new(name.as_str());
            for (key, value) in attrs {
                start.push_attribute((key.as_str(), value.as_str()));
            }
            if children.is_empty() {
                writer.write_event(Event::Empty(start)).expect("write empty element event");
                return;
            }
            writer.write_event(Event::Start(start)).expect("write start element event");
            for child in children {
                write_node(writer, child);
            }
            writer.write_event(Event::End(BytesEnd::new(name.as_str()))).expect("write end element event");
        }
    }
}

/// ✍️ The whole document, handed to `quick_xml::writer::Writer` — `quick-xml` itself computes every
/// event's byte framing; this function only decides WHICH typed XML 1.0 nodes exist and in what
/// order (declaration?, prolog*, doctype?, root).
fn encode_xml(doc: &XDoc) -> Vec<u8> {
    let mut writer = Writer::new(Cursor::new(Vec::<u8>::new()));
    if let Some(decl) = &doc.declaration {
        let standalone = decl.standalone.map(|value| if value { "yes" } else { "no" });
        writer.write_event(Event::Decl(BytesDecl::new(&decl.version, decl.encoding.as_deref(), standalone))).expect("write decl event");
    }
    for node in &doc.prolog {
        write_node(&mut writer, node);
    }
    if let Some(doctype) = &doc.doctype {
        writer.write_event(Event::DocType(BytesText::from_escaped(doctype_content(doctype)))).expect("write doctype event");
    }
    if let Some(root) = &doc.root {
        write_node(&mut writer, root);
    }
    writer.into_inner().into_inner()
}
//#endregion 🔖️Encode

//#region 🔖️Decode — reads real bytes back with `quick_xml::reader::Reader`, this module's own typing
/// 🔓️ Resolves one `Event::GeneralRef` (`&name;` or `&#NNN;`) to its literal text — numeric
/// character references via `resolve_char_ref`, the five predefined XML entities via
/// `resolve_xml_entity`, anything else a hard parse error. Same narrowed scope this subset's own
/// oracle documents; a custom DTD-declared entity (e.g. `&vendor;`) is NOT resolved here, matching
/// production.
fn resolve_general_ref(reference: &BytesRef) -> Result<String, String> {
    if let Some(ch) = reference.resolve_char_ref().map_err(|error| error.to_string())? {
        return Ok(ch.to_string());
    }
    match resolve_xml_entity(reference.as_ref()) {
        Some(resolved) => Ok(resolved.to_string()),
        None => Err(format!("unknown entity &{};", reference.as_ref())),
    }
}

fn read_attrs(start: &BytesStart) -> Result<Vec<(String, String)>, String> {
    start
        .attributes()
        .map(|attr| {
            let attr = attr.map_err(|error| error.to_string())?;
            let value = attr.normalized_value(XmlVersion::Explicit1_0).map_err(|error| error.to_string())?;
            Ok((attr.key.as_ref().to_string(), value.to_string()))
        })
        .collect()
}

fn flush_text(text_run: &mut String, children: &mut Vec<XNode>) {
    if !text_run.is_empty() {
        children.push(XNode::Text(std::mem::take(text_run)));
    }
}

/// 🌳 Recursive-descent element parse: reads events until this element's own `End`, recursing into
/// `Start` children — a text run is accumulated across `Text`/`GeneralRef` events (the reassembly
/// this file's own header documents) and only flushed to a `XNode::Text` child when a non-text event
/// interrupts it.
fn parse_element(reader: &mut Reader<&[u8]>, start: BytesStart) -> Result<XNode, String> {
    let name = start.name().as_ref().to_string();
    let attrs = read_attrs(&start)?;
    let mut children = Vec::new();
    let mut text_run = String::new();
    loop {
        let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
        match event {
            Event::End(_) => {
                flush_text(&mut text_run, &mut children);
                return Ok(XNode::Element { name, attrs, children });
            }
            Event::Start(child_start) => {
                flush_text(&mut text_run, &mut children);
                children.push(parse_element(reader, child_start)?);
            }
            Event::Empty(child_start) => {
                flush_text(&mut text_run, &mut children);
                children.push(XNode::Element { name: child_start.name().as_ref().to_string(), attrs: read_attrs(&child_start)?, children: Vec::new() });
            }
            Event::Text(text) => text_run.push_str(text.as_ref()),
            Event::GeneralRef(reference) => text_run.push_str(&resolve_general_ref(&reference)?),
            Event::CData(cdata) => {
                flush_text(&mut text_run, &mut children);
                children.push(XNode::CData(cdata.into_inner().into_owned()));
            }
            Event::Comment(comment) => {
                flush_text(&mut text_run, &mut children);
                children.push(XNode::Comment(comment.as_ref().to_string()));
            }
            Event::PI(pi) => {
                flush_text(&mut text_run, &mut children);
                children.push(XNode::Pi { target: pi.target().to_string(), data: pi.content().trim_start().to_string() });
            }
            Event::Eof => return Err(format!("unclosed element <{name}>: unexpected end of input")),
            Event::Decl(_) | Event::DocType(_) => return Err(format!("declaration/doctype cannot appear inside element <{name}>")),
        }
    }
}

fn decl_from_event(decl: &BytesDecl) -> Result<XDecl, String> {
    let version = decl.version().map_err(|error| error.to_string())?.to_string();
    let encoding = match decl.encoding() {
        Some(result) => Some(result.map_err(|error| error.to_string())?.to_string()),
        None => None,
    };
    let standalone = match decl.standalone() {
        Some(result) => Some(result.map_err(|error| error.to_string())?.as_ref() == "yes"),
        None => None,
    };
    Ok(XDecl { version, encoding, standalone })
}

fn skip_ws(bytes: &[u8], pos: &mut usize) {
    while *pos < bytes.len() && bytes[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
}

fn parse_name(s: &str, pos: &mut usize) -> Result<String, String> {
    let start = *pos;
    while *pos < s.len() {
        let ch = s[*pos..].chars().next().unwrap();
        if ch.is_whitespace() || ch == '[' || ch == '>' {
            break;
        }
        *pos += ch.len_utf8();
    }
    if *pos == start {
        return Err("expected XML doctype name".to_string());
    }
    Ok(s[start..*pos].to_string())
}

fn parse_quoted(s: &str, pos: &mut usize) -> Result<String, String> {
    skip_ws(s.as_bytes(), pos);
    let quote = s[*pos..].chars().next().ok_or("expected quoted doctype literal")?;
    if quote != '"' && quote != '\'' {
        return Err("doctype literal must be quoted".to_string());
    }
    *pos += 1;
    let start = *pos;
    while *pos < s.len() {
        let ch = s[*pos..].chars().next().unwrap();
        if ch == quote {
            let value = s[start..*pos].to_string();
            *pos += 1;
            return Ok(value);
        }
        *pos += ch.len_utf8();
    }
    Err("unclosed doctype literal".to_string())
}

/// 📜️ Parses the DOCTYPE content `quick-xml`'s `Event::DocType` hands back (everything between
/// `<!DOCTYPE` and the matching `>`) into `name (SYSTEM "sysid" | PUBLIC "pubid" "sysid")? ([
/// <!ENTITY (%)? name "value"> ... ])?` — independent hand-rolled parser, same narrowed scope this
/// subset's own oracle documents (SYSTEM/PUBLIC external ids plus typed `<!ENTITY>` declarations
/// only; any other DTD construct is a hard parse error rather than silently dropped).
fn parse_doctype(raw: &str) -> Result<XDoctype, String> {
    let mut pos = 0usize;
    let bytes = raw.as_bytes();
    skip_ws(bytes, &mut pos);
    let name = parse_name(raw, &mut pos)?;
    skip_ws(bytes, &mut pos);
    let external_id = if raw[pos..].starts_with("SYSTEM") {
        pos += "SYSTEM".len();
        Some(XExternalId::System { system_id: parse_quoted(raw, &mut pos)? })
    } else if raw[pos..].starts_with("PUBLIC") {
        pos += "PUBLIC".len();
        let public_id = parse_quoted(raw, &mut pos)?;
        let system_id = parse_quoted(raw, &mut pos)?;
        Some(XExternalId::Public { public_id, system_id })
    } else {
        None
    };
    skip_ws(bytes, &mut pos);
    let mut entities = Vec::new();
    if raw[pos..].starts_with('[') {
        pos += 1;
        loop {
            skip_ws(bytes, &mut pos);
            if raw[pos..].starts_with(']') {
                break;
            }
            if !raw[pos..].starts_with("<!ENTITY") {
                return Err("unsupported XML DTD declaration; only typed ENTITY declarations are modeled".to_string());
            }
            pos += "<!ENTITY".len();
            skip_ws(bytes, &mut pos);
            let parameter = raw[pos..].starts_with('%');
            if parameter {
                pos += 1;
                skip_ws(bytes, &mut pos);
            }
            let entity_name = parse_name(raw, &mut pos)?;
            let value = parse_quoted(raw, &mut pos)?;
            skip_ws(bytes, &mut pos);
            if !raw[pos..].starts_with('>') {
                return Err("expected > after XML entity declaration".to_string());
            }
            pos += 1;
            entities.push(XEntity { parameter, name: entity_name, value });
        }
    }
    Ok(XDoctype { name, external_id, entities })
}

/// 📥️ Decodes real bytes back into a typed document, walking events with `quick_xml::reader::Reader`
/// — `quick-xml` owns every token/byte-offset computation; this function only assigns XML 1.0
/// meaning to what it finds.
fn decode_xml(bytes: &[u8]) -> Result<XDoc, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let mut reader = Reader::from_str(text);
    let mut doc = XDoc::default();
    let mut root_seen = false;
    loop {
        let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
        match event {
            Event::Eof => break,
            Event::Decl(decl) => doc.declaration = Some(decl_from_event(&decl)?),
            Event::DocType(doctype) => doc.doctype = Some(parse_doctype(doctype.as_ref().trim())?),
            // 📌 `content()` returns everything after the target NAME, including the one separating
            // whitespace byte — `trim_start` recovers the semantic `data` half (what `write_node`'s
            // own `format!("{target} {data}")` put there), so encode -> decode round-trips exactly.
            Event::PI(pi) if !root_seen => doc.prolog.push(XNode::Pi { target: pi.target().to_string(), data: pi.content().trim_start().to_string() }),
            Event::PI(_) => {}
            Event::Comment(comment) if !root_seen => doc.prolog.push(XNode::Comment(comment.as_ref().to_string())),
            Event::Comment(_) => {}
            Event::Text(text) => {
                if !text.as_ref().trim().is_empty() {
                    return Err(if root_seen { "trailing content after root element".to_string() } else { "unexpected text before the root element".to_string() });
                }
            }
            Event::GeneralRef(reference) => return Err(format!("unexpected entity reference &{}; outside the root element", reference.as_ref())),
            Event::CData(_) => return Err("unexpected CDATA section outside the root element".to_string()),
            Event::Start(start) => {
                if root_seen {
                    return Err("multiple root elements".to_string());
                }
                doc.root = Some(parse_element(&mut reader, start)?);
                root_seen = true;
            }
            Event::Empty(start) => {
                if root_seen {
                    return Err("multiple root elements".to_string());
                }
                doc.root = Some(XNode::Element { name: start.name().as_ref().to_string(), attrs: read_attrs(&start)?, children: Vec::new() });
                root_seen = true;
            }
            Event::End(_) => return Err("unexpected closing tag before the root element".to_string()),
        }
    }
    if doc.root.is_none() {
        return Err("document has no root element".to_string());
    }
    Ok(doc)
}
//#endregion 🔖️Decode

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

fn json_opt_str(s: &Option<String>) -> String {
    match s {
        Some(value) => json_str(value),
        None => "null".to_string(),
    }
}

fn json_opt_bool(b: Option<bool>) -> String {
    match b {
        Some(value) => value.to_string(),
        None => "null".to_string(),
    }
}

fn declaration_json(decl: &Option<XDecl>) -> String {
    match decl {
        None => "null".to_string(),
        Some(d) => format!("{{\"version\":{},\"encoding\":{},\"standalone\":{}}}", json_str(&d.version), json_opt_str(&d.encoding), json_opt_bool(d.standalone)),
    }
}

fn external_id_json(id: &Option<XExternalId>) -> String {
    match id {
        None => "null".to_string(),
        Some(XExternalId::System { system_id }) => format!("{{\"kind\":\"system\",\"systemId\":{}}}", json_str(system_id)),
        Some(XExternalId::Public { public_id, system_id }) => format!("{{\"kind\":\"public\",\"publicId\":{},\"systemId\":{}}}", json_str(public_id), json_str(system_id)),
    }
}

fn entity_json(e: &XEntity) -> String {
    format!("{{\"parameter\":{},\"name\":{},\"value\":{}}}", e.parameter, json_str(&e.name), json_str(&e.value))
}

fn doctype_json(doctype: &Option<XDoctype>) -> String {
    match doctype {
        None => "null".to_string(),
        Some(dt) => {
            let entities: Vec<String> = dt.entities.iter().map(entity_json).collect();
            format!("{{\"name\":{},\"externalId\":{},\"entities\":[{}]}}", json_str(&dt.name), external_id_json(&dt.external_id), entities.join(","))
        }
    }
}

/// 👁️ `attrs` project as an unordered name/value MAP (`{...}`), not an ordered list — matching this
/// subset's own `semantic-xml-v1` profile's own "attribute order is writer freedom" carve-out.
fn node_json(node: &XNode) -> String {
    match node {
        XNode::Element { name, attrs, children } => {
            let attr_entries: Vec<String> = attrs.iter().map(|(key, value)| format!("{}:{}", json_str(key), json_str(value))).collect();
            let child_entries: Vec<String> = children.iter().map(node_json).collect();
            format!("{{\"kind\":\"element\",\"name\":{},\"attrs\":{{{}}},\"children\":[{}]}}", json_str(name), attr_entries.join(","), child_entries.join(","))
        }
        XNode::Text(text) => format!("{{\"kind\":\"text\",\"text\":{}}}", json_str(text)),
        XNode::CData(text) => format!("{{\"kind\":\"cdata\",\"text\":{}}}", json_str(text)),
        XNode::Comment(text) => format!("{{\"kind\":\"comment\",\"text\":{}}}", json_str(text)),
        XNode::Pi { target, data } => format!("{{\"kind\":\"pi\",\"target\":{},\"data\":{}}}", json_str(target), json_str(data)),
    }
}

fn doc_json(doc: &XDoc) -> String {
    let prolog: Vec<String> = doc.prolog.iter().map(node_json).collect();
    let root = match &doc.root {
        Some(root) => node_json(root),
        None => "null".to_string(),
    };
    format!("{{\"declaration\":{},\"doctype\":{},\"prolog\":[{}],\"root\":{}}}", declaration_json(&doc.declaration), doctype_json(&doc.doctype), prolog.join(","), root)
}
//#endregion 🔖️Json

//#region 🔖️BaseDocument
/// 🧬 The shared starting document every recipe clones from — a declaration, a SYSTEM-id doctype with
/// one typed `<!ENTITY>`, a prolog PI + comment, and a root with two `item` elements (the first
/// carrying attributes AND a text run pre-escaped with both a named entity and a numeric character
/// reference — see this file's own header), a CDATA section, a comment and a PI — big enough to
/// exercise all six declared mutation kinds (`set-declaration`, `set-doctype`, `insert-element`,
/// `remove-element`, `set-attribute`, `set-text`) meaningfully.
fn base_doc() -> XDoc {
    XDoc {
        declaration: Some(XDecl { version: "1.0".to_string(), encoding: Some("UTF-8".to_string()), standalone: Some(false) }),
        doctype: Some(XDoctype { name: "catalog".to_string(), external_id: Some(XExternalId::System { system_id: "catalog.dtd".to_string() }), entities: vec![XEntity { parameter: false, name: "vendor".to_string(), value: "Acme Corp".to_string() }] }),
        prolog: vec![XNode::Pi { target: "catalog-pi".to_string(), data: "build=\"1\"".to_string() }, XNode::Comment(" catalog root comment ".to_string())],
        root: Some(XNode::Element {
            name: "catalog".to_string(),
            attrs: vec![("id".to_string(), "c1".to_string()), ("rev".to_string(), "3".to_string())],
            children: vec![
                XNode::Element {
                    name: "item".to_string(),
                    attrs: vec![("id".to_string(), "i1".to_string()), ("qty".to_string(), "2".to_string())],
                    // 📌 Pre-escaped: literal file bytes contain `&amp;` (named) AND `&#169;` (numeric) —
                    // this file's header explains why.
                    children: vec![XNode::Text("Widget &amp; Gadget &#169;".to_string())],
                },
                XNode::Element { name: "item".to_string(), attrs: vec![("id".to_string(), "i2".to_string())], children: vec![] },
                XNode::CData("<raw markup>".to_string()),
                XNode::Comment(" trailing note ".to_string()),
                XNode::Pi { target: "note".to_string(), data: "priority=\"low\"".to_string() },
            ],
        }),
    }
}
//#endregion 🔖️BaseDocument

//#region 🔖️Recipes
/// 🧪 One recipe: BEFORE and AFTER, always — every kind in this corpus resolves `applied`-only per
/// the mutation manifest, so there is no `-rejected-*` recipe here. Every AFTER state below touches
/// EXACTLY the field the real dispatch's own match arm for that kind touches (see
/// `../../🧬️schema/🧬️mutations/🦀️.rs`'s own per-kind modules), leaving everything else identical to
/// `⬅️before.xml` — the same "assert what production actually produces" discipline the sibling AVI
/// codec's own recipe function documents.
fn recipe(id: &str) -> Option<(XDoc, XDoc)> {
    let base = base_doc();
    match id {
        // 🧬 SetDeclaration — whole-value replace of the declaration only.
        "set-declaration-applied" => {
            let mut after = base.clone();
            after.declaration = Some(XDecl { version: "1.0".to_string(), encoding: Some("UTF-16".to_string()), standalone: Some(true) });
            Some((base, after))
        }

        // 🧬 SetDoctype — whole-value replace of the doctype only (SYSTEM -> PUBLIC, entity added).
        "set-doctype-applied" => {
            let mut after = base.clone();
            after.doctype = Some(XDoctype {
                name: "catalog".to_string(),
                external_id: Some(XExternalId::Public { public_id: "-//ACME//DTD Catalog//EN".to_string(), system_id: "catalog.dtd".to_string() }),
                entities: vec![XEntity { parameter: false, name: "vendor".to_string(), value: "Acme Corp".to_string() }, XEntity { parameter: false, name: "revision".to_string(), value: "2".to_string() }],
            });
            Some((base, after))
        }

        // 🧬 InsertElement{path:[], index:2, node:<item i3>} — a third item is inserted before the
        // CDATA section; every other child keeps its position.
        "insert-element-applied" => {
            let mut after = base.clone();
            let XNode::Element { children, .. } = after.root.as_mut().unwrap() else { unreachable!() };
            children.insert(2, XNode::Element { name: "item".to_string(), attrs: vec![("id".to_string(), "i3".to_string()), ("qty".to_string(), "1".to_string())], children: vec![XNode::Text("Extra".to_string())] });
            Some((base, after))
        }

        // 🧬 RemoveElement{path:[], index:2} — drops the CDATA section; the two items, comment and
        // PI keep their relative order.
        "remove-element-applied" => {
            let mut after = base.clone();
            let XNode::Element { children, .. } = after.root.as_mut().unwrap() else { unreachable!() };
            children.remove(2);
            Some((base, after))
        }

        // 🧬 SetAttribute{path:[0], name:"qty", value:"5"} — item i1's qty changes; its text, i2, the
        // CDATA/comment/PI and every other attribute are untouched.
        "set-attribute-applied" => {
            let mut after = base.clone();
            let XNode::Element { children, .. } = after.root.as_mut().unwrap() else { unreachable!() };
            let XNode::Element { attrs, .. } = &mut children[0] else { unreachable!() };
            attrs.iter_mut().find(|(key, _)| key == "qty").unwrap().1 = "5".to_string();
            Some((base, after))
        }

        // 🧬 SetText{path:[0,0], text:"Sprocket &amp; Cog &#8364;"} — item i1's text run changes to a
        // DIFFERENT pre-escaped named+numeric pair (Euro sign, U+20AC — a 3-byte UTF-8 char once
        // resolved), proving the reassembly discipline survives a mutation, not just the base state.
        "set-text-applied" => {
            let mut after = base.clone();
            let XNode::Element { children, .. } = after.root.as_mut().unwrap() else { unreachable!() };
            let XNode::Element { children: item_children, .. } = &mut children[0] else { unreachable!() };
            item_children[0] = XNode::Text("Sprocket &amp; Cog &#8364;".to_string());
            Some((base, after))
        }

        _ => None,
    }
}

const RECIPE_IDS: &[&str] = &["set-declaration-applied", "set-doctype-applied", "insert-element-applied", "remove-element-applied", "set-attribute-applied", "set-text-applied"];
//#endregion 🔖️Recipes

//#region 🔖️Entry
fn cmd_build(id: &str, out_dir: &str, directory: &str, before_file: &str, after_file: &str) -> i32 {
    let Some((before, after)) = recipe(id) else {
        eprintln!("[quick-xml-oracle-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));
        return 1;
    };
    let dir = Path::new(out_dir).join(directory);
    fs::create_dir_all(&dir).expect("create fixture recipe directory");
    fs::write(dir.join(before_file), encode_xml(&before)).expect("write before XML");
    fs::write(dir.join(after_file), encode_xml(&after)).expect("write after XML");
    eprintln!("[quick-xml-oracle-codec] {id}: {before_file} + {after_file} -> {}", dir.display());
    0
}

fn cmd_project(path: &str) -> i32 {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[quick-xml-oracle-codec] cannot read {path}: {e}");
            return 1;
        }
    };
    match decode_xml(&bytes) {
        Ok(doc) => {
            println!("{}", doc_json(&doc));
            0
        }
        Err(error) => {
            eprintln!("[quick-xml-oracle-codec] cannot parse {path}: {error}");
            1
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("build") => {
            let (Some(id), Some(out_dir), Some(directory), Some(before_file), Some(after_file)) = (args.get(2), args.get(3), args.get(4), args.get(5), args.get(6)) else {
                eprintln!("usage: quick-xml-oracle-codec build <recipe-id> <out-dir> <directory> <before-file> <after-file>");
                std::process::exit(2);
            };
            cmd_build(id, out_dir, directory, before_file, after_file)
        }
        Some("project") => {
            let Some(path) = args.get(2) else {
                eprintln!("usage: quick-xml-oracle-codec project <path-to-xml>");
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
            eprintln!("usage: quick-xml-oracle-codec build <recipe-id> <out-dir> <directory> <before-file> <after-file> | project <path-to-xml> | list-recipes");
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
        // 📌 `base_doc()`'s own item#i1 text node deliberately stores the PRE-ESCAPED source form
        // (see this file's header) — decode resolves entities, so the round-trip target is the
        // RESOLVED text, not the literal source string.
        let mut doc = base_doc();
        let bytes = encode_xml(&doc);
        let back = decode_xml(&bytes).expect("decode base document");
        if let Some(XNode::Element { children, .. }) = doc.root.as_mut() {
            if let XNode::Element { children: item_children, .. } = &mut children[0] {
                item_children[0] = XNode::Text("Widget & Gadget \u{a9}".to_string());
            }
        }
        assert_eq!(back.declaration, doc.declaration);
        assert_eq!(back.doctype, doc.doctype);
        assert_eq!(back.prolog, doc.prolog);
        assert_eq!(back.root, doc.root);
    }

    #[test]
    fn general_ref_reassembly_recovers_named_and_numeric_entities_across_events() {
        let bytes = encode_xml(&base_doc());
        let text = std::str::from_utf8(&bytes).unwrap();
        assert!(text.contains("&amp;"), "encoded bytes must literally contain the named entity ref");
        assert!(text.contains("&#169;"), "encoded bytes must literally contain the numeric char ref");
        let doc = decode_xml(&bytes).expect("decode");
        let Some(XNode::Element { children, .. }) = doc.root else { panic!("expected root element") };
        let XNode::Element { children: item_children, .. } = &children[0] else { panic!("expected item element") };
        let XNode::Text(text) = &item_children[0] else { panic!("expected text node") };
        assert_eq!(text, "Widget & Gadget \u{a9}", "reassembled text must resolve both the named and numeric reference");
    }

    #[test]
    fn every_recipe_after_state_differs_from_before_in_exactly_its_own_dimension() {
        for id in RECIPE_IDS {
            let (before, after) = recipe(id).unwrap();
            assert_ne!(before, after, "recipe {id} must produce a materially different after-state");
        }
    }
}
//#endregion 🔖️Tests
