//! 🌳️ The generic XML node tree with its reader and writer, and the OPC container types, relationships and zip read and write.

use super::*;

//#region 🔖️GenericXmlTree
/// 🌳 Owned XML node, used ONLY for the small typed OPC/WordprocessingML documents this oracle
/// reads (`[Content_Types].xml`, `*.rels`, `word/document.xml`, `word/styles.xml`) — no CDATA,
/// comment or processing-instruction support, since a real OOXML part never emits any of those
/// (unlike `📰️xml`'s general-purpose oracle, which has to).
#[derive(Clone, Debug, PartialEq)]
pub(super) enum XNode {
    Element { name: String, attrs: Vec<(String, String)>, children: Vec<XNode> },
    Text(String),
}

pub(super) fn elem(name: &str, attrs: Vec<(&str, String)>, children: Vec<XNode>) -> XNode {
    XNode::Element { name: name.to_string(), attrs: attrs.into_iter().map(|(k, v)| (k.to_string(), v)).collect(), children }
}

pub(super) fn find_attr<'a>(attrs: &'a [(String, String)], name: &str) -> Option<&'a str> {
    attrs.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
}

/// 🔓️ Resolves one `Event::GeneralRef` (`&name;` or `&#NNN;`) to its literal text — numeric
/// character references via `resolve_char_ref`, the five predefined XML entities via
/// `resolve_xml_entity`, anything else a hard parse error. Same technique
/// `📰️xml/…/🦀️oracle.rs`'s own `resolve_general_ref` uses, independently rewritten
/// here (this file imports nothing from that module).
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

/// 🌳 Recursive-descent element parse: reads events until this element's own `End`, recursing
/// into `Start` children. Comments/CDATA/PI are rejected — real OOXML parts never carry them.
fn parse_element(reader: &mut Reader<&[u8]>, start: BytesStart) -> Result<XNode, String> {
    let name = start.name().as_ref().to_string();
    let attrs = read_attrs(&start)?;
    let mut children = Vec::new();
    let mut text_run = String::new();
    loop {
        let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {} in <{name}>: {error}", reader.error_position()))?;
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
            Event::Eof => return Err(format!("unclosed element <{name}>: unexpected end of input")),
            other => return Err(format!("unexpected {other:?} inside <{name}> (OOXML parts carry no comments/CDATA/PI)")),
        }
    }
}

/// 📄 Parses a real OOXML/OPC part's bytes into its root `XNode`, skipping the leading
/// declaration (this oracle re-emits its own on write; the declaration itself carries no
/// semantics this subset's vocabulary ever mutates).
pub(super) fn parse_xml(bytes: &[u8]) -> Result<XNode, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let mut reader = Reader::from_str(text);
    loop {
        let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
        match event {
            Event::Decl(_) => continue,
            Event::Text(text) if text.as_ref().trim().is_empty() => continue,
            Event::Start(start) => return parse_element(&mut reader, start),
            Event::Empty(start) => return Ok(XNode::Element { name: start.name().as_ref().to_string(), attrs: read_attrs(&start)?, children: Vec::new() }),
            Event::Eof => return Err("document has no root element".to_string()),
            other => return Err(format!("unexpected {other:?} before the root element")),
        }
    }
}

fn write_node<W: Write>(writer: &mut Writer<W>, node: &XNode) -> Result<(), String> {
    match node {
        XNode::Text(text) => writer.write_event(Event::Text(BytesText::new(text))).map_err(|error| error.to_string()),
        XNode::Element { name, attrs, children } => {
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

/// 📝 Serializes `root` as a real, standalone-declared OOXML part — the same declaration form
/// every real Word-produced part carries.
pub(super) fn write_xml(root: &XNode) -> Result<Vec<u8>, String> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), Some("yes")))).map_err(|error| error.to_string())?;
    write_node(&mut writer, root)?;
    Ok(writer.into_inner().into_inner())
}
//#endregion 🔖️GenericXmlTree

//#region 🔖️OpcTypes
const CONTENT_TYPES_PART: &str = "[Content_Types].xml";
const CONTENT_TYPES_NS: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
const RELATIONSHIPS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const RELS_CONTENT_TYPE: &str = "application/vnd.openxmlformats-package.relationships+xml";
pub(super) const REL_TYPE_OFFICE_DOCUMENT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
pub(super) const REL_TYPE_STYLES: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
pub(super) const W_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

#[derive(Clone, Debug, Default)]
struct OContentTypes {
    pub(super) defaults: Vec<(String, String)>,
    pub(super) overrides: Vec<(String, String)>,
}

impl OContentTypes {
    fn resolve(&self, part_path: &str) -> Option<&str> {
        let part_name = format!("/{}", part_path.trim_start_matches('/'));
        if let Some((_, ct)) = self.overrides.iter().find(|(p, _)| *p == part_name) {
            return Some(ct);
        }
        let ext = part_path.rsplit('.').next()?.to_ascii_lowercase();
        self.defaults.iter().find(|(e, _)| *e == ext).map(|(_, ct)| ct.as_str())
    }

