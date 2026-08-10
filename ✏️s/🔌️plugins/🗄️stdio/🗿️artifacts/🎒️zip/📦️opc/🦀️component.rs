//! 📦️ OPC — Open Packaging Conventions (ECMA-376 Part 2 / ISO 29500-2 §9-10), the zip+XML
//! container shape shared by every OOXML format (`📜️docx`/`📕️xlsx`/`🎞️pptx`). Real zip parsing is
//! reused from `crate::artifacts::zip::engine::{decode_zip, encode_zip}` and real XML parsing from
//! `crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text}` —
//! neither is reimplemented here. This module owns exactly two typed metadata channels
//! (`[Content_Types].xml` and every `*.rels` file) plus the verbatim byte payload of every other
//! part — nothing observed in a real package is ever dropped.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::artifacts::xml::schema::snapshot::{
    xml_document_from_text, xml_document_to_text, XmlAttr, XmlDocument, XmlNode,
};
use crate::artifacts::zip::schema::snapshot::{ZipCompressionMethod, ZipEntry};
use crate::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

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

fn xml_attr(name: &str, value: &str) -> XmlAttr {
    XmlAttr { name: name.into(), value: value.into() }
}

fn xml_elem(name: &str, attrs: Vec<XmlAttr>, children: Vec<XmlNode>) -> XmlNode {
    XmlNode::Element { name: name.into(), attrs, children }
}
//#endregion 🔖️Constants

//#region 🔖️Part
/// 📦️ One package part: its name (no leading `/`), resolved content type, and verbatim bytes.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpcPart {
    pub path: String,
    pub content_type: String,
    #[serde(default)]
    pub bytes: Vec<u8>,
}
//#endregion 🔖️Part

//#region 🔖️ContentTypes
/// 🏷️ Typed `[Content_Types].xml`: `Default` entries key by lowercase extension (no dot),
/// `Override` entries key by absolute part name (`/word/document.xml`). Overrides win.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpcContentTypes {
    #[serde(default)]
    pub defaults: Vec<(String, String)>,
    #[serde(default)]
    pub overrides: Vec<(String, String)>,
}

impl OpcContentTypes {
    /// 🔎️ Resolves the content type for `part_path` (no leading `/`): override by exact part
    /// name first, else default by extension. `None` when neither applies — the caller decides
    /// whether that is fatal.
    pub fn resolve(&self, part_path: &str) -> Option<&str> {
        let part_name = format!("/{}", part_path.trim_start_matches('/'));
        if let Some((_, ct)) = self.overrides.iter().find(|(p, _)| *p == part_name) {
            return Some(ct);
        }
        let ext = part_path.rsplit('.').next()?.to_ascii_lowercase();
        self.defaults.iter().find(|(e, _)| *e == ext).map(|(_, ct)| ct.as_str())
    }

    /// ✍️ Inserts or replaces the `Override` entry for `part_path`.
    pub fn set_override(&mut self, part_path: &str, content_type: &str) {
        let part_name = format!("/{}", part_path.trim_start_matches('/'));
        if let Some(existing) = self.overrides.iter_mut().find(|(p, _)| *p == part_name) {
            existing.1 = content_type.to_string();
        } else {
            self.overrides.push((part_name, content_type.to_string()));
        }
    }

    /// ✍️ Inserts or replaces the `Default` entry for `extension` (case-insensitive, no dot).
    pub fn set_default(&mut self, extension: &str, content_type: &str) {
        let ext = extension.to_ascii_lowercase();
        if let Some(existing) = self.defaults.iter_mut().find(|(e, _)| *e == ext) {
            existing.1 = content_type.to_string();
        } else {
            self.defaults.push((ext, content_type.to_string()));
        }
    }

    fn to_xml(&self) -> XmlDocument {
        let mut children = Vec::with_capacity(self.defaults.len() + self.overrides.len());
        for (ext, ct) in &self.defaults {
            children.push(xml_elem("Default", vec![xml_attr("Extension", ext), xml_attr("ContentType", ct)], vec![]));
        }
        for (part, ct) in &self.overrides {
            children.push(xml_elem("Override", vec![xml_attr("PartName", part), xml_attr("ContentType", ct)], vec![]));
        }
        XmlDocument {
            root: Some(xml_elem("Types", vec![xml_attr("xmlns", CONTENT_TYPES_NS)], children)),
            doctype: None,
        }
    }

