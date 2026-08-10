//! 🧩 set_snapshot diff leaf.

use crate::artifacts::pdf::schema::diff::{PdfDiff, diff_set_snapshot};
use crate::artifacts::pdf::PdfSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &PdfSnapshot) -> PdfDiff {
    diff_set_snapshot(snapshot)
}
