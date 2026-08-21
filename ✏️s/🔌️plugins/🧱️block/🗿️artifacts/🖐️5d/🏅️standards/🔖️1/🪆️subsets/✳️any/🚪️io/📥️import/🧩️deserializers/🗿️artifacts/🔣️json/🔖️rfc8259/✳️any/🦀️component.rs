//! block5d <- json
use crate::artifacts::block5d::Block5dSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &JsonSnapshot) -> Result<Block5dSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let snap: Block5dSnapshot = serde_json::from_value(from.to_serde_value()).map_err(|e| store::TextError::new(format!("block5d<-json: {e}"), dsl::TextSpan::at(1, 1)))?;

    Ok(snap)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Block5dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot::from_value(value))
}