    fn from_xml(doc: &XmlDocument) -> Result<Self, OpcError> {
        let root = doc.root.as_ref().ok_or(OpcError::MissingContentTypes)?;
        let XmlNode::Element { name, children, .. } = root else {
            return Err(OpcError::MalformedContentTypes("root is not an element".into()));
        };
        if name != "Types" {
            return Err(OpcError::MalformedContentTypes(format!("expected <Types>, got <{name}>")));
        }
        let mut out = OpcContentTypes::default();
        for child in children {
            let XmlNode::Element { name, attrs, .. } = child else { continue };
            match name.as_str() {
                "Default" => {
                    let ext = find_attr(attrs, "Extension")
                        .ok_or_else(|| OpcError::MalformedContentTypes("<Default> missing Extension".into()))?;
                    let ct = find_attr(attrs, "ContentType")
                        .ok_or_else(|| OpcError::MalformedContentTypes("<Default> missing ContentType".into()))?;
                    out.defaults.push((ext.to_ascii_lowercase(), ct.to_string()));
                }
                "Override" => {
                    let part = find_attr(attrs, "PartName")
                        .ok_or_else(|| OpcError::MalformedContentTypes("<Override> missing PartName".into()))?;
                    let ct = find_attr(attrs, "ContentType")
                        .ok_or_else(|| OpcError::MalformedContentTypes("<Override> missing ContentType".into()))?;
                    out.overrides.push((part.to_string(), ct.to_string()));
                }
                _ => {}
            }
        }
        Ok(out)
    }
}

fn find_attr<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}
//#endregion 🔖️ContentTypes

//#region 🔖️Relationships
/// 🎯️ Whether a relationship's `Target` is a package-internal part path or an external URI.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OpcTargetMode {
    Internal,
    External,
}

/// 🔗️ One `<Relationship>` entry from some owner part's `*.rels` file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpcRelationship {
    pub id: String,
    pub rel_type: String,
    pub target: String,
    pub target_mode: OpcTargetMode,
}

/// 📍 The `*.rels` part path that carries `owner`'s relationships (`""` = package root ->
/// `_rels/.rels`; `"word/document.xml"` -> `"word/_rels/document.xml.rels"`).
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
        root: Some(xml_elem("Relationships", vec![xml_attr("xmlns", RELATIONSHIPS_NS)], children)),
        doctype: None,
    }
}

fn relationships_from_xml(doc: &XmlDocument, part: &str) -> Result<Vec<OpcRelationship>, OpcError> {
    let malformed = |detail: String| OpcError::MalformedRelationships { part: part.into(), detail };
    let root = doc.root.as_ref().ok_or_else(|| malformed("empty document".into()))?;
    let XmlNode::Element { children, .. } = root else {
        return Err(malformed("root is not an element".into()));
    };
    let mut out = Vec::new();
    for child in children {
        let XmlNode::Element { name, attrs, .. } = child else { continue };
        if name != "Relationship" {
            continue;
        }
        let id = find_attr(attrs, "Id").ok_or_else(|| malformed("<Relationship> missing Id".into()))?.to_string();
        let rel_type = find_attr(attrs, "Type").ok_or_else(|| malformed("<Relationship> missing Type".into()))?.to_string();
        let target = find_attr(attrs, "Target").ok_or_else(|| malformed("<Relationship> missing Target".into()))?.to_string();
        let target_mode = match find_attr(attrs, "TargetMode") {
            Some("External") => OpcTargetMode::External,
            _ => OpcTargetMode::Internal,
        };
        out.push(OpcRelationship { id, rel_type, target, target_mode });
    }
    Ok(out)
}
//#endregion 🔖️Relationships

//#region 🔖️Package
/// 📦️ A fully decoded OPC package: every non-metadata part verbatim, plus the two typed
/// metadata channels (content types, relationships-by-owner) that `docx`/`xlsx`/`pptx` interpret
/// semantically on top of. Lossless by construction — `parts ∪ content_types ∪ relationships`
/// covers every zip entry a real package can contain.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpcPackage {
    #[serde(default)]
    pub parts: Vec<OpcPart>,
    #[serde(default)]
    pub content_types: OpcContentTypes,
    /// 🗺️ Owner part path (`""` = package root) -> that owner's relationships.
    #[serde(default)]
    pub relationships: HashMap<String, Vec<OpcRelationship>>,
}

impl OpcPackage {
    /// 🌱️ An empty package (no parts, no content types, no relationships) — callers building a
    /// fresh document from scratch start here and add parts/relationships/content-types.
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn part(&self, path: &str) -> Option<&OpcPart> {
        let p = path.trim_start_matches('/');
        self.parts.iter().find(|part| part.path == p)
    }

    pub fn part_bytes(&self, path: &str) -> Option<&[u8]> {
        self.part(path).map(|p| p.bytes.as_slice())
    }

    /// ✍️ Inserts or replaces a content part, keeping its `[Content_Types].xml` `Override` in
    /// sync in the same call — the two can never drift apart through this API.
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

