//! home <- zip
use crate::artifacts::home::SHomeSnapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &ZipSnapshot) -> Result<SHomeSnapshot, store::TextError> {
    let _ = STDIO_ZIP_DOCUMENT_SCHEMA;
    dsl::FromValue::from_value(dsl::ToValue::to_value(from)).map_err(|e: dsl::ValueError| store::TextError::new(format!("home<-zip: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<SHomeSnapshot, store::TextError> {
    let wire = <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&wire)
}
