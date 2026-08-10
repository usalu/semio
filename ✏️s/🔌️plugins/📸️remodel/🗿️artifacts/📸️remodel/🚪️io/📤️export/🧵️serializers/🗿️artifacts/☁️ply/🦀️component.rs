//! remodel -> ply
use crate::artifacts::remodel::schema::snapshot::RemodelSnapshot;
use semio_s_plugin_stdio::artifacts::ply::{PlySnapshot, STDIO_PLY_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &RemodelSnapshot) -> Result<PlySnapshot, store::TextError> {
    let _ = STDIO_PLY_DOCUMENT_SCHEMA;
    let bytes = <RemodelSnapshot as store::DocumentPack>::encode_pack(snapshot);
    <PlySnapshot as store::DocumentPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &RemodelSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<PlySnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
