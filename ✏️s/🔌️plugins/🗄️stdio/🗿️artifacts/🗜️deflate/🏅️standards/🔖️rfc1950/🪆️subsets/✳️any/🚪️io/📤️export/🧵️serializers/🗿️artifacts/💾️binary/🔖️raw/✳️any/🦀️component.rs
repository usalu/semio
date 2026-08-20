//! Serialize stdio.deflate to stdio.binary (zlib-inflate payload).

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::deflate::DeflateSnapshot;

//#region Codec
/// Register serializer hooks.
pub async fn register() {}

/// 🗜️ Zlib-inflate deflate stream into a BinarySnapshot payload.
pub async fn serialize(from: &DeflateSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::deflate::standards::v_rfc1950::subsets::any::io::zlib_decompress(&from.payload).await.map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}

/// Inflate then encode as binary pack bytes.
pub async fn serialize_bytes(from: &DeflateSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from).await?, &store::PackEncodeOptions::default()).await
}
//#endregion Codec