    pub fn relationships_for(&self, owner: &str) -> &[OpcRelationship] {
        self.relationships.get(owner).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// ✍️ Appends one internal relationship under `owner` (`""` = package root).
    pub fn add_relationship(&mut self, owner: &str, id: &str, rel_type: &str, target: &str) {
        self.relationships.entry(owner.to_string()).or_default().push(OpcRelationship {
            id: id.into(),
            rel_type: rel_type.into(),
            target: target.into(),
            target_mode: OpcTargetMode::Internal,
        });
    }

    /// 🔎️ Follows a single relationship of `rel_type` owned by `owner`, resolving its target to
    /// an absolute part path. `None` when no such relationship exists.
    pub fn resolve_relationship(&self, owner: &str, rel_type: &str) -> Option<String> {
        let rel = self.relationships_for(owner).iter().find(|r| r.rel_type == rel_type)?;
        Some(resolve_relationship_target(owner, &rel.target))
    }
}

/// 📦️ Decode OPC container bytes (a real zip archive) into a typed, lossless `OpcPackage`.
/// Every zip entry becomes exactly one of: the typed `content_types` table, a typed
/// relationship list, or a verbatim content `OpcPart` — never dropped, never fabricated.
pub fn decode_opc(data: &[u8]) -> Result<OpcPackage, OpcError> {
    let zip = crate::artifacts::zip::engine::decode_zip(data).map_err(|e| OpcError::Zip(e.to_string()))?;

    let ct_entry = zip.entries.iter().find(|e| e.name == CONTENT_TYPES_PART).ok_or(OpcError::MissingContentTypes)?;
    let ct_text = String::from_utf8(ct_entry.data.clone())
        .map_err(|_| OpcError::MalformedContentTypes("not valid utf-8".into()))?;
    let ct_doc = xml_document_from_text(&ct_text).map_err(|e| OpcError::Xml { part: CONTENT_TYPES_PART.into(), detail: e })?;
    let content_types = OpcContentTypes::from_xml(&ct_doc)?;

    let mut parts = Vec::new();
    let mut relationships: HashMap<String, Vec<OpcRelationship>> = HashMap::new();

    for entry in &zip.entries {
        if entry.name == CONTENT_TYPES_PART {
            continue;
        }
        if entry.name.ends_with(".rels") {
            let text = String::from_utf8(entry.data.clone())
                .map_err(|_| OpcError::MalformedRelationships { part: entry.name.clone(), detail: "not valid utf-8".into() })?;
            let doc = xml_document_from_text(&text).map_err(|e| OpcError::Xml { part: entry.name.clone(), detail: e })?;
            let rels = relationships_from_xml(&doc, &entry.name)?;
            let owner = owner_for_rels_path(&entry.name)
                .ok_or_else(|| OpcError::Malformed(format!("relationship part at unexpected path: {}", entry.name)))?;
            relationships.insert(owner, rels);
            continue;
        }
        let content_type = content_types
            .resolve(&entry.name)
            .ok_or_else(|| OpcError::Malformed(format!("part {} has no resolvable content type", entry.name)))?
            .to_string();
        parts.push(OpcPart { path: entry.name.clone(), content_type, bytes: entry.data.clone() });
    }

    Ok(OpcPackage { parts, content_types, relationships })
}

/// 📦️ Re-encode an `OpcPackage` as OPC container bytes: `[Content_Types].xml` and every owner's
/// `*.rels` file are regenerated from the typed tables (never carried as stray verbatim parts —
/// see `decode_opc`), every content part is re-emitted deflated via the zip artifact's real
/// codec. Semantically equivalent to a conformant reader, matching the zip artifact's own
/// encode/decode contract (not necessarily byte-identical: XML attribute serialization order and
/// zip local-header metadata have legitimate freedom).
pub fn encode_opc(pkg: &OpcPackage) -> Result<Vec<u8>, OpcError> {
    let mut entries = Vec::with_capacity(pkg.parts.len() + pkg.relationships.len() + 1);

    let ct_text = xml_document_to_text(&pkg.content_types.to_xml());
    entries.push(ZipEntry { name: CONTENT_TYPES_PART.into(), data: ct_text.into_bytes(), method: ZipCompressionMethod::Deflate, ..Default::default() });

    let mut owners: Vec<&String> = pkg.relationships.keys().collect();
    owners.sort();
    for owner in owners {
        let rels = &pkg.relationships[owner];
        if rels.is_empty() {
            continue;
        }
        let path = rels_part_path_for(owner);
        let text = xml_document_to_text(&relationships_to_xml(rels));
        entries.push(ZipEntry { name: path, data: text.into_bytes(), method: ZipCompressionMethod::Deflate, ..Default::default() });
    }

    for part in &pkg.parts {
        entries.push(ZipEntry { name: part.path.clone(), data: part.bytes.clone(), method: ZipCompressionMethod::Deflate, ..Default::default() });
    }

    let snap = ZipSnapshot { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries, comment: String::new() };
    crate::artifacts::zip::engine::encode_zip(&snap).map_err(|e| OpcError::Zip(e.to_string()))
}

/// 🕵️ Structural sniff of OOXML-shaped bytes: recognizes the zip magic *and* the presence of a
/// `[Content_Types].xml` entry — real OOXML disambiguation from a plain zip peeks part names
/// (docx/xlsx/pptx callers inspect `word/`/`xl/`/`ppt/`-prefixed parts on top of this).
pub fn sniff_opc_bytes(data: &[u8]) -> bool {
    let Ok(zip) = crate::artifacts::zip::engine::decode_zip(data) else { return false };
    zip.entries.iter().any(|e| e.name == CONTENT_TYPES_PART)
}
//#endregion 🔖️Package

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_package() -> OpcPackage {
        let mut pkg = OpcPackage::empty();
        pkg.content_types.set_default("rels", RELS_CONTENT_TYPE);
        pkg.content_types.set_default("xml", "application/xml");
        pkg.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", b"<w:document/>".to_vec());
        pkg.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
        pkg
    }

