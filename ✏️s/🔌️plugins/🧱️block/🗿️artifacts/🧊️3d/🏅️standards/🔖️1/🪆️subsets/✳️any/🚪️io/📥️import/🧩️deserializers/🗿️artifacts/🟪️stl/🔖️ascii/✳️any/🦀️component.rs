//! block3d <- stl
use crate::artifacts::block3d::Block3dSnapshot;
use semio_s_plugin_stdio::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &StlSnapshot) -> Result<Block3dSnapshot, store::TextError> {
    let _ = (STDIO_STL_DOCUMENT_SCHEMA, from);
    Ok(Block3dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Block3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Block3dSnapshot::default())
}
