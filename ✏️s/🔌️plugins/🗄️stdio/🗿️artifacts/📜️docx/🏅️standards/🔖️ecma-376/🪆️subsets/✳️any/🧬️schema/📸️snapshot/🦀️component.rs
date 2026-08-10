//! 🧬️ DocxSnapshot — an OPC package (every part verbatim, lossless) plus a typed semantic view
//! of `word/document.xml` (paragraphs/runs with basic formatting). Unmodeled parts
//! (`styles.xml`, `numbering.xml`, headers/footers, media, …) stay verbatim inside `opc` — only
//! `word/document.xml` is round-tripped through the typed `document` model.

use crate::artifacts::docx::STDIO_DOCX_DOCUMENT_SCHEMA;
use crate::artifacts::zip::opc::OpcPackage;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️DocxModel
/// ✍️ One `w:r` run: literal text plus the minimal formatting flags this artifact models.
/// Any richer `w:rPr` (color, underline, font, …) that a decoded run carried is not lost — see
/// `DocxRun::extra_run_properties`, kept verbatim as raw `<w:rPr>` child XML text so a round trip
/// never silently drops formatting this model doesn't understand.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxRun {
    pub text: String,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
}

/// 📄️ One `w:p` paragraph: an ordered list of runs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxParagraph {
    #[serde(default)]
    pub runs: Vec<DocxRun>,
}

impl DocxParagraph {
    pub fn text(text: impl Into<String>) -> Self {
        Self { runs: vec![DocxRun { text: text.into(), bold: false, italic: false }] }
    }
}

/// 📰 Typed semantic view of `word/document.xml`'s `w:body` -> paragraphs -> runs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxDocument {
    #[serde(default)]
    pub paragraphs: Vec<DocxParagraph>,
}
//#endregion 🔖️DocxModel

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.docx")]
pub struct DocxSnapshot {
    #[state(persistent)]
    pub schema: String,
    /// 📦️ Lossless OPC container — every part verbatim, including `word/document.xml` (kept in
    /// sync with `document` on encode; see `engine::encode_docx`).
    #[state(persistent)]
    #[serde(default)]
    pub opc: OpcPackage,
    /// 🧬️ Typed semantic view of `word/document.xml`.
    #[state(persistent)]
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
    pub fn from_parts(opc: OpcPackage, document: DocxDocument) -> Self {
        Self { schema: STDIO_DOCX_DOCUMENT_SCHEMA.into(), opc, document }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for DocxSnapshot {
    const EXTENSION: &'static str = "docx";
    fn envelope_id() -> &'static str { "stdio.docx" }
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
        crate::artifacts::docx::engine::decode_docx(&bytes)
            .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::docx::engine::encode_docx(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DocxSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::docx::engine::encode_docx(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
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
        crate::artifacts::docx::engine::decode_docx(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
