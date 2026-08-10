//! 📤️ Serialize `stdio.gltf` to stdio.json. Embeds any BIN-chunk-sourced (no-`uri`) buffer as a
//! base64 data uri first (via the shared `.gltf` JSON codec), since plain `stdio.json` has nowhere
//! else to carry those bytes -- same reasoning as `serialize_gltf_document`.
use crate::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use crate::artifacts::gltf::GltfSnapshot;
pub fn register() {}
pub fn serialize(from: &GltfSnapshot) -> Result<JsonSnapshot, store::PackError> {
    let embedded = crate::artifacts::gltf::engine::serialize_gltf_document(from);
    let value: serde_json::Value = serde_json::from_slice(&embedded).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
