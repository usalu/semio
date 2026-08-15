//! 🔺️ `insert-scene` validated sparse diff.

use super::mutation::InsertScene;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &InsertScene, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::InsertScene(payload.clone())).unwrap_or_default()
}
