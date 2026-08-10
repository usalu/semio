//! Serialize stdio.binary to stdio.binary.

use crate::artifacts::binary::BinarySnapshot;

//#region Codec
/// Register serializer hooks (identity for terminal binary).
pub fn register() {}

/// Encode a BinarySnapshot to pack bytes.
pub fn serialize(snapshot: &BinarySnapshot) -> Result<Vec<u8>, store::PackError> {
    store::DocumentPack::encode_pack_with(snapshot, &store::PackEncodeOptions::default())
}
//#endregion Codec
