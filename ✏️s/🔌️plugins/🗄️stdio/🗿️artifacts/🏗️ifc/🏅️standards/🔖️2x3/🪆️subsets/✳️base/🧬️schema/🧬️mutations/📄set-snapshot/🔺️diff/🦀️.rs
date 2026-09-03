//! 🧩 set_snapshot diff leaf.

use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::diff::{diff_set_snapshot, Ifc2x3Diff};
use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;

/// 🔺️ Diff helper for set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &Ifc2x3Snapshot, snapshot: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(Ifc2x3Diff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
