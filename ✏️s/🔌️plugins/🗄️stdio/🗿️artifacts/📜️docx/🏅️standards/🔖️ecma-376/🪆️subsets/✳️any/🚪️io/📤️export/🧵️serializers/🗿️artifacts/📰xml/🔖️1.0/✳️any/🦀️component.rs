//! xml bridge stub for stdio.docx
use crate::artifacts::docx::DocxSnapshot;
use crate::artifacts::xml::{schema::snapshot::XmlDocument, XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
pub async fn register() {}
pub async fn serialize(_from: &DocxSnapshot) -> Result<XmlSnapshot, store::PackError> {
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() })
}
