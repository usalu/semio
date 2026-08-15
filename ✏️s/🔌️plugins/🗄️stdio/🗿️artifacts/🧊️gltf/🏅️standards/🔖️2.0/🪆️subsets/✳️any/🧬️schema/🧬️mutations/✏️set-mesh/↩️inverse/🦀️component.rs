//! ↩️ `SetMesh` semantic inverse.

use super::mutation::SetMesh;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &SetMesh, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.meshes.get(payload.index).map(|mesh| vec![GltfMutation::SetMesh(SetMesh { index: payload.index, mesh: mesh.clone() })]).unwrap_or_default()
}
