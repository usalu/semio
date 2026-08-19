//! puzzle2d <- dxf
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use semio_s_plugin_stdio::artifacts::dxf::{DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &DxfSnapshot) -> Result<Puzzle2dSnapshot, store::TextError> {
    let _ = (STDIO_DXF_DOCUMENT_SCHEMA, from);
    Ok(Puzzle2dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle2dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Puzzle2dSnapshot::default())
}
