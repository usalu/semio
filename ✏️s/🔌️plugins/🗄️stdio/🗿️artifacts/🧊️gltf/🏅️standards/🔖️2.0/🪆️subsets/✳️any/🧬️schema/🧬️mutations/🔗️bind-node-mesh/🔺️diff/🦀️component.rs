//! 🔺️ `bind-node-mesh` validated sparse diff.

use super::mutation::BindNodeMesh;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::{plan_gltf_mutation, GltfMutation};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &BindNodeMesh, base: &GltfSnapshot) -> GltfDiff {
    plan_gltf_mutation(base, &GltfMutation::BindNodeMesh(payload.clone())).unwrap_or_default()
}
