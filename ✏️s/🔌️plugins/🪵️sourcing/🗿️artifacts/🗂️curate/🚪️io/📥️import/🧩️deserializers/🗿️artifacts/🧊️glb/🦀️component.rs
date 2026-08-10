//! curate <- glb
use crate::artifacts::curate::CurateSnapshot;
use semio_s_plugin_stdio::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &GlbSnapshot) -> Result<CurateSnapshot, store::TextError> {
    let _ = STDIO_GLB_DOCUMENT_SCHEMA;
    let bytes = semio_s_plugin_stdio::artifacts::glb::engine::encode_glb(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <CurateSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <CurateSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<CurateSnapshot, store::TextError> {
    deserialize(&semio_s_plugin_stdio::artifacts::glb::engine::decode_glb(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}
