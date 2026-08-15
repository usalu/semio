//! 🧩 set_snapshot diff leaf.

use crate::artifacts::ply::schema::diff::{diff_set_snapshot, PlyDiff};
use crate::artifacts::ply::PlySnapshot;

/// 🔺️ Diff helper for set-snapshot — the sparse field-by-field `between(base, next)` (no
/// full-replace slot exists on `PlyDiff` to short-circuit into).
pub fn diff(base: &PlySnapshot, snapshot: &PlySnapshot) -> PlyDiff {
    diff_set_snapshot(base, snapshot)
}
