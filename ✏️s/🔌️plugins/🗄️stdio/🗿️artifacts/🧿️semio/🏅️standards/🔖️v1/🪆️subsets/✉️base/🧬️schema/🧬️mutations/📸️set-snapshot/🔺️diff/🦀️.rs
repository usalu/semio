use crate::artifacts::semio::standards::v1::subsets::base::schema::diff::{diff_set_snapshot, SemioDiff};
use crate::artifacts::semio::standards::v1::subsets::base::schema::snapshot::SemioSnapshot;

/// 🔺️ Diff helper for set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioSnapshot, snapshot: &SemioSnapshot) -> protocol::MutationOutcome<SemioDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(SemioDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
