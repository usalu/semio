//! 🧩 set_snapshot diff leaf.

use crate::artifacts::txt::schema::diff::{diff_set_snapshot, TxtDiff};
use crate::artifacts::txt::TxtSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub async fn diff(base: &TxtSnapshot, snapshot: &TxtSnapshot) -> protocol::MutationOutcome<TxtDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(TxtDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one").await;
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
