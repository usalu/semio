//! Deserialize stdio.gif from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifSnapshot, STDIO_GIF89A_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &BinarySnapshot) -> Result<GifSnapshot, store::PackError> {
    let mut snap = crate::artifacts::gif::standards::v89a::engine::decode_gif(&from.bytes).map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_GIF89A_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<GifSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
