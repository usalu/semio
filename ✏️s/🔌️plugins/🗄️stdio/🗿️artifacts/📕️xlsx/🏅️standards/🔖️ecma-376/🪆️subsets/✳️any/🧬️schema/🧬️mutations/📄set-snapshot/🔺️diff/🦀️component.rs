//! 🧩 set_snapshot diff leaf.

use crate::artifacts::xlsx::schema::diff::{diff_set_snapshot, XlsxDiff};
use crate::artifacts::xlsx::XlsxSnapshot;

/// 🔺️ Diff helper for set-snapshot: the sparse field-by-field delta from `base` to `snapshot`.
pub fn diff(base: &XlsxSnapshot, snapshot: &XlsxSnapshot) -> XlsxDiff {
    diff_set_snapshot(base, snapshot)
}
