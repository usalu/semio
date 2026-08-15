//! 📥️ Deserialize `stdio.bmp` from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::bmp::{BmpSnapshot, STDIO_BMP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &BinarySnapshot) -> Result<BmpSnapshot, store::PackError> {
    let mut snap = crate::artifacts::bmp::engine::decode_bmp(&from.bytes).map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_BMP_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<BmpSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
