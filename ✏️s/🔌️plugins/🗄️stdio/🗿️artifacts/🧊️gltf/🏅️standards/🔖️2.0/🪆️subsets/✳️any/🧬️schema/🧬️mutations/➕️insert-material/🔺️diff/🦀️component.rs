//! 🔺️ `insert-material` validated sparse diff.

use super::mutation::InsertMaterial;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &InsertMaterial, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::InsertMaterial(payload.clone())).unwrap_or_default()
}
