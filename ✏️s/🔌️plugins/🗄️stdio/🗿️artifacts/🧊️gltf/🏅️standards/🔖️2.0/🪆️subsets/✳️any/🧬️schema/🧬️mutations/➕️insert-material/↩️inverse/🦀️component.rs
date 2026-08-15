//! ↩️ `InsertMaterial` semantic inverse.

use super::mutation::InsertMaterial;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &InsertMaterial, _base: &GltfSnapshot) -> Vec<GltfMutation> {
    vec![GltfMutation::RemoveMaterial(RemoveMaterial { index: payload.index })]
}
