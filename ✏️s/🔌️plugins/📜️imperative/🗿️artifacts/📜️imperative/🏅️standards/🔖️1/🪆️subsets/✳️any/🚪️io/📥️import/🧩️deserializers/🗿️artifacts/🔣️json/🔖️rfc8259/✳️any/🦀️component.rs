//! imperative <- json
use crate::artifacts::imperative::ImperativeSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

/// 🩹️ `stdio_gap` fix (see the CSV import leaf's doc comment for the wave that caused this) —
/// `JsonSnapshot.value` moved from `serde_json::Value` to stdio's own lexeme-preserving `JsonValue`
/// (`#[serde(tag = "kind")]`, an intentional boundary type, not structurally plain JSON); mirrors
/// `🔱️jack`'s own fix, bridging via stdio's own `write_json_text`/`parse_json_text` codec instead
/// of a hand-rolled structural converter.
pub fn deserialize(from: &JsonSnapshot) -> Result<ImperativeSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let out: ImperativeSnapshot = serde_json::from_value(from.to_serde_value()).map_err(|e| store::TextError::new(format!("imperative<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ImperativeSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
