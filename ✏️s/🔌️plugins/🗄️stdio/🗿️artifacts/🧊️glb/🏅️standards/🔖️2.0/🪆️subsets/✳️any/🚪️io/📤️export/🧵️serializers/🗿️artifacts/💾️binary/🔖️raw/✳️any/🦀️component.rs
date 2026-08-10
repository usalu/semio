//! Serialize stdio.glb to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::glb::GlbSnapshot;

pub fn register() {}

pub fn serialize(from: &GlbSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::glb::engine::encode_glb(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