    fn set_override(&mut self, part_path: &str, content_type: &str) {
        let part_name = format!("/{}", part_path.trim_start_matches('/'));
        match self.overrides.iter_mut().find(|(p, _)| *p == part_name) {
            Some(existing) => existing.1 = content_type.to_string(),
            None => self.overrides.push((part_name, content_type.to_string())),
        }
    }

    fn from_xnode(node: &XNode) -> Result<Self, String> {
        let XNode::Element { name, children, .. } = node else { return Err("[Content_Types].xml: root is not an element".to_string()) };
        if name != "Types" {
            return Err(format!("[Content_Types].xml: expected <Types>, got <{name}>"));
        }
        let mut out = OContentTypes::default();
        for child in children {
            let XNode::Element { name, attrs, .. } = child else { continue };
            match name.as_str() {
                "Default" => {
                    let ext = find_attr(attrs, "Extension").ok_or("<Default> missing Extension")?;
                    let ct = find_attr(attrs, "ContentType").ok_or("<Default> missing ContentType")?;
                    out.defaults.push((ext.to_ascii_lowercase(), ct.to_string()));
                }
                "Override" => {
                    let part = find_attr(attrs, "PartName").ok_or("<Override> missing PartName")?;
                    let ct = find_attr(attrs, "ContentType").ok_or("<Override> missing ContentType")?;
                    out.overrides.push((part.to_string(), ct.to_string()));
                }
                _ => {}
            }
        }
        Ok(out)
    }

    fn to_xnode(&self) -> XNode {
        let mut children = Vec::with_capacity(self.defaults.len() + self.overrides.len());
        for (ext, ct) in &self.defaults {
            children.push(elem("Default", vec![("Extension", ext.clone()), ("ContentType", ct.clone())], vec![]));
        }
        for (part, ct) in &self.overrides {
            children.push(elem("Override", vec![("PartName", part.clone()), ("ContentType", ct.clone())], vec![]));
        }
        elem("Types", vec![("xmlns", CONTENT_TYPES_NS.to_string())], children)
    }
}

#[derive(Clone, Debug)]
pub(super) struct ORel {
    pub(super) id: String,
    pub(super) rel_type: String,
    pub(super) target: String,
    pub(super) external: bool,
}

fn relationships_from_xnode(node: &XNode) -> Result<Vec<ORel>, String> {
    let XNode::Element { children, .. } = node else { return Err("relationships part: root is not an element".to_string()) };
    let mut out = Vec::new();
    for child in children {
        let XNode::Element { name, attrs, .. } = child else { continue };
        if name != "Relationship" {
            continue;
        }
        out.push(ORel {
            id: find_attr(attrs, "Id").ok_or("<Relationship> missing Id")?.to_string(),
            rel_type: find_attr(attrs, "Type").ok_or("<Relationship> missing Type")?.to_string(),
            target: find_attr(attrs, "Target").ok_or("<Relationship> missing Target")?.to_string(),
            external: find_attr(attrs, "TargetMode") == Some("External"),
        });
    }
    Ok(out)
}

fn relationships_to_xnode(rels: &[ORel]) -> XNode {
    let children = rels
        .iter()
        .map(|r| {
            let mut attrs = vec![("Id", r.id.clone()), ("Type", r.rel_type.clone()), ("Target", r.target.clone())];
            if r.external {
                attrs.push(("TargetMode", "External".to_string()));
            }
            elem("Relationship", attrs, vec![])
        })
        .collect();
    elem("Relationships", vec![("xmlns", RELATIONSHIPS_NS.to_string())], children)
}

/// 📍 The `*.rels` part path that carries `owner`'s relationships (`""` = package root ->
/// `_rels/.rels`; `"word/document.xml"` -> `"word/_rels/document.xml.rels"`). Independent
/// reimplementation of the same OPC §9 convention `semio_s_artifact_stdio_zip::opc` encodes.
fn rels_part_path_for(owner: &str) -> String {
    if owner.is_empty() {
        "_rels/.rels".to_string()
    } else if let Some(slash) = owner.rfind('/') {
        format!("{}/_rels/{}.rels", &owner[..slash], &owner[slash + 1..])
    } else {
        format!("_rels/{owner}.rels")
    }
}

fn owner_for_rels_path(path: &str) -> Option<String> {
    let file = path.rsplit('/').next()?;
    let name = file.strip_suffix(".rels")?;
    let dir = &path[..path.len() - file.len()];
    let dir = dir.strip_suffix("_rels/")?;
    let name = if name == "." { "" } else { name };
    Some(format!("{dir}{name}"))
}

/// 🧭️ Resolves a relationship `Target` against the directory of its owner part (OPC §9.3).
fn resolve_relationship_target(owner: &str, target: &str) -> String {
    if let Some(stripped) = target.strip_prefix('/') {
        return stripped.to_string();
    }
    let base_dir = match owner.rfind('/') {
        Some(slash) => &owner[..=slash],
        None => "",
    };
    normalize_path(&format!("{base_dir}{target}"))
}

