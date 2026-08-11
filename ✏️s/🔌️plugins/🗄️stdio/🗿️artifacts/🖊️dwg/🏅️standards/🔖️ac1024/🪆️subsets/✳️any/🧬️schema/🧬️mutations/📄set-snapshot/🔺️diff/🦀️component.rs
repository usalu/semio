//! 🧩 set_snapshot diff leaf.

use crate::artifacts::dwg::schema::diff::{DwgDiff, diff_set_snapshot};
use crate::artifacts::dwg::DwgSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field `between(base, next)`.
pub fn diff(base: &DwgSnapshot, snapshot: &DwgSnapshot) -> DwgDiff {
    diff_set_snapshot(base, snapshot)
}
