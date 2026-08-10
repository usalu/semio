//! home -> zip
use crate::artifacts::home::SHomeSnapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &SHomeSnapshot) -> Result<ZipSnapshot, store::TextError> {
    let _ = STDIO_ZIP_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("home->zip: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &SHomeSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<ZipSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
