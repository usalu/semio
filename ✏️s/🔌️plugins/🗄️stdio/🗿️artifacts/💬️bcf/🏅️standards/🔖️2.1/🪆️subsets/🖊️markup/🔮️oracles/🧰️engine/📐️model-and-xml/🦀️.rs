//! 📐️ Model and XML of the BCF oracle: the independent BCF document model, the XML tree, its parser and writer and the archive entries. See [`super`].

use super::*;

//#region 🔖️Model
/// 📐 A 3D point/vector — the independent mirror of `visinfo.xsd`'s `Point`/`Direction`
/// (`{X,Y,Z}` triples), unrelated to `semio-s-plugin-stdio`'s own `BcfPoint3`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct OPoint3 {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) z: f64,
}

/// 📷 A viewpoint's camera — `visinfo.xsd`'s `PerspectiveCamera`/`OrthogonalCamera` `xs:choice`.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum OCamera {
    Perspective { view_point: OPoint3, direction: OPoint3, up_vector: OPoint3, field_of_view: f64 },
    Orthogonal { view_point: OPoint3, direction: OPoint3, up_vector: OPoint3, view_to_world_scale: f64 },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct OVisibility {
    pub(super) default_visibility: bool,
    pub(super) exceptions: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct OColoring {
    pub(super) color: String,
    pub(super) components: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct OComponents {
    pub(super) selection: Vec<String>,
    pub(super) visibility: OVisibility,
    pub(super) coloring: Vec<OColoring>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct OComment {
    pub(super) guid: String,
    pub(super) date: String,
    pub(super) author: String,
    pub(super) text: String,
    pub(super) viewpoint_ref: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct OViewpoint {
    pub(super) guid: String,
    pub(super) camera: Option<OCamera>,
    pub(super) components: Option<OComponents>,
    pub(super) snapshot: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct OTopic {
    pub(super) guid: String,
    pub(super) title: String,
    pub(super) description: String,
    pub(super) status: String,
    pub(super) priority: String,
    pub(super) labels: Vec<String>,
    pub(super) creation_date: String,
    pub(super) creation_author: String,
    pub(super) comments: Vec<OComment>,
    pub(super) viewpoints: Vec<OViewpoint>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct ORawPart {
    pub(super) name: String,
    pub(super) data: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct ODoc {
    pub(super) version: String,
    pub(super) topics: Vec<OTopic>,
    pub(super) parts: Vec<ORawPart>,
}
//#endregion 🔖️Model

//#region 🔖️XmlTree
/// 🌳 Minimal owned XML node — element-with-attributes-and-children or leaf text — sufficient
/// for BCF-XML 2.1's `markup.xsd`/`visinfo.xsd` shapes (no CDATA/comment/PI content anywhere in
/// a real BCF part). Independent of every other subset's own tree type (each subset that builds
/// on `quick-xml` owns its own, per the wave 7 fleet brief's XML precedent).
#[derive(Clone, Debug, PartialEq)]
pub(super) enum BNode {
    Element { name: String, attrs: Vec<(String, String)>, children: Vec<BNode> },
    Text(String),
}

pub(super) fn as_element(node: &BNode) -> Option<(&str, &[(String, String)], &[BNode])> {
    match node {
        BNode::Element { name, attrs, children } => Some((name.as_str(), attrs.as_slice(), children.as_slice())),
        BNode::Text(_) => None,
    }
}

pub(super) fn find_child<'a>(children: &'a [BNode], name: &str) -> Option<&'a BNode> {
    children.iter().find(|child| as_element(child).map(|(n, _, _)| n == name).unwrap_or(false))
}

pub(super) fn find_children<'a>(children: &'a [BNode], name: &str) -> Vec<&'a BNode> {
    children.iter().filter(|child| as_element(child).map(|(n, _, _)| n == name).unwrap_or(false)).collect()
}

pub(super) fn attr<'a>(attrs: &'a [(String, String)], name: &str) -> Option<&'a str> {
    attrs.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
}

pub(super) fn text_content(node: &BNode) -> String {
    let Some((_, _, children)) = as_element(node) else { return String::new() };
    children.iter().filter_map(|child| if let BNode::Text(text) = child { Some(text.as_str()) } else { None }).collect()
}

pub(super) fn text_element(name: &str, text: &str) -> Option<BNode> {
    if text.is_empty() {
        return None;
    }
    Some(BNode::Element { name: name.to_string(), attrs: Vec::new(), children: vec![BNode::Text(text.to_string())] })
}
//#endregion 🔖️XmlTree

//#region 🔖️XmlParse
/// 🔓️ Resolves one `Event::GeneralRef` (`&name;` or `&#NNN;`) to its literal text — this
/// `quick-xml` version splits every entity reference out of `Text` into its own event, exactly
/// the same `📰️xml`/`🎨️svg` precedent already established.
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

fn flush_text(text_run: &mut String, children: &mut Vec<BNode>) {
    if !text_run.is_empty() {
        children.push(BNode::Text(std::mem::take(text_run)));
    }
}

fn parse_element(reader: &mut Reader<&[u8]>, start: BytesStart) -> Result<BNode, String> {
    let name = start.name().as_ref().to_string();
    let attrs = read_attrs(&start)?;
    let mut children = Vec::new();
    let mut text_run = String::new();
    loop {
        let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
        match event {
            Event::End(_) => {
                flush_text(&mut text_run, &mut children);
                return Ok(BNode::Element { name, attrs, children });
            }
            Event::Start(child_start) => {
                flush_text(&mut text_run, &mut children);
                children.push(parse_element(reader, child_start)?);
            }
            Event::Empty(child_start) => {
                flush_text(&mut text_run, &mut children);
                children.push(BNode::Element { name: child_start.name().as_ref().to_string(), attrs: read_attrs(&child_start)?, children: Vec::new() });
            }
            Event::Text(text) => text_run.push_str(text.as_ref()),
            Event::GeneralRef(reference) => text_run.push_str(&resolve_general_ref(&reference)?),
            Event::CData(_) => return Err(format!("unexpected CDATA inside <{name}> — BCF-XML 2.1 parts never carry it")),
            Event::Comment(_) | Event::PI(_) => {}
            Event::Eof => return Err(format!("unclosed element <{name}>: unexpected end of input")),
            Event::Decl(_) | Event::DocType(_) => return Err(format!("declaration/doctype cannot appear inside element <{name}>")),
        }
    }
}

pub(super) fn parse_xml(bytes: &[u8]) -> Result<BNode, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let mut reader = Reader::from_str(text);
    let mut root: Option<BNode> = None;
    loop {
        let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
        match event {
            Event::Eof => break,
            Event::Decl(_) | Event::PI(_) | Event::Comment(_) | Event::DocType(_) => {}
            Event::Text(text) => {
                if !text.as_ref().trim().is_empty() {
                    return Err("unexpected text outside the root element".to_string());
                }
            }
            Event::GeneralRef(reference) => return Err(format!("unexpected entity reference &{}; outside the root element", reference.as_ref())),
            Event::CData(_) => return Err("unexpected CDATA outside the root element".to_string()),
            Event::Start(start) => {
                if root.is_some() {
                    return Err("multiple root elements".to_string());
                }
                root = Some(parse_element(&mut reader, start)?);
            }
            Event::Empty(start) => {
                if root.is_some() {
                    return Err("multiple root elements".to_string());
                }
                root = Some(BNode::Element { name: start.name().as_ref().to_string(), attrs: read_attrs(&start)?, children: Vec::new() });
            }
            Event::End(_) => return Err("unexpected closing tag before the root element".to_string()),
        }
    }
    root.ok_or_else(|| "document has no root element".to_string())
}
//#endregion 🔖️XmlParse

