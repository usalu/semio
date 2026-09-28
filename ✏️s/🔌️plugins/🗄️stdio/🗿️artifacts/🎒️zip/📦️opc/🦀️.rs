//! 📦️ OPC — Open Packaging Conventions (ECMA-376 Part 2 / ISO 29500-2 §9-10), the zip+XML
//! container shape shared by every OOXML format (`📜️docx`/`📕️xlsx`/`🎞️pptx`). Real zip parsing is
//! reused from `crate::standards::v2_0::subsets::base::io::{decode_zip, encode_zip}` and real XML parsing from
//! `semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, xml_document_to_text}` —
//! neither is reimplemented here. This module owns exactly two typed metadata channels
//! (`[Content_Types].xml` and every `*.rels` file) plus the verbatim byte payload of every other
//! part. Metadata XML extension nodes and archive headers need their own retained representation.

use std::collections::{HashMap, HashSet};

use crate::schema::snapshot::ZipEntry;
use crate::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_xml::schema::snapshot::{validate_xml_document_boundaries, xml_document_from_text, xml_document_to_text_checked, XmlAttr, XmlDocument, XmlNode};

//#region 🔖️Error
/// ⚠️ Typed OPC decode/encode failure — an unreadable or non-conformant container never silently
/// decodes into a partial/fabricated package.
#[derive(Clone, Debug, PartialEq)]
pub enum OpcError {
    Zip(String),
    Xml { part: String, detail: String },
    MissingContentTypes,
    MalformedContentTypes(String),
    MalformedRelationships { part: String, detail: String },
    Malformed(String),
}

impl std::fmt::Display for OpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zip(e) => write!(f, "opc: zip layer: {e}"),
            Self::Xml { part, detail } => write!(f, "opc: xml parse of {part}: {detail}"),
            Self::MissingContentTypes => write!(f, "opc: missing [Content_Types].xml"),
            Self::MalformedContentTypes(detail) => write!(f, "opc: malformed [Content_Types].xml: {detail}"),
            Self::MalformedRelationships { part, detail } => write!(f, "opc: malformed relationships in {part}: {detail}"),
            Self::Malformed(detail) => write!(f, "opc: {detail}"),
        }
    }
}

impl std::error::Error for OpcError {}
//#endregion 🔖️Error

