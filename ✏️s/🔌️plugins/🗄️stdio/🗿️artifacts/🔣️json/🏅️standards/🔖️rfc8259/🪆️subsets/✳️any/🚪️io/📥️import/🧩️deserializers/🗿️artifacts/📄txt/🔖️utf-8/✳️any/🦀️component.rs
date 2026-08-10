//! deser json via txt
use crate::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;
pub fn register() {}
pub fn deserialize(from: &TxtSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let value = serde_json::from_str(from.text.trim()).map_err(|e| store::TextError::new(format!("json parse: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
pub fn deserialize_text(text: &str) -> Result<JsonSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}