fn normalize_path(path: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    for seg in path.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out.join("/")
}

#[derive(Clone, Debug)]
pub(super) struct OPart {
    pub(super) path: String,
    pub(super) content_type: String,
    pub(super) bytes: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct OPackage {
    pub(super) parts: Vec<OPart>,
    pub(super) content_types: OContentTypes,
    /// 🗺️ Owner part path (`""` = package root) -> that owner's relationships.
    pub(super) relationships: HashMap<String, Vec<ORel>>,
}

impl OPackage {
    pub(super) fn part(&self, path: &str) -> Option<&OPart> {
        let p = path.trim_start_matches('/');
        self.parts.iter().find(|part| part.path == p)
    }

    pub(super) fn set_part(&mut self, path: &str, content_type: &str, bytes: Vec<u8>) {
        let p = path.trim_start_matches('/').to_string();
        self.content_types.set_override(&p, content_type);
        match self.parts.iter_mut().find(|part| part.path == p) {
            Some(existing) => {
                existing.bytes = bytes;
                existing.content_type = content_type.to_string();
            }
            None => self.parts.push(OPart { path: p, content_type: content_type.to_string(), bytes }),
        }
    }

    pub(super) fn resolve_relationship(&self, owner: &str, rel_type: &str) -> Option<String> {
        let rel = self.relationships.get(owner)?.iter().find(|r| r.rel_type == rel_type)?;
        Some(resolve_relationship_target(owner, &rel.target))
    }
}

/// 📦 Unzips `bytes` with the registered `zip` reference reader and interprets it as an OPC
/// package: `[Content_Types].xml` and every `*.rels` become the typed metadata tables, every
/// other entry becomes a verbatim `OPart`.
pub(super) fn read_opc(bytes: &[u8]) -> Result<OPackage, String> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes.to_vec())).map_err(|error| format!("independent zip reader could not parse the OPC container: {error}"))?;
    let mut entries: Vec<(String, Vec<u8>)> = Vec::with_capacity(archive.len());
    for index in 0..archive.len() {
        let mut member = archive.by_index(index).map_err(|error| format!("independent zip reader could not read entry {index}: {error}"))?;
        if member.is_dir() {
            continue;
        }
        let name = member.name().to_string();
        let mut data = Vec::new();
        member.read_to_end(&mut data).map_err(|error| format!("independent zip reader could not decompress {name}: {error}"))?;
        entries.push((name, data));
    }

    let (_, ct_bytes) = entries.iter().find(|(name, _)| name == CONTENT_TYPES_PART).ok_or("OPC package has no [Content_Types].xml")?;
    let content_types = OContentTypes::from_xnode(&parse_xml(ct_bytes)?)?;

    let mut parts = Vec::new();
    let mut relationships: HashMap<String, Vec<ORel>> = HashMap::new();
    for (name, data) in &entries {
        if name == CONTENT_TYPES_PART {
            continue;
        }
        if name.ends_with(".rels") {
            let rels = relationships_from_xnode(&parse_xml(data)?)?;
            let owner = owner_for_rels_path(name).ok_or_else(|| format!("relationship part at unexpected path: {name}"))?;
            relationships.insert(owner, rels);
            continue;
        }
        let content_type = content_types.resolve(name).ok_or_else(|| format!("part {name} has no resolvable content type"))?.to_string();
        parts.push(OPart { path: name.clone(), content_type, bytes: data.clone() });
    }
    Ok(OPackage { parts, content_types, relationships })
}

/// 📦 Re-encodes `pkg` as a real OPC container with the registered `zip` reference writer:
/// `[Content_Types].xml` and every owner's `*.rels` are regenerated from the typed tables (never
/// carried as stray verbatim parts), every content part is re-emitted deflated.
pub(super) fn write_opc(pkg: &OPackage) -> Result<Vec<u8>, String> {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::<u8>::new()));
    let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    writer.start_file(CONTENT_TYPES_PART, options).map_err(|error| format!("zip start_file [Content_Types].xml: {error}"))?;
    writer.write_all(&write_xml(&pkg.content_types.to_xnode())?).map_err(|error| error.to_string())?;

    let mut owners: Vec<&String> = pkg.relationships.keys().collect();
    owners.sort();
    for owner in owners {
        let rels = &pkg.relationships[owner];
        if rels.is_empty() {
            continue;
        }
        let rels_path = rels_part_path_for(owner);
        writer.start_file(&rels_path, options).map_err(|error| format!("zip start_file {rels_path}: {error}"))?;
        writer.write_all(&write_xml(&relationships_to_xnode(rels))?).map_err(|error| error.to_string())?;
    }

    for part in &pkg.parts {
        writer.start_file(&part.path, options).map_err(|error| format!("zip start_file {}: {error}", part.path))?;
        writer.write_all(&part.bytes).map_err(|error| error.to_string())?;
    }

    let cursor = writer.finish().map_err(|error| format!("zip finish: {error}"))?;
    Ok(cursor.into_inner())
}
//#endregion 🔖️OpcTypes
