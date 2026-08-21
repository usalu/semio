use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::diff::{diff_set_snapshot, Mp3Diff};
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;

/// 🔺️ Diff helper for set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &Mp3Snapshot, snapshot: &Mp3Snapshot) -> protocol::MutationOutcome<Mp3Diff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(Mp3Diff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
