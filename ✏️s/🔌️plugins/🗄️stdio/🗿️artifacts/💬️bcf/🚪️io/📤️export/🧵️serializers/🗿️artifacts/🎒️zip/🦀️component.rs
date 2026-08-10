//! Serialize stdio.bcf to stdio.binary (encode ZIP bytes).

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::bcf::BcfSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// 🎒️ Encode BcfSnapshot as ZIP container bytes.
pub fn serialize(from: &BcfSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::bcf::engine::encode_bcf(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot {
        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        bytes,
    })
}

/// Encode ZIP then wrap as binary pack bytes.
pub fn serialize_bytes(from: &BcfSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::DocumentPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
