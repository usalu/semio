//! rewrite <- json
use crate::artifacts::rewrite::{RewriteSnapshot};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{parse_json_text, write_json_text};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<RewriteSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let text = write_json_text(&from.value);
    let snap: RewriteSnapshot = serde_json::from_str(&text)
        .map_err(|e| store::TextError::new(format!("rewrite<-json: {e}"), dsl::TextSpan::at(1, 1)))?;

    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RewriteSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
