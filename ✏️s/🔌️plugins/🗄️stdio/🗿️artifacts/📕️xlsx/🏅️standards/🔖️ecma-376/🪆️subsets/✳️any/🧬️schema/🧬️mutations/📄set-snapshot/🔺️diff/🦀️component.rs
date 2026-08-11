//! 🧩 set_snapshot diff leaf.

use crate::artifacts::xlsx::schema::diff::{XlsxDiff, diff_set_snapshot};
use crate::artifacts::xlsx::XlsxSnapshot;

/// 🔺️ Diff helper for set-snapshot: the sparse field-by-field delta from `base` to `snapshot`.
pub fn diff(base: &XlsxSnapshot, snapshot: &XlsxSnapshot) -> XlsxDiff {
    diff_set_snapshot(base, snapshot)
}
