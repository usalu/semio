//! 🧩 set_snapshot diff leaf.

use crate::artifacts::bmp::schema::diff::{diff_set_snapshot, BmpDiff};
use crate::artifacts::bmp::BmpSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field delta, never a full-replace slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &BmpSnapshot, snapshot: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(BmpDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
