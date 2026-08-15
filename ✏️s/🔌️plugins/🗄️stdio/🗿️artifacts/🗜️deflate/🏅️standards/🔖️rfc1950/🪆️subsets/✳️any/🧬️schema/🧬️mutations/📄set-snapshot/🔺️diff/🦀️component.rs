//! 🧩 set_snapshot diff leaf.

use crate::artifacts::deflate::schema::diff::{diff_set_snapshot, DeflateDiff};
use crate::artifacts::deflate::DeflateSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &DeflateSnapshot, snapshot: &DeflateSnapshot) -> DeflateDiff {
    diff_set_snapshot(base, snapshot)
}
