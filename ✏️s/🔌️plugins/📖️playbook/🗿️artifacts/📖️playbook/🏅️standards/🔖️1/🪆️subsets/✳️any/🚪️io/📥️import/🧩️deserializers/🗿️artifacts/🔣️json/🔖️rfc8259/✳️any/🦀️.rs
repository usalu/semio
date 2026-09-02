//! playbook <- json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix: `JsonSnapshot.value` was retyped from `serde_json::Value` to
//! stdio's own lexeme-preserving `JsonValue` (own type, `#[serde(tag = "kind")]` — an intentional
//! boundary per that schema's own doc comment, NOT structurally plain JSON) by a concurrent stdio
//! wave, breaking this pre-existing leaf's compile. Fixed as a minimal lagging-call-site update
//! (same shape as note's own fix for this exact leaf,
//! `🗒️note/.../🔣️json/🔖️rfc8259/✳️any/🦀️.rs`): goes through stdio's own
//! `JsonSnapshot::to_serde_value` bridge plus stdio's own real `parse_json_text`.
use crate::artifacts::playbook::{PlaybookSnapshot, PLAYBOOK_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{parse_json_text, JsonSnapshot};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;
pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<PlaybookSnapshot, String> {
    let mut snap: PlaybookSnapshot = serde_json::from_value(from.to_serde_value()).map_err(|e| e.to_string())?;
    if snap.schema.is_empty() {
        snap.schema = PLAYBOOK_DOCUMENT_SCHEMA.into();
    }
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    Ok(snap)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<PlaybookSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let value = parse_json_text(text).map_err(|e| e.to_string())?;
    deserialize(&JsonSnapshot::from_value(value))
}
