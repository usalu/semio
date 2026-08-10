//! 🧬️ MdSnapshot schema — persistent fields + real codecs.

use crate::artifacts::md::STDIO_MD_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.md` snapshot (lossless markdown text).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md")]
pub struct MdSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub body: String,
}

impl Default for MdSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
            body: String::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for MdSnapshot {
    const EXTENSION: &'static str = "md";
    fn envelope_id() -> &'static str { "stdio.md" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest.to_string(),
            Err(_) => text.to_string(),
        };
        Ok(Self { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), body })
    }
    fn print_dsl(&self) -> String {
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &self.body)
    }
}

impl store::DocumentPack for MdSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = self.body.as_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, raw))
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
        let body = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(Self { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), body })
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
