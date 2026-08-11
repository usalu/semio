use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{SemioBrepDiff, diff_set_snapshot};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioBrepSnapshot, snapshot: &SemioBrepSnapshot) -> SemioBrepDiff {
    diff_set_snapshot(base, snapshot)
}
