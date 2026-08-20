//! deser step via binary
use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::step::StepSnapshot;
pub async fn register() {}
pub async fn deserialize(from: &BinarySnapshot) -> Result<StepSnapshot, store::PackError> {
    let text = String::from_utf8(from.bytes.clone()).map_err(|e| store::PackError::Schema(e.to_string()))?;
    <StepSnapshot as store::ArtifactDsl>::parse_dsl(&text).await.map_err(|e| store::PackError::Schema(e.to_string()))
}
pub async fn deserialize_bytes(bytes: &[u8]) -> Result<StepSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes).await?).await
}
