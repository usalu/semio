//! 🧩 set_snapshot diff leaf.

use crate::artifacts::docx::schema::diff::{DocxDiff, diff_set_snapshot};
use crate::artifacts::docx::DocxSnapshot;

/// 🔺️ Diff helper for set-snapshot: the sparse field-by-field delta from `base` to `snapshot`.
pub fn diff(base: &DocxSnapshot, snapshot: &DocxSnapshot) -> DocxDiff {
    diff_set_snapshot(base, snapshot)
}
