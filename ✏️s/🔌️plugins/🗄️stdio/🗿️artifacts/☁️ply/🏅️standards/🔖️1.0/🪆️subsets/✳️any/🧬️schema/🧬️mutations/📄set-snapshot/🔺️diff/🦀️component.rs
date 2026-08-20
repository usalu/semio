//! 🧩 set_snapshot diff leaf.

use crate::artifacts::ply::schema::diff::{diff_set_snapshot, PlyDiff};
use crate::artifacts::ply::PlySnapshot;

/// 🔺️ Diff helper for set-snapshot — the sparse field-by-field `between(base, next)` (no
/// full-replace slot exists on `PlyDiff` to short-circuit into).
pub async fn diff(base: &PlySnapshot, snapshot: &PlySnapshot) -> protocol::MutationOutcome<PlyDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(PlyDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one").await;
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
