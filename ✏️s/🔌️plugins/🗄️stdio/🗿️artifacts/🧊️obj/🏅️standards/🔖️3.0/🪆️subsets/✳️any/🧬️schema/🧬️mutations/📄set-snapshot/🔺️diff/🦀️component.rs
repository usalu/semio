//! 🧩 set_snapshot diff leaf.

use crate::artifacts::obj::schema::diff::{diff_set_snapshot, ObjDiff};
use crate::artifacts::obj::ObjSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field `between(base, snapshot)`, per the
/// recipe's "no full-replace slot, even for SetSnapshot" rule.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &ObjSnapshot, snapshot: &ObjSnapshot) -> protocol::MutationOutcome<ObjDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(ObjDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
