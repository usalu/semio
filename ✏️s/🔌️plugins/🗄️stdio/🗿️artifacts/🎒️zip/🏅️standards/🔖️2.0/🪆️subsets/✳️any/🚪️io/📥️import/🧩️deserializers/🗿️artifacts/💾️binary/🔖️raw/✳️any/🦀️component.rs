//! Deserialize stdio.zip from stdio.binary (parse ZIP bytes).

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
pub async fn register() {}

/// 🎒️ Parse ZIP container bytes into a ZipSnapshot.
pub async fn deserialize(from: &BinarySnapshot) -> Result<ZipSnapshot, store::PackError> {
    let mut snap = crate::artifacts::zip::standards::v2_0::subsets::any::io::decode_zip(&from.bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
    snap.schema = STDIO_ZIP_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode a Binary pack then parse ZIP.
pub async fn deserialize_bytes(bytes: &[u8]) -> Result<ZipSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
//#endregion Codec
