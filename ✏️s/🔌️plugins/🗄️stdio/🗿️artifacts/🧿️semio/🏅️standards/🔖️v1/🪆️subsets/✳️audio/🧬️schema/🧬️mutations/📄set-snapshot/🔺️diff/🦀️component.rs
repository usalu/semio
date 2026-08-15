use crate::artifacts::semio::standards::v1::subsets::audio::schema::diff::{diff_set_snapshot, SemioAudioDiff};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioAudioSnapshot, snapshot: &SemioAudioSnapshot) -> SemioAudioDiff {
    diff_set_snapshot(base, snapshot)
}
