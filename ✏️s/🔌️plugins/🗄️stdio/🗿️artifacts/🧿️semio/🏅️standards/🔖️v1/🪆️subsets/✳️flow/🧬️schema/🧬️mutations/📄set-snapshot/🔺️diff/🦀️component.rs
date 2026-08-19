use crate::artifacts::semio::standards::v1::subsets::flow::schema::diff::{diff_set_snapshot, SemioFlowDiff};
use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub async fn diff(base: &SemioFlowSnapshot, snapshot: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(SemioFlowDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
