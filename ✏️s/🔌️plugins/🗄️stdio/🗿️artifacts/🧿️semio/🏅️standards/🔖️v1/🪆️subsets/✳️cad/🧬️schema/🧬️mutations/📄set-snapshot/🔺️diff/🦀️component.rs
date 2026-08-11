use crate::artifacts::semio::standards::v1::subsets::cad::schema::diff::{SemioCadDiff, diff_set_snapshot};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioCadSnapshot, snapshot: &SemioCadSnapshot) -> SemioCadDiff {
    diff_set_snapshot(base, snapshot)
}
