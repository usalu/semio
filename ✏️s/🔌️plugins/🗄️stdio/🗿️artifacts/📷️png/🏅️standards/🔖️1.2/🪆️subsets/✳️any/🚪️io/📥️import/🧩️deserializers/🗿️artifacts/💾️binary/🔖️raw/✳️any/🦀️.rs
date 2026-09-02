//! Deserialize stdio.png from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<PngSnapshot, store::PackError> {
    let mut snap = crate::artifacts::png::engine::decode_png(&from.bytes).map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_PNG_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<PngSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
