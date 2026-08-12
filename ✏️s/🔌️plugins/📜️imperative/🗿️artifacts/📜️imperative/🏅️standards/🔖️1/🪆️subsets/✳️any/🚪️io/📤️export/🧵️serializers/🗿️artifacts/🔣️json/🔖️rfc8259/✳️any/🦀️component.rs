//! imperative -> json
use crate::artifacts::imperative::ImperativeSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{parse_json_text, write_json_pretty};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

/// 🩹️ `stdio_gap` fix (see the paired import leaf's doc comment) — bridges via json's own RFC8259
/// text codec (`JsonSnapshot::value` is `JsonValue`, not `serde_json::Value`), mirroring `🔱️jack`'s
/// own fix. `serialize_bytes` now goes through `write_json_pretty` rather than
/// `serde_json::to_vec_pretty(&value)`, which would have serialized the internally-tagged
/// `JsonValue` shape verbatim instead of real JSON text.
pub fn serialize(snapshot: &ImperativeSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &ImperativeSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
