//! 📥️ Deserialize `stdio.las` from stdio.binary.
use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::las::LasSnapshot;
pub async fn register() {}
pub async fn deserialize(from: &BinarySnapshot) -> Result<LasSnapshot, store::PackError> {
    crate::artifacts::las::engine::decode_las(&from.bytes).map_err(|e| store::PackError::Schema(e))
}
