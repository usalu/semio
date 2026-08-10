//! deser xml via txt
use crate::artifacts::xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;
pub fn register() {}
pub fn deserialize(from: &TxtSnapshot) -> Result<XmlSnapshot, store::TextError> {
    let value = serde_xml::from_str(from.text.trim()).map_err(|e| store::TextError::new(format!("xml parse: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), value })
}
pub fn deserialize_text(text: &str) -> Result<XmlSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}
