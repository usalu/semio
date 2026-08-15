//! ↩️ `InsertMesh` semantic inverse.

use super::mutation::InsertMesh;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &InsertMesh, _base: &GltfSnapshot) -> Vec<GltfMutation> {
    vec![GltfMutation::RemoveMesh(RemoveMesh { index: payload.index })]
}
