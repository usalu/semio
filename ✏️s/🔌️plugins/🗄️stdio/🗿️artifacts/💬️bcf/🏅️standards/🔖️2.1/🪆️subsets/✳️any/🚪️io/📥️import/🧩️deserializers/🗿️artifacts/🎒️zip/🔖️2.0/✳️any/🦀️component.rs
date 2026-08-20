//! Deserialize stdio.bcf from stdio.binary (parse ZIP bytes).

use crate::artifacts::bcf::{BcfSnapshot, STDIO_BCF_DOCUMENT_SCHEMA};
use crate::artifacts::binary::BinarySnapshot;

//#region Codec
/// Register deserializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 🎒️ Parse ZIP container bytes into a BcfSnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<BcfSnapshot, store::PackError> {
    let mut snap = crate::artifacts::bcf::io::decode_bcf(&from.bytes).map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_BCF_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode a Binary pack then parse ZIP.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<BcfSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
//#endregion Codec
