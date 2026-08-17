use crate::artifacts::semio::standards::v1::subsets::animation::schema::diff::{diff_set_snapshot, SemioAnimationDiff};
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioAnimationSnapshot, snapshot: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(SemioAnimationDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
