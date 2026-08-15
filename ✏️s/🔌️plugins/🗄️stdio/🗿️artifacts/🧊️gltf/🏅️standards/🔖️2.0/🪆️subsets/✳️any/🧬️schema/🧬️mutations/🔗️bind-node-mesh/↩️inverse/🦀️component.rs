//! ↩️ `BindNodeMesh` semantic inverse.

use super::mutation::BindNodeMesh;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &BindNodeMesh, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.nodes.get(payload.index).map(|node| vec![GltfMutation::BindNodeMesh(BindNodeMesh { index: payload.index, mesh: node.mesh })]).unwrap_or_default()
}
