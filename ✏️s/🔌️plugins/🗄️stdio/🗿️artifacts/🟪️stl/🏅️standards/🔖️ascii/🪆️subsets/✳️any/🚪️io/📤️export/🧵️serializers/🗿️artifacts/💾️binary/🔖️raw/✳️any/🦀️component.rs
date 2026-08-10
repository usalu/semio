//! 📤️ Serialize `stdio.stl` to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::stl::StlSnapshot;

pub fn register() {}

pub fn serialize(from: &StlSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::stl::engine::encode_stl_binary(from);
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
