//! puzzle5d -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired import leaf's doc comment (same wave,
//! `JsonSnapshot.value: serde_json::Value` -> stdio's own `JsonValue`). Routes through
//! `JsonSnapshot::from_value` (stdio's own real reverse `serde_json::Value -> JsonValue` bridge,
//! no hand-rolled converter here) and stdio's own real `write_json_pretty` for `serialize_bytes`
//! (the previous `serde_json::to_vec_pretty(&value)` would have serialized the internally-tagged
//! `JsonValue` shape verbatim, not real JSON text — a latent bug this fix also corrects).
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &Puzzle5dSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(value))
}

pub async fn serialize_bytes(snapshot: &Puzzle5dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
