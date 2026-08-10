//! glb to json
use crate::artifacts::glb::GlbSnapshot;
use crate::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn serialize(from: &GlbSnapshot) -> Result<JsonSnapshot, store::PackError> {
    let value = serde_json::from_str(from.payload.gltf_json.trim()).unwrap_or(serde_json::Value::Null);
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
