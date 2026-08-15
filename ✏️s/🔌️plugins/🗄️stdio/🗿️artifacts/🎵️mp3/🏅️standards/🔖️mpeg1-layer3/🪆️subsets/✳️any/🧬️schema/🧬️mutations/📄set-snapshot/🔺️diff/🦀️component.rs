use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::diff::{diff_set_snapshot, Mp3Diff};
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &Mp3Snapshot, snapshot: &Mp3Snapshot) -> Mp3Diff {
    diff_set_snapshot(base, snapshot)
}
