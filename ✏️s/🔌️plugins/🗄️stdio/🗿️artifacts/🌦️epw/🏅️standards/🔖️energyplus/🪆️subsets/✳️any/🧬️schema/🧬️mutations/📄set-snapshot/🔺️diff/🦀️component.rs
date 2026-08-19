use crate::artifacts::epw::standards::energyplus::subsets::any::schema::diff::{diff_set_snapshot, EpwDiff};
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub async fn diff(base: &EpwSnapshot, snapshot: &EpwSnapshot) -> protocol::MutationOutcome<EpwDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(EpwDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
