//! 📥️ Deserialize `stdio.las` from stdio.binary.
use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::las::LasSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<LasSnapshot, store::PackError> {
    crate::artifacts::las::engine::decode_las(&from.bytes).map_err(|e| store::PackError::Schema(e))
}
