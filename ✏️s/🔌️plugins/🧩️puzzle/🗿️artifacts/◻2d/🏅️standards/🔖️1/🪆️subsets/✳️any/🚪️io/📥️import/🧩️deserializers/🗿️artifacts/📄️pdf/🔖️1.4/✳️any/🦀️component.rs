//! puzzle2d <- pdf
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &PdfSnapshot) -> Result<Puzzle2dSnapshot, store::TextError> {
    let _ = (STDIO_PDF_DOCUMENT_SCHEMA, from);
    Ok(Puzzle2dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle2dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Puzzle2dSnapshot::default())
}
