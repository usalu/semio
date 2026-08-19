use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::diff::{diff_set_snapshot, Mp4Diff};
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;

/// 🔺️ Diff helper for set-snapshot.
pub async fn diff(base: &Mp4Snapshot, snapshot: &Mp4Snapshot) -> protocol::MutationOutcome<Mp4Diff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(Mp4Diff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
