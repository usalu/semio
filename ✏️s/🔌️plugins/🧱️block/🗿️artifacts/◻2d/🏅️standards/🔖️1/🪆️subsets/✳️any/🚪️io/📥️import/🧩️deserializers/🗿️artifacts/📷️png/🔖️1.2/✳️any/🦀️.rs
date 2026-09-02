//! block2d <- png
use crate::artifacts::block2d::Block2dSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &PngSnapshot) -> Result<Block2dSnapshot, store::TextError> {
    let _ = (STDIO_PNG_DOCUMENT_SCHEMA, from);
    Ok(Block2dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Block2dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Block2dSnapshot::default())
}
