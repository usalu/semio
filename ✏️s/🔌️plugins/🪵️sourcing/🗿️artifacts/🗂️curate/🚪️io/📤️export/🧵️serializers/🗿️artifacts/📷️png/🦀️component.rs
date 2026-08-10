//! curate -> png
use crate::artifacts::curate::schema::snapshot::CurateSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &CurateSnapshot) -> Result<PngSnapshot, store::TextError> {
    let _ = STDIO_PNG_DOCUMENT_SCHEMA;
    let bytes = <CurateSnapshot as store::DocumentPack>::encode_pack(snapshot);
    <PngSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &CurateSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<PngSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
