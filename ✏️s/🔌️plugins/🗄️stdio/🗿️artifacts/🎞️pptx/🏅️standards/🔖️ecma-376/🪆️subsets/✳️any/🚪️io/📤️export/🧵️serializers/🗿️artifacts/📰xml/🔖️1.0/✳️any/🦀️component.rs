//! xml bridge stub for stdio.pptx
use crate::artifacts::pptx::PptxSnapshot;
use crate::artifacts::xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA, schema::snapshot::XmlDocument};
pub fn register() {}
pub fn serialize(_from: &PptxSnapshot) -> Result<XmlSnapshot, store::PackError> {
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() })
}
