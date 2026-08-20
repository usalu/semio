//! Serialize stdio.xlsx to stdio.binary (encode ZIP bytes).

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::xlsx::XlsxSnapshot;

//#region Codec
/// Register serializer hooks.
pub async fn register() {}

/// 🎒️ Encode XlsxSnapshot as ZIP container bytes.
pub async fn serialize(from: &XlsxSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::encode_xlsx(from).await.map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}

/// Encode ZIP then wrap as binary pack bytes.
pub async fn serialize_bytes(from: &XlsxSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from).await?, &store::PackEncodeOptions::default()).await
}
//#endregion Codec
