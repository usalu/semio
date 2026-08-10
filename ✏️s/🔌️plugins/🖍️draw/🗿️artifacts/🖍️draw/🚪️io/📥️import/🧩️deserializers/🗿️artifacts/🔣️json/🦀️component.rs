//! draw <- json
use crate::artifacts::draw::{DrawSnapshot, DRAW_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn deserialize(from: &JsonSnapshot) -> Result<DrawSnapshot, String> {
    let mut snap: DrawSnapshot = serde_json::from_value(from.value.clone()).map_err(|e| e.to_string())?;
    if snap.schema.is_empty() { snap.schema = DRAW_DOCUMENT_SCHEMA.into(); }
    Ok(snap)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<DrawSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
