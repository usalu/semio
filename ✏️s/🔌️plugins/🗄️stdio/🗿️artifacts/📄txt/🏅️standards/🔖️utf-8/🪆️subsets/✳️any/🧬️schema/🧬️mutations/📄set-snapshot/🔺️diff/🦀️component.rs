//! 🧩 set_snapshot diff leaf.

use crate::artifacts::txt::schema::diff::{diff_set_snapshot, TxtDiff};
use crate::artifacts::txt::TxtSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &TxtSnapshot, snapshot: &TxtSnapshot) -> TxtDiff {
    diff_set_snapshot(base, snapshot)
}
