//! 🧩 set_snapshot diff leaf.

use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::diff::{diff_set_snapshot, PdfDiff};
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;

/// 🔺️ Diff helper for set-snapshot -- the sparse field-by-field `between(base, snapshot)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &PdfSnapshot, snapshot: &PdfSnapshot) -> protocol::MutationOutcome<PdfDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(PdfDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
