//! 🧩 set_snapshot diff leaf.

use crate::artifacts::json::schema::diff::{JsonDiff, diff_set_snapshot};
use crate::artifacts::json::JsonSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &JsonSnapshot) -> JsonDiff {
    diff_set_snapshot(snapshot)
}
