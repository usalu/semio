use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::{diff_set_snapshot, SemioImageDiff};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioImageSnapshot, snapshot: &SemioImageSnapshot) -> SemioImageDiff {
    diff_set_snapshot(base, snapshot)
}
