//! puzzle5d <- zip
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &ZipSnapshot) -> Result<Puzzle5dSnapshot, store::TextError> {
    let _ = (STDIO_ZIP_DOCUMENT_SCHEMA, from);
    Ok(Puzzle5dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle5dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Puzzle5dSnapshot::default())
}
