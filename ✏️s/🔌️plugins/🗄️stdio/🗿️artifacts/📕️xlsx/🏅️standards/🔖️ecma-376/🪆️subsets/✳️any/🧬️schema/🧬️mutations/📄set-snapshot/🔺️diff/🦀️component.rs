//! 🧩 set_snapshot diff leaf.

use crate::artifacts::xlsx::schema::diff::{diff_set_snapshot, XlsxDiff};
use crate::artifacts::xlsx::XlsxSnapshot;

/// 🔺️ Diff helper for set-snapshot: the sparse field-by-field delta from `base` to `snapshot`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &XlsxSnapshot, snapshot: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(XlsxDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
