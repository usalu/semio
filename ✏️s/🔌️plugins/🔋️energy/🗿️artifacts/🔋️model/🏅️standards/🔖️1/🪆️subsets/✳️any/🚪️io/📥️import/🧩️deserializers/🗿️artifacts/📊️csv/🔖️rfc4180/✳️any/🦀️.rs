//! model <- csv
use crate::artifacts::model::EnergyModelSnapshot;
use semio_s_artifact_stdio_csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &CsvSnapshot) -> Result<EnergyModelSnapshot, store::TextError> {
    let _ = (STDIO_CSV_DOCUMENT_SCHEMA, from);
    Ok(EnergyModelSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<EnergyModelSnapshot, store::TextError> {
    let _ = bytes;
    Ok(EnergyModelSnapshot::default())
}
