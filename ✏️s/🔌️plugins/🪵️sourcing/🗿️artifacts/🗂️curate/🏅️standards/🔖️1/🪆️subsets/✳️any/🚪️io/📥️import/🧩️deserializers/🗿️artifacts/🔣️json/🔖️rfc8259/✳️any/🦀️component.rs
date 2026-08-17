//! curate <- json
//!
//! 🌉️ stdio_gap/foreign-lag fix (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) —
//! `JsonSnapshot::value` is stdio's own `JsonValue` (key-order/lexeme-preserving RFC8259 model,
//! never `serde_json::Value` — see that snapshot module's own doc). Bridges via json's own text
//! codec rather than a per-leaf structural converter, mirroring `s/plugin/lowpoly`'s identical
//! leaf (same wave-independent pattern, e.g. the semio/drawing svg leaves' mirrored base64 codec).
use crate::artifacts::curate::CurateSnapshot;
use crate::artifacts::curate::SOURCING_CURATE_SCHEMA;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<CurateSnapshot, store::TextError> {
    let _ = SOURCING_CURATE_SCHEMA;
    let out: CurateSnapshot = serde_json::from_value(from.to_serde_value())
        .map_err(|e| store::TextError::new(format!("curate<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<CurateSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
