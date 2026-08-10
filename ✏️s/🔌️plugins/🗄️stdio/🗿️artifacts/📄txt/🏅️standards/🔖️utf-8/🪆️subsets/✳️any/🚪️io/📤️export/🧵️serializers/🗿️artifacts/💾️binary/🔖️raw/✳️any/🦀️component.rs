//! Serialize stdio.txt to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// UTF-8 encode text into a BinarySnapshot.
pub fn serialize(from: &TxtSnapshot) -> BinarySnapshot {
    BinarySnapshot {
        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        bytes: from.text.as_bytes().to_vec(),
    }
}

/// Encode as binary pack bytes.
pub fn serialize_bytes(from: &TxtSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from), &store::PackEncodeOptions::default())
}
//#endregion Codec
