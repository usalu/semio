//! model <- json
use crate::artifacts::model::{EnergyModelSnapshot, ENERGY_MODEL_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{parse_json_text, write_json_text};

/// 🌉 Bridges via json's own RFC8259 text codec (`JsonSnapshot::value` is `JsonValue`, json's
/// own key-order/lexeme-preserving model, not `serde_json::Value` — see json's snapshot module).
pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<EnergyModelSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let text = write_json_text(&from.value);
    let mut snap: EnergyModelSnapshot = serde_json::from_str(&text)
        .map_err(|e| store::TextError::new(format!("model<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    if snap.schema.is_empty() { snap.schema = ENERGY_MODEL_DOCUMENT_SCHEMA.into(); }
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<EnergyModelSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
