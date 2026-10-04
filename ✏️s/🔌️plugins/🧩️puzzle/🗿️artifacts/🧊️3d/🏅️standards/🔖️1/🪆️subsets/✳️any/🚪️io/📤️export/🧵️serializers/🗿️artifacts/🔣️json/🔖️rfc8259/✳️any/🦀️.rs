//! puzzle3d -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired import leaf's doc comment (same wave,
//! `JsonSnapshot.value: serde_json::Value` -> stdio's own `JsonValue`). Routes through
//! `JsonSnapshot::from_value` (stdio's own real reverse `serde_json::Value -> JsonValue` bridge,
//! no hand-rolled converter here) and stdio's own real `write_json_pretty` for `serialize_bytes`
//! (the previous `serde_json::to_vec_pretty(&value)` would have serialized the internally-tagged
//! `JsonValue` shape verbatim, not real JSON text — a latent bug this fix also corrects).
//!
//! 🩹️ Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: no longer
//! routes through `serde_json::to_value` — `Puzzle3dSnapshot` only derives `Serialize` under
//! `#[cfg(test)]` now. `dsl::ToValue::to_value` (first-party) -> `semio_framework_pack_json::from_dsl_value`
//! (`DslValue` -> stdio's own `JsonValue`) instead, same shape the sibling `block3d` leaf already
//! uses.
use crate::Puzzle3dSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Puzzle3dSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw = crate::standards::v1::subsets::any::io::json_native::convert(semio_framework_value::ToValue::to_value(snapshot),false).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e,semio_framework_diagnostic::TextSpan::at(1,1)))?;
    Ok(JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&raw)))
}

pub fn serialize_bytes(snapshot: &Puzzle3dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
