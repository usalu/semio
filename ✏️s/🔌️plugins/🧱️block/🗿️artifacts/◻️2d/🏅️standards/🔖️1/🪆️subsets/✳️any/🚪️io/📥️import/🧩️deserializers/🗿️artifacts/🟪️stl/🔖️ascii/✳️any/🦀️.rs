//! block2d <- stl
use crate::artifacts::block2d::Block2dSnapshot;
use semio_s_plugin_stdio::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &StlSnapshot) -> Result<Block2dSnapshot, store::TextError> {
    let _ = (STDIO_STL_DOCUMENT_SCHEMA, from);
    Ok(Block2dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Block2dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Block2dSnapshot::default())
}
