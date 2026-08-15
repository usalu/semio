//! ↩️ `RemoveMesh` semantic inverse.

use super::mutation::RemoveMesh;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &RemoveMesh, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.meshes.get(payload.index).map(|mesh| vec![GltfMutation::InsertMesh(InsertMesh { index: payload.index, mesh: mesh.clone() })]).unwrap_or_default()
}
