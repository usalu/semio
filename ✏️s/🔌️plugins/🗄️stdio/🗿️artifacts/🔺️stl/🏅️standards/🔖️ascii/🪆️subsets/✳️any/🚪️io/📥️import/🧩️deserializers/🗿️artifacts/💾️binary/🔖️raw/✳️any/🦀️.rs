//! 📥️ Deserialize `stdio.stl` from stdio.binary.

use semio_s_artifact_stdio_binary::BinarySnapshot;
use crate::StlSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<StlSnapshot, store::PackError> {
    crate::engine::decode_stl_auto(&from.bytes).map_err(store::PackError::Schema)
}
