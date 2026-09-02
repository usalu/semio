//! model <- xlsx
use crate::artifacts::model::EnergyModelSnapshot;
use semio_s_plugin_stdio::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &XlsxSnapshot) -> Result<EnergyModelSnapshot, store::TextError> {
    let _ = (STDIO_XLSX_DOCUMENT_SCHEMA, from);
    Ok(EnergyModelSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<EnergyModelSnapshot, store::TextError> {
    let _ = bytes;
    Ok(EnergyModelSnapshot::default())
}
