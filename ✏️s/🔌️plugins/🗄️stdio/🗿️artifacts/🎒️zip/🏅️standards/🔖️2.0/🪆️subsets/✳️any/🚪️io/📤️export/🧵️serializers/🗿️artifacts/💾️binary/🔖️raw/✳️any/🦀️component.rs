//! Serialize stdio.zip to stdio.binary (encode ZIP bytes).

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::zip::ZipSnapshot;

//#region Codec
/// Register serializer hooks.
pub async fn register() {}

/// 🎒️ Encode ZipSnapshot as ZIP container bytes.
pub async fn serialize(from: &ZipSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::zip::standards::v2_0::subsets::any::io::encode_zip(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}

/// Encode ZIP then wrap as binary pack bytes.
pub async fn serialize_bytes(from: &ZipSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
