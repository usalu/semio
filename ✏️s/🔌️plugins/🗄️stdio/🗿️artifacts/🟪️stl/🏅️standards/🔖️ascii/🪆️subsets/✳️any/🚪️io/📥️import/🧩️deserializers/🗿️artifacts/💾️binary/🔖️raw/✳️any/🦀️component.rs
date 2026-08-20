//! 📥️ Deserialize `stdio.stl` from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::stl::StlSnapshot;

pub async fn register() {}

pub async fn deserialize(from: &BinarySnapshot) -> Result<StlSnapshot, store::PackError> {
    crate::artifacts::stl::engine::decode_stl_auto(&from.bytes).await.map_err(store::PackError::Schema)
}
