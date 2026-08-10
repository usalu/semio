//! Serialize stdio.zip to stdio.deflate (encode ZIP then zlib-compress).

use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use crate::artifacts::zip::ZipSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// Encode ZIP bytes then zlib-compress via deflate artifact.
pub fn serialize(from: &ZipSnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let zip_bytes = crate::artifacts::zip::engine::encode_zip(from, true)
        .map_err(|e| store::PackError::Schema(e))?;
    let bytes = crate::artifacts::deflate::engine::zlib_compress(&zip_bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(DeflateSnapshot {
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        bytes,
    })
}

/// Encode as deflate pack bytes.
pub fn serialize_bytes(from: &ZipSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
