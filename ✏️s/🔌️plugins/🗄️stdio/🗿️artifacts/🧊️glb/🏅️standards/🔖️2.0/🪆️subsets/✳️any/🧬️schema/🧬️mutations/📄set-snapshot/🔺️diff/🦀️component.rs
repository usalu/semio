//! 🧩 set_snapshot diff leaf.

use crate::artifacts::glb::schema::diff::{GlbDiff, diff_set_snapshot};
use crate::artifacts::glb::GlbSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &GlbSnapshot) -> GlbDiff {
    diff_set_snapshot(snapshot)
}
