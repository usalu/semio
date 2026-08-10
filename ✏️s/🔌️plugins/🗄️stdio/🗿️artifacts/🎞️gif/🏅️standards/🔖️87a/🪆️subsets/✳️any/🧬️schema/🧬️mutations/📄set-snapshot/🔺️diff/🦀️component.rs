//! 🧩 set_snapshot diff leaf.

use crate::artifacts::gif::standards::v87a::subsets::any::schema::diff::{GifDiff, diff_set_snapshot};
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &GifSnapshot) -> GifDiff {
    diff_set_snapshot(snapshot)
}
