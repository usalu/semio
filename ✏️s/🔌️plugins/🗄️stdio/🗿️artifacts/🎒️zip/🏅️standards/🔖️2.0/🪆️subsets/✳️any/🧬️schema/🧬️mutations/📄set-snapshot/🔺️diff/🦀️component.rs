//! 🧩 set_snapshot diff leaf.

use crate::artifacts::zip::schema::diff::{ZipDiff, diff_set_snapshot};
use crate::artifacts::zip::ZipSnapshot;

/// 🔺️ Diff helper for set-snapshot — the sparse field-by-field `between(base, next)` (no
/// full-replace slot exists on `ZipDiff` to short-circuit into).
pub fn diff(base: &ZipSnapshot, snapshot: &ZipSnapshot) -> ZipDiff {
    diff_set_snapshot(base, snapshot)
}
