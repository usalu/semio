//! note <- json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's svg/dwg-pattern scope — see
//! `w5b--report.md`): `JsonSnapshot.value` was retyped from `serde_json::Value` to stdio's own
//! lexeme-preserving `JsonValue` (own type, `#[serde(tag = "kind")]` — an intentional boundary
//! per that schema's own doc comment, NOT structurally plain JSON) by a concurrent stdio wave,
//! breaking this pre-existing leaf's compile. Fixed as a minimal lagging-call-site update (same
//! shape as the 🎞️animate plugin's own fix for its `🎬️present` artifact): a real, honest
//! structural `JsonValue -> serde_json::Value` converter plus stdio's own real `parse_json_text`.
use crate::artifacts::note::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{parse_json_text, JsonSnapshot, JsonValue};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;
use std::str::FromStr;
pub fn register() {}

/// 🔁️ Structural `JsonValue -> serde_json::Value` conversion (stdio's own `JsonValue` has no
/// built-in bridge to `serde_json::Value` — see this file's module doc comment).
fn json_value_to_serde(value: &JsonValue) -> serde_json::Value {
    match value {
        JsonValue::Null => serde_json::Value::Null,
        JsonValue::Bool { value } => serde_json::Value::Bool(*value),
        JsonValue::Number { lexeme } => serde_json::Number::from_str(lexeme).map(serde_json::Value::Number).unwrap_or(serde_json::Value::Null),
        JsonValue::String { value } => serde_json::Value::String(value.clone()),
        JsonValue::Array { items } => serde_json::Value::Array(items.iter().map(json_value_to_serde).collect()),
        JsonValue::Object { members } => serde_json::Value::Object(members.iter().map(|member| (member.key.clone(), json_value_to_serde(&member.value))).collect()),
    }
}

pub fn deserialize(from: &JsonSnapshot) -> Result<NoteSnapshot, String> {
    let mut snap: NoteSnapshot = serde_json::from_value(json_value_to_serde(&from.value)).map_err(|e| e.to_string())?;
    if snap.schema.is_empty() { snap.schema = NOTE_DOCUMENT_SCHEMA.into(); }
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    Ok(snap)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let value = parse_json_text(text).map_err(|e| e.to_string())?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
