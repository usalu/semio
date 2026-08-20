//! deser txt via binary
use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::txt::TxtSnapshot;
pub async fn register() {}
pub async fn deserialize(from: &BinarySnapshot) -> Result<TxtSnapshot, store::PackError> {
    let body = String::from_utf8(from.bytes.clone()).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(TxtSnapshot::from_body(&body).await)
}
pub async fn deserialize_bytes(bytes: &[u8]) -> Result<TxtSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes).await?).await
}
