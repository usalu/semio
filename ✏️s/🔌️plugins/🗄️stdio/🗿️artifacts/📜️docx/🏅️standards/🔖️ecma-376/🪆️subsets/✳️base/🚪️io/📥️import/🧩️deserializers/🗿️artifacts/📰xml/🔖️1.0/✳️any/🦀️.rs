//! xml bridge stub for stdio.docx
use crate::artifacts::docx::{DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};
use crate::artifacts::xml::XmlSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(_from: &XmlSnapshot) -> Result<DocxSnapshot, store::TextError> {
    Ok(DocxSnapshot { schema: STDIO_DOCX_DOCUMENT_SCHEMA.into(), ..Default::default() })
}
