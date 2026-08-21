//! rewrite -> json
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

/// 🌉 Bridges via json's own RFC8259 text codec (`JsonSnapshot::value` is `JsonValue`, json's
/// own key-order/lexeme-preserving model, not `serde_json::Value` -- see json's snapshot module).
pub async fn register() {}

pub async fn serialize(snapshot: &RewriteSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(value))
}

pub async fn serialize_bytes(snapshot: &RewriteSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
