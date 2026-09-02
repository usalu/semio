//! block2d <- json
use crate::artifacts::block2d::Block2dSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &JsonSnapshot) -> Result<Block2dSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw: dsl::DslValue = from.to_serde_value().into();
    let snap: Block2dSnapshot = dsl::FromValue::from_value(raw).map_err(|e| store::TextError::new(format!("block2d<-json: {e}"), dsl::TextSpan::at(1, 1)))?;

    Ok(snap)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Block2dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
