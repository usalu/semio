//! 📤️ Serialize `stdio.las` to stdio.binary.
use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::las::LasSnapshot;
pub fn register() {}
pub fn serialize(from: &LasSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::las::schema::snapshot::las_bytes_from_vertices(&from.vertices);
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
