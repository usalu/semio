//! curate -> zip
use crate::artifacts::curate::CurateSnapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &CurateSnapshot) -> Result<ZipSnapshot, store::TextError> {
    let _ = STDIO_ZIP_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("curate->zip: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &CurateSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<ZipSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
