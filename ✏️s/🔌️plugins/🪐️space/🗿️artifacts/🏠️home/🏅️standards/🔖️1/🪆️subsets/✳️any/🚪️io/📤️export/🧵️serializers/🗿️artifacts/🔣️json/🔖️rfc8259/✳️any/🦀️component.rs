//! home -> json
use crate::artifacts::home::SHomeSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub async fn register() {}

/// 🌉 `JsonSnapshot::value` is stdio's own key-order/lexeme-preserving `JsonValue`, not
/// `pack::JsonValue` directly (stdio's RFC8259 rework) — bridge via `JsonSnapshot::from_value`,
/// which now accepts `pack::JsonValue` too (`impl From<pack::JsonValue> for JsonValue`, stdio's
/// own snapshot component).
pub async fn serialize(snapshot: &SHomeSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = pack::json_from_dsl_value(&dsl::ToValue::to_value(snapshot));
    Ok(JsonSnapshot::from_value(value))
}

pub async fn serialize_bytes(snapshot: &SHomeSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
