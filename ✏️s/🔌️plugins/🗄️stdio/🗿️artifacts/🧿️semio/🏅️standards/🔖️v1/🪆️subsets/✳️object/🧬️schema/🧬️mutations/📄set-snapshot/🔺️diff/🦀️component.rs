use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::{SemioObjectDiff, diff_set_snapshot};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioObjectSnapshot, snapshot: &SemioObjectSnapshot) -> SemioObjectDiff {
    diff_set_snapshot(base, snapshot)
}
