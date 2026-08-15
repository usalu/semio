//! 🔺️ `set-material` validated sparse diff.

use super::mutation::SetMaterial;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &SetMaterial, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::SetMaterial(payload.clone())).unwrap_or_default()
}