//#region 🔖️Constants
/// 📄️ The fixed, case-sensitive part name every OPC package's content-type table lives at.
pub const CONTENT_TYPES_PART: &str = "[Content_Types].xml";
/// 🏷️ Content type of every `*.rels` part (ECMA-376 Part 2 §9.2.1).
pub const RELS_CONTENT_TYPE: &str = "application/vnd.openxmlformats-package.relationships+xml";
const CONTENT_TYPES_NS: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
const RELATIONSHIPS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
/// 🔗️ Relationship type of the package-level pointer to a format's primary "root" part
/// (e.g. `word/document.xml`, `xl/workbook.xml`, `ppt/presentation.xml`).
pub const REL_TYPE_OFFICE_DOCUMENT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_attr(name: &str, value: &str) -> XmlAttr {
    XmlAttr { name: name.into(), value: value.into() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_elem(name: &str, attrs: Vec<XmlAttr>, children: Vec<XmlNode>) -> XmlNode {
    XmlNode::Element { name: name.into(), attrs, children }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn opc_escape_text(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn opc_escape_attr(value: &str) -> String {
    opc_escape_text(value).replace('"', "&quot;").replace('\t', "&#x9;").replace('\n', "&#xA;").replace('\r', "&#xD;")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn opc_node_to_text(node: &XmlNode, out: &mut String) {
    match node {
        XmlNode::Text { text } => out.push_str(&opc_escape_text(text)),
        XmlNode::CData { text } => {
            out.push_str("<![CDATA[");
            out.push_str(text);
            out.push_str("]]>");
        }
        XmlNode::Comment { text } => {
            out.push_str("<!--");
            out.push_str(text);
            out.push_str("-->");
        }
        XmlNode::ProcessingInstruction { target, data } => {
            out.push_str("<?");
            out.push_str(target);
            if !data.is_empty() {
                out.push(' ');
                out.push_str(data);
            }
            out.push_str("?>");
        }
        XmlNode::Element { name, attrs, children } => {
            out.push('<');
            out.push_str(name);
            for attr in attrs {
                out.push(' ');
                out.push_str(&attr.name);
                out.push_str("=\"");
                out.push_str(&opc_escape_attr(&attr.value));
                out.push('"');
            }
            if children.is_empty() {
                out.push_str("/>");
            } else {
                out.push('>');
                for child in children {
                    opc_node_to_text(child, out);
                }
                out.push_str("</");
                out.push_str(name);
                out.push('>');
            }
        }
    }
}

/// 📝️ Deterministically materializes logical OPC XML using the compact ECMA-376 convention.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xml_document_to_opc_text(doc: &XmlDocument) -> String {
    xml_document_to_opc_text_checked(doc).expect("valid OPC XML document boundaries")
}

/// 📝️ Deterministically materializes validated logical OPC XML using the compact ECMA-376 convention.
pub fn xml_document_to_opc_text_checked(doc: &XmlDocument) -> Result<String, String> {
    validate_xml_document_boundaries(doc)?;
    if doc.doctype.is_some() {
        return xml_document_to_text_checked(doc);
    }
    let mut out = String::new();
    if let Some(declaration) = &doc.declaration {
        out.push_str("<?xml version=\"");
        out.push_str(&declaration.version);
        out.push('"');
        if let Some(encoding) = &declaration.encoding {
            out.push_str(" encoding=\"");
            out.push_str(encoding);
            out.push('"');
        }
        if let Some(standalone) = declaration.standalone {
            out.push_str(" standalone=\"");
            out.push_str(if standalone { "yes" } else { "no" });
            out.push('"');
        }
        out.push_str("?>\r\n");
    }
    for node in &doc.prolog {
        opc_node_to_text(node, &mut out);
    }
    if let Some(root) = &doc.root {
        opc_node_to_text(root, &mut out);
    }
    for node in &doc.epilog {
        opc_node_to_text(node, &mut out);
    }
    Ok(out)
}
//#endregion 🔖️Constants

//#region 🔖️Part
/// 📦️ One package part: its name (no leading `/`), resolved content type, and verbatim bytes.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct OpcPart {
    pub path: String,
    pub content_type: String,
    #[value(default)]
    pub bytes: Vec<u8>,
}
//#endregion 🔖️Part

//#region 🔖️ContentTypes
/// 🏷️ Typed `[Content_Types].xml`: `Default` entries retain extension spelling (no dot),
/// `Override` entries key by absolute part name (`/word/document.xml`). Overrides win.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct OpcContentTypes {
    #[value(default)]
    pub defaults: Vec<(String, String)>,
    #[value(default)]
    pub overrides: Vec<(String, String)>,
}

impl OpcContentTypes {
    fn validate_identities(&self) -> Result<(), OpcError> {
        let mut extensions = HashSet::new();
        for (extension, _) in &self.defaults {
            if !extensions.insert(extension.to_ascii_lowercase()) {
                return Err(OpcError::MalformedContentTypes(format!("duplicate default extension: {extension}")));
            }
        }
        let mut parts = HashSet::new();
        for (part, _) in &self.overrides {
            if !parts.insert(part) {
                return Err(OpcError::MalformedContentTypes(format!("duplicate content type override: {part}")));
            }
        }
        Ok(())
    }

    /// 🔎️ Resolves the content type for `part_path` (no leading `/`): override by exact part
    /// name first, else default by extension. `None` when neither applies — the caller decides
    /// whether that is fatal.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn resolve(&self, part_path: &str) -> Option<&str> {
        let part_name = format!("/{}", part_path.trim_start_matches('/'));
        if let Some((_, ct)) = self.overrides.iter().find(|(p, _)| *p == part_name) {
            return Some(ct);
        }
        let ext = part_path.rsplit('.').next()?.to_ascii_lowercase();
        self.defaults.iter().find(|(e, _)| e.eq_ignore_ascii_case(&ext)).map(|(_, ct)| ct.as_str())
    }

    /// ✍️ Inserts or replaces the `Override` entry for `part_path`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_override(&mut self, part_path: &str, content_type: &str) {
        let part_name = format!("/{}", part_path.trim_start_matches('/'));
        if let Some(existing) = self.overrides.iter_mut().find(|(p, _)| *p == part_name) {
            existing.1 = content_type.to_string();
        } else {
            self.overrides.push((part_name, content_type.to_string()));
        }
    }

    /// ✍️ Inserts or replaces the `Default` entry for `extension` (case-insensitive, no dot).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_default(&mut self, extension: &str, content_type: &str) {
        let ext = extension.to_ascii_lowercase();
        if let Some(existing) = self.defaults.iter_mut().find(|(e, _)| e.eq_ignore_ascii_case(&ext)) {
            existing.1 = content_type.to_string();
        } else {
            self.defaults.push((ext, content_type.to_string()));
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn to_xml(&self) -> XmlDocument {
        let mut children = Vec::with_capacity(self.defaults.len() + self.overrides.len());
        for (ext, ct) in &self.defaults {
            children.push(xml_elem("Default", vec![xml_attr("Extension", ext), xml_attr("ContentType", ct)], vec![]));
        }
        for (part, ct) in &self.overrides {
            children.push(xml_elem("Override", vec![xml_attr("PartName", part), xml_attr("ContentType", ct)], vec![]));
        }
        XmlDocument {
            prolog: Vec::new(),
            epilog: Vec::new(),
            root: Some(xml_elem("Types", vec![xml_attr("xmlns", CONTENT_TYPES_NS)], children)),
            doctype: None,
            declaration: Some(semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true), ..Default::default() }),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn from_xml(doc: &XmlDocument) -> Result<Self, OpcError> {
        let root = doc.root.as_ref().ok_or(OpcError::MissingContentTypes)?;
        let XmlNode::Element { name, attrs: root_attrs, children } = root else {
            return Err(OpcError::MalformedContentTypes("root is not an element".into()));
        };
        if !metadata_element_name(name, root_attrs, &[], "Types", CONTENT_TYPES_NS) {
            return Err(OpcError::MalformedContentTypes(format!("expected <Types>, got <{name}>")));
        }
        let mut out = OpcContentTypes::default();
        for child in children {
            let XmlNode::Element { name, attrs, .. } = child else { continue };
            if metadata_element_name(name, attrs, root_attrs, "Default", CONTENT_TYPES_NS) {
                let ext = find_attr(attrs, "Extension").ok_or_else(|| OpcError::MalformedContentTypes("<Default> missing Extension".into()))?;
                let ct = find_attr(attrs, "ContentType").ok_or_else(|| OpcError::MalformedContentTypes("<Default> missing ContentType".into()))?;
                out.defaults.push((ext.to_string(), ct.to_string()));
            } else if metadata_element_name(name, attrs, root_attrs, "Override", CONTENT_TYPES_NS) {
                let part = find_attr(attrs, "PartName").ok_or_else(|| OpcError::MalformedContentTypes("<Override> missing PartName".into()))?;
                let ct = find_attr(attrs, "ContentType").ok_or_else(|| OpcError::MalformedContentTypes("<Override> missing ContentType".into()))?;
                out.overrides.push((part.to_string(), ct.to_string()));
            }
        }
        out.validate_identities()?;
        Ok(out)
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_attr<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}

fn metadata_element_name(name: &str, attrs: &[XmlAttr], inherited: &[XmlAttr], local: &str, namespace: &str) -> bool {
    let (prefix, actual) = name.split_once(':').map_or((None, name), |(prefix, local)| (Some(prefix), local));
    if actual != local {
        return false;
    }
    let binding = prefix.map_or_else(|| "xmlns".into(), |prefix| format!("xmlns:{prefix}"));
    find_attr(attrs, &binding).or_else(|| find_attr(inherited, &binding)) == Some(namespace)
}
//#endregion 🔖️ContentTypes

//#region 🔖️Relationships
/// 🎯️ Whether a relationship's `Target` is a package-internal part path or an external URI.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum OpcTargetMode {
    Internal,
    External,
}

/// 🔗️ One `<Relationship>` entry from some owner part's `*.rels` file.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct OpcRelationship {
    pub id: String,
    pub rel_type: String,
    pub target: String,
    pub target_mode: OpcTargetMode,
}

