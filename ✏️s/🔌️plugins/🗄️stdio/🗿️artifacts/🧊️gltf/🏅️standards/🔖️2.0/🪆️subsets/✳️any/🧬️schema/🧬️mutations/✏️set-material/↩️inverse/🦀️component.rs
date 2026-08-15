//! ↩️ `SetMaterial` semantic inverse.

use super::mutation::SetMaterial;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &SetMaterial, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.materials.get(payload.index).map(|material| vec![GltfMutation::SetMaterial(SetMaterial { index: payload.index, material: material.clone() })]).unwrap_or_default()
}
