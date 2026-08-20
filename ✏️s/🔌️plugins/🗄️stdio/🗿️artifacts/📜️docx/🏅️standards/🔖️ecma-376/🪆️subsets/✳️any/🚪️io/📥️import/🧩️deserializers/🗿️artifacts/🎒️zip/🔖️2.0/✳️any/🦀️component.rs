//! Deserialize stdio.docx from stdio.binary (parse ZIP bytes).

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::docx::{DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
pub async fn register() {}

/// 🎒️ Parse ZIP container bytes into a DocxSnapshot.
pub async fn deserialize(from: &BinarySnapshot) -> Result<DocxSnapshot, store::PackError> {
    let mut snap = crate::artifacts::docx::engine::decode_docx(&from.bytes).await.map_err(|e| store::PackError::Schema(e.to_string()))?;
    snap.schema = STDIO_DOCX_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode a Binary pack then parse ZIP.
pub async fn deserialize_bytes(bytes: &[u8]) -> Result<DocxSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes).await?).await
}
//#endregion Codec
