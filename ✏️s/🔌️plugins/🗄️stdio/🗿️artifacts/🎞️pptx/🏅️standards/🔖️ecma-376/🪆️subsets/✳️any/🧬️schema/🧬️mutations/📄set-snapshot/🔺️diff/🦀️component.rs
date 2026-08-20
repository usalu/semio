//! 🧩 set_snapshot diff leaf.

use crate::artifacts::pptx::schema::diff::{diff_set_snapshot, PptxDiff};
use crate::artifacts::pptx::PptxSnapshot;

/// 🔺️ Diff helper for set-snapshot -- sparse field-by-field `between(base, next)`, matching
/// `PptxDiff::between`'s real shape (no `snapshot: Option<PptxSnapshot>` full-replace slot).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &PptxSnapshot, snapshot: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(PptxDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
