//! remodeling -> gltf
use crate::artifacts::remodeling::schema::snapshot::RemodelingSnapshot;
use semio_s_plugin_stdio::artifacts::gltf::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &RemodelingSnapshot) -> Result<GltfSnapshot, store::TextError> {
    let _ = STDIO_GLTF_DOCUMENT_SCHEMA;
    let bytes = <RemodelingSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <GltfSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub async fn serialize_bytes(snapshot: &RemodelingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<GltfSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
