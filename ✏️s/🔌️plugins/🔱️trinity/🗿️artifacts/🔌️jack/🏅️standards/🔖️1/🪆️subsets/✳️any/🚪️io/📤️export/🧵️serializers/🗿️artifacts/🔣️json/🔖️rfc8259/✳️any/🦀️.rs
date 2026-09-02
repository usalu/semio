//! jack -> json
use crate::artifacts::jack::JackSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

/// 🌉 Bridges via json's own RFC8259 text codec (`JsonSnapshot::value` is `JsonValue`, json's
/// own key-order/lexeme-preserving model, not `pack::JsonValue` -- see json's snapshot module).
pub fn register() {}

pub fn serialize(snapshot: &JackSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = pack::json_from_dsl_value(&dsl::ToValue::to_value(snapshot));
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &JackSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
