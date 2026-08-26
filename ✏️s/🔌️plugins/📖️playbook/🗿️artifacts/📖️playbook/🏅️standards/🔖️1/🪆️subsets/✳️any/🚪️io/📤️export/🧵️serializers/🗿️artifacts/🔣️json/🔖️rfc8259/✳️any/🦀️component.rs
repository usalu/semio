//! playbook -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired import leaf's doc comment (same wave,
//! `JsonSnapshot.value: serde_json::Value` -> stdio's own `JsonValue`). Mirrors note's identical
//! fix (`🗒️note/.../🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`), going through stdio's own
//! `From<serde_json::Value> for JsonValue` bridge, and stdio's own real `write_json_pretty` for
//! `serialize_bytes` (the previous `serde_json::to_vec_pretty(&value)` would have serialized the
//! internally-tagged `JsonValue` shape verbatim, not real JSON text — a latent bug this fix also
//! corrects).
use crate::artifacts::playbook::PlaybookSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{write_json_pretty, JsonSnapshot};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;
pub fn register() {}

pub fn serialize(snapshot: &PlaybookSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(value))
}
pub fn serialize_bytes(snapshot: &PlaybookSnapshot) -> Result<Vec<u8>, String> {
    Ok(write_json_pretty(&serialize(snapshot).map_err(|e| e.to_string())?.value).into_bytes())
}
