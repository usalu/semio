use crate::artifacts::avi::standards::v1_0::subsets::any::schema::diff::{AviDiff, diff_set_snapshot};
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &AviSnapshot, snapshot: &AviSnapshot) -> AviDiff {
    diff_set_snapshot(base, snapshot)
}
