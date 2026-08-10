//! Serialize stdio.pptx to stdio.binary (encode ZIP bytes).

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::pptx::PptxSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// 🎒️ Encode PptxSnapshot as ZIP container bytes.
pub fn serialize(from: &PptxSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::pptx::engine::encode_pptx(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot {
        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        bytes,
    })
}

/// Encode ZIP then wrap as binary pack bytes.
pub fn serialize_bytes(from: &PptxSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
