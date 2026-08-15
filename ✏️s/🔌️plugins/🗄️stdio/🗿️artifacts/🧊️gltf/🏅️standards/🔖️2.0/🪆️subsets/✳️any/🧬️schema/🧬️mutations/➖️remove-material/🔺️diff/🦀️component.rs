//! 🔺️ `remove-material` validated sparse diff.

use super::mutation::RemoveMaterial;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &RemoveMaterial, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::RemoveMaterial(payload.clone())).unwrap_or_default()
}
