//! rewrite <- docx
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_s_plugin_stdio::artifacts::docx::{DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &DocxSnapshot) -> Result<RewriteSnapshot, store::TextError> {
    let _ = (STDIO_DOCX_DOCUMENT_SCHEMA, from);
    Ok(RewriteSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RewriteSnapshot, store::TextError> {
    let _ = bytes;
    Ok(RewriteSnapshot::default())
}
