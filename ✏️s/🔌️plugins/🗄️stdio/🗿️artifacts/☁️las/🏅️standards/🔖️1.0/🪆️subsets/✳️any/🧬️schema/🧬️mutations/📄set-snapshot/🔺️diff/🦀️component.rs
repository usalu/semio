//! 🧩 set_snapshot diff leaf.

use crate::artifacts::las::schema::diff::{diff_set_snapshot, LasDiff};
use crate::artifacts::las::LasSnapshot;

/// 🔺️ Diff helper for set-snapshot — the sparse field-by-field `between(base, next)` (no
/// full-replace slot exists on `LasDiff` to short-circuit into).
pub fn diff(base: &LasSnapshot, snapshot: &LasSnapshot) -> LasDiff {
    diff_set_snapshot(base, snapshot)
}
