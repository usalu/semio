//! Deserialize stdio.zip from stdio.deflate (inflate then parse ZIP).

use crate::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_deflate::DeflateSnapshot;

//#region Codec
/// Register deserializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 🎒️ `from.payload` is already the decompressed RFC1950 payload (typed `DeflateSnapshot`
/// decoding already inflated it) -- parse it as ZIP directly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &DeflateSnapshot) -> Result<ZipSnapshot, store::PackError> {
    let mut snap = crate::standards::v2_0::subsets::base::io::decode_zip(&from.payload).map_err(|e| store::PackError::Schema(e.to_string()))?;
    snap.schema = STDIO_ZIP_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode deflate pack then parse.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<ZipSnapshot, store::PackError> {
    deserialize(&<DeflateSnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
//#endregion Codec