/// 🆔️ Reserves the lowest available relationship identity in one owner namespace.
pub fn fresh_relationship_id(taken: &mut Vec<String>) -> String {
    let occupied: HashSet<&str> = taken.iter().map(String::as_str).collect();
    let mut number = 1usize;
    let id = loop {
        let candidate = format!("rId{number}");
        if !occupied.contains(candidate.as_str()) {
            break candidate;
        }
        number += 1;
    };
    taken.push(id.clone());
    id
}

/// 📍 The `*.rels` part path that carries `owner`'s relationships (`""` = package root ->
/// `_rels/.rels`; `"word/document.xml"` -> `"word/_rels/document.xml.rels"`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rels_part_path_for(owner: &str) -> String {
    if owner.is_empty() {
        "_rels/.rels".into()
    } else if let Some(slash) = owner.rfind('/') {
        format!("{}/_rels/{}.rels", &owner[..slash], &owner[slash + 1..])
    } else {
        format!("_rels/{owner}.rels")
    }
}

/// 📍 Inverse of `rels_part_path_for`: recovers the owner part path from a `*.rels` part path.
/// `None` when `path` isn't shaped like a rels part at all (should never happen for a
/// conformant package, but never silently misattributed either).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn owner_for_rels_path(path: &str) -> Option<String> {
    let file = path.rsplit('/').next()?;
    let name = file.strip_suffix(".rels")?;
    let dir = &path[..path.len() - file.len()];
    let dir = dir.strip_suffix("_rels/")?;
    let name = if name == "." { "" } else { name };
    Some(format!("{dir}{name}"))
}

