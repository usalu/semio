//! ↩️ `BindPrimitiveMaterial` semantic inverse.

use super::mutation::BindPrimitiveMaterial;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &BindPrimitiveMaterial, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document
        .meshes
        .get(payload.mesh)
        .and_then(|mesh| mesh.primitives.get(payload.primitive))
        .map(|primitive| vec![GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterial { mesh: payload.mesh, primitive: payload.primitive, material: primitive.material })])
        .unwrap_or_default()
}
