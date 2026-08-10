//! 🧩 set_snapshot diff leaf.

use crate::artifacts::gif::schema::diff::{GifDiff, diff_set_snapshot};
use crate::artifacts::gif::GifSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &GifSnapshot) -> GifDiff {
    diff_set_snapshot(snapshot)
}
