//! curate <- json
use crate::artifacts::curate::CurateSnapshot;
use crate::artifacts::curate::SOURCING_CURATE_SCHEMA;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<CurateSnapshot, store::TextError> {
    let _ = SOURCING_CURATE_SCHEMA;
    let mut out: CurateSnapshot = serde_json::from_value(from.value.clone())
        .map_err(|e| store::TextError::new(format!("curate<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<CurateSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
