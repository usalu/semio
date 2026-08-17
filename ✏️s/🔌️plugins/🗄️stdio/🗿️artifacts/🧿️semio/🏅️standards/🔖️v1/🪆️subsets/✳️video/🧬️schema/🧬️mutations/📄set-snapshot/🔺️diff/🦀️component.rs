use crate::artifacts::semio::standards::v1::subsets::video::schema::diff::{diff_set_snapshot, SemioVideoDiff};
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioVideoSnapshot, snapshot: &SemioVideoSnapshot) -> protocol::MutationOutcome<SemioVideoDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(SemioVideoDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
