//! 🧩 set_snapshot diff leaf.

use crate::artifacts::json::schema::diff::{diff_set_snapshot, JsonDiff};
use crate::artifacts::json::JsonSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse `between(base, next)`, never a full-replace slot.
pub fn diff(base: &JsonSnapshot, next: &JsonSnapshot) -> JsonDiff {
    diff_set_snapshot(base, next)
}
