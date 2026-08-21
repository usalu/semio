use crate::artifacts::semio::standards::v1::subsets::document::schema::diff::{diff_set_snapshot, SemioDocumentDiff};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;

/// 🔺️ Diff helper for set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioDocumentSnapshot, snapshot: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(SemioDocumentDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
