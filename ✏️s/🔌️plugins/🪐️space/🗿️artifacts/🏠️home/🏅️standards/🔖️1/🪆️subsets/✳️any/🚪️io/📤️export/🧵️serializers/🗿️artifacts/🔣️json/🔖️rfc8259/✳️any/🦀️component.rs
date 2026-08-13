//! home -> json
use crate::artifacts::home::SHomeSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

/// 🌉 `JsonSnapshot::value` is stdio's own key-order/lexeme-preserving `JsonValue`, not
/// `serde_json::Value` directly (stdio's RFC8259 rework) — bridge via `JsonSnapshot::from_value`
/// like every other already-migrated plugin's `-> json` serializer.
pub fn serialize(snapshot: &SHomeSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &SHomeSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
