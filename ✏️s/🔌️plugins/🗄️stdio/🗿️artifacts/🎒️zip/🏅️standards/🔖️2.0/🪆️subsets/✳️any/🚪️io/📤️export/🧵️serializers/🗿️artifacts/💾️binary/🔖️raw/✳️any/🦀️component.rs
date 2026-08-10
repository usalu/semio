//! Serialize stdio.zip to stdio.binary (encode ZIP bytes).

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::zip::ZipSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// 🎒️ Encode ZipSnapshot as ZIP container bytes.
pub fn serialize(from: &ZipSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::zip::engine::encode_zip(from, true)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot {
        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        bytes,
    })
}

/// Encode ZIP then wrap as binary pack bytes.
pub fn serialize_bytes(from: &ZipSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
