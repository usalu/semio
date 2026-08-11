//! 🧩 set_snapshot diff leaf.

use crate::artifacts::gltf::schema::diff::{GltfDiff, diff_set_snapshot};
use crate::artifacts::gltf::GltfSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field `between(base, snapshot)`, never a
/// full-replace slot.
pub fn diff(base: &GltfSnapshot, snapshot: &GltfSnapshot) -> GltfDiff {
    diff_set_snapshot(base, snapshot)
}
