//! jack <- csv
use crate::artifacts::jack::JackSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &CsvSnapshot) -> Result<JackSnapshot, store::TextError> {
    let _ = (STDIO_CSV_DOCUMENT_SCHEMA, from);
    Ok(JackSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<JackSnapshot, store::TextError> {
    let _ = bytes;
    Ok(JackSnapshot::default())
}
