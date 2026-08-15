//! 🔺️ `set-buffer` validated sparse diff.

use super::mutation::SetBuffer;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &SetBuffer, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::SetBuffer(payload.clone())).unwrap_or_default()
}
