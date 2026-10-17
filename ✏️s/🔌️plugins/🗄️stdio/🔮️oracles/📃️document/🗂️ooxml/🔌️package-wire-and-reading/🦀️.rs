//! 🔌️ Package wire and reading of the OOXML engine: the logical XML parts, their arena and wire form and the independent reading. See [`super`].

use super::*;

//#region 🔖️PackageWire
/// 🔗️ The `.rels` part that holds `source`'s relationships (`""` is the package itself: `_rels/.rels`).
pub fn relationships_part_path(source: &str) -> String {
    match source.rsplit_once('/') {
        Some((directory, file)) => format!("{directory}/_rels/{file}.rels"),
        None if source.is_empty() => "_rels/.rels".to_string(),
        None => format!("_rels/{source}.rels"),
    }
}

/// 🔗️ One source's wire relationships (`[{id, relType, target, targetMode: internal|external}]`) as a `.rels` part.
fn relationships_text(relationships: &[Json]) -> String {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">");
    for relationship in relationships {
        out.push_str(&format!("<Relationship Id=\"{}\" Type=\"{}\" Target=\"{}\"", escape_xml(&relationship.str("id"), true), escape_xml(&relationship.str("relType"), true), escape_xml(&relationship.str("target"), true)));
        if relationship.str("targetMode") == "external" {
            out.push_str(" TargetMode=\"External\"");
        }
        out.push_str("/>");
    }
    out.push_str("</Relationships>");
    out
}

/// 📦️ The package an OOXML snapshot wire (`{opc: {parts, contentTypes, relationships}, xmlParts}`) describes, written by
/// this engine alone: `[Content_Types].xml` from the typed table, one `.rels` part per relationship source, every opaque
/// part verbatim and every logical XML part serialized from its own document — "the document becomes this snapshot",
/// reproduced by a second producer that never reaches this repository's OPC codec.
pub fn write_snapshot_package(snapshot: &Json) -> Result<Vec<u8>, String> {
    let opc = snapshot.get("opc").ok_or("the snapshot carries no `opc` package")?;
    let content_types = opc.get("contentTypes").cloned().unwrap_or(Json::Null);
    let pairs = |key: &str| -> Vec<(String, String)> {
        content_types
            .array(key)
            .iter()
            .map(|pair| match pair {
                Json::Array(items) => (items.first().and_then(|item| if let Json::String(text) = item { Some(text.clone()) } else { None }).unwrap_or_default(), items.get(1).and_then(|item| if let Json::String(text) = item { Some(text.clone()) } else { None }).unwrap_or_default()),
                _ => (String::new(), String::new()),
            })
            .collect()
    };
    let mut parts = Vec::new();
    write_content_types(&mut parts, &pairs("defaults"), &pairs("overrides"))?;
    if let Some(Json::Object(sources)) = opc.get("relationships") {
        for (source, relationships) in sources {
            if let Json::Array(relationships) = relationships {
                set_part(&mut parts, &relationships_part_path(source), relationships_text(relationships).into_bytes());
            }
        }
    }
    for part in opc.array("parts") {
        let bytes = part.array("bytes").iter().filter_map(|byte| if let Json::Number(value) = byte { Some(*value as u8) } else { None }).collect();
        set_part(&mut parts, &part.str("path"), bytes);
    }
    for part in snapshot.array("xmlParts") {
        let document = part.get("document").ok_or_else(|| format!("logical XML part {} carries no document", part.str("path")))?;
        set_part(&mut parts, &part.str("path"), xml_document_text(document)?.into_bytes());
    }
    write_parts(&parts)
}
//#endregion 🔖️PackageWire

//#region 🔖️PackageReading
/// 🗂️ How an OOXML snapshot shapes each logical XML part's `document`: the `XmlDocument` tree (`{root}`, xlsx and pptx)
/// or the `RetainedXmlDocument` arena (docx) — post-order `nodes` linked by `first_child`/`nextSibling`, `attributes` in
/// element pre-order, `root` the root element's ordinal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XmlPartsShape {
    Tree,
    Retained,
}

/// 📰️ One logical XML part as read: its archive index, the text before and after its root element, and the root as a
/// wire `XmlNode` tree.
struct LogicalPart {
    pub(super) index: usize,
    pub(super) path: String,
    pub(super) prefix: String,
    pub(super) root: Json,
    suffix: String,
}

fn resolved_reference(reference: &quick_xml::events::BytesRef) -> Result<String, String> {
    if let Some(character) = reference.resolve_char_ref().map_err(|error| error.to_string())? {
        return Ok(character.to_string());
    }
    quick_xml::escape::resolve_xml_entity(reference.as_ref()).map(str::to_string).ok_or_else(|| format!("unknown entity &{};", reference.as_ref()))
}

