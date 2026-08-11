//! 🧩 set_snapshot diff leaf.

use crate::artifacts::png::schema::diff::{PngDiff, diff_set_snapshot};
use crate::artifacts::png::PngSnapshot;

/// 🔺️ Diff helper for set-snapshot: sparse field-by-field `between(base, next)`.
pub fn diff(base: &PngSnapshot, next: &PngSnapshot) -> PngDiff {
    diff_set_snapshot(base, next)
}
