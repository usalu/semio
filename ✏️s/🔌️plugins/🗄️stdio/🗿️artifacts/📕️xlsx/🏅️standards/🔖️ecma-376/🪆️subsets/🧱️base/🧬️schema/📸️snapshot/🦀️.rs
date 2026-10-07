//! 🧬️ XLSX snapshot with one authoritative logical document per XML-bearing OPC part.

use crate::STDIO_XLSX_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument;
use semio_s_artifact_stdio_zip::opc::{resolve_relationship_target, OpcPackage, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};
use std::collections::HashSet;

//#region 🔖️XlsxModel
/// 🔢️ A cell's decoded value — a real typed union over every SpreadsheetML cell-type ECMA-376
/// §18.3.1.4 (`t` attribute) distinguishes: `Number` (`t` absent, the numeric default),
/// `SharedString` (`t="s"`, an index into `workbook.shared_strings` — kept as an index, never
/// resolved here), `InlineString` (`t="inlineStr"`, literal `<is><t>` text; `t="str"`/`t="e"`
/// non-formula cells normalize to this on decode — a documented normalization, see the engine),
/// `Boolean` (`t="b"`), `Formula` (a `<f>` child present; `cached` is the cell's own `<v>`,
/// re-typed by ITS `t` attribute, `None` when the workbook has no cached value), `Empty` (no
/// `<v>` at all).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
#[derive(Default)]
pub enum XlsxCellValue {
    Number(f64),
    SharedString(usize),
    InlineString(String),
    Boolean(bool),
    Error(String),
    Formula {
        expr: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        cached: Option<Box<XlsxCellValue>>,
    },
    #[default]
    Empty,
}

/// 🧮 One worksheet cell, addressed by `(row, col)` rather than an A1-style string — `row` is
/// 1-based (the literal SpreadsheetML `<row r="N">` index), `col` is 0-based (matches
/// `schema::vocabulary::column_letter`'s `0 -> "A"` convention). `row`/`col` are this cell's IDENTITY (the
/// key `XlsxCellsDiff` diffs by) and are never themselves diffed — only `value` is.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XlsxCell {
    pub row: u32,
    pub col: u32,
    #[value(default)]
    pub value: XlsxCellValue,
}

/// 📄 One worksheet: a sparse `(row, col)`-addressed cell list (a real spreadsheet is mostly
/// empty — no dense row/col grid is materialized). `name` is this sheet's IDENTITY (the key
/// `XlsxSheetsDiff` diffs by, per the recipe's name-keyed-collection convention); renaming a
/// sheet is therefore a remove-old-name + add-new-name at the diff level (documented — same
/// category as docx's OPC-part-rename gotcha), never a `name` field mutation.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XlsxSheet {
    pub name: String,
    #[value(default)]
    pub cells: Vec<XlsxCell>,
}

/// 📘 Typed semantic view of the workbook: `xl/workbook.xml`'s sheet list, each resolved
/// through `xl/_rels/workbook.xml.rels` to its `xl/worksheets/sheetN.xml` part, plus the SST
/// (`xl/sharedStrings.xml`) kept as its own index-keyed `shared_strings` table — `t="s"` cells
/// reference it by index (`XlsxCellValue::SharedString(usize)`), never resolved eagerly.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XlsxWorkbook {
    #[value(default)]
    pub sheets: Vec<XlsxSheet>,
    #[value(default)]
    pub shared_strings: Vec<String>,
}
//#endregion 🔖️XlsxModel

//#region 🔖️XmlParts
/// 📄️ One authoritative XML-bearing OPC part. `OpcPackage.parts` contains only non-XML payloads.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XlsxXmlPart {
    pub path: String,
    pub content_type: String,
    pub document: XmlDocument,
}

/// 📄️ Classifies XML-bearing package parts independently from their relationship role.
pub fn xlsx_part_is_xml(path: &str, content_type: &str) -> bool {
    let lower_path = path.to_ascii_lowercase();
    let lower_type = content_type.to_ascii_lowercase();
    lower_path.ends_with(".xml") || lower_path.ends_with(".vml") || lower_type.ends_with("+xml") || lower_type.ends_with("/xml") || lower_type.contains("vmldrawing")
}
//#endregion 🔖️XmlParts

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xlsx")]
pub struct XlsxSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub opc: OpcPackage,
    /// 📄️ Complete logical XML parts, each represented exactly once.
    #[state(artifact)]
    #[value(default)]
    pub xml_parts: Vec<XlsxXmlPart>,
}

impl Default for XlsxSnapshot {
    fn default() -> Self {
        crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook::default())
    }
}

impl XlsxSnapshot {
    /// 🏗️ Builds a snapshot from non-XML OPC state plus authoritative logical XML parts.
    pub fn from_parts(opc: OpcPackage, xml_parts: Vec<XlsxXmlPart>) -> Self {
        Self { schema: STDIO_XLSX_DOCUMENT_SCHEMA.into(), opc, xml_parts }
    }

    /// 📄️ Finds one authoritative logical XML part by normalized OPC path.
    pub fn xml_part(&self, path: &str) -> Option<&XlsxXmlPart> {
        let key = path.trim_start_matches('/');
        self.xml_parts.iter().find(|part| part.path == key)
    }

