//! Serialize stdio.bcf to stdio.binary (encode ZIP bytes).

use crate::artifacts::bcf::BcfSnapshot;
use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};

//#region Codec
/// Register serializer hooks.
pub async fn register() {}

/// 🎒️ Encode BcfSnapshot as ZIP container bytes.
pub async fn serialize(from: &BcfSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::bcf::io::encode_bcf(from).await.map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}

/// Encode ZIP then wrap as binary pack bytes.
pub async fn serialize_bytes(from: &BcfSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from).await?, &store::PackEncodeOptions::default()).await
}
//#endregion Codec
