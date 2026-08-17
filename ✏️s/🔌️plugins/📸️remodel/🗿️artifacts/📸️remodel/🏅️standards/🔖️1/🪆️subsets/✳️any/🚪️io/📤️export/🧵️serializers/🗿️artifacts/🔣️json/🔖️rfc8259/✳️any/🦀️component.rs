//! remodel -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's video/image codec-extraction scope):
//! `JsonSnapshot.value` was retyped from `serde_json::Value` to stdio's own lexeme-preserving
//! `JsonValue` (`#[serde(tag = "kind")]`, NOT structurally plain JSON by design) by a concurrent
//! stdio wave, breaking this pre-existing placeholder leaf's compile. Fixed as a minimal
//! lagging-call-site update, mirroring the same pattern animate/fem/architect used for the
//! identical gap: a real, honest structural `serde_json::Value -> JsonValue` converter (stdio
//! provides no such bridge) plus stdio's own real `write_json_pretty` text codec for
//! `serialize_bytes`.
use crate::artifacts::remodel::RemodelSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;

pub fn register() {}

pub fn serialize(snapshot: &RemodelSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &RemodelSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
