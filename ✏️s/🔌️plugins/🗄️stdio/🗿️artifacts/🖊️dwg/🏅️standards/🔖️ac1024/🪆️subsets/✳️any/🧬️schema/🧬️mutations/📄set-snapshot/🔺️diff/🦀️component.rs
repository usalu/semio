//! 🧩 set_snapshot diff leaf.

use crate::artifacts::dwg::schema::diff::{diff_set_snapshot, DwgDiff};
use crate::artifacts::dwg::DwgSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field `between(base, next)`.
pub async fn diff(base: &DwgSnapshot, snapshot: &DwgSnapshot) -> protocol::MutationOutcome<DwgDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(DwgDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
