//! 🔺️ `insert-animation` validated sparse diff.

use super::mutation::InsertAnimation;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &InsertAnimation, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::InsertAnimation(payload.clone())).unwrap_or_default()
}
