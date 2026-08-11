use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::diff::{WavDiff, diff_set_snapshot};
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &WavSnapshot, snapshot: &WavSnapshot) -> WavDiff {
    diff_set_snapshot(base, snapshot)
}
