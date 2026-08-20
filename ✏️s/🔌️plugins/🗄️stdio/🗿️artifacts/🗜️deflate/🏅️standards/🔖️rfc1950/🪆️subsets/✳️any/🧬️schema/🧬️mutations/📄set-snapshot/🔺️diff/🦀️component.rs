//! 🧩 set_snapshot diff leaf.

use crate::artifacts::deflate::schema::diff::{diff_set_snapshot, DeflateDiff};
use crate::artifacts::deflate::DeflateSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub async fn diff(base: &DeflateSnapshot, snapshot: &DeflateSnapshot) -> protocol::MutationOutcome<DeflateDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(DeflateDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one").await;
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
