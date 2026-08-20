use crate::artifacts::semio::standards::v1::subsets::model::schema::diff::{diff_set_snapshot, SemioModelDiff};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;

/// 🔺️ Diff helper for set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioModelSnapshot, snapshot: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(SemioModelDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
