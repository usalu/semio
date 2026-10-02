//! 🧬️ DOCX snapshot with one authoritative logical document per XML-bearing OPC part.

use crate::{
    standards::v_ecma_376::subsets::base::io::{DocxError, REL_TYPE_STYLES, STRICT_REL_TYPE_OFFICE_DOCUMENT, STRICT_REL_TYPE_STYLES},
    STDIO_DOCX_DOCUMENT_SCHEMA,
};
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_to_text, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::{resolve_relationship_target, OpcPackage, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};
use std::collections::HashSet;

//#region 🔖️DocxModel
/// ✍️ Semantic run projection with direct formatting flags; style inheritance is not resolved here.
/// Canonical XML parts retain absent, explicitly disabled, and richer formatting independently.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxRun {
    pub text: String,
    #[value(default)]
    pub bold: bool,
    #[value(default)]
    pub italic: bool,
    #[value(default)]
    pub underline: bool,
    /// 🗄️ Raw retention of `<w:rPr>` children this model doesn't interpret (color, font, size,
    /// strike, highlight, …), in original order.
    #[value(default)]
    pub extra_run_properties: Vec<XmlNode>,
}

/// 📄️ One `w:p` paragraph: an ordered list of runs plus an optional named style reference.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxParagraph {
    #[value(default)]
    pub runs: Vec<DocxRun>,
    /// 🎨️ `<w:pPr><w:pStyle w:val="…"/></w:pPr>` — references a `DocxStyle::id`.
    #[value(default)]
    pub style: Option<String>,
    /// 🗄️ Raw retention of `<w:pPr>` children other than `<w:pStyle>` (alignment, numbering,
    /// spacing, …), in original order.
    #[value(default)]
    pub extra_paragraph_properties: Vec<XmlNode>,
}

impl DocxParagraph {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn text(text: impl Into<String>) -> Self {
        Self { runs: vec![DocxRun { text: text.into(), ..Default::default() }], style: None, extra_paragraph_properties: Vec::new() }
    }
}

/// 🔲️ One `w:tc` table cell: recursively holds its own block content (WordprocessingML cells may
/// contain paragraphs and nested tables).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxTableCell {
    #[value(default)]
    pub blocks: Vec<DocxBlock>,
    /// 🗄️ Raw retention of `<w:tcPr>` children (width, span, merge, shading, …).
    #[value(default)]
    pub extra_cell_properties: Vec<XmlNode>,
}

/// ➖️ One `w:tr` table row.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxTableRow {
    #[value(default)]
    pub cells: Vec<DocxTableCell>,
    /// 🗄️ Raw retention of `<w:trPr>` children (height, header-row flag, …).
    #[value(default)]
    pub extra_row_properties: Vec<XmlNode>,
}

/// 🏛️ One `w:tbl` table.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxTable {
    #[value(default)]
    pub rows: Vec<DocxTableRow>,
    /// 🗄️ Raw retention of `<w:tblPr>` children (borders, width, look, …).
    #[value(default)]
    pub extra_table_properties: Vec<XmlNode>,
}

/// 🧱️ One block-level content item inside `word/document.xml`'s `w:body` (or a table cell) — a
/// paragraph or a table, matching WordprocessingML's own block-content model.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum DocxBlock {
    Paragraph(DocxParagraph),
    Table(DocxTable),
}

impl DocxBlock {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn paragraph(text: impl Into<String>) -> Self {
        Self::Paragraph(DocxParagraph::text(text))
    }
}

/// 🎨️ One `<w:style>` entry from `word/styles.xml`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxStyle {
    pub id: String,
    pub name: String,
    #[value(default)]
    pub based_on: Option<String>,
}

/// 📰 Typed semantic view of `word/document.xml`'s `w:body` (a block tree) plus `word/styles.xml`
/// (name-keyed by `DocxStyle::id`, styleId in WordprocessingML terms).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxDocument {
    #[value(default)]
    pub body: Vec<DocxBlock>,
    #[value(default)]
    pub styles: Vec<DocxStyle>,
}
//#endregion 🔖️DocxModel

