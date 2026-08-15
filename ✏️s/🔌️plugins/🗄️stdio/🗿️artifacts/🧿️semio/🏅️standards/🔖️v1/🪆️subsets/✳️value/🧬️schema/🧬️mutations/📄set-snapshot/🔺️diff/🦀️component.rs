use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::{diff_set_snapshot, SemioValueTreeDiff};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioValueSnapshot, snapshot: &SemioValueSnapshot) -> SemioValueTreeDiff {
    diff_set_snapshot(base, snapshot)
}
