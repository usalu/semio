use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{SemioDrawingDiff, diff_set_snapshot};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioDrawingSnapshot, snapshot: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    diff_set_snapshot(base, snapshot)
}
