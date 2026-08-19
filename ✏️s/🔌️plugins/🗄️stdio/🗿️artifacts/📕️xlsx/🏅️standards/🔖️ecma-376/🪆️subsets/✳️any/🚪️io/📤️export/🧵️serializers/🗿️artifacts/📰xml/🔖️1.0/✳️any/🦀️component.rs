//! xml bridge stub for stdio.xlsx
use crate::artifacts::xlsx::XlsxSnapshot;
use crate::artifacts::xml::{schema::snapshot::XmlDocument, XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
pub async fn register() {}
pub async fn serialize(_from: &XlsxSnapshot) -> Result<XmlSnapshot, store::PackError> {
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() })
}
