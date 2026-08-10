//! playbook -> json
use crate::artifacts::playbook::PlaybookSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn serialize(snapshot: &PlaybookSnapshot) -> Result<JsonSnapshot, String> {
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: serde_json::to_value(snapshot).map_err(|e| e.to_string())? })
}
pub fn serialize_bytes(snapshot: &PlaybookSnapshot) -> Result<Vec<u8>, String> {
    serde_json::to_vec_pretty(&serialize(snapshot)?.value).map_err(|e| e.to_string())
}
