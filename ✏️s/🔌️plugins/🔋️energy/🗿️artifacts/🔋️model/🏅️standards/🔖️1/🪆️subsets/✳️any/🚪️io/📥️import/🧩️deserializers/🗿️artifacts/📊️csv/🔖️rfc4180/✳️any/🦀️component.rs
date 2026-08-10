//! model <- csv
use crate::artifacts::model::EnergyModelSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &CsvSnapshot) -> Result<EnergyModelSnapshot, store::TextError> {
    let _ = (STDIO_CSV_DOCUMENT_SCHEMA, from);
    Ok(EnergyModelSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<EnergyModelSnapshot, store::TextError> {
    let _ = bytes;
    Ok(EnergyModelSnapshot::default())
}
