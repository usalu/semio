//! 📥️ Deserialize `stdio.dwg` from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::dwg::DwgSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<DwgSnapshot, store::PackError> {
    crate::artifacts::dwg::schema::snapshot::decode_dwg(&from.bytes).map_err(|e| store::PackError::Schema(e))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<DwgSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
