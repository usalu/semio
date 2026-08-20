//! Serialize stdio.txt to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;

//#region Codec
/// Register serializer hooks.
pub async fn register() {}

/// UTF-8 encode text into a BinarySnapshot.
pub async fn serialize(from: &TxtSnapshot) -> BinarySnapshot {
    BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: from.to_body().await.into_bytes() }
}

/// Encode as binary pack bytes.
pub async fn serialize_bytes(from: &TxtSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from), &store::PackEncodeOptions::default())
}
//#endregion Codec
