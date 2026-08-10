//! Deserialize stdio.zip from stdio.deflate (inflate then parse ZIP).

use crate::artifacts::deflate::DeflateSnapshot;
use crate::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
pub fn register() {}

/// 🎒️ Inflate zlib stream then parse ZIP.
pub fn deserialize(from: &DeflateSnapshot) -> Result<ZipSnapshot, store::PackError> {
    let payload = crate::artifacts::deflate::engine::zlib_decompress(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    let mut snap = crate::artifacts::zip::engine::decode_zip(&payload)
        .map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_ZIP_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode deflate pack then parse.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<ZipSnapshot, store::PackError> {
    deserialize(&<DeflateSnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
//#endregion Codec