/// 🧭️ Resolves a relationship `Target` against the directory of its owner part (OPC §9.3:
/// relative targets are resolved relative to the *source part's* base URI, not the package
/// root). A leading `/` is package-root-absolute.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn resolve_relationship_target(owner: &str, target: &str) -> String {
    if let Some(stripped) = target.strip_prefix('/') {
        return stripped.to_string();
    }
    let base_dir = match owner.rfind('/') {
        Some(slash) => &owner[..=slash],
        None => "",
    };
    normalize_path(&format!("{base_dir}{target}"))
}

/// 🧹️ Collapses `./` and `../` segments in a `/`-joined path (no filesystem access — pure
/// string logic, since OPC part paths are always package-internal).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn relationships_to_xml(rels: &[OpcRelationship]) -> XmlDocument {
    let children = rels
        .iter()
        .map(|r| {
            let mut attrs = vec![xml_attr("Id", &r.id), xml_attr("Type", &r.rel_type), xml_attr("Target", &r.target)];
            if r.target_mode == OpcTargetMode::External {
                attrs.push(xml_attr("TargetMode", "External"));
            }
            xml_elem("Relationship", attrs, vec![])
        })
        .collect();
    XmlDocument {
        prolog: Vec::new(),
        epilog: Vec::new(),
        root: Some(xml_elem("Relationships", vec![xml_attr("xmlns", RELATIONSHIPS_NS)], children)),
        doctype: None,
        declaration: Some(semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true), ..Default::default() }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn relationships_from_xml(doc: &XmlDocument, part: &str) -> Result<Vec<OpcRelationship>, OpcError> {
    let malformed = |detail: String| OpcError::MalformedRelationships { part: part.into(), detail };
    let root = doc.root.as_ref().ok_or_else(|| malformed("empty document".into()))?;
    let XmlNode::Element { name, attrs: root_attrs, children } = root else {
        return Err(malformed("root is not an element".into()));
    };
    if !metadata_element_name(name, root_attrs, &[], "Relationships", RELATIONSHIPS_NS) {
        return Err(malformed(format!("expected package Relationships root, got <{name}>")));
    }
    let mut out = Vec::new();
    for child in children {
        let XmlNode::Element { name, attrs, .. } = child else { continue };
        if !metadata_element_name(name, attrs, root_attrs, "Relationship", RELATIONSHIPS_NS) {
            continue;
        }
        let id = find_attr(attrs, "Id").ok_or_else(|| malformed("<Relationship> missing Id".into()))?.to_string();
        let rel_type = find_attr(attrs, "Type").ok_or_else(|| malformed("<Relationship> missing Type".into()))?.to_string();
        let target = find_attr(attrs, "Target").ok_or_else(|| malformed("<Relationship> missing Target".into()))?.to_string();
        let target_mode = match find_attr(attrs, "TargetMode") {
            Some("External") => OpcTargetMode::External,
            None | Some("Internal") => OpcTargetMode::Internal,
            Some(mode) => return Err(malformed(format!("invalid relationship target mode: {mode}"))),
        };
        out.push(OpcRelationship { id, rel_type, target, target_mode });
    }
    validate_relationship_identities(&out, part)?;
    Ok(out)
}

