//! remodel -> glb
use crate::artifacts::remodel::schema::snapshot::RemodelSnapshot;
use semio_s_plugin_stdio::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &RemodelSnapshot) -> Result<GlbSnapshot, store::TextError> {
    let _ = STDIO_GLB_DOCUMENT_SCHEMA;
    let bytes = <RemodelSnapshot as store::DocumentPack>::encode_pack(snapshot);
    <GlbSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &RemodelSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<GlbSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
