use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::diff::{Mp4Diff, diff_set_snapshot};
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &Mp4Snapshot, snapshot: &Mp4Snapshot) -> Mp4Diff {
    diff_set_snapshot(base, snapshot)
}
