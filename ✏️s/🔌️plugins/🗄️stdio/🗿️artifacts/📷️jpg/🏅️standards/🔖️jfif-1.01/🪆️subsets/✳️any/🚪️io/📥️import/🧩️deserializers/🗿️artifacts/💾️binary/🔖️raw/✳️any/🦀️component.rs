//! Deserialize stdio.jpg from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::jpg::{JpgSnapshot, STDIO_JPG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &BinarySnapshot) -> Result<JpgSnapshot, store::PackError> {
    let mut snap = crate::artifacts::jpg::engine::decode_jpg(&from.bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
    snap.schema = STDIO_JPG_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<JpgSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
