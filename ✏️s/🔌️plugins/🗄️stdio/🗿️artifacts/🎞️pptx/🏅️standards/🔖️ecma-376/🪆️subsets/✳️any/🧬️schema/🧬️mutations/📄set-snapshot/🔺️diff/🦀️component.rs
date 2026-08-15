//! 🧩 set_snapshot diff leaf.

use crate::artifacts::pptx::schema::diff::{diff_set_snapshot, PptxDiff};
use crate::artifacts::pptx::PptxSnapshot;

/// 🔺️ Diff helper for set-snapshot -- sparse field-by-field `between(base, next)`, matching
/// `PptxDiff::between`'s real shape (no `snapshot: Option<PptxSnapshot>` full-replace slot).
pub fn diff(base: &PptxSnapshot, snapshot: &PptxSnapshot) -> PptxDiff {
    diff_set_snapshot(base, snapshot)
}
