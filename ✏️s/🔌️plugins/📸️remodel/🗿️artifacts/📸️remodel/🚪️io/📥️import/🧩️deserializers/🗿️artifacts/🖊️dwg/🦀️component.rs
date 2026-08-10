//! remodel <- dwg
use crate::artifacts::remodel::WatertightReportSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::{DwgSnapshot, STDIO_DWG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &DwgSnapshot) -> Result<WatertightReportSnapshot, store::TextError> {
    let _ = STDIO_DWG_DOCUMENT_SCHEMA;
    let bytes = semio_s_plugin_stdio::artifacts::dwg::engine::encode_dwg(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <WatertightReportSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <WatertightReportSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<WatertightReportSnapshot, store::TextError> {
    deserialize(&semio_s_plugin_stdio::artifacts::dwg::engine::decode_dwg(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}
