//! 🧩 set_snapshot diff leaf.

use crate::artifacts::docx::schema::diff::{diff_set_snapshot, DocxDiff};
use crate::artifacts::docx::DocxSnapshot;

/// 🔺️ Diff helper for set-snapshot: the sparse field-by-field delta from `base` to `snapshot`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &DocxSnapshot, snapshot: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(DocxDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
