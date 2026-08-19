//! remodel -> ply
use crate::artifacts::remodel::schema::snapshot::RemodelSnapshot;
use semio_s_plugin_stdio::artifacts::ply::{PlySnapshot, STDIO_PLY_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &RemodelSnapshot) -> Result<PlySnapshot, store::TextError> {
    let _ = STDIO_PLY_DOCUMENT_SCHEMA;
    let bytes = <RemodelSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <PlySnapshot as store::ArtifactPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub async fn serialize_bytes(snapshot: &RemodelSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<PlySnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
