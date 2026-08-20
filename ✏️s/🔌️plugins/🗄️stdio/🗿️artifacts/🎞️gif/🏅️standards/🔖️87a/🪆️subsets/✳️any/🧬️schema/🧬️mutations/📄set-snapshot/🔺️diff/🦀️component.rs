//! 🧩 set_snapshot diff leaf.

use crate::artifacts::gif::standards::v87a::subsets::any::schema::diff::{diff_set_snapshot, GifDiff};
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field `between(base, snapshot)`, never a
/// full-replace slot.
pub async fn diff(base: &GifSnapshot, snapshot: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(GifDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one").await;
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
