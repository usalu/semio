//! model -> json
use crate::artifacts::model::EnergyModelSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

/// 🌉 Bridges via json's own RFC8259 text codec (`JsonSnapshot::value` is `JsonValue`, json's
/// own key-order/lexeme-preserving model, not `serde_json::Value` — see json's snapshot module).
///
/// 🌱️ NOT converted to `pack::json`/`ToValue` — `JsonSnapshot::from_value`/`.to_serde_value()`
/// (`semio_s_plugin_stdio::artifacts::json`) are hard-typed to `serde_json::Value`, a foreign
/// plugin's own API this crate does not own (same documented blocker as `➗️mathematical`'s sibling
/// `import/json`/`export/json` leaves). `EnergyModelSnapshot` keeps its `Serialize` derive
/// alongside `ToValue` specifically so this bridge — the only reason this crate cannot yet drop
/// `serde_json` from `Cargo.toml` — still compiles.
pub async fn register() {}

pub fn serialize(snapshot: &EnergyModelSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(value))
}

pub async fn serialize_bytes(snapshot: &EnergyModelSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
