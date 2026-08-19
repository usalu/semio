//! 📤️ Serialize `stdio.las` to stdio.binary.
use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::las::LasSnapshot;
pub async fn register() {}
pub async fn serialize(from: &LasSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::las::engine::encode_las(from).map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
