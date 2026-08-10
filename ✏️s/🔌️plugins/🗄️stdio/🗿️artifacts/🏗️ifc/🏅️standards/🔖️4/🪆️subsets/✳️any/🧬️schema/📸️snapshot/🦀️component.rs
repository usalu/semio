//! 🧬️ IfcSnapshot schema — persistent fields + real Part-21 codecs (shared `step::engine::part21`
//! tokenizer — IFC4 is STEP Part-21 syntax with a different EXPRESS schema layered on top).

use crate::artifacts::step::engine::part21::{parse_part21, write_part21, Part21Document};
use crate::artifacts::ifc::STDIO_IFC_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.ifc` snapshot — the full, lossless generic Part-21 graph. Spatial
/// structure/placements/psets are derived analyzer views (`crate::artifacts::ifc::engine::spatial`),
/// not stored here.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc")]
pub struct IfcSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub document: Part21Document,
}

impl Default for IfcSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_IFC_DOCUMENT_SCHEMA.into(),
            document: Part21Document::default(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Part21Codec
impl store::ArtifactDsl for IfcSnapshot {
    const EXTENSION: &'static str = "ifc";
    fn envelope_id() -> &'static str { "stdio.ifc" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let document = parse_part21(body).map_err(|e| {
            store::TextError::new(format!("ifc parse: {e}"), dsl::TextSpan::at(1, 1))
        })?;
        Ok(Self { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), document })
    }
    fn print_dsl(&self) -> String {
        let body = write_part21(&self.document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for IfcSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_part21(&self.document).into_bytes();
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
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let document = parse_part21(&text).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(Self { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), document })
    }
}
//#endregion 🔖️Part21Codec
