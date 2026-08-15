use crate::artifacts::semio::standards::v1::subsets::model::schema::diff::{diff_set_snapshot, SemioModelDiff};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioModelSnapshot, snapshot: &SemioModelSnapshot) -> SemioModelDiff {
    diff_set_snapshot(base, snapshot)
}
