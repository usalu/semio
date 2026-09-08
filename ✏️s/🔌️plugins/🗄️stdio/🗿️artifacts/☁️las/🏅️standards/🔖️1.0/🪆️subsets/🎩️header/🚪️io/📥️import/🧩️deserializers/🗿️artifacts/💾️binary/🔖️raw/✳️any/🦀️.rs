//! 📥️ Deserialize `stdio.las` from stdio.binary.
use semio_s_artifact_stdio_binary::BinarySnapshot;
use crate::LasSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<LasSnapshot, store::PackError> {
    crate::engine::decode_las(&from.bytes).map_err(store::PackError::Schema)
}
