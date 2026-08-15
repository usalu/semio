use crate::artifacts::semio::standards::v1::subsets::animation::schema::diff::{diff_set_snapshot, SemioAnimationDiff};
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioAnimationSnapshot, snapshot: &SemioAnimationSnapshot) -> SemioAnimationDiff {
    diff_set_snapshot(base, snapshot)
}
