use crate::artifacts::semio::standards::v1::subsets::workflow::schema::diff::{SemioWorkflowDiff, diff_set_snapshot};
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::SemioWorkflowSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioWorkflowSnapshot, snapshot: &SemioWorkflowSnapshot) -> SemioWorkflowDiff {
    diff_set_snapshot(base, snapshot)
}
