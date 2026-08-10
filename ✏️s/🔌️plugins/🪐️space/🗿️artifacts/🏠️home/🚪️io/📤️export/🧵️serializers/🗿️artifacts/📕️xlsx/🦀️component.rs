//! home -> xlsx
use crate::artifacts::home::HomeSnapshot;
use semio_s_plugin_stdio::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &HomeSnapshot) -> Result<XlsxSnapshot, store::TextError> {
    let bytes = <HomeSnapshot as store::DocumentPack>::encode_pack(snapshot)
        .or_else(|_| Ok(<HomeSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes()))?;
    semio_s_plugin_stdio::artifacts::xlsx::engine::decode_xlsx(&bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &HomeSnapshot) -> Result<Vec<u8>, store::TextError> {
    semio_s_plugin_stdio::artifacts::xlsx::engine::encode_xlsx(&serialize(snapshot)?)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