    #[test]
    fn round_trip_preserves_parts_and_relationships() {
        let pkg = sample_package();
        let bytes = encode_opc(&pkg).expect("encode");
        let decoded = decode_opc(&bytes).expect("decode");
        assert_eq!(decoded.part_bytes("word/document.xml"), Some(b"<w:document/>".as_slice()));
        assert_eq!(
            decoded.content_types.resolve("word/document.xml"),
            Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml")
        );
        let root_rels = decoded.relationships_for("");
        assert_eq!(root_rels.len(), 1);
        assert_eq!(root_rels[0].target, "word/document.xml");
        assert_eq!(root_rels[0].rel_type, REL_TYPE_OFFICE_DOCUMENT);
    }

    #[test]
    fn resolve_relationship_target_is_relative_to_owner_directory() {
        // A relationship owned by "word/document.xml" (rels file at
        // "word/_rels/document.xml.rels") targeting "media/image1.png" resolves against
        // "word/", not the package root — the #1 OPC relative-target gotcha.
        assert_eq!(resolve_relationship_target("word/document.xml", "media/image1.png"), "word/media/image1.png");
        assert_eq!(resolve_relationship_target("word/document.xml", "/media/image1.png"), "media/image1.png");
        assert_eq!(resolve_relationship_target("", "word/document.xml"), "word/document.xml");
    }

    #[test]
    fn owner_and_rels_path_round_trip_including_root() {
        assert_eq!(rels_part_path_for(""), "_rels/.rels");
        assert_eq!(owner_for_rels_path("_rels/.rels"), Some(String::new()));
        assert_eq!(rels_part_path_for("word/document.xml"), "word/_rels/document.xml.rels");
        assert_eq!(owner_for_rels_path("word/_rels/document.xml.rels"), Some("word/document.xml".to_string()));
        assert_eq!(rels_part_path_for("xl/workbook.xml"), "xl/_rels/workbook.xml.rels");
        assert_eq!(owner_for_rels_path("xl/_rels/workbook.xml.rels"), Some("xl/workbook.xml".to_string()));
    }

    #[test]
    fn content_types_override_wins_over_default() {
        let mut ct = OpcContentTypes::default();
        ct.set_default("xml", "application/xml");
        ct.set_override("word/document.xml", "application/vnd.custom+xml");
        assert_eq!(ct.resolve("word/document.xml"), Some("application/vnd.custom+xml"));
        assert_eq!(ct.resolve("word/styles.xml"), Some("application/xml"));
        assert_eq!(ct.resolve("word/unknownext.bin"), None);
    }

    #[test]
    fn decode_rejects_missing_content_types() {
        let snap = ZipSnapshot {
            schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: vec![ZipEntry { name: "word/document.xml".into(), data: b"<x/>".to_vec(), ..Default::default() }],
            comment: String::new(),
        };
        let bytes = crate::artifacts::zip::engine::encode_zip(&snap).unwrap();
        let err = decode_opc(&bytes).expect_err("must reject a zip with no [Content_Types].xml");
        assert_eq!(err, OpcError::MissingContentTypes);
    }

    #[test]
    fn sniff_recognizes_content_types_entry() {
        let pkg = sample_package();
        let bytes = encode_opc(&pkg).unwrap();
        assert!(sniff_opc_bytes(&bytes));
        assert!(!sniff_opc_bytes(b"not a zip"));

        let plain_zip = crate::artifacts::zip::engine::encode_zip(&ZipSnapshot {
            schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: vec![ZipEntry { name: "a.txt".into(), data: b"hi".to_vec(), ..Default::default() }],
            comment: String::new(),
        })
        .unwrap();
        assert!(!sniff_opc_bytes(&plain_zip), "a plain zip with no [Content_Types].xml must not sniff as OPC");
    }
}
//#endregion 🧪️Tests
