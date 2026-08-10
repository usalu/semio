//! Serialize stdio.tiff to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::tiff::TiffSnapshot;

pub fn register() {}

pub fn serialize(from: &TiffSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::tiff::engine::encode_tiff(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
