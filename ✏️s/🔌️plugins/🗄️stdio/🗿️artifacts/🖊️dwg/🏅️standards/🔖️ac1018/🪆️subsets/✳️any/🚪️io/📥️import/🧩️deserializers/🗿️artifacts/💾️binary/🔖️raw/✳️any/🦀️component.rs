//! 📥️ Deserialize `stdio.dwg` from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::dwg::DwgSnapshot;

pub async fn register() {}

pub async fn deserialize(from: &BinarySnapshot) -> Result<DwgSnapshot, store::PackError> {
    crate::artifacts::dwg::schema::snapshot::decode_dwg(&from.bytes).await.map_err(|e| store::PackError::Schema(e.to_string()))
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<DwgSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes).await?).await
}
