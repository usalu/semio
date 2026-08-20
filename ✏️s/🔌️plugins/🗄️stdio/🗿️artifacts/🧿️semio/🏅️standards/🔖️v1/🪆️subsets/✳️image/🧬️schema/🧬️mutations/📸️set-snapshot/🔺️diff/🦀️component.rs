use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::{diff_set_snapshot, SemioImageDiff};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;

/// 🔺️ Diff helper for set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioImageSnapshot, snapshot: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(SemioImageDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
