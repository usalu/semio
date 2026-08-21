use crate::artifacts::semio::standards::v1::subsets::cad::schema::diff::{diff_set_snapshot, SemioCadDiff};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;

/// 🔺️ Diff helper for set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioCadSnapshot, snapshot: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(SemioCadDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
