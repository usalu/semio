//! rewriting <- pdf
use crate::artifacts::rewriting::RewritingSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PdfSnapshot) -> Result<RewritingSnapshot, store::TextError> {
    let _ = (STDIO_PDF_DOCUMENT_SCHEMA, from);
    Ok(RewritingSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RewritingSnapshot, store::TextError> {
    let _ = bytes;
    Ok(RewritingSnapshot::default())
}
