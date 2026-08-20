//! 🧩 set_snapshot diff leaf.

use crate::artifacts::png::schema::diff::{diff_set_snapshot, PngDiff};
use crate::artifacts::png::PngSnapshot;

/// 🔺️ Diff helper for set-snapshot: sparse field-by-field `between(base, next)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &PngSnapshot, next: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
    if base == next {
        return protocol::MutationOutcome::new(PngDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, next))
}
