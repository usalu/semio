//! 🧩 set_snapshot diff leaf.

use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::diff::{PdfDiff, diff_set_snapshot};
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;

/// 🔺️ Diff helper for set-snapshot -- the sparse field-by-field `between(base, snapshot)`.
pub fn diff(base: &PdfSnapshot, snapshot: &PdfSnapshot) -> PdfDiff {
    diff_set_snapshot(base, snapshot)
}
