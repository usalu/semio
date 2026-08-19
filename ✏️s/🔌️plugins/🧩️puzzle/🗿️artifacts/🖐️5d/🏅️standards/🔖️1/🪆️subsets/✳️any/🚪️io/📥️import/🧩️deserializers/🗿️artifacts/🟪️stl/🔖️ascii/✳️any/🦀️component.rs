//! puzzle5d <- stl
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use semio_s_plugin_stdio::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &StlSnapshot) -> Result<Puzzle5dSnapshot, store::TextError> {
    let _ = (STDIO_STL_DOCUMENT_SCHEMA, from);
    Ok(Puzzle5dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle5dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Puzzle5dSnapshot::default())
}
