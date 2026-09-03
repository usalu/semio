//! Serialize stdio.txt to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;

//#region Codec
/// Register serializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// UTF-8 encode text into a BinarySnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &TxtSnapshot) -> BinarySnapshot {
    BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: from.to_body().into_bytes() }
}

/// Encode as binary pack bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_bytes(from: &TxtSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from), &store::PackEncodeOptions::default())
}
//#endregion Codec
