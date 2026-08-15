use crate::artifacts::semio::standards::v1::subsets::flow::schema::diff::{diff_set_snapshot, SemioFlowDiff};
use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioFlowSnapshot, snapshot: &SemioFlowSnapshot) -> SemioFlowDiff {
    diff_set_snapshot(base, snapshot)
}
