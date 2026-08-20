//! xml bridge stub for stdio.pptx
use crate::artifacts::pptx::PptxSnapshot;
use crate::artifacts::xml::{schema::snapshot::XmlDocument, XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(_from: &PptxSnapshot) -> Result<XmlSnapshot, store::PackError> {
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() })
}
