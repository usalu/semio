//! 🧬️ DocxSnapshot — an OPC package (every part verbatim, lossless) plus a typed semantic view
//! of `word/document.xml` (a block tree: paragraphs-with-runs and tables) and `word/styles.xml`
//! (named styles). Unmodeled parts (`numbering.xml`, headers/footers, media, …) stay verbatim
//! inside `opc` — only `word/document.xml` and `word/styles.xml` are round-tripped through the
//! typed `document` model. Regions of `word/document.xml` this model doesn't understand (unknown
//! `w:rPr`/`w:pPr`/`w:trPr`/`w:tcPr`/`w:tblPr` children) are never dropped — they are carried
//! verbatim as raw XML child nodes on the owning `extra_*_properties` field and re-emitted on
//! encode, per the raw-retention rule.

use crate::artifacts::docx::STDIO_DOCX_DOCUMENT_SCHEMA;
use crate::artifacts::xml::schema::snapshot::XmlNode;
use crate::artifacts::zip::opc::OpcPackage;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️DocxModel
/// ✍️ One `w:r` run: literal text plus the formatting flags this artifact models. Any richer
/// `w:rPr` (color, font, size, …) that a decoded run carried is not lost — see
/// `extra_run_properties`, kept verbatim as raw `<w:rPr>` child XML so a round trip never silently
/// drops formatting this model doesn't understand.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxRun {
    pub text: String,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    /// 🗄️ Raw retention of `<w:rPr>` children this model doesn't interpret (color, font, size,
    /// strike, highlight, …), in original order.
    #[serde(default)]
    pub extra_run_properties: Vec<XmlNode>,
}

/// 📄️ One `w:p` paragraph: an ordered list of runs plus an optional named style reference.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxParagraph {
    #[serde(default)]
    pub runs: Vec<DocxRun>,
    /// 🎨️ `<w:pPr><w:pStyle w:val="…"/></w:pPr>` — references a `DocxStyle::id`.
    #[serde(default)]
    pub style: Option<String>,
    /// 🗄️ Raw retention of `<w:pPr>` children other than `<w:pStyle>` (alignment, numbering,
    /// spacing, …), in original order.
    #[serde(default)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxTableCell {
    #[serde(default)]
    pub blocks: Vec<DocxBlock>,
    /// 🗄️ Raw retention of `<w:tcPr>` children (width, span, merge, shading, …).
    #[serde(default)]
    pub extra_cell_properties: Vec<XmlNode>,
}

/// ➖️ One `w:tr` table row.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxTableRow {
    #[serde(default)]
    pub cells: Vec<DocxTableCell>,
    /// 🗄️ Raw retention of `<w:trPr>` children (height, header-row flag, …).
    #[serde(default)]
    pub extra_row_properties: Vec<XmlNode>,
}

/// 🏛️ One `w:tbl` table.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxTable {
    #[serde(default)]
    pub rows: Vec<DocxTableRow>,
    /// 🗄️ Raw retention of `<w:tblPr>` children (borders, width, look, …).
    #[serde(default)]
    pub extra_table_properties: Vec<XmlNode>,
}

/// 🧱️ One block-level content item inside `word/document.xml`'s `w:body` (or a table cell) — a
/// paragraph or a table, matching WordprocessingML's own block-content model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxStyle {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub based_on: Option<String>,
}

/// 📰 Typed semantic view of `word/document.xml`'s `w:body` (a block tree) plus `word/styles.xml`
/// (name-keyed by `DocxStyle::id`, styleId in WordprocessingML terms).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxDocument {
    #[serde(default)]
    pub body: Vec<DocxBlock>,
    #[serde(default)]
    pub styles: Vec<DocxStyle>,
}
//#endregion 🔖️DocxModel

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.docx")]
pub struct DocxSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 📦️ Lossless OPC container — every part verbatim, including `word/document.xml` and
    /// `word/styles.xml` (kept in sync with `document` on encode; see `engine::encode_docx`).
    #[state(artifact)]
    #[serde(default)]
    pub opc: OpcPackage,
    /// 🧬️ Typed semantic view of `word/document.xml` + `word/styles.xml`.
    #[state(artifact)]
    #[serde(default)]
    pub document: DocxDocument,
}

impl Default for DocxSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_DOCX_DOCUMENT_SCHEMA.into(), opc: OpcPackage::default(), document: DocxDocument::default() }
    }
}

impl DocxSnapshot {
    /// 🏗️ Builds a snapshot from an already-decoded OPC package plus its interpreted document.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_parts(opc: OpcPackage, document: DocxDocument) -> Self {
        Self { schema: STDIO_DOCX_DOCUMENT_SCHEMA.into(), opc, document }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for DocxSnapshot {
    const EXTENSION: &'static str = "docx";
    async fn envelope_id() -> &'static str {
        "stdio.docx"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
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
        crate::artifacts::docx::engine::decode_docx(&bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::docx::engine::encode_docx(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DocxSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::docx::engine::encode_docx(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::artifacts::docx::engine::decode_docx(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
