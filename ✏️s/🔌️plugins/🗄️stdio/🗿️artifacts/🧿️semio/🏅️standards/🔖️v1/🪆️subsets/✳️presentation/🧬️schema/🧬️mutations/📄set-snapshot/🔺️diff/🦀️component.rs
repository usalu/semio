use crate::artifacts::semio::standards::v1::subsets::presentation::schema::diff::{diff_set_snapshot, SemioPresentationDiff};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;

/// 🔺️ Diff helper for set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioPresentationSnapshot, snapshot: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(SemioPresentationDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
