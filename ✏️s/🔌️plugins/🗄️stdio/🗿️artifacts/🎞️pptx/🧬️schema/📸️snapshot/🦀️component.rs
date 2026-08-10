//! 🧬️ PptxSnapshot — OOXML zip parts.

use crate::artifacts::pptx::STDIO_PPTX_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxEntry {
    pub name: String,
    #[serde(default)]
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pptx")]
pub struct PptxSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub entries: Vec<PptxEntry>,
}

impl Default for PptxSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), entries: Vec::new() }
    }
}

impl store::DocumentDsl for PptxSnapshot {
    const EXTENSION: &'static str = "pptx";
    fn envelope_id() -> &'static str { "stdio.pptx" }
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
        crate::artifacts::pptx::engine::decode_pptx(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::pptx::engine::encode_pptx(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for PptxSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::pptx::engine::encode_pptx(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::artifacts::pptx::engine::decode_pptx(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
