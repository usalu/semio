//! rewriting <- json
use crate::artifacts::rewriting::RewritingSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<RewritingSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let out: RewritingSnapshot = dsl::FromValue::from_value(pack::json_to_dsl_value(&from.to_pack_value())).map_err(|e: dsl::ValueError| store::TextError::new(format!("rewriting<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RewritingSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
