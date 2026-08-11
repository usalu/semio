use crate::artifacts::semio::standards::v1::subsets::presentation::schema::diff::{SemioPresentationDiff, diff_set_snapshot};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioPresentationSnapshot, snapshot: &SemioPresentationSnapshot) -> SemioPresentationDiff {
    diff_set_snapshot(base, snapshot)
}
