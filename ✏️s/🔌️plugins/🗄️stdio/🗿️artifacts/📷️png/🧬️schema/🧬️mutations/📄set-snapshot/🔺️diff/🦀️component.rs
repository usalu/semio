//! 🧩 set_snapshot diff leaf.

use crate::artifacts::png::schema::diff::{PngDiff, diff_set_snapshot};
use crate::artifacts::png::PngSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &PngSnapshot) -> PngDiff {
    diff_set_snapshot(snapshot)
}
