//! block3d <- obj
use crate::artifacts::block3d::Block3dSnapshot;
use semio_s_plugin_stdio::artifacts::obj::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &ObjSnapshot) -> Result<Block3dSnapshot, store::TextError> {
    let _ = (STDIO_OBJ_DOCUMENT_SCHEMA, from);
    Ok(Block3dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Block3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Block3dSnapshot::default())
}
