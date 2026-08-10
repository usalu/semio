//! 🧩 set_snapshot diff leaf.

use crate::artifacts::tiff::schema::diff::{TiffDiff, diff_set_snapshot};
use crate::artifacts::tiff::TiffSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &TiffSnapshot) -> TiffDiff {
    diff_set_snapshot(snapshot)
}
