//! puzzle5d -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired import leaf's doc comment (same wave,
//! `JsonSnapshot.value: serde_json::Value` -> stdio's own `JsonValue`). Mirrors it with the
//! reverse structural converter and stdio's own real `write_json_pretty` for `serialize_bytes`
//! (the previous `serde_json::to_vec_pretty(&value)` would have serialized the internally-tagged
//! `JsonValue` shape verbatim, not real JSON text — a latent bug this fix also corrects).
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{write_json_pretty, JsonMember, JsonValue};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

/// 🔁️ Structural `serde_json::Value -> JsonValue` conversion (reverse of the import leaf's
/// converter — see this file's module doc comment and that leaf's for the stdio_gap this fixes).
fn serde_to_json_value(value: &serde_json::Value) -> JsonValue {
    match value {
        serde_json::Value::Null => JsonValue::Null,
        serde_json::Value::Bool(value) => JsonValue::Bool { value: *value },
        serde_json::Value::Number(number) => JsonValue::Number { lexeme: number.to_string() },
        serde_json::Value::String(value) => JsonValue::String { value: value.clone() },
        serde_json::Value::Array(items) => JsonValue::Array { items: items.iter().map(serde_to_json_value).collect() },
        serde_json::Value::Object(members) => JsonValue::Object { members: members.iter().map(|(key, value)| JsonMember { key: key.clone(), value: serde_to_json_value(value) }).collect() },
    }
}

pub fn serialize(snapshot: &Puzzle5dSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &Puzzle5dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
