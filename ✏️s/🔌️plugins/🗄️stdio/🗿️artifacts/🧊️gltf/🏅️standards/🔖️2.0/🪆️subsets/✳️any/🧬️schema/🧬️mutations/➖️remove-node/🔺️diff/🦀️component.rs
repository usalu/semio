//! 🔺️ `remove-node` validated sparse diff.

use super::mutation::RemoveNode;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &RemoveNode, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::RemoveNode(payload.clone())).unwrap_or_default()
}
