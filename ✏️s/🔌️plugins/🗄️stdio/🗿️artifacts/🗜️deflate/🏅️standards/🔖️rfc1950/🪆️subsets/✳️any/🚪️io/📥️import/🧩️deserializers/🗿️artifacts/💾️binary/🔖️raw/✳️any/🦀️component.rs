//! Deserialize stdio.deflate from stdio.binary (zlib-compress payload).

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
pub fn register() {}

/// 🗜️ Zlib-compress binary payload into a DeflateSnapshot.
pub fn deserialize(from: &BinarySnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let payload = crate::artifacts::deflate::engine::zlib_compress(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(DeflateSnapshot {
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        payload,
        ..Default::default()
    })
}

/// Decode a Binary pack then zlib-compress.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<DeflateSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
//#endregion Codec
