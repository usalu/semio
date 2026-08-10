//! Serialize stdio.gif to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::gif::GifSnapshot;

pub fn register() {}

pub fn serialize(from: &GifSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::gif::engine::encode_gif(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
