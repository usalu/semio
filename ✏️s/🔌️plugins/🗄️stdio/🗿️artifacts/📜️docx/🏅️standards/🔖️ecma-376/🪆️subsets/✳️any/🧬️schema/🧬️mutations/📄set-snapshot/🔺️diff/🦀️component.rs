//! 🧩 set_snapshot diff leaf.

use crate::artifacts::docx::schema::diff::{DocxDiff, diff_set_snapshot};
use crate::artifacts::docx::DocxSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &DocxSnapshot) -> DocxDiff {
    diff_set_snapshot(snapshot)
}
