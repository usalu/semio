//! 🧩 set_snapshot diff leaf.

use crate::artifacts::bmp::schema::diff::{BmpDiff, diff_set_snapshot};
use crate::artifacts::bmp::BmpSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field delta, never a full-replace slot.
pub fn diff(base: &BmpSnapshot, snapshot: &BmpSnapshot) -> BmpDiff {
    diff_set_snapshot(base, snapshot)
}
