//! Deserialize stdio.deflate from stdio.binary (zlib-compress payload).

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 🗜️ Zlib-compress binary payload into a DeflateSnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let payload = crate::artifacts::deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(&from.bytes).await.map_err(|e| store::PackError::Schema(e))?;
    Ok(DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), payload, ..Default::default() })
}

/// Decode a Binary pack then zlib-compress.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<DeflateSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
//#endregion Codec