    /// 📄️ Finds one mutable authoritative logical XML part by normalized OPC path.
    pub fn xml_part_mut(&mut self, path: &str) -> Option<&mut XlsxXmlPart> {
        let key = path.trim_start_matches('/');
        self.xml_parts.iter_mut().find(|part| part.path == key)
    }

    /// 🛡️ Refuses duplicate, mismatched, or unresolved XML/package authority.
    pub fn validate_authority(&self) -> Result<(), crate::standards::v_ecma_376::subsets::base::schema::refusal::XlsxError> {
        use crate::schema::refusal::XlsxError;
        use crate::schema::vocabulary::REL_TYPE_OFFICE_DOCUMENT_STRICT;
        fn valid_path(path: &str) -> bool {
            !path.is_empty() && !path.starts_with('/') && !path.contains('\\') && path.split('/').all(|segment| !segment.is_empty() && segment != "." && segment != "..")
        }
        fn metadata_path(path: &str) -> bool {
            let lower = path.to_ascii_lowercase();
            lower == "[content_types].xml" || lower == "_rels/.rels" || lower.ends_with(".rels")
        }
        let mut paths = HashSet::new();
        for part in &self.xml_parts {
            if !valid_path(&part.path) || metadata_path(&part.path) {
                return Err(XlsxError::Malformed(format!("invalid XML content part path: {}", part.path)));
            }
            if !xlsx_part_is_xml(&part.path, &part.content_type) {
                return Err(XlsxError::Malformed(format!("XML authority carries a non-XML content type: {}", part.path)));
            }
            if !paths.insert(part.path.as_str()) {
                return Err(XlsxError::Malformed(format!("duplicate XML part authority: {}", part.path)));
            }
            if self.opc.content_types.resolve(&part.path) != Some(part.content_type.as_str()) {
                return Err(XlsxError::Malformed(format!("content type metadata disagrees for XML part {}", part.path)));
            }
        }
        for part in &self.opc.parts {
            if !valid_path(&part.path) || metadata_path(&part.path) {
                return Err(XlsxError::Malformed(format!("invalid binary content part path: {}", part.path)));
            }
            if xlsx_part_is_xml(&part.path, &part.content_type) {
                return Err(XlsxError::Malformed(format!("binary authority carries an XML content part: {}", part.path)));
            }
            if !paths.insert(part.path.as_str()) {
                return Err(XlsxError::Malformed(format!("duplicate OPC part authority: {}", part.path)));
            }
            if self.opc.content_types.resolve(&part.path) != Some(part.content_type.as_str()) {
                return Err(XlsxError::Malformed(format!("content type metadata disagrees for binary part {}", part.path)));
            }
        }
        for owner in self.opc.relationships.groups().map(|(owner, _)| owner).filter(|owner| !owner.is_empty()) {
            if !paths.contains(owner.as_str()) {
                return Err(XlsxError::Malformed(format!("relationship owner is not a content part: {owner}")));
            }
        }
        let main: Vec<_> = self.opc.relationships_for("").iter().filter(|relationship| relationship.rel_type == REL_TYPE_OFFICE_DOCUMENT || relationship.rel_type == REL_TYPE_OFFICE_DOCUMENT_STRICT).collect();
        if main.len() != 1 || main[0].target_mode != OpcTargetMode::Internal {
            return Err(XlsxError::MissingWorkbookRelationship);
        }
        let main_path = resolve_relationship_target("", &main[0].target);
        if self.xml_part(&main_path).is_none() {
            return Err(XlsxError::MissingPart(main_path));
        }
        self.project_workbook()?;
        Ok(())
    }

    /// 📗️ Resolves the main workbook part from the package root's officeDocument relationship (Transitional or Strict
    /// relationship type).
    pub fn workbook_part_path(&self) -> Option<String> {
        use crate::standards::v_ecma_376::subsets::base::schema::vocabulary::REL_TYPE_OFFICE_DOCUMENT_STRICT;
        self.opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).or_else(|| self.opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT_STRICT))
    }

    /// 📑️ Resolves every worksheet part by its ROLE — the targets of the main workbook's worksheet relationships
    /// (Transitional or Strict type), independent of the part's path or its declared content type.
    pub fn worksheet_part_paths(&self) -> Vec<String> {
        use crate::schema::vocabulary::{REL_TYPE_WORKSHEET, REL_TYPE_WORKSHEET_STRICT};
        let Some(workbook) = self.workbook_part_path() else { return Vec::new() };
        self.opc
            .relationships_for(&workbook)
            .iter()
            .filter(|relationship| relationship.rel_type == REL_TYPE_WORKSHEET || relationship.rel_type == REL_TYPE_WORKSHEET_STRICT)
            .map(|relationship| resolve_relationship_target(&workbook, &relationship.target))
            .collect()
    }

    /// 📘️ Projects the spreadsheet view without creating persisted semantic authority.
    pub fn project_workbook(&self) -> Result<XlsxWorkbook, crate::standards::v_ecma_376::subsets::base::schema::refusal::XlsxError> {
        crate::standards::v_ecma_376::subsets::base::schema::inferences::workbook::project_snapshot_workbook(self)
    }


}
//#endregion 🔖️Snapshot
