//! 🧩 set_snapshot diff leaf.

use crate::artifacts::txt::schema::diff::{TxtDiff, diff_set_snapshot};
use crate::artifacts::txt::TxtSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &TxtSnapshot) -> TxtDiff {
    diff_set_snapshot(snapshot)
}
