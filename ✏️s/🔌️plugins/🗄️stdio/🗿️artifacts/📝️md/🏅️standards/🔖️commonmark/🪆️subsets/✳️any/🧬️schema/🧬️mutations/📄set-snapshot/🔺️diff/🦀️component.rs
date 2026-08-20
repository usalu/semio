//! 🧩 set_snapshot diff leaf.

use crate::artifacts::md::schema::diff::{diff_set_snapshot, MdDiff};
use crate::artifacts::md::MdSnapshot;

/// 🔺️ Diff helper for set-snapshot: the sparse field-by-field delta from `base` to `snapshot`
/// (never a full-replace slot -- see `diff_set_snapshot`'s own doc comment).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &MdSnapshot, snapshot: &MdSnapshot) -> protocol::MutationOutcome<MdDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(MdDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
