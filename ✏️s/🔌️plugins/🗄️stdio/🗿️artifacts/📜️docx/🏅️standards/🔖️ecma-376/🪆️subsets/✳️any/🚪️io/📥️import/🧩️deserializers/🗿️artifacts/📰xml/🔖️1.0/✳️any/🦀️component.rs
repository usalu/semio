//! xml bridge stub for stdio.docx
use crate::artifacts::docx::{DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};
use crate::artifacts::xml::XmlSnapshot;
pub fn register() {}
pub fn deserialize(_from: &XmlSnapshot) -> Result<DocxSnapshot, store::TextError> {
    Ok(DocxSnapshot { schema: STDIO_DOCX_DOCUMENT_SCHEMA.into(), ..Default::default() })
}