fn validate_relationship_identities(relationships: &[OpcRelationship], part: &str) -> Result<(), OpcError> {
    let mut ids = HashSet::new();
    for relationship in relationships {
        if !ids.insert(relationship.id.as_str()) {
            return Err(OpcError::MalformedRelationships { part: part.into(), detail: format!("duplicate relationship identity: {}", relationship.id) });
        }
    }
    Ok(())
}
//#endregion 🔖️Relationships

//#region 🔖️Package
/// 📦️ A fully decoded OPC package: every non-metadata part verbatim, plus the two typed
/// metadata channels (content types, relationships-by-owner) that `docx`/`xlsx`/`pptx` interpret
/// semantically on top of. Content-part payloads remain verbatim; metadata tables retain their
/// declared semantic fields.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct OpcPackage {
    #[value(default)]
    pub parts: Vec<OpcPart>,
    #[value(default)]
    pub content_types: OpcContentTypes,
    /// 🗺️ Owner part path (`""` = package root) -> that owner's relationships.
    #[value(default)]
    pub relationships: HashMap<String, Vec<OpcRelationship>>,
    #[value(default)]
    pub comment: String,
}

impl OpcPackage {
    /// 🌱️ An empty package (no parts, no content types, no relationships) — callers building a
    /// fresh document from scratch start here and add parts/relationships/content-types.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn empty() -> Self {
        Self::default()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn part(&self, path: &str) -> Option<&OpcPart> {
        let p = path.trim_start_matches('/');
        self.parts.iter().find(|part| part.path == p)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn part_bytes(&self, path: &str) -> Option<&[u8]> {
        self.part(path).map(|p| p.bytes.as_slice())
    }

    /// ✍️ Inserts or replaces a content part, keeping its `[Content_Types].xml` `Override` in
    /// sync in the same call — the two can never drift apart through this API.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_part(&mut self, path: &str, content_type: &str, bytes: Vec<u8>) {
        let p = path.trim_start_matches('/').to_string();
        self.content_types.set_override(&p, content_type);
        if let Some(existing) = self.parts.iter_mut().find(|part| part.path == p) {
            existing.bytes = bytes;
            existing.content_type = content_type.to_string();
        } else {
            self.parts.push(OpcPart { path: p, content_type: content_type.to_string(), bytes });
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn relationships_for(&self, owner: &str) -> &[OpcRelationship] {
        self.relationships.get(owner).map_or(&[][..], |v| v.as_slice())
    }

    /// ✍️ Appends one internal relationship under `owner` (`""` = package root).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_relationship(&mut self, owner: &str, id: &str, rel_type: &str, target: &str) {
        self.relationships.entry(owner.to_string()).or_default().push(OpcRelationship { id: id.into(), rel_type: rel_type.into(), target: target.into(), target_mode: OpcTargetMode::Internal });
    }

    /// 🧷️ Adds a relationship without overwriting any owner-local identity.
    pub fn add_generated_relationship(&mut self, owner: &str, rel_type: &str, target: &str) -> String {
        let mut taken = self.relationships_for(owner).iter().map(|relationship| relationship.id.clone()).collect();
        let id = fresh_relationship_id(&mut taken);
        self.add_relationship(owner, &id, rel_type, target);
        id
    }

    /// 🔎️ Follows a single relationship of `rel_type` owned by `owner`, resolving its target to
    /// an absolute part path. `None` when no such relationship exists.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn resolve_relationship(&self, owner: &str, rel_type: &str) -> Option<String> {
        let rel = self.relationships_for(owner).iter().find(|r| r.rel_type == rel_type)?;
        Some(resolve_relationship_target(owner, &rel.target))
    }
}

