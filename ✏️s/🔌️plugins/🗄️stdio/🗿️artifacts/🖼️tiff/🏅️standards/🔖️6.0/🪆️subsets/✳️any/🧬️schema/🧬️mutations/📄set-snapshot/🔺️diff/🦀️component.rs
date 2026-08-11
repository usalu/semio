//! 🧩 set_snapshot diff leaf.

use crate::artifacts::tiff::schema::diff::{TiffDiff, diff_set_snapshot};
use crate::artifacts::tiff::TiffSnapshot;

/// 🔺️ Diff helper for set-snapshot: sparse field-by-field `between(base, next)`.
pub fn diff(base: &TiffSnapshot, next: &TiffSnapshot) -> TiffDiff {
    diff_set_snapshot(base, next)
}
