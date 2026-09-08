//! puzzle3d <- gltf
use crate::Puzzle3dSnapshot;
use semio_s_artifact_stdio_gltf::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &GltfSnapshot) -> Result<Puzzle3dSnapshot, store::TextError> {
    let _ = (STDIO_GLTF_DOCUMENT_SCHEMA, from);
    Ok(Puzzle3dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Puzzle3dSnapshot::default())
}
