//! Deserialize stdio.deflate from stdio.binary (zlib-compress payload).

use crate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_binary::BinarySnapshot;

//#region Codec
/// Register deserializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 🗜️ Zlib-compress binary payload into a DeflateSnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let payload = crate::standards::v_rfc1950::subsets::any::io::zlib_compress(&from.bytes).map_err(store::PackError::Schema)?;
    Ok(DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), payload, ..Default::default() })
}

/// Decode a Binary pack then zlib-compress.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<DeflateSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
//#endregion Codec
