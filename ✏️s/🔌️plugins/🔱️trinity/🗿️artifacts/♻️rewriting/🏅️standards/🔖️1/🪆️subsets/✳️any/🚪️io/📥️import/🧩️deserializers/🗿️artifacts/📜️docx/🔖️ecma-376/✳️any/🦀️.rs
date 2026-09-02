//! rewriting <- docx
use crate::artifacts::rewriting::RewritingSnapshot;
use semio_s_plugin_stdio::artifacts::docx::{DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &DocxSnapshot) -> Result<RewritingSnapshot, store::TextError> {
    let _ = (STDIO_DOCX_DOCUMENT_SCHEMA, from);
    Ok(RewritingSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RewritingSnapshot, store::TextError> {
    let _ = bytes;
    Ok(RewritingSnapshot::default())
}
