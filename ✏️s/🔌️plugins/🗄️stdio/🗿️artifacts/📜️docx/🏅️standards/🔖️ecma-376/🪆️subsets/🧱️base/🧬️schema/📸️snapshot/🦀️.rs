//! 🧬️ DOCX snapshot with one authoritative logical document per XML-bearing OPC part.

use crate::{
    standards::v_ecma_376::subsets::base::{schema::{refusal::{DocxError},vocabulary::{REL_TYPE_STYLES,STRICT_REL_TYPE_OFFICE_DOCUMENT,STRICT_REL_TYPE_STYLES}}},
    STDIO_DOCX_DOCUMENT_SCHEMA,
};
use framework_schema::ArtifactSchema;
use semio_framework_value::list::PagedList;
use semio_s_artifact_stdio_xml::schema::snapshot::{retained::RetainedXmlDocument, ownership::{retire_xml_document, retire_xml_document_with_frontier}, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::{resolve_relationship_target, retained::RetainedOpcPackage, OpcPackage, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DocxXmlPart {
    pub path: String,
    pub content_type: String,
    pub document: RetainedXmlDocument,
}

pub const DOCX_MAX_XML_PARTS: usize = 1_048_576;
pub type DocxXmlParts = PagedList<DocxXmlPart, DOCX_MAX_XML_PARTS>;

struct RetiringXmlDocument(Option<XmlDocument>);
impl Drop for RetiringXmlDocument { fn drop(&mut self) { if let Some(document)=self.0.take(){retire_xml_document(document);} } }

impl DocxXmlPart {
    pub fn try_from_document(path: String, content_type: String, document: XmlDocument) -> Result<Self, semio_framework_value::ValueError> {
        let mut source=RetiringXmlDocument(Some(document));
        let retained=RetainedXmlDocument::try_from_document(source.0.as_ref().expect("owned DOCX XML source"))?;
        retire_xml_document(source.0.take().expect("owned DOCX XML source"));
        Ok(Self { path, content_type, document: retained })
    }

    pub fn try_from_document_controlled(
        path: String,
        content_type: String,
        document: XmlDocument,
        control: &mut semio_framework_value::NativeDecodeControl<'_>,
    ) -> Result<Self, semio_framework_value::ValueError> {
        let mut source=RetiringXmlDocument(Some(document));
        let retained=RetainedXmlDocument::try_from_document_controlled(source.0.as_ref().expect("owned DOCX XML source"),control)?;
        let frontier=control.allocate_vec::<XmlNode>(retained.nodes.len())?;
        retire_xml_document_with_frontier(source.0.take().expect("owned DOCX XML source"),frontier);
        Ok(Self { path, content_type, document: retained })
    }

    pub fn materialize_document(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<XmlDocument, semio_framework_value::ValueError> {
        self.document.materialize(control)
    }

    pub fn materialize_document_exact(&self) -> Result<XmlDocument, semio_framework_value::ValueError> {
        self.document.materialize_exact()
    }

    pub fn replace_document(&mut self, document: XmlDocument) -> Result<(), semio_framework_value::ValueError> {
        let mut source=RetiringXmlDocument(Some(document));
        self.document=RetainedXmlDocument::try_from_document(source.0.as_ref().expect("owned DOCX XML source"))?;
        retire_xml_document(source.0.take().expect("owned DOCX XML source"));
        Ok(())
    }
}

pub fn docx_xml_parts_from_iter_controlled(
    values: impl IntoIterator<Item = DocxXmlPart>,
    control: &mut semio_framework_value::NativeDecodeControl<'_>,
) -> Result<DocxXmlParts, semio_framework_value::ValueError> {
    let mut output = DocxXmlParts::default();
    for value in values {
        while !output.has_reserved_slot() {
            let required = output.next_allocation_bytes()?;
            control.charge(required)?;
            let progress = output.reserve_one(required).map_err(|error| semio_framework_value::ValueError::from(error.refusal()))?;
            if !progress.progressed {
                return Err(semio_framework_value::ValueError::new(
                    semio_framework_value::ValueRefusalKind::InvariantViolated,
                    "DOCX XML part controlled allocation did not progress",
                ));
            }
        }
        output
            .push_reserved(value)
            .map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "DOCX XML part controlled append rejected its admitted slot"))?;
    }
    Ok(output)
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.docx")]
pub struct DocxSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 📦️ OPC metadata and non-XML payloads.
    #[state(artifact)]
    #[value(default)]
    pub opc: RetainedOpcPackage,
    /// 📄️ Complete logical XML parts, each represented exactly once.
    #[state(artifact)]
    #[value(default)]
    pub xml_parts: DocxXmlParts,
}

impl Default for DocxSnapshot {
    fn default() -> Self {
        crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx(DocxDocument::default())
    }
}

impl DocxSnapshot {
    /// 🏗️ Builds a snapshot from non-XML OPC state plus authoritative logical XML parts.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_parts(mut opc: OpcPackage, mut xml_parts: Vec<DocxXmlPart>) -> Result<Self, semio_framework_value::ValueError> {
        opc.parts.sort_by(|left, right| left.path.cmp(&right.path));
        xml_parts.sort_by(|left, right| left.path.cmp(&right.path));
        Ok(Self { schema: STDIO_DOCX_DOCUMENT_SCHEMA.into(), opc: RetainedOpcPackage::try_from_package(opc)?, xml_parts: DocxXmlParts::try_from_iter(xml_parts)? })
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
        fn role_relationship<'a>(snapshot: &'a DocxSnapshot, owner: &str, kinds: &[&str], role: &str) -> Result<Option<&'a semio_s_artifact_stdio_zip::opc::retained::RetainedOpcRelationship>, DocxError> {
            let matches: Vec<_> = snapshot.opc.relationships_for(owner).into_iter().flat_map(|relationships| relationships.iter()).filter(|relationship| kinds.iter().any(|kind| relationship.rel_type.eq_str(kind))).collect();
            if matches.len() > 1 {
                return Err(DocxError::Malformed(format!("multiple {role} relationships for {owner}")));
            }
            if matches.first().is_some_and(|relationship| relationship.target_mode != semio_s_artifact_stdio_zip::opc::retained::RetainedOpcTargetMode::Internal) {
                return Err(DocxError::Malformed(format!("{role} relationship for {owner} is external")));
            }
            Ok(matches.into_iter().next())
        }

        let mut paths = HashSet::new();
        for part in self.xml_parts.iter() {
            if !valid_path(&part.path) || metadata_path(&part.path) {
                return Err(DocxError::Malformed(format!("invalid XML content part path: {}", part.path)));
            }
            if !docx_part_is_xml(&part.path, &part.content_type) {
                return Err(DocxError::Malformed(format!("XML authority carries a non-XML content type: {}", part.path)));
            }
            if !paths.insert(part.path.clone()) {
                return Err(DocxError::Malformed(format!("duplicate XML part authority: {}", part.path)));
            }
            if self.opc.content_types.resolve(&part.path).is_none_or(|content_type| !content_type.eq_str(&part.content_type)) {
                return Err(DocxError::Malformed(format!("content type metadata disagrees for XML part {}", part.path)));
            }
        }
        for part in self.opc.parts.iter() {
            let path = part.path.to_string_owner();
            let content_type = part.content_type.to_string_owner();
            if !valid_path(&path) || metadata_path(&path) {
                return Err(DocxError::Malformed(format!("invalid binary content part path: {path}")));
            }
            if docx_part_is_xml(&path, &content_type) {
                return Err(DocxError::Malformed(format!("binary authority carries an XML content part: {path}")));
            }
            if !paths.insert(path.clone()) {
                return Err(DocxError::Malformed(format!("duplicate OPC part authority: {path}")));
            }
            if self.opc.content_types.resolve(&path).is_none_or(|resolved| !resolved.eq_str(&content_type)) {
                return Err(DocxError::Malformed(format!("content type metadata disagrees for binary part {path}")));
            }
        }
        for owner in self.opc.relationships.keys().filter(|owner| !owner.is_empty()) {
            if !paths.contains(owner.as_str()) {
                return Err(DocxError::Malformed(format!("relationship owner is not a content part: {owner}")));
            }
        }
        let main = role_relationship(self, "", &[REL_TYPE_OFFICE_DOCUMENT, STRICT_REL_TYPE_OFFICE_DOCUMENT], "officeDocument")?.ok_or(DocxError::MissingMainDocumentRelationship)?;
        let main_path = resolve_relationship_target("", &main.target.to_string_owner());
        if self.xml_part(&main_path).is_none() {
            return Err(DocxError::MissingPart(main_path));
        }
        if let Some(styles) = role_relationship(self, &main_path, &[REL_TYPE_STYLES, STRICT_REL_TYPE_STYLES], "styles")? {
            let styles_path = resolve_relationship_target(&main_path, &styles.target.to_string_owner());
            if self.xml_part(&styles_path).is_none() {
                return Err(DocxError::MissingPart(styles_path));
            }
        }
        Ok(())
    }

    /// 📰️ Projects the editable semantic view without creating a second persisted authority.
    pub fn project_document(&self) -> Result<DocxDocument, crate::standards::v_ecma_376::subsets::base::schema::refusal::DocxError> {
        crate::standards::v_ecma_376::subsets::base::schema::inferences::document::project_snapshot_document(self)
    }

}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs


//#endregion 🔖️HandcraftedArtifactCodecs





#[path="🛡️subset/🦀️.rs"]
pub(crate) mod subset;
