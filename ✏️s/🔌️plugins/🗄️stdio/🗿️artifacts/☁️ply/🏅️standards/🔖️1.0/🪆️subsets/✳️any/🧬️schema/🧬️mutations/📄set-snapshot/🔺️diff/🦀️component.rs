//! 🧩 set_snapshot diff leaf.

use crate::artifacts::ply::schema::diff::{PlyDiff, diff_set_snapshot};
use crate::artifacts::ply::PlySnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &PlySnapshot) -> PlyDiff {
    diff_set_snapshot(snapshot)
}
