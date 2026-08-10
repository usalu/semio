//! 🧩 set_snapshot diff leaf.

use crate::artifacts::stl::schema::diff::{StlDiff, diff_set_snapshot};
use crate::artifacts::stl::StlSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &StlSnapshot) -> StlDiff {
    diff_set_snapshot(snapshot)
}
