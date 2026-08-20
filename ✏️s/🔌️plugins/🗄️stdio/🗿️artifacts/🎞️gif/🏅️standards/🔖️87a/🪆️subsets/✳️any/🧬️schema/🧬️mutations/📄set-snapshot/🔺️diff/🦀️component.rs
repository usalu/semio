//! 🧩 set_snapshot diff leaf.

use crate::artifacts::gif::standards::v87a::subsets::any::schema::diff::{diff_set_snapshot, GifDiff};
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field `between(base, snapshot)`, never a
/// full-replace slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &GifSnapshot, snapshot: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(GifDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
