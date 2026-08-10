//! 🧬️ XlsxSnapshot — an OPC package (every part verbatim, lossless) plus a typed semantic view
//! of the workbook: sheets and cells, with shared-string references already resolved to literal
//! text (never left as a raw index — the #1 xlsx decode gotcha). Unmodeled parts (`styles.xml`,
//! themes, calc chain, …) stay verbatim inside `opc`.

use crate::artifacts::xlsx::STDIO_XLSX_DOCUMENT_SCHEMA;
use crate::artifacts::zip::opc::OpcPackage;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️XlsxModel
/// 🔢️ A cell's decoded value — text is always the literal string, whether the source encoded it
/// as a shared-string reference or an inline string.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum XlsxCellValue {
    Number(f64),
    Text(String),
    Bool(bool),
    Empty,
}

impl Default for XlsxCellValue {
    fn default() -> Self { Self::Empty }
}

/// 🧮 One worksheet cell.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxCell {
    /// 🏷️ A1-style cell reference, e.g. `"B2"`.
    pub reference: String,
    #[serde(default)]
    pub value: XlsxCellValue,
}

/// ➡️ One worksheet row.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxRow {
    pub index: u32,
    #[serde(default)]
    pub cells: Vec<XlsxCell>,
}

/// 📄 One worksheet.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxSheet {
    pub name: String,
    #[serde(default)]
    pub rows: Vec<XlsxRow>,
}

/// 📘 Typed semantic view of the workbook: `xl/workbook.xml`'s sheet list, each resolved
/// through `xl/_rels/workbook.xml.rels` to its `xl/worksheets/sheetN.xml` part, with `t="s"`
/// cells resolved through `xl/sharedStrings.xml`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxWorkbook {
    #[serde(default)]
    pub sheets: Vec<XlsxSheet>,
}
//#endregion 🔖️XlsxModel

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xlsx")]
pub struct XlsxSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub opc: OpcPackage,
    #[state(persistent)]
    #[serde(default)]
    pub workbook: XlsxWorkbook,
}

impl Default for XlsxSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_XLSX_DOCUMENT_SCHEMA.into(), opc: OpcPackage::default(), workbook: XlsxWorkbook::default() }
    }
}

impl XlsxSnapshot {
    pub fn from_parts(opc: OpcPackage, workbook: XlsxWorkbook) -> Self {
        Self { schema: STDIO_XLSX_DOCUMENT_SCHEMA.into(), opc, workbook }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for XlsxSnapshot {
    const EXTENSION: &'static str = "xlsx";
    fn envelope_id() -> &'static str { "stdio.xlsx" }
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
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1))
            })?);
        }
        crate::artifacts::xlsx::engine::decode_xlsx(&bytes)
            .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::xlsx::engine::encode_xlsx(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for XlsxSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::xlsx::engine::encode_xlsx(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::artifacts::xlsx::engine::decode_xlsx(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
