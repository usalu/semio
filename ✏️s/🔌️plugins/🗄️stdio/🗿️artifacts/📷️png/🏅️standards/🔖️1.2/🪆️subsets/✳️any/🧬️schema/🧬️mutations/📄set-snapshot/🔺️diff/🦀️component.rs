//! 🧩 set_snapshot diff leaf.

use crate::artifacts::png::schema::diff::{diff_set_snapshot, PngDiff};
use crate::artifacts::png::PngSnapshot;

/// 🔺️ Diff helper for set-snapshot: sparse field-by-field `between(base, next)`.
pub fn diff(base: &PngSnapshot, next: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
    if base == next {
        return protocol::MutationOutcome::new(PngDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, next))
}
