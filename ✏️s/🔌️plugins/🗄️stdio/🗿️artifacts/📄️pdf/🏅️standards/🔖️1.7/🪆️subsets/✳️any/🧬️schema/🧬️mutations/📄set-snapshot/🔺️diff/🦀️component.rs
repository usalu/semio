//! 🧩 set_snapshot diff leaf.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::{PdfDiff, diff_set_snapshot};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &PdfSnapshot) -> PdfDiff {
    diff_set_snapshot(snapshot)
}
