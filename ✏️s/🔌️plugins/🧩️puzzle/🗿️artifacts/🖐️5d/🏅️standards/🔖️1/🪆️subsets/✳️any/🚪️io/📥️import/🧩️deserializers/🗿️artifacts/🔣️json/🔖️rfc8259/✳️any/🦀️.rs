//! puzzle5d <- json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's svg/dwg-pattern scope — see
//! `w5b--puzzle-report.md`): `JsonSnapshot.value` was retyped from `serde_json::Value` to stdio's
//! own lexeme-preserving `JsonValue` (own type, `#[serde(tag = "kind")]` — an intentional boundary
//! per that schema's own doc comment, NOT structurally plain JSON) by a concurrent stdio wave,
//! breaking this pre-existing leaf's compile. Fixed as a minimal lagging-call-site update: routes
//! through `JsonSnapshot::to_serde_value` (stdio's own real `JsonValue -> serde_json::Value`
//! bridge) plus stdio's own real `parse_json_text` — no hand-rolled converter here.
//!
//! 🩹️ Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: no longer
//! routes through `serde_json::from_value` — `Puzzle5dSnapshot` only derives `Deserialize` under
//! `#[cfg(test)]` now. The number lexemes cross into `dsl::DslValue` through stdio's own
//! first-party `to_pack_value()` bridge, then `dsl::FromValue::from_value` hydrates the typed
//! snapshot — same shape the sibling `block5d` leaf already uses. NOT through
//! `to_serde_value()`: `serde_json`'s default (no `float_roundtrip`) number parser reconstructs an
//! f64 as `significand as f64 * 10^exponent`, which is off by one ULP for any 17-significant-digit
//! literal (`0.42839899821678995` came back as `0.4283989982167899`) and made every example whose
//! geometry needs full f64 precision fail its lossless-round-trip oracle.
use crate::Puzzle5dSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<Puzzle5dSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw: dsl::DslValue = dsl::json::to_dsl_value(&from.to_pack_value());
    let snap: Puzzle5dSnapshot = dsl::FromValue::from_value(raw).map_err(|e| store::TextError::new(format!("puzzle5d<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle5dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
