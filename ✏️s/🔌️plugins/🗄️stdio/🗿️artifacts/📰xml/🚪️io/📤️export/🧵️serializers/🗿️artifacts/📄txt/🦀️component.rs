//! ser xml to txt
use crate::artifacts::xml::XmlSnapshot;
use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn serialize(from: &XmlSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = serde_xml::to_string_pretty(PLACEHOLDER_VALUE_REF).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })
}
pub fn serialize_text(from: &XmlSnapshot) -> Result<String, store::PackError> {
    Ok(store::DocumentDsl::print_dsl(&serialize(from)?))
}
