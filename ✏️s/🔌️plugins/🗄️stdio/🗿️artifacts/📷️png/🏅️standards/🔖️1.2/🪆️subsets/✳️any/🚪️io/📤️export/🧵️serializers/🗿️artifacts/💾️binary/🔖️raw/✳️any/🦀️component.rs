//! Serialize stdio.png to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::png::PngSnapshot;

pub async fn register() {}

pub async fn serialize(from: &PngSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::png::engine::encode_png(from).map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
