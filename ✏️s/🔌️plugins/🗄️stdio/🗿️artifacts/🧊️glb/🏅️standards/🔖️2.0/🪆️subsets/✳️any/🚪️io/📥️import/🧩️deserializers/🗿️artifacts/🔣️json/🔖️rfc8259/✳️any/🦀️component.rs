//! glb from json
use crate::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA, schema::snapshot::GlbPayload};
use crate::artifacts::json::JsonSnapshot;
pub fn register() {}
pub fn deserialize(from: &JsonSnapshot) -> Result<GlbSnapshot, store::TextError> {
    let gltf_json = serde_json::to_string(&from.value).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1,1)))?;
    Ok(GlbSnapshot { schema: STDIO_GLB_DOCUMENT_SCHEMA.into(), payload: GlbPayload { gltf_json, bin: Vec::new() } })
}
