//! xml bridge stub for stdio.pptx
use crate::artifacts::pptx::{PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};
use crate::artifacts::xml::XmlSnapshot;
pub async fn register() {}
pub async fn deserialize(_from: &XmlSnapshot) -> Result<PptxSnapshot, store::TextError> {
    Ok(PptxSnapshot { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), ..Default::default() })
}
