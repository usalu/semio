//! home <- zip
use crate::artifacts::home::HomeSnapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &ZipSnapshot) -> Result<HomeSnapshot, store::TextError> {
    let _ = STDIO_ZIP_DOCUMENT_SCHEMA;
    let bytes = semio_s_plugin_stdio::artifacts::zip::engine::encode_zip(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <HomeSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <HomeSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<HomeSnapshot, store::TextError> {
    deserialize(&semio_s_plugin_stdio::artifacts::zip::engine::decode_zip(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}
