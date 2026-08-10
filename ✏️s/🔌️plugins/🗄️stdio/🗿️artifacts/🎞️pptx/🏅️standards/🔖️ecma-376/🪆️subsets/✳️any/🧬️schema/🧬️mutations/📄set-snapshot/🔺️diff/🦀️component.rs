//! 🧩 set_snapshot diff leaf.

use crate::artifacts::pptx::schema::diff::{PptxDiff, diff_set_snapshot};
use crate::artifacts::pptx::PptxSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &PptxSnapshot) -> PptxDiff {
    diff_set_snapshot(snapshot)
}
