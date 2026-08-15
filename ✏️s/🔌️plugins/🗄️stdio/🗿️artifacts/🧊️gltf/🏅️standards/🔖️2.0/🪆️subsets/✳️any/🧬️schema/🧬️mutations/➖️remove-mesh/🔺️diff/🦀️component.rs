//! 🔺️ `remove-mesh` validated sparse diff.

use super::mutation::RemoveMesh;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &RemoveMesh, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::RemoveMesh(payload.clone())).unwrap_or_default()
}
