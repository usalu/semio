//! raster <- json
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — see w5b-close-report.md):
//! see the paired export leaf's doc comment. Mirrors it with the reverse structural converter and
//! stdio's own real `parse_json_text`.
use crate::artifacts::raster::{RasterSnapshot, RASTER_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{parse_json_text, JsonSnapshot, JsonValue};
use std::str::FromStr;
pub fn register() {}

/// 🔁️ Structural `JsonValue -> serde_json::Value` conversion (reverse of the export leaf's
/// converter — see this file's module doc comment).
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

pub fn deserialize(from: &JsonSnapshot) -> Result<RasterSnapshot, String> {
    let mut snap: RasterSnapshot = serde_json::from_value(from.to_serde_value()).map_err(|e| e.to_string())?;
    if snap.schema.is_empty() { snap.schema = RASTER_DOCUMENT_SCHEMA.into(); }
    Ok(snap)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let value = parse_json_text(text).map_err(|e| e.to_string())?;
    deserialize(&JsonSnapshot::from_value(value))
}