/// 📦️ Decode OPC container bytes (a real zip archive) into a typed `OpcPackage`.
/// Every zip entry becomes exactly one of: the typed `content_types` table, a typed
/// relationship list, or a verbatim content `OpcPart`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_opc(data: &[u8]) -> Result<OpcPackage, OpcError> {
    let zip = crate::standards::v2_0::subsets::base::io::decode_zip(data).map_err(|e| OpcError::Zip(e.to_string()))?;
    let mut paths = HashSet::with_capacity(zip.entries.len());
    for entry in &zip.entries {
        if !paths.insert(entry.name.as_str()) {
            return Err(OpcError::Malformed(format!("duplicate package member: {}", entry.name)));
        }
    }

    let ct_entry = zip.entries.iter().find(|e| e.name == CONTENT_TYPES_PART).ok_or(OpcError::MissingContentTypes)?;
    let ct_text = String::from_utf8(ct_entry.data.clone()).map_err(|_| OpcError::MalformedContentTypes("not valid utf-8".into()))?;
    let ct_doc = xml_document_from_text(&ct_text).map_err(|e| OpcError::Xml { part: CONTENT_TYPES_PART.into(), detail: e })?;
    let content_types = OpcContentTypes::from_xml(&ct_doc)?;

    let mut parts = Vec::new();
    let mut relationships: HashMap<String, Vec<OpcRelationship>> = HashMap::new();

    for entry in &zip.entries {
        if entry.name == CONTENT_TYPES_PART {
            continue;
        }
        if entry.name.ends_with(".rels") {
            let text = String::from_utf8(entry.data.clone()).map_err(|_| OpcError::MalformedRelationships { part: entry.name.clone(), detail: "not valid utf-8".into() })?;
            let doc = xml_document_from_text(&text).map_err(|e| OpcError::Xml { part: entry.name.clone(), detail: e })?;
            let rels = relationships_from_xml(&doc, &entry.name)?;
            let owner = owner_for_rels_path(&entry.name).ok_or_else(|| OpcError::Malformed(format!("relationship part at unexpected path: {}", entry.name)))?;
            relationships.insert(owner, rels);
            continue;
        }
        let content_type = content_types.resolve(&entry.name).ok_or_else(|| OpcError::Malformed(format!("part {} has no resolvable content type", entry.name)))?.to_string();
        parts.push(OpcPart { path: entry.name.clone(), content_type, bytes: entry.data.clone() });
    }

    Ok(OpcPackage { parts, content_types, relationships, comment: zip.comment })
}

/// 📦️ Re-encode an `OpcPackage` as OPC container bytes: `[Content_Types].xml` and every owner's
/// `*.rels` file are regenerated from the typed tables (never carried as stray verbatim parts —
/// see `decode_opc`), every content part is re-emitted deflated via the zip artifact's real
/// codec. Content parts retain their authored sequence; format serializers can supply a
/// standard-specific member sequence through `encode_opc_with_path_order`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_opc(pkg: &OpcPackage) -> Result<Vec<u8>, OpcError> {
    encode_opc_with_package_order(pkg)
}

/// 📦️ Re-encode an OPC package with metadata entries first and every remaining entry in the
/// package's authoritative content-part order. Metadata members use their declared package order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_opc_with_package_order(pkg: &OpcPackage) -> Result<Vec<u8>, OpcError> {
    encode_opc_with_path_order(pkg, |paths| {
        let mut ordered = Vec::with_capacity(paths.len());
        let mut pending: std::collections::HashSet<String> = std::mem::take(paths).into_iter().collect();
        let mut take = |path: String| {
            if let Some(path) = pending.take(&path) {
                ordered.push(path);
            }
        };
        take(CONTENT_TYPES_PART.into());
        if pkg.relationships.contains_key("") {
            take("_rels/.rels".into());
        }
        for part in &pkg.parts {
            let rels_path = if let Some((directory, file)) = part.path.rsplit_once('/') { format!("{directory}/_rels/{file}.rels") } else { format!("_rels/{}.rels", part.path) };
            if pkg.relationships.contains_key(&part.path) {
                take(rels_path);
            }
        }
        for part in &pkg.parts {
            take(part.path.clone());
        }

        let mut remaining: Vec<String> = pending.into_iter().collect();
        remaining.sort();
        ordered.append(&mut remaining);
        *paths = ordered;
    })
}

