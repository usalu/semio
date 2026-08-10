//! Deserialize stdio.gif from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::gif::{GifSnapshot, STDIO_GIF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &BinarySnapshot) -> Result<GifSnapshot, store::PackError> {
    let mut snap = crate::artifacts::gif::engine::decode_gif(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_GIF_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GifSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
