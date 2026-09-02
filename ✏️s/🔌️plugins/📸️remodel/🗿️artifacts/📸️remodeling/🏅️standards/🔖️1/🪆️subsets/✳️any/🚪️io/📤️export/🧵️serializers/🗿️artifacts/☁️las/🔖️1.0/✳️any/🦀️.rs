//! remodeling -> las
use crate::artifacts::remodeling::schema::snapshot::RemodelingSnapshot;
use semio_s_plugin_stdio::artifacts::las::{LasSnapshot, STDIO_LAS_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &RemodelingSnapshot) -> Result<LasSnapshot, store::TextError> {
    let _ = STDIO_LAS_DOCUMENT_SCHEMA;
    let bytes = <RemodelingSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <LasSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub async fn serialize_bytes(snapshot: &RemodelingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<LasSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
