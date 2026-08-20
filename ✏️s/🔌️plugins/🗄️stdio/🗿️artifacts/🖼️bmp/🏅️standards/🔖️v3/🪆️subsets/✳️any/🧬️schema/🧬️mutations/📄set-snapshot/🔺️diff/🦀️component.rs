//! 🧩 set_snapshot diff leaf.

use crate::artifacts::bmp::schema::diff::{diff_set_snapshot, BmpDiff};
use crate::artifacts::bmp::BmpSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field delta, never a full-replace slot.
pub async fn diff(base: &BmpSnapshot, snapshot: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(BmpDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one").await;
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
