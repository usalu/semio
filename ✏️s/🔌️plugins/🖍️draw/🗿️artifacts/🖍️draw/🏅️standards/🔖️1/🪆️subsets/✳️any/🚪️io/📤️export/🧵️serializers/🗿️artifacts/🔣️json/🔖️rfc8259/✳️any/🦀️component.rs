//! draw -> json
//!
//! 🔁️ Bridges via stdio's own real RFC8259 text codec (`parse_json_text`/`write_json_pretty`), not
//! `serde_json::Value`, since `JsonSnapshot.value` is stdio's own lexeme-preserving `JsonValue`
//! model now — see the sibling import leaf's module doc for the full note.
use crate::artifacts::draw::DrawSnapshot;
use semio_s_plugin_stdio::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn serialize(snapshot: &DrawSnapshot) -> Result<JsonSnapshot, String> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| e.to_string())?;
    Ok(JsonSnapshot::from_value(value))
}
pub fn serialize_bytes(snapshot: &DrawSnapshot) -> Result<Vec<u8>, String> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
