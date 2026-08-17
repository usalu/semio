use crate::artifacts::semio::standards::v1::subsets::audio::schema::diff::{diff_set_snapshot, SemioAudioDiff};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioAudioSnapshot, snapshot: &SemioAudioSnapshot) -> protocol::MutationOutcome<SemioAudioDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(SemioAudioDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
