//! block5d <- png
use crate::artifacts::block5d::Block5dSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PngSnapshot) -> Result<Block5dSnapshot, store::TextError> {
    let _ = (STDIO_PNG_DOCUMENT_SCHEMA, from);
    Ok(Block5dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Block5dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Block5dSnapshot::default())
}
