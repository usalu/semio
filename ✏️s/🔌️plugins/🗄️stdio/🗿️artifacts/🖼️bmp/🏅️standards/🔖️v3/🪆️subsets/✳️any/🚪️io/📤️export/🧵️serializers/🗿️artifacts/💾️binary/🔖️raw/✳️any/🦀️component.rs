//! 📤️ Serialize `stdio.bmp` to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::bmp::BmpSnapshot;

pub fn register() {}

pub fn serialize(from: &BmpSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::bmp::engine::encode_bmp(from).map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
