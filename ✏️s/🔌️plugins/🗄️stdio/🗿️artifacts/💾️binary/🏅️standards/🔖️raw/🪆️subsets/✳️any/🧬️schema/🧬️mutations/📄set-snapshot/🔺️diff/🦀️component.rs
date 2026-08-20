//! 🧩 set_snapshot diff leaf.

use crate::artifacts::binary::schema::diff::{diff_set_snapshot, BinaryDiff};
use crate::artifacts::binary::BinarySnapshot;

/// 🔺️ Diff helper for set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &BinarySnapshot, snapshot: &BinarySnapshot) -> protocol::MutationOutcome<BinaryDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(BinaryDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
