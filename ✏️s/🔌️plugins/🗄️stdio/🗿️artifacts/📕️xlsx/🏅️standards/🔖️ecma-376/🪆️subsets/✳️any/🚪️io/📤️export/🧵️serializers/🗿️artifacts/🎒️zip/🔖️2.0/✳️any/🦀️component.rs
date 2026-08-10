//! Serialize stdio.xlsx to stdio.binary (encode ZIP bytes).

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::xlsx::XlsxSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// 🎒️ Encode XlsxSnapshot as ZIP container bytes.
pub fn serialize(from: &XlsxSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::xlsx::engine::encode_xlsx(from)
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(BinarySnapshot {
        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        bytes,
    })
}

/// Encode ZIP then wrap as binary pack bytes.
pub fn serialize_bytes(from: &XlsxSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
