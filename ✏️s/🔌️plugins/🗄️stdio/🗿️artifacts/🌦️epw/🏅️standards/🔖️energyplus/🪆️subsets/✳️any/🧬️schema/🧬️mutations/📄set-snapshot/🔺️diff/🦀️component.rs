use crate::artifacts::epw::standards::energyplus::subsets::any::schema::diff::{EpwDiff, diff_set_snapshot};
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &EpwSnapshot, snapshot: &EpwSnapshot) -> EpwDiff {
    diff_set_snapshot(base, snapshot)
}
