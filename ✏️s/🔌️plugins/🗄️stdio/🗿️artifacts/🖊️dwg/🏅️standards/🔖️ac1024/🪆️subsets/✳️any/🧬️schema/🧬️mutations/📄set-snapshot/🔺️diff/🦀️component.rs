//! 🧩 set_snapshot diff leaf.

use crate::artifacts::dwg::schema::diff::{DwgDiff, diff_set_snapshot};
use crate::artifacts::dwg::DwgSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &DwgSnapshot) -> DwgDiff {
    diff_set_snapshot(snapshot)
}
