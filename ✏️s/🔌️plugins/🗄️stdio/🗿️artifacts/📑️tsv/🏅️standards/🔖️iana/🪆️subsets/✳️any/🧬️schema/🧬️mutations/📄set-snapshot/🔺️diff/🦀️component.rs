use crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::{diff_set_snapshot, TsvDiff};
use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub async fn diff(base: &TsvSnapshot, snapshot: &TsvSnapshot) -> protocol::MutationOutcome<TsvDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(TsvDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
