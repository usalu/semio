//! 🧩 set_snapshot diff leaf.

use crate::artifacts::deflate::schema::diff::{diff_set_snapshot, DeflateDiff};
use crate::artifacts::deflate::DeflateSnapshot;

/// 🔺️ Diff helper for set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &DeflateSnapshot, snapshot: &DeflateSnapshot) -> protocol::MutationOutcome<DeflateDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(DeflateDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
