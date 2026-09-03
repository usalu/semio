//! puzzle2d -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired import leaf's doc comment (same wave,
//! `JsonSnapshot.value: serde_json::Value` -> stdio's own `JsonValue`). Routes through
//! `JsonSnapshot::from_value` (stdio's own real reverse `serde_json::Value -> JsonValue` bridge,
//! no hand-rolled converter here) and stdio's own real `write_json_pretty` for `serialize_bytes`
//! (the previous `serde_json::to_vec_pretty(&value)` would have serialized the internally-tagged
//! `JsonValue` shape verbatim, not real JSON text — a latent bug this fix also corrects).
//!
//! 🩹️ Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: no longer
//! routes through `serde_json::to_value` — `Puzzle2dSnapshot` only derives `Serialize` under
//! `#[cfg(test)]` now. `dsl::ToValue::to_value` (first-party) -> `dsl::json::from_dsl_value`
//! (`DslValue` -> stdio's own `JsonValue`) instead, same shape the sibling `block2d` leaf already
//! uses.
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Puzzle2dSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw = dsl::ToValue::to_value(snapshot);
    Ok(JsonSnapshot::from_value(dsl::json::from_dsl_value(&raw)))
}

pub fn serialize_bytes(snapshot: &Puzzle2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
