//! 📤️ Serialize `stdio.dwg` to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::dwg::DwgSnapshot;

pub async fn register() {}

pub async fn serialize(from: &DwgSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::dwg::schema::snapshot::encode_dwg(from).await.map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
