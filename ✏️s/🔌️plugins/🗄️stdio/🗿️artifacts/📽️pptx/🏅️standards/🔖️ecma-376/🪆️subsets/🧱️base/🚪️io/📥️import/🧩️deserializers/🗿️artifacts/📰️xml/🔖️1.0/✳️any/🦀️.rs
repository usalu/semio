//! xml bridge stub for stdio.pptx
use crate::{PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_xml::XmlSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(_from: &XmlSnapshot) -> Result<PptxSnapshot, store::TextError> {
    Ok(PptxSnapshot { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), ..Default::default() })
}
