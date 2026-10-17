//! 🌳️ XML tree of the PPTX oracle: the owned element and text node, the quick-xml parser and the serializer. See [`super`].

use super::*;

//#region 🔖️Tree
/// 🌳 Owned XML node — element or text only, the scope every real OPC XML part this oracle
/// touches (`presentation.xml`, `slideN.xml`, `.rels`, `[Content_Types].xml`) actually needs.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum XNode {
    Element { name: String, attrs: Vec<(String, String)>, children: Vec<XNode> },
    Text(String),
}

impl XNode {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) — see R9
    pub(super) fn child(&self, name: &str) -> Option<&XNode> {
        let XNode::Element { children, .. } = self else { return None };
        children.iter().find(|c| matches!(c, XNode::Element { name: n, .. } if n == name))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) — see R9
    pub(super) fn attr(&self, key: &str) -> Option<&str> {
        let XNode::Element { attrs, .. } = self else { return None };
        attrs.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) — see R9
    pub(super) fn children_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a XNode> {
        let empty: &[XNode] = &[];
        let items = match self {
            XNode::Element { children, .. } => children.as_slice(),
            XNode::Text(_) => empty,
        };
        items.iter().filter(move |c| matches!(c, XNode::Element { name: n, .. } if n == name))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) — see R9
    pub(super) fn text(&self) -> String {
        match self {
            XNode::Text(t) => t.clone(),
            XNode::Element { children, .. } => children.iter().map(|c| c.text()).collect(),
        }
    }
}
//#endregion 🔖️Tree

//#region 🔖️Parse
/// 🔓️ Resolves one `Event::GeneralRef` (`&name;` or `&#NNN;`) to its literal text.
fn resolve_general_ref(reference: &BytesRef) -> Result<String, String> {
    if let Some(ch) = reference.resolve_char_ref().map_err(|e| e.to_string())? {
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
            let attr = attr.map_err(|e| e.to_string())?;
            let value = attr.normalized_value(XmlVersion::Explicit1_0).map_err(|e| e.to_string())?;
            Ok((attr.key.as_ref().to_string(), value.to_string()))
        })
        .collect()
}

fn flush_text(run: &mut String, children: &mut Vec<XNode>) {
    if !run.is_empty() {
        children.push(XNode::Text(std::mem::take(run)));
    }
}

/// 🌳 Recursive-descent element parse: reads events until this element's own `End`.
fn parse_element(reader: &mut Reader<&[u8]>, start: BytesStart) -> Result<XNode, String> {
    let name = start.name().as_ref().to_string();
    let attrs = read_attrs(&start)?;
    let mut children = Vec::new();
    let mut run = String::new();
    loop {
        let event = reader.read_event().map_err(|e| format!("quick-xml parse error at byte {}: {e}", reader.error_position()))?;
        match event {
            Event::End(_) => {
                flush_text(&mut run, &mut children);
                return Ok(XNode::Element { name, attrs, children });
            }
            Event::Start(child_start) => {
                flush_text(&mut run, &mut children);
                children.push(parse_element(reader, child_start)?);
            }
            Event::Empty(child_start) => {
                flush_text(&mut run, &mut children);
                children.push(XNode::Element { name: child_start.name().as_ref().to_string(), attrs: read_attrs(&child_start)?, children: Vec::new() });
            }
            Event::Text(text) => run.push_str(text.as_ref()),
            Event::GeneralRef(reference) => run.push_str(&resolve_general_ref(&reference)?),
            Event::CData(cdata) => run.push_str(&cdata.into_inner()),
            Event::Comment(_) | Event::PI(_) => {}
            Event::Eof => return Err(format!("unclosed element <{name}>: unexpected end of input")),
            Event::Decl(_) | Event::DocType(_) => return Err(format!("declaration/doctype cannot appear inside element <{name}>")),
        }
    }
}

/// 📄 Parses one whole OPC XML part (decl + a single root element) into its root [`XNode`].
pub(super) fn parse_document(bytes: &[u8]) -> Result<XNode, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let mut reader = Reader::from_str(text);
    loop {
        let event = reader.read_event().map_err(|e| format!("quick-xml parse error at byte {}: {e}", reader.error_position()))?;
        match event {
            Event::Decl(_) | Event::Comment(_) | Event::PI(_) => {}
            Event::Text(text) if text.as_ref().trim_ascii().is_empty() => {}
            Event::Start(start) => return parse_element(&mut reader, start),
            Event::Empty(start) => return Ok(XNode::Element { name: start.name().as_ref().to_string(), attrs: read_attrs(&start)?, children: Vec::new() }),
            Event::Eof => return Err("document has no root element".to_string()),
            other => return Err(format!("unexpected event before the root element: {other:?}")),
        }
    }
}
//#endregion 🔖️Parse

//#region 🔖️Serialize
fn write_node<W: Write>(writer: &mut Writer<W>, node: &XNode) -> Result<(), String> {
    match node {
        XNode::Text(text) => writer.write_event(Event::Text(BytesText::new(text))).map_err(|e| e.to_string()),
        XNode::Element { name, attrs, children } => {
            let mut start = BytesStart::new(name.as_str());
            for (key, value) in attrs {
                start.push_attribute((key.as_str(), value.as_str()));
            }
            if children.is_empty() {
                return writer.write_event(Event::Empty(start)).map_err(|e| e.to_string());
            }
            writer.write_event(Event::Start(start)).map_err(|e| e.to_string())?;
            for child in children {
                write_node(writer, child)?;
            }
            writer.write_event(Event::End(BytesEnd::new(name.as_str()))).map_err(|e| e.to_string())
        }
    }
}

/// 📄 Serializes one whole OPC XML part: the standard declaration plus `root`.
pub(super) fn serialize_document(root: &XNode) -> Result<Vec<u8>, String> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new("1.0", Some("UTF-8"), Some("yes")))).map_err(|e| e.to_string())?;
    write_node(&mut writer, root)?;
    Ok(writer.into_inner().into_inner())
}
//#endregion 🔖️Serialize
