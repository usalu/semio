//! 🔺️ `set-mesh` validated sparse diff.

use super::super::planning::GltfSemanticMutation;
use super::mutation::SetMesh;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &SetMesh, base: &GltfSnapshot) -> GltfDiff {
    payload.plan(base).unwrap_or_default()
}
