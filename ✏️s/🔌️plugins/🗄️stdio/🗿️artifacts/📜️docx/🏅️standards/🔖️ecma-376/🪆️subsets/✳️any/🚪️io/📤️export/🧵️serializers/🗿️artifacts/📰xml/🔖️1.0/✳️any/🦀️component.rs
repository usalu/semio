//! xml bridge stub for stdio.docx
use crate::artifacts::docx::DocxSnapshot;
use crate::artifacts::xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA, schema::snapshot::XmlDocument};
pub fn register() {}
pub fn serialize(_from: &DocxSnapshot) -> Result<XmlSnapshot, store::PackError> {
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() })
}
