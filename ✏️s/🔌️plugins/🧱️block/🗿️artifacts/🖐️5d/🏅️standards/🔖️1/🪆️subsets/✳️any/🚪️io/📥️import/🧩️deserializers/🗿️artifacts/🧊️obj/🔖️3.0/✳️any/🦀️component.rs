//! block5d <- obj
use crate::artifacts::block5d::Block5dSnapshot;
use semio_s_plugin_stdio::artifacts::obj::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &ObjSnapshot) -> Result<Block5dSnapshot, store::TextError> {
    let _ = (STDIO_OBJ_DOCUMENT_SCHEMA, from);
    Ok(Block5dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Block5dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Block5dSnapshot::default())
}
