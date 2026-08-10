//! Deserialize stdio.glb from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &BinarySnapshot) -> Result<GlbSnapshot, store::PackError> {
    let mut snap = crate::artifacts::glb::engine::decode_glb(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_GLB_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GlbSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?)
}
