//! 🧩 set_snapshot diff leaf.

use crate::artifacts::xlsx::schema::diff::{diff_set_snapshot, XlsxDiff};
use crate::artifacts::xlsx::XlsxSnapshot;

/// 🔺️ Diff helper for set-snapshot: the sparse field-by-field delta from `base` to `snapshot`.
pub async fn diff(base: &XlsxSnapshot, snapshot: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(XlsxDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
