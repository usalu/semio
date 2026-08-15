//! ↩️ `RemoveMaterial` semantic inverse.

use super::mutation::RemoveMaterial;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &RemoveMaterial, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.materials.get(payload.index).map(|material| vec![GltfMutation::InsertMaterial(InsertMaterial { index: payload.index, material: material.clone() })]).unwrap_or_default()
}
