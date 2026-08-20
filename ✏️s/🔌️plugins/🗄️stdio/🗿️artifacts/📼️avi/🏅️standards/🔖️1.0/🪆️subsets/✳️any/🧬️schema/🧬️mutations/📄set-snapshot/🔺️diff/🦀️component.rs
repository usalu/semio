use crate::artifacts::avi::standards::v1_0::subsets::any::schema::diff::{diff_set_snapshot, AviDiff};
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;

/// 🔺️ Diff helper for set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &AviSnapshot, snapshot: &AviSnapshot) -> protocol::MutationOutcome<AviDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(AviDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
