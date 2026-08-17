use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::diff::{diff_set_snapshot, WavDiff};
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &WavSnapshot, snapshot: &WavSnapshot) -> protocol::MutationOutcome<WavDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(WavDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
