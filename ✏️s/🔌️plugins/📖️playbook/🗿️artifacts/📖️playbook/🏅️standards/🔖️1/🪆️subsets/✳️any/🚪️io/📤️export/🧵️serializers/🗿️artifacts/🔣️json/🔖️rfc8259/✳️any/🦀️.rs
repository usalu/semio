//! playbook -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired import leaf's doc comment (same wave,
//! `JsonSnapshot.value: serde_json::Value` -> stdio's own `JsonValue`). Goes through
//! `pack::json`'s own `from_dsl_value` (`protocol::json` here) and stdio's own
//! `From<pack::JsonValue> for JsonValue` bridge — no `serde_json` crossing left — plus stdio's real
//! `write_json_pretty` for `serialize_bytes` (the previous `serde_json::to_vec_pretty(&value)`
//! would have serialized the internally-tagged `JsonValue` shape verbatim, not real JSON text — a
//! latent bug this fix also corrects).
use crate::artifacts::playbook::PlaybookSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{write_json_pretty, JsonSnapshot};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;
pub fn register() {}

pub fn serialize(snapshot: &PlaybookSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = protocol::json::from_dsl_value(&protocol::ToValue::to_value(snapshot));
    Ok(JsonSnapshot::from_value(value))
}
pub fn serialize_bytes(snapshot: &PlaybookSnapshot) -> Result<Vec<u8>, String> {
    Ok(write_json_pretty(&serialize(snapshot).map_err(|e| e.to_string())?.value).into_bytes())
}