fn text_node(text: String) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String("text".to_string())), ("text".to_string(), Json::String(text))])
}

fn element_node(start: &BytesStart, children: Vec<Json>) -> Result<Json, String> {
    let attrs = attributes_of(start)?.into_iter().map(|(name, value)| Json::Object(vec![("name".to_string(), Json::String(name)), ("value".to_string(), Json::String(value))])).collect();
    Ok(Json::Object(vec![("kind".to_string(), Json::String("element".to_string())), ("name".to_string(), Json::String(element_name(start)?)), ("attrs".to_string(), Json::Array(attrs)), ("children".to_string(), Json::Array(children))]))
}

/// 🌳 One element read to its own end as a wire `XmlNode` tree. Character data and entity references between two
/// markup boundaries join one text node, whitespace included, while CDATA, comments and processing instructions
/// retain their distinct logical forms.
fn wire_element(reader: &mut Reader<&[u8]>, start: BytesStart) -> Result<Json, String> {
    let mut children = Vec::new();
    let mut run = String::new();
    loop {
        let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
        if !matches!(event, Event::Text(_) | Event::GeneralRef(_)) && !run.is_empty() {
            children.push(text_node(std::mem::take(&mut run)));
        }
        match event {
            Event::Text(text) => run.push_str(text.as_ref()),
            Event::GeneralRef(reference) => run.push_str(&resolved_reference(&reference)?),
            Event::Start(child) => children.push(wire_element(reader, child)?),
            Event::Empty(child) => children.push(element_node(&child, Vec::new())?),
            Event::Comment(text) => children.push(Json::Object(vec![("kind".into(), Json::String("comment".into())), ("text".into(), Json::String(text.as_ref().into()))])),
            Event::CData(text) => children.push(Json::Object(vec![("kind".into(), Json::String("cData".into())), ("text".into(), Json::String(text.as_ref().into()))])),
            Event::PI(pi) => children.push(Json::Object(vec![("kind".into(), Json::String("processingInstruction".into())), ("target".into(), Json::String(pi.target().into())), ("data".into(), Json::String(pi.content().trim_start().into()))])),
            Event::End(_) => return element_node(&start, children),
            Event::Eof => return Err("unexpected end of input inside an element".to_string()),
            other => return Err(format!("unexpected {other:?} inside an OOXML part")),
        }
    }
}

/// 🧬️ Owned VML fixture read independently through quick-xml.
pub fn vml_document() -> Json { wire_xml_document(VML_MARKUP.as_bytes()).expect("valid authored VML fixture") }

pub(super) fn wire_xml_document(bytes: &[u8]) -> Result<Json, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let mut reader = Reader::from_str(text);
    let mut root = None;
    let mut fields = Vec::new();
    let mut prolog = Vec::new();
    let mut epilog = Vec::new();
    loop {
        let before = reader.buffer_position() as usize;
        let misc = match reader.read_event().map_err(|error| error.to_string())? {
            Event::Start(start) => { root = Some(wire_element(&mut reader, start)?); continue; }
            Event::Empty(start) => { root = Some(element_node(&start, Vec::new())?); continue; }
            Event::Decl(decl) => {
                let mut declaration = vec![("version".into(), Json::String(decl.version().map_err(|error| error.to_string())?.into_owned())), ("quote".into(), Json::String(if text[before..reader.buffer_position() as usize].contains('\'') { "single" } else { "double" }.into()))];
                if let Some(encoding) = decl.encoding() { declaration.push(("encoding".into(), Json::String(encoding.map_err(|error| error.to_string())?.into_owned()))); }
                if let Some(standalone) = decl.standalone() { declaration.push(("standalone".into(), Json::Bool(standalone.map_err(|error| error.to_string())?.as_ref() == "yes"))); }
                fields.push(("declaration".into(), Json::Object(declaration)));
                continue;
            }
            Event::Comment(text) => Json::Object(vec![("kind".into(), Json::String("comment".into())), ("text".into(), Json::String(text.as_ref().into()))]),
            Event::PI(pi) => Json::Object(vec![("kind".into(), Json::String("processingInstruction".into())), ("target".into(), Json::String(pi.target().into())), ("data".into(), Json::String(pi.content().trim_start().into()))]),
            Event::DocType(_) => return Err("an OPC XML part carries no document type declaration".into()),
            Event::Eof => break,
            Event::Text(text) if text.as_ref().trim().is_empty() => continue,
            other => return Err(format!("unexpected {other:?} outside the XML root")),
        };
        if root.is_some() { epilog.push(misc); } else { prolog.push(misc); }
    }
    fields.push(("root".into(), root.ok_or_else(|| "XML document has no root".to_string())?));
    if !prolog.is_empty() { fields.push(("prolog".into(), Json::Array(prolog))); }
    if !epilog.is_empty() { fields.push(("epilog".into(), Json::Array(epilog))); }
    Ok(Json::Object(fields))
}

