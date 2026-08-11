use crate::artifacts::semio::standards::v1::subsets::model::schema::diff::{SemioModelDiff, diff_set_snapshot};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioModelSnapshot, snapshot: &SemioModelSnapshot) -> SemioModelDiff {
    diff_set_snapshot(base, snapshot)
}
