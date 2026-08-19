//! puzzle3d <- gltf
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use semio_s_plugin_stdio::artifacts::gltf::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &GltfSnapshot) -> Result<Puzzle3dSnapshot, store::TextError> {
    let _ = (STDIO_GLTF_DOCUMENT_SCHEMA, from);
    Ok(Puzzle3dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Puzzle3dSnapshot::default())
}
