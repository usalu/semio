//! model <- json
use crate::artifacts::model::EnergyModelSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

/// 🌉 Bridges via json's own RFC8259 text codec (`JsonSnapshot::value` is `JsonValue`, json's
/// own key-order/lexeme-preserving model, not `serde_json::Value` — see json's snapshot module).
///
/// 🌱️ NOT converted to `pack::json`/`ToValue` — see the sibling `export/json` leaf's own docstring
/// note; `JsonSnapshot::to_serde_value()` is a foreign plugin API hard-typed to `serde_json::Value`.
pub async fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<EnergyModelSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let out: EnergyModelSnapshot = serde_json::from_value(from.to_serde_value()).map_err(|e| store::TextError::new(format!("energy_model<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(out)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<EnergyModelSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
