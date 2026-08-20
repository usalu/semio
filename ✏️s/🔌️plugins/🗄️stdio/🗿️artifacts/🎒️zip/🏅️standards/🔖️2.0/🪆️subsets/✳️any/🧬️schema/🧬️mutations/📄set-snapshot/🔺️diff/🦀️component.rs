//! 🧩 set_snapshot diff leaf.

use crate::artifacts::zip::schema::diff::{diff_set_snapshot, ZipDiff};
use crate::artifacts::zip::ZipSnapshot;

/// 🔺️ Diff helper for set-snapshot — the sparse field-by-field `between(base, next)` (no
/// full-replace slot exists on `ZipDiff` to short-circuit into).
pub async fn diff(base: &ZipSnapshot, snapshot: &ZipSnapshot) -> protocol::MutationOutcome<ZipDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(ZipDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one").await;
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
