//! 🧩 set_snapshot diff leaf.

use crate::artifacts::binary::schema::diff::{diff_set_snapshot, BinaryDiff};
use crate::artifacts::binary::BinarySnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &BinarySnapshot, snapshot: &BinarySnapshot) -> BinaryDiff {
    diff_set_snapshot(base, snapshot)
}
