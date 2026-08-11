use crate::artifacts::semio::standards::v1::subsets::video::schema::diff::{SemioVideoDiff, diff_set_snapshot};
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioVideoSnapshot, snapshot: &SemioVideoSnapshot) -> SemioVideoDiff {
    diff_set_snapshot(base, snapshot)
}
