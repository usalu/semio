//! 📥️ Deserialize `stdio.stl` from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::stl::StlSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<StlSnapshot, store::PackError> {
    crate::artifacts::stl::engine::decode_stl_auto(&from.bytes).map_err(store::PackError::Schema)
}
