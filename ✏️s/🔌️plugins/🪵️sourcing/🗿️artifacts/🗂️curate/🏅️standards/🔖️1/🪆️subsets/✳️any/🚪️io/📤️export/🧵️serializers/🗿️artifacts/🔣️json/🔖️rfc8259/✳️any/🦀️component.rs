//! curate -> json
//!
//! 🌉️ stdio_gap/foreign-lag fix (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) — mirrors
//! the paired import leaf's doc comment: bridges via json's own RFC8259 text codec rather than a
//! per-leaf structural converter, matching `s/plugin/lowpoly`'s identical export leaf.
use crate::artifacts::curate::CurateSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &CurateSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &CurateSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
