use crate::artifacts::semio::standards::v1::subsets::any::schema::diff::{SemioDiff, diff_set_snapshot};
use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioSnapshot, snapshot: &SemioSnapshot) -> SemioDiff {
    diff_set_snapshot(base, snapshot)
}