//#region 🔖️XmlParts
/// 📄️ One authoritative XML-bearing OPC part. `OpcPackage.parts` contains only non-XML payloads.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxXmlPart {
    pub path: String,
    pub content_type: String,
    pub document: XmlDocument,
}

/// 📄️ Classifies XML-bearing package parts without inferring semantic roles from fixed paths.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn docx_part_is_xml(path: &str, content_type: &str) -> bool {
    let lower_path = path.to_ascii_lowercase();
    let lower_type = content_type.to_ascii_lowercase();
    lower_path.ends_with(".xml") || lower_path.ends_with(".vml") || lower_type.ends_with("+xml") || lower_type.ends_with("/xml") || lower_type.contains("vmldrawing")
}
//#endregion 🔖️XmlParts

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.docx")]
pub struct DocxSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 📦️ OPC metadata and non-XML payloads.
    #[state(artifact)]
    #[value(default)]
    pub opc: OpcPackage,
    /// 📄️ Complete logical XML parts, each represented exactly once.
    #[state(artifact)]
    #[value(default)]
    pub xml_parts: Vec<DocxXmlPart>,
}

impl Default for DocxSnapshot {
    fn default() -> Self {
        crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_docx(DocxDocument::default())
    }
}

impl DocxSnapshot {
    /// 🏗️ Builds a snapshot from non-XML OPC state plus authoritative logical XML parts.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_parts(mut opc: OpcPackage, mut xml_parts: Vec<DocxXmlPart>) -> Self {
        opc.parts.sort_by(|left, right| left.path.cmp(&right.path));
        xml_parts.sort_by(|left, right| left.path.cmp(&right.path));
        Self { schema: STDIO_DOCX_DOCUMENT_SCHEMA.into(), opc, xml_parts }
    }

    /// 📄️ Finds one authoritative logical XML part by normalized OPC path.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn xml_part(&self, path: &str) -> Option<&DocxXmlPart> {
        let key = path.trim_start_matches('/');
        self.xml_parts.iter().find(|part| part.path == key)
    }

    /// 📄️ Finds one mutable authoritative logical XML part by normalized OPC path.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn xml_part_mut(&mut self, path: &str) -> Option<&mut DocxXmlPart> {
        let key = path.trim_start_matches('/');
        self.xml_parts.iter_mut().find(|part| part.path == key)
    }

    /// 🛡️ Refuses authored package state whose XML authority, OPC metadata, or semantic roles disagree.
    pub fn validate_authority(&self) -> Result<(), DocxError> {
        fn valid_path(path: &str) -> bool {
            !path.is_empty() && !path.starts_with('/') && !path.contains('\\') && path.split('/').all(|segment| !segment.is_empty() && segment != "." && segment != "..")
        }
        fn metadata_path(path: &str) -> bool {
            let lower = path.to_ascii_lowercase();
            lower == "[content_types].xml" || lower == "_rels/.rels" || lower.ends_with(".rels")
        }
        fn role_relationship<'a>(snapshot: &'a DocxSnapshot, owner: &str, kinds: &[&str], role: &str) -> Result<Option<&'a semio_s_artifact_stdio_zip::opc::OpcRelationship>, DocxError> {
            let matches: Vec<_> = snapshot.opc.relationships_for(owner).iter().filter(|relationship| kinds.contains(&relationship.rel_type.as_str())).collect();
            if matches.len() > 1 {
                return Err(DocxError::Malformed(format!("multiple {role} relationships for {owner}")));
            }
            if matches.first().is_some_and(|relationship| relationship.target_mode != OpcTargetMode::Internal) {
                return Err(DocxError::Malformed(format!("{role} relationship for {owner} is external")));
            }
            Ok(matches.into_iter().next())
        }

        let mut paths = HashSet::new();
        for part in &self.xml_parts {
            if !valid_path(&part.path) || metadata_path(&part.path) {
                return Err(DocxError::Malformed(format!("invalid XML content part path: {}", part.path)));
            }
            if !docx_part_is_xml(&part.path, &part.content_type) {
                return Err(DocxError::Malformed(format!("XML authority carries a non-XML content type: {}", part.path)));
            }
            if !paths.insert(part.path.as_str()) {
                return Err(DocxError::Malformed(format!("duplicate XML part authority: {}", part.path)));
            }
            if self.opc.content_types.resolve(&part.path) != Some(part.content_type.as_str()) {
                return Err(DocxError::Malformed(format!("content type metadata disagrees for XML part {}", part.path)));
            }
        }
        for part in &self.opc.parts {
            if !valid_path(&part.path) || metadata_path(&part.path) {
                return Err(DocxError::Malformed(format!("invalid binary content part path: {}", part.path)));
            }
            if docx_part_is_xml(&part.path, &part.content_type) {
                return Err(DocxError::Malformed(format!("binary authority carries an XML content part: {}", part.path)));
            }
            if !paths.insert(part.path.as_str()) {
                return Err(DocxError::Malformed(format!("duplicate OPC part authority: {}", part.path)));
            }
            if self.opc.content_types.resolve(&part.path) != Some(part.content_type.as_str()) {
                return Err(DocxError::Malformed(format!("content type metadata disagrees for binary part {}", part.path)));
            }
        }
        for owner in self.opc.relationships.keys().filter(|owner| !owner.is_empty()) {
            if !paths.contains(owner.as_str()) {
                return Err(DocxError::Malformed(format!("relationship owner is not a content part: {owner}")));
            }
        }
        let main = role_relationship(self, "", &[REL_TYPE_OFFICE_DOCUMENT, STRICT_REL_TYPE_OFFICE_DOCUMENT], "officeDocument")?.ok_or(DocxError::MissingMainDocumentRelationship)?;
        let main_path = resolve_relationship_target("", &main.target);
        if self.xml_part(&main_path).is_none() {
            return Err(DocxError::MissingPart(main_path));
        }
        if let Some(styles) = role_relationship(self, &main_path, &[REL_TYPE_STYLES, STRICT_REL_TYPE_STYLES], "styles")? {
            let styles_path = resolve_relationship_target(&main_path, &styles.target);
            if self.xml_part(&styles_path).is_none() {
                return Err(DocxError::MissingPart(styles_path));
            }
        }
        Ok(())
    }

    /// 📰️ Projects the editable semantic view without creating a second persisted authority.
    pub fn project_document(&self) -> Result<DocxDocument, crate::standards::v_ecma_376::subsets::base::io::DocxError> {
        crate::standards::v_ecma_376::subsets::base::io::import::deserializers::project_snapshot_document(self)
    }

    pub fn part_text(&self, path: &str) -> Option<String> {
        let key = path.trim_start_matches('/');
        self.xml_part(key).map(|part| xml_document_to_text(&part.document)).or_else(|| self.opc.part_bytes(key).and_then(|bytes| String::from_utf8(bytes.to_vec()).ok()))
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for DocxSnapshot {
 const EXTENSION:&'static str="docx";
 fn envelope_id()->&'static str{"stdio.docx"}
 fn parse_dsl(text:&str)->Result<Self,store::TextError>{native::decode_text(text,&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default())).map_err(|error|store::TextError::new(error,dsl::TextSpan::at(1,1)))}
 fn print_dsl(&self)->String{match native::encode(self,semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Text,&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default())).expect("DOCX native ownership admission"){semio_framework_os_kernel::io_schema::IoPayload::Text(text)=>text,semio_framework_os_kernel::io_schema::IoPayload::Binary(_)=>unreachable!()}}
}
impl store::ArtifactPack for DocxSnapshot {
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{match native::encode(self,semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Binary,&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,native::pack_limits(&options.limits))).map_err(store::PackError::Schema)?{semio_framework_os_kernel::io_schema::IoPayload::Binary(bytes)=>Ok(bytes),semio_framework_os_kernel::io_schema::IoPayload::Text(_)=>unreachable!()}}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{native::decode_binary(bytes,&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,native::pack_limits(&options.limits))).map_err(store::PackError::Schema)}
}
//#endregion 🔖️HandcraftedArtifactCodecs

#[path="🪶️sqlite/🦀️.rs"]
pub mod sqlite;
#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

#[path="🧩️native/🦀️.rs"]
mod native;

#[path="🛡️subset/🦀️.rs"]
mod subset;
