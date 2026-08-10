//! block5d <- stl
use crate::artifacts::block5d::Block5dSnapshot;
use semio_s_plugin_stdio::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &StlSnapshot) -> Result<Block5dSnapshot, store::TextError> {
    let _ = (STDIO_STL_DOCUMENT_SCHEMA, from);
    Ok(Block5dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Block5dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Block5dSnapshot::default())
}
