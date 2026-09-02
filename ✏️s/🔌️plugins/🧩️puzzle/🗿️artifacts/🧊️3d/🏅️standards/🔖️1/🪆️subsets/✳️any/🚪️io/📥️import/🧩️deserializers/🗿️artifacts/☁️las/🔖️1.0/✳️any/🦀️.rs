//! puzzle3d <- las
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use semio_s_plugin_stdio::artifacts::las::{LasSnapshot, STDIO_LAS_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &LasSnapshot) -> Result<Puzzle3dSnapshot, store::TextError> {
    let _ = (STDIO_LAS_DOCUMENT_SCHEMA, from);
    Ok(Puzzle3dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Puzzle3dSnapshot::default())
}
