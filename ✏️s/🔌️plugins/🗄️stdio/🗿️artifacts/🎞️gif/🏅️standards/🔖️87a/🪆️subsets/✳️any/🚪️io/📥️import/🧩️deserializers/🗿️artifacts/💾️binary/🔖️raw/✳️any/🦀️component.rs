//! Deserialize stdio.gif from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;
use crate::artifacts::gif::STDIO_GIF_DOCUMENT_SCHEMA;

pub async fn register() {}

pub async fn deserialize(from: &BinarySnapshot) -> Result<GifSnapshot, store::PackError> {
    let mut snap = crate::artifacts::gif::standards::v87a::engine::decode_gif(&from.bytes).await.map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_GIF_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<GifSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes).await?).await
}