/// 📖️ Every logical XML part of a package in archive order — the order an OOXML snapshot lists its `xmlParts`: neither
/// `[Content_Types].xml` nor a `.rels` part, and XML by path (`.xml`, `.vml`) or by effective content type.
fn logical_parts(parts: &[(String, Vec<u8>)]) -> Result<Vec<LogicalPart>, String> {
    let (defaults, overrides) = content_types(parts)?;
    let mut logical = Vec::new();
    for (index, (path, bytes)) in parts.iter().enumerate() {
        let lower = path.to_ascii_lowercase();
        let kind = resolve_content_type(&defaults, &overrides, path).to_ascii_lowercase();
        if path == CONTENT_TYPES_PART || lower.ends_with(".rels") || !(lower.ends_with(".xml") || lower.ends_with(".vml") || kind.ends_with("+xml") || kind.ends_with("/xml") || kind.contains("vmldrawing")) {
            continue;
        }
        let text = std::str::from_utf8(bytes).map_err(|error| format!("{path} is not valid utf-8: {error}"))?;
        let mut reader = Reader::from_str(text);
        loop {
            let before = reader.buffer_position() as usize;
            match reader.read_event().map_err(|error| format!("{path}: quick-xml parse error at byte {}: {error}", reader.error_position()))? {
                Event::Start(start) => {
                    let root = wire_element(&mut reader, start)?;
                    logical.push(LogicalPart { index, path: path.clone(), prefix: text[..before].to_string(), root, suffix: text[reader.buffer_position() as usize..].to_string() });
                    break;
                }
                Event::Empty(start) => {
                    logical.push(LogicalPart { index, path: path.clone(), prefix: text[..before].to_string(), root: element_node(&start, Vec::new())?, suffix: text[reader.buffer_position() as usize..].to_string() });
                    break;
                }
                Event::Eof => return Err(format!("{path} has no root element")),
                _ => {}
            }
        }
    }
    Ok(logical)
}

/// 🧮️ Appends `node` and its subtree to a retained arena the way the subject builds one — the element's attributes when
/// it is entered, the node itself once its children are placed — and returns its ordinal.
fn arena(node: &Json, nodes: &mut Vec<Json>, attributes: &mut Vec<Json>) -> usize {
    let value = if node.str("kind") == "element" {
        let first_attribute = attributes.len();
        let attrs = node.array("attrs");
        attributes.extend(attrs.iter().cloned());
        let children: Vec<usize> = node.array("children").iter().map(|child| arena(child, nodes, attributes)).collect();
        for pair in children.windows(2) {
            if let Json::Object(members) = &mut nodes[pair[0]] {
                members[0].1 = Json::Number(pair[1] as f64);
            }
        }
        let first_child = children.first().map_or(Json::Null, |ordinal| Json::Number(*ordinal as f64));
        Json::Object(vec![("kind".to_string(), Json::String("element".to_string())), ("name".to_string(), Json::String(node.str("name"))), ("first_attribute".to_string(), Json::Number(first_attribute as f64)), ("attribute_count".to_string(), Json::Number(attrs.len() as f64)), ("first_child".to_string(), first_child)])
    } else {
        text_node(node.str("text"))
    };
    nodes.push(Json::Object(vec![("nextSibling".to_string(), Json::Null), ("value".to_string(), value)]));
    nodes.len() - 1
}

fn ordinal(value: Option<&Json>, what: &str) -> Result<usize, String> {
    match value {
        Some(Json::Number(number)) if *number >= 0.0 && number.fract() == 0.0 => Ok(*number as usize),
        _ => Err(format!("a retained document's {what} is no ordinal")),
    }
}

