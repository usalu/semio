//! Serialize stdio.ifc.2x3 to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;

//#region Codec
/// Register serializer hooks.
pub async fn register() {}

/// Encode via the real IFC2X3 SPF writer into a BinarySnapshot.
pub async fn serialize(from: &Ifc2x3Snapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(from).await.map_err(store::PackError::Schema)?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}

/// Encode as binary pack bytes.
pub async fn serialize_bytes(from: &Ifc2x3Snapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from).await?, &store::PackEncodeOptions::default()).await
}
//#endregion Codec
