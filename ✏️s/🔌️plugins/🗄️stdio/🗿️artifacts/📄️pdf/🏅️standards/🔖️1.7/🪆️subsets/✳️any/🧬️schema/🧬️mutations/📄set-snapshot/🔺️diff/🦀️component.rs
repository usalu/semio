//! 🧩 set_snapshot diff leaf.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::{diff_set_snapshot, PdfDiff};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;

/// 🔺️ Diff helper for set-snapshot -- the sparse field-by-field `between(base, snapshot)` (no
/// `snapshot: Option<PdfSnapshot>` full-replace slot exists on `PdfDiff` to short-circuit into).
pub fn diff(base: &PdfSnapshot, snapshot: &PdfSnapshot) -> PdfDiff {
    diff_set_snapshot(base, snapshot)
}
