//! 🧬️ XlsxSnapshot — an OPC package (every part verbatim, lossless) plus a typed semantic view
//! of the workbook: name-keyed sheets, each a sparse `(row, col)`-addressed cell list, plus the
//! package's `shared_strings` table kept as its OWN index-keyed collection (never eagerly
//! resolved into cell text — the #1 xlsx decode gotcha is precisely THAT eager resolution, since
//! it silently collapses the `t="s"` shared-string-reference/`t="inlineStr"` literal distinction
//! a real workbook depends on for lossless round-trip and for `SharedString`/`InlineString` to
//! mean anything different in a diff). Unmodeled parts (`styles.xml`, themes, calc chain, …) stay
//! verbatim inside `opc`.

use crate::artifacts::xlsx::STDIO_XLSX_DOCUMENT_SCHEMA;
use crate::artifacts::zip::opc::OpcPackage;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️XlsxModel
/// 🔢️ A cell's decoded value — a real typed union over every SpreadsheetML cell-type ECMA-376
/// §18.3.1.4 (`t` attribute) distinguishes: `Number` (`t` absent, the numeric default),
/// `SharedString` (`t="s"`, an index into `workbook.shared_strings` — kept as an index, never
/// resolved here), `InlineString` (`t="inlineStr"`, literal `<is><t>` text; `t="str"`/`t="e"`
/// non-formula cells normalize to this on decode — a documented normalization, see the engine),
/// `Boolean` (`t="b"`), `Formula` (a `<f>` child present; `cached` is the cell's own `<v>`,
/// re-typed by ITS `t` attribute, `None` when the workbook has no cached value), `Empty` (no
/// `<v>` at all).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum XlsxCellValue {
    Number(f64),
    SharedString(usize),
    InlineString(String),
    Boolean(bool),
    Formula {
        expr: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cached: Option<Box<XlsxCellValue>>,
    },
    Empty,
}

impl Default for XlsxCellValue {
    fn default() -> Self {
        Self::Empty
    }
}

/// 🧮 One worksheet cell, addressed by `(row, col)` rather than an A1-style string — `row` is
/// 1-based (the literal SpreadsheetML `<row r="N">` index), `col` is 0-based (matches
/// `engine::column_letter`'s `0 -> "A"` convention). `row`/`col` are this cell's IDENTITY (the
/// key `XlsxCellsDiff` diffs by) and are never themselves diffed — only `value` is.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxCell {
    pub row: u32,
    pub col: u32,
    #[serde(default)]
    pub value: XlsxCellValue,
}

/// 📄 One worksheet: a sparse `(row, col)`-addressed cell list (a real spreadsheet is mostly
/// empty — no dense row/col grid is materialized). `name` is this sheet's IDENTITY (the key
/// `XlsxSheetsDiff` diffs by, per the recipe's name-keyed-collection convention); renaming a
/// sheet is therefore a remove-old-name + add-new-name at the diff level (documented — same
/// category as docx's OPC-part-rename gotcha), never a `name` field mutation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxSheet {
    pub name: String,
    #[serde(default)]
    pub cells: Vec<XlsxCell>,
}

/// 📘 Typed semantic view of the workbook: `xl/workbook.xml`'s sheet list, each resolved
/// through `xl/_rels/workbook.xml.rels` to its `xl/worksheets/sheetN.xml` part, plus the SST
/// (`xl/sharedStrings.xml`) kept as its own index-keyed `shared_strings` table — `t="s"` cells
/// reference it by index (`XlsxCellValue::SharedString(usize)`), never resolved eagerly.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxWorkbook {
    #[serde(default)]
    pub sheets: Vec<XlsxSheet>,
    #[serde(default)]
    pub shared_strings: Vec<String>,
}
//#endregion 🔖️XlsxModel

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xlsx")]
pub struct XlsxSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub opc: OpcPackage,
    #[state(artifact)]
    #[serde(default)]
    pub workbook: XlsxWorkbook,
}

impl Default for XlsxSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_XLSX_DOCUMENT_SCHEMA.into(), opc: OpcPackage::default(), workbook: XlsxWorkbook::default() }
    }
}

impl XlsxSnapshot {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_parts(opc: OpcPackage, workbook: XlsxWorkbook) -> Self {
        Self { schema: STDIO_XLSX_DOCUMENT_SCHEMA.into(), opc, workbook }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for XlsxSnapshot {
    const EXTENSION: &'static str = "xlsx";
    fn envelope_id() -> &'static str {
        "stdio.xlsx"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for i in (0..hex.len()).step_by(2) {
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?);
        }
        crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_xlsx(&bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::encode_xlsx(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for XlsxSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::encode_xlsx(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_xlsx(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
