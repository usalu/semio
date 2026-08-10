//! 🧬️ PdfSnapshot schema — persistent fields + real codecs.

use crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region RasterModel
/// 🖼️ RGBA raster (`width` × `height` × 4 bytes).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RasterImage {
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub rgba: Vec<u8>,
}
//#endregion RasterModel

//#region Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf")]
pub struct PdfSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub image: RasterImage,
}

impl Default for PdfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), image: RasterImage::default() }
    }
}
//#endregion Snapshot

//#region HandcraftedDocumentCodecs
impl store::DocumentDsl for PdfSnapshot {
    const EXTENSION: &'static str = "pdf";
    fn envelope_id() -> &'static str { "stdio.pdf" }

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
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1))
            })?;
            bytes.push(byte);
            i += 2;
        }
        crate::artifacts::pdf::engine::decode_pdf(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::pdf::engine::encode_pdf(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for PdfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::pdf::engine::encode_pdf(self).map_err(|e| store::PackError::Schema(e))?;
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
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        crate::artifacts::pdf::engine::decode_pdf(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion HandcraftedDocumentCodecs
