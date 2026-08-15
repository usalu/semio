//! 🔺️ `no-mutation` validated sparse diff.

use super::mutation::NoMutation;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &NoMutation, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::NoMutation(payload.clone())).unwrap_or_default()
}
