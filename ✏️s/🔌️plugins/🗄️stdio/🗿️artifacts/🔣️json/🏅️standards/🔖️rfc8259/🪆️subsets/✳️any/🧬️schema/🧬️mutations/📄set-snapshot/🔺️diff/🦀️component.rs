//! 🧩 set_snapshot diff leaf.

use crate::artifacts::json::schema::diff::{diff_set_snapshot, JsonDiff};
use crate::artifacts::json::JsonSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse `between(base, next)`, never a full-replace slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &JsonSnapshot, next: &JsonSnapshot) -> protocol::MutationOutcome<JsonDiff> {
    if base == next {
        return protocol::MutationOutcome::new(JsonDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, next))
}
