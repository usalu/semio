//! Serialize stdio.pdf to stdio.deflate.

use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use crate::artifacts::pdf::PdfSnapshot;

pub fn register() {}

pub fn serialize(from: &PdfSnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let bytes = crate::artifacts::pdf::engine::encode_pdf(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), bytes })
}
