//! 🧩 set_snapshot diff leaf.

use crate::artifacts::gltf::schema::diff::{GltfDiff, diff_set_snapshot};
use crate::artifacts::gltf::GltfSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &GltfSnapshot) -> GltfDiff {
    diff_set_snapshot(snapshot)
}