/// 🌳 The wire tree a retained arena (`nodes`, `attributes`) describes, walked from `at` through
/// `first_child`/`nextSibling`; a link outside the arena or a node reached twice is refused.
fn from_arena(nodes: &[Json], attributes: &[Json], at: usize, budget: &mut usize) -> Result<Json, String> {
    *budget = budget.checked_sub(1).ok_or("a retained document links a node twice")?;
    let node = nodes.get(at).ok_or_else(|| format!("a retained document links absent node {at}"))?;
    let value = node.get("value").ok_or("a retained node carries no value")?;
    let link = |owner: &Json, key: &str| owner.get(key).filter(|target| !matches!(target, Json::Null)).map(|target| ordinal(Some(target), key)).transpose();
    match value.str("kind").as_str() {
        "element" => {
            let first = ordinal(value.get("first_attribute"), "first attribute")?;
            let count = ordinal(value.get("attribute_count"), "attribute count")?;
            let attrs = attributes.get(first..first + count).ok_or("a retained element addresses absent attributes")?.to_vec();
            let mut children = Vec::new();
            let mut next = link(value, "first_child")?;
            while let Some(child) = next {
                children.push(from_arena(nodes, attributes, child, budget)?);
                next = link(&nodes[child], "nextSibling")?;
            }
            Ok(Json::Object(vec![("kind".to_string(), Json::String("element".to_string())), ("name".to_string(), Json::String(value.str("name"))), ("attrs".to_string(), Json::Array(attrs)), ("children".to_string(), Json::Array(children))]))
        }
        "text" => Ok(text_node(value.str("text"))),
        other => Err(format!("a retained {other:?} node has no place in an OOXML part")),
    }
}

fn document_of(root: &Json, shape: XmlPartsShape) -> Json {
    match shape {
        XmlPartsShape::Tree => Json::Object(vec![("root".to_string(), root.clone())]),
        XmlPartsShape::Retained => {
            let (mut nodes, mut attributes) = (Vec::new(), Vec::new());
            let root = arena(root, &mut nodes, &mut attributes);
            Json::Object(vec![
                ("nodes".to_string(), Json::Array(nodes)),
                ("attributes".to_string(), Json::Array(attributes)),
                ("prolog".to_string(), Json::Array(Vec::new())),
                ("epilog".to_string(), Json::Array(Vec::new())),
                ("root".to_string(), Json::Number(root as f64)),
                ("doctype".to_string(), Json::Null),
                ("declaration".to_string(), Json::Null),
            ])
        }
    }
}

fn reading(logical: &[LogicalPart], shape: XmlPartsShape) -> Json {
    let parts = logical.iter().map(|part| Json::Object(vec![("path".to_string(), Json::String(part.path.clone())), ("document".to_string(), document_of(&part.root, shape))])).collect();
    Json::Object(vec![("xmlParts".to_string(), Json::Array(parts))])
}

/// 📖️ This engine's own `{xmlParts: [{path, document}]}` reading of a package, in the snapshot's part order and shape —
/// what a `patch-snapshot` pointer into an OOXML snapshot's `xmlParts` resolves against.
pub fn xml_parts_reading(input: &[u8], shape: XmlPartsShape) -> Result<Json, String> {
    Ok(reading(&logical_parts(&read_parts(input)?)?, shape))
}

/// 🩹️ A `patch-snapshot` row's one pointer operation applied to that reading (`semio_repo_test_host::law::patched_snapshot`)
/// and written back: every part whose root changed is re-serialized between its own original prologue and epilogue text,
/// every other entry is kept verbatim, and the container is rebuilt. A patch that adds, removes or renames a part, or
/// edits anything of a document beyond its element tree, is refused.
pub fn patched_xml_parts(input: &[u8], shape: XmlPartsShape, patch: &Json) -> Result<Vec<u8>, String> {
    let mut parts = read_parts(input)?;
    let logical = logical_parts(&parts)?;
    let before = reading(&logical, shape);
    let patched = semio_repo_test_host::law::patched_snapshot(&before, patch)?;
    let after = patched.array("xmlParts");
    if after.len() != logical.len() {
        return Err("a patch-snapshot may not add or remove an XML part".to_string());
    }
    for ((part, read), written) in logical.iter().zip(before.array("xmlParts")).zip(after) {
        let (read_document, written_document) = (read.get("document").cloned().unwrap_or(Json::Null), written.get("document").cloned().unwrap_or(Json::Null));
        if written.str("path") != part.path {
            return Err(format!("a patch-snapshot may not rename {}", part.path));
        }
        if written_document == read_document {
            continue;
        }
        let root = match shape {
            XmlPartsShape::Tree => written_document.get("root").cloned().ok_or_else(|| format!("the patched {} lost its root", part.path))?,
            XmlPartsShape::Retained => {
                for member in ["prolog", "epilog", "doctype", "declaration"] {
                    if written_document.get(member) != read_document.get(member) {
                        return Err(format!("a patch-snapshot of {} edits its {member}, which this engine keeps verbatim", part.path));
                    }
                }
                let (nodes, attributes) = (written_document.array("nodes"), written_document.array("attributes"));
                let mut budget = nodes.len();
                from_arena(&nodes, &attributes, ordinal(written_document.get("root"), "root")?, &mut budget)?
            }
        };
        let mut text = part.prefix.clone();
        xml_node_text(&root, &mut text)?;
        text.push_str(&part.suffix);
        parts[part.index].1 = text.into_bytes();
    }
    write_parts(&parts)
}
//#endregion 🔖️PackageReading
