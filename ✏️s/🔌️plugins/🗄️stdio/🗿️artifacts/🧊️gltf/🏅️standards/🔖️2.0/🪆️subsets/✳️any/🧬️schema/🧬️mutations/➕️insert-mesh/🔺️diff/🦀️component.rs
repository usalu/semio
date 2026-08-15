//! 🔺️ `insert-mesh` validated sparse diff.

use super::mutation::InsertMesh;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &InsertMesh, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::InsertMesh(payload.clone())).unwrap_or_default()
}
