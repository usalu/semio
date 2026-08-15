//! 🔺️ `set-asset` validated sparse diff.

use super::mutation::SetAsset;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &SetAsset, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::SetAsset(payload.clone())).unwrap_or_default()
}