/// 🧭 Re-encodes an OPC package after applying a caller-owned deterministic path order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_opc_with_path_order(pkg: &OpcPackage, order: impl FnOnce(&mut Vec<String>)) -> Result<Vec<u8>, OpcError> {
    pkg.content_types.validate_identities()?;
    for (owner, relationships) in &pkg.relationships {
        validate_relationship_identities(relationships, &rels_part_path_for(owner))?;
    }
    for part in &pkg.parts {
        if pkg.content_types.resolve(&part.path) != Some(part.content_type.as_str()) {
            return Err(OpcError::Malformed(format!("content type disagrees with metadata for part: {}", part.path)));
        }
    }
    let mut payloads = HashMap::<String, Vec<u8>>::new();
    let ct_text = xml_document_to_opc_text_checked(&pkg.content_types.to_xml()).map_err(|detail| OpcError::Xml { part: CONTENT_TYPES_PART.into(), detail })?;
    payloads.insert(CONTENT_TYPES_PART.into(), ct_text.into_bytes());

    let mut owners: Vec<&String> = pkg.relationships.keys().collect();
    owners.sort();
    for owner in owners {
        let rels = &pkg.relationships[owner];
        let path = rels_part_path_for(owner);
        if owner_for_rels_path(&path).as_ref() != Some(owner) {
            return Err(OpcError::Malformed(format!("invalid relationship owner: {owner}")));
        }
        let text = xml_document_to_opc_text_checked(&relationships_to_xml(rels)).map_err(|detail| OpcError::Xml { part: path.clone(), detail })?;
        if payloads.insert(path.clone(), text.into_bytes()).is_some() {
            return Err(OpcError::Malformed(format!("duplicate relationship part: {path}")));
        }
    }

    for part in &pkg.parts {
        if part.path == CONTENT_TYPES_PART || part.path.ends_with(".rels") || payloads.contains_key(&part.path) {
            return Err(OpcError::Malformed(format!("ambiguous content part: {}", part.path)));
        }
        payloads.insert(part.path.clone(), part.bytes.clone());
    }

    let mut entries = Vec::with_capacity(payloads.len());
    let mut new_paths: Vec<String> = payloads.keys().cloned().collect();
    order(&mut new_paths);
    if new_paths.len() != payloads.len() {
        return Err(OpcError::Malformed("package member order must include every path exactly once".into()));
    }
    for path in &new_paths {
        let data = payloads.remove(path).ok_or_else(|| OpcError::Malformed(format!("unknown or repeated package member in order: {path}")))?;
        entries.push(ZipEntry { name: path.clone(), data, ..Default::default() });
    }

    let snap = ZipSnapshot { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries, comment: pkg.comment.clone(), ..Default::default() };
    crate::standards::v2_0::subsets::base::io::encode_zip_with_entry_names(&snap, &new_paths).map_err(|e| OpcError::Zip(e.to_string()))
}

/// 🕵️ Structural sniff of OOXML-shaped bytes: recognizes the zip magic *and* the presence of a
/// `[Content_Types].xml` entry — real OOXML disambiguation from a plain zip peeks part names
/// (docx/xlsx/pptx callers inspect `word/`/`xl/`/`ppt/`-prefixed parts on top of this).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_opc_bytes(data: &[u8]) -> bool {
    let Ok(zip) = crate::standards::v2_0::subsets::base::io::decode_zip(data) else { return false };
    zip.entries.iter().any(|e| e.name == CONTENT_TYPES_PART)
}
//#endregion 🔖️Package

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