//#region 🔖️XmlWrite
fn write_node<W: std::io::Write>(writer: &mut Writer<W>, node: &BNode) -> Result<(), String> {
    match node {
        BNode::Text(text) => writer.write_event(Event::Text(BytesText::new(text))).map_err(|error| error.to_string()),
        BNode::Element { name, attrs, children } => {
            let mut start = BytesStart::new(name.as_str());
            for (key, value) in attrs {
                start.push_attribute((key.as_str(), value.as_str()));
            }
            if children.is_empty() {
                return writer.write_event(Event::Empty(start)).map_err(|error| error.to_string());
            }
            writer.write_event(Event::Start(start)).map_err(|error| error.to_string())?;
            for child in children {
                write_node(writer, child)?;
            }
            writer.write_event(Event::End(BytesEnd::new(name.as_str()))).map_err(|error| error.to_string())
        }
    }
}

/// 📝️ Serializes one XML part (`bcf.version`, a `markup.bcf`, a `.bcfv`) with a leading
/// `<?xml version="1.0" encoding="UTF-8"?>` declaration — every write to an in-memory
/// `Cursor<Vec<u8>>` here, so failure is not a real possibility for a tree this module built
/// itself.
pub(super) fn write_xml(root: &BNode) -> Vec<u8> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None))).expect("declaration write to an in-memory buffer cannot fail");
    write_node(&mut writer, root).expect("element write to an in-memory buffer cannot fail");
    writer.into_inner().into_inner()
}
//#endregion 🔖️XmlWrite

//#region 🔖️Archive
/// 🎒️ Reads every non-directory ZIP member with the registered `zip` reference reader.
pub(super) fn read_entries(input: &[u8]) -> Result<Vec<(String, Vec<u8>)>, String> {
    use std::io::Read;
    let mut archive = zip::ZipArchive::new(Cursor::new(input.to_vec())).map_err(|error| format!("independent reader could not parse the ZIP: {error}"))?;
    let mut entries = Vec::with_capacity(archive.len());
    for index in 0..archive.len() {
        let mut member = archive.by_index(index).map_err(|error| format!("independent reader could not read ZIP entry {index}: {error}"))?;
        if member.is_dir() {
            continue;
        }
        let name = member.name().to_string();
        let mut data = Vec::new();
        member.read_to_end(&mut data).map_err(|error| format!("independent reader could not decompress {name}: {error}"))?;
        entries.push((name, data));
    }
    Ok(entries)
}

/// 🎒️ Writes every ZIP member with the registered `zip` reference writer, deflated — the same
/// flat bcfzip shape `bcfzip` uses (no OPC apparatus, per the format's own definition).
pub(super) fn write_entries(entries: &[(String, Vec<u8>)]) -> Result<Vec<u8>, String> {
    use std::io::Write;
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::<u8>::new()));
    let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name, data) in entries {
        writer.start_file(name.clone(), options).map_err(|error| format!("zip start_file {name}: {error}"))?;
        writer.write_all(data).map_err(|error| format!("zip write {name}: {error}"))?;
    }
    Ok(writer.finish().map_err(|error| format!("zip finish: {error}"))?.into_inner())
}
//#endregion 🔖️Archive
