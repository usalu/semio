//! 🧩 set_snapshot diff leaf.

use crate::artifacts::bmp::schema::diff::{BmpDiff, diff_set_snapshot};
use crate::artifacts::bmp::BmpSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &BmpSnapshot) -> BmpDiff {
    diff_set_snapshot(snapshot)
}
