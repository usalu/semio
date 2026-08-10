//! block2d <- glb
use crate::artifacts::block2d::Block2dSnapshot;
use semio_s_plugin_stdio::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &GlbSnapshot) -> Result<Block2dSnapshot, store::TextError> {
    let _ = (STDIO_GLB_DOCUMENT_SCHEMA, from);
    Ok(Block2dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Block2dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Block2dSnapshot::default())
}
