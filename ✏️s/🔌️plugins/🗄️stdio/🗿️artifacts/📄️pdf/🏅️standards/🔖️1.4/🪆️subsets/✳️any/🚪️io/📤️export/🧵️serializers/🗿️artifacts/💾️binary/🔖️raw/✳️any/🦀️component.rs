//! Serialize stdio.pdf to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::pdf::PdfSnapshot;

pub fn register() {}

pub fn serialize(from: &PdfSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::pdf::engine::encode_pdf(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
