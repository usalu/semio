//! 🔺️ `set-snapshot` sparse diff.

use super::mutation::SetSnapshot;
use crate::artifacts::gltf::schema::diff::{diff_set_snapshot, GltfDiff};
use crate::artifacts::gltf::GltfSnapshot;

pub fn diff(payload: &SetSnapshot, base: &GltfSnapshot) -> GltfDiff {
    diff_set_snapshot(base, &payload.snapshot)
}
