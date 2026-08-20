//! Serialize stdio.jpg to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::jpg::JpgSnapshot;

pub async fn register() {}

pub async fn serialize(from: &JpgSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::jpg::engine::encode_jpg(from).await.map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
