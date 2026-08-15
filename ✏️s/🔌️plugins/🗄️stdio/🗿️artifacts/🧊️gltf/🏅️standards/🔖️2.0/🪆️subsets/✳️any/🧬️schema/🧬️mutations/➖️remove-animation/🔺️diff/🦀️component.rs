//! 🔺️ `remove-animation` validated sparse diff.

use super::mutation::RemoveAnimation;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &RemoveAnimation, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::RemoveAnimation(payload.clone())).unwrap_or_default()
}
