//! 📤️ Serialize `stdio.gltf` to stdio.json. Embeds any BIN-chunk-sourced (no-`uri`) buffer as a
//! base64 data uri first (via the shared `.gltf` JSON codec), since plain `stdio.json` has nowhere
//! else to carry those bytes -- same reasoning as `serialize_gltf_document`.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::json::schema::snapshot::parse_json_text;
use crate::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
pub async fn register() {}
pub async fn serialize(from: &GltfSnapshot) -> Result<JsonSnapshot, store::PackError> {
    let embedded = crate::artifacts::gltf::engine::serialize_gltf_document(from);
    let text = String::from_utf8(embedded).map_err(|e| store::PackError::Schema(e.to_string()))?;
    let value = parse_json_text(&text).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
