//! xml bridge stub for stdio.pptx
use crate::artifacts::pptx::{PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};
use crate::artifacts::xml::XmlSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(_from: &XmlSnapshot) -> Result<PptxSnapshot, store::TextError> {
    Ok(PptxSnapshot { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), ..Default::default() })
}
