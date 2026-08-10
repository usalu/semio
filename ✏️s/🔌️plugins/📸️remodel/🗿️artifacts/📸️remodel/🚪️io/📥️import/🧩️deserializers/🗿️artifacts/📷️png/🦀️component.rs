//! remodel <- png
use crate::artifacts::remodel::schema::snapshot::RemodelSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PngSnapshot) -> Result<RemodelSnapshot, store::TextError> {
    let _ = STDIO_PNG_DOCUMENT_SCHEMA;
    let bytes = <PngSnapshot as store::DocumentPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RemodelSnapshot, store::TextError> {
    <RemodelSnapshot as store::DocumentPack>::decode_pack(bytes).or_else(|_| {
        <RemodelSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    })
}
