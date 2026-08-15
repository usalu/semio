//! 🔺️ `set-scene` validated sparse diff.

use super::mutation::SetScene;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &SetScene, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::SetScene(payload.clone())).unwrap_or_default()
}
