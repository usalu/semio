//! 🔺️ `transform-node` validated sparse diff.

use super::mutation::TransformNode;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &TransformNode, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::TransformNode(payload.clone())).unwrap_or_default()
}
