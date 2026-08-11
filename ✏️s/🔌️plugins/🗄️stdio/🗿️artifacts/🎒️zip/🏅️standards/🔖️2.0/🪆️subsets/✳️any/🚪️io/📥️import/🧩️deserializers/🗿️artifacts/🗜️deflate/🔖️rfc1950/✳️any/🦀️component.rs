//! Deserialize stdio.zip from stdio.deflate (inflate then parse ZIP).

use crate::artifacts::deflate::DeflateSnapshot;
use crate::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
pub fn register() {}

/// 🎒️ `from.payload` is already the decompressed RFC1950 payload (typed `DeflateSnapshot`
/// decoding already inflated it) -- parse it as ZIP directly.
pub fn deserialize(from: &DeflateSnapshot) -> Result<ZipSnapshot, store::PackError> {
    let mut snap = crate::artifacts::zip::engine::decode_zip(&from.payload)
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
    snap.schema = STDIO_ZIP_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode deflate pack then parse.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<ZipSnapshot, store::PackError> {
    deserialize(&<DeflateSnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
//#endregion Codec
