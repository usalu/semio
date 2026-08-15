//! 🧩 set_snapshot diff leaf.

use crate::artifacts::gif::standards::v87a::subsets::any::schema::diff::{diff_set_snapshot, GifDiff};
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field `between(base, snapshot)`, never a
/// full-replace slot.
pub fn diff(base: &GifSnapshot, snapshot: &GifSnapshot) -> GifDiff {
    diff_set_snapshot(base, snapshot)
}
