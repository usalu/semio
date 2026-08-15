//! 🔺️ `set-accessor` validated sparse diff.

use super::mutation::SetAccessor;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &SetAccessor, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::SetAccessor(payload.clone())).unwrap_or_default()
}
