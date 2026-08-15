//! 🔺️ `remove-buffer` validated sparse diff.

use super::mutation::RemoveBuffer;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &RemoveBuffer, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::RemoveBuffer(payload.clone())).unwrap_or_default()
}
