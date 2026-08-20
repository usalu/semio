//! 📥️ Deserialize `stdio.bmp` from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::bmp::{BmpSnapshot, STDIO_BMP_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<BmpSnapshot, store::PackError> {
    let mut snap = crate::artifacts::bmp::engine::decode_bmp(&from.bytes).map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_BMP_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<BmpSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
