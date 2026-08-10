//! 🧬️ GlbSnapshot schema.

use crate::artifacts::glb::STDIO_GLB_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlbPayload {
    #[serde(default)]
    pub gltf_json: String,
    #[serde(default)]
    pub bin: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.glb")]
pub struct GlbSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub payload: GlbPayload,
}

impl Default for GlbSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_GLB_DOCUMENT_SCHEMA.into(), payload: GlbPayload { gltf_json: r#"{"asset":{"version":"2.0"}}"#.into(), bin: Vec::new() } }
    }
}

impl store::DocumentDsl for GlbSnapshot {
    const EXTENSION: &'static str = "glb";
    fn envelope_id() -> &'static str { "stdio.glb" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) { Ok((_, r)) => r, Err(_) => text };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        let mut bytes = Vec::new();
        for i in (0..hex.len()).step_by(2) {
            bytes.push(u8::from_str_radix(&hex[i..i+2], 16).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1,1)))?);
        }
        crate::artifacts::glb::engine::decode_glb(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1,1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::glb::engine::encode_glb(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id("stdio.glb", store::semio_format::Component::Dsl, 1).unwrap();
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for GlbSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::glb::engine::encode_glb(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id("stdio.glb", store::semio_format::Component::Pack, 1)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != "stdio.glb" { return Err(store::PackError::Schema("envelope mismatch".into())); }
        let _ = options;
        crate::artifacts::glb::engine::decode_glb(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
