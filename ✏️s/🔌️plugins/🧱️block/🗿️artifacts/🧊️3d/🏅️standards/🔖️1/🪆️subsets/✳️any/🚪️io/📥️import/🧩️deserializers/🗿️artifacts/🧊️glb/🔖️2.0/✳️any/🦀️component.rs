//! block3d <- glb
use crate::artifacts::block3d::Block3dSnapshot;
use semio_s_plugin_stdio::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &GlbSnapshot) -> Result<Block3dSnapshot, store::TextError> {
    let _ = (STDIO_GLB_DOCUMENT_SCHEMA, from);
    Ok(Block3dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Block3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Block3dSnapshot::default())
}
