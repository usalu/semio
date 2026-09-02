//! puzzle2d <- json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's svg/dwg-pattern scope — see
//! `w5b--puzzle-report.md`): `JsonSnapshot.value` was retyped from `serde_json::Value` to stdio's
//! own lexeme-preserving `JsonValue` (own type, `#[serde(tag = "kind")]` — an intentional boundary
//! per that schema's own doc comment, NOT structurally plain JSON) by a concurrent stdio wave,
//! breaking this pre-existing leaf's compile. Fixed as a minimal lagging-call-site update: routes
//! through `JsonSnapshot::to_serde_value` (stdio's own real `JsonValue -> serde_json::Value`
//! bridge) plus stdio's own real `parse_json_text` — no hand-rolled converter here.
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<Puzzle2dSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let snap: Puzzle2dSnapshot = serde_json::from_value(from.to_serde_value()).map_err(|e| store::TextError::new(format!("puzzle2d<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle2dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
