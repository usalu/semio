//! xml bridge stub for stdio.xlsx
use crate::artifacts::xlsx::XlsxSnapshot;
use crate::artifacts::xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA, schema::snapshot::XmlDocument};
pub fn register() {}
pub fn serialize(_from: &XlsxSnapshot) -> Result<XmlSnapshot, store::PackError> {
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() })
}
