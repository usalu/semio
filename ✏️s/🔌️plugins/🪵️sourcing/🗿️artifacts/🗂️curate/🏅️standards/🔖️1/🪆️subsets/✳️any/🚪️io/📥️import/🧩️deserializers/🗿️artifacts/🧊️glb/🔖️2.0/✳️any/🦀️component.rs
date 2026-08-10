//! curate <- glb
use crate::artifacts::curate::schema::snapshot::CurateSnapshot;
use semio_s_plugin_stdio::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &GlbSnapshot) -> Result<CurateSnapshot, store::TextError> {
    let _ = STDIO_GLB_DOCUMENT_SCHEMA;
    let bytes = <GlbSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<CurateSnapshot, store::TextError> {
    <CurateSnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| {
        <CurateSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    })
}
