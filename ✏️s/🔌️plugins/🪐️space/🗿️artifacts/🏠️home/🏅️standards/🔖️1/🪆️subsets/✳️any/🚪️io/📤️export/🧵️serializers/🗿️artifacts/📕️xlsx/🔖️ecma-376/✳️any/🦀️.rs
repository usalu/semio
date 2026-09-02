//! home -> xlsx
use crate::artifacts::home::SHomeSnapshot;
use semio_s_plugin_stdio::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &SHomeSnapshot) -> Result<XlsxSnapshot, store::TextError> {
    let _ = STDIO_XLSX_DOCUMENT_SCHEMA;
    dsl::FromValue::from_value(dsl::ToValue::to_value(snapshot)).map_err(|e: dsl::ValueError| store::TextError::new(format!("home->xlsx: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &SHomeSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<XlsxSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
