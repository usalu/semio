//! remodel -> png
use crate::artifacts::remodel::schema::snapshot::RemodelSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &RemodelSnapshot) -> Result<PngSnapshot, store::TextError> {
    let _ = STDIO_PNG_DOCUMENT_SCHEMA;
    let bytes = <RemodelSnapshot as store::DocumentPack>::encode_pack(snapshot);
    <PngSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &RemodelSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<PngSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
