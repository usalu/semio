//! 🧩 set_snapshot diff leaf.

use crate::artifacts::tiff::schema::diff::{diff_set_snapshot, TiffDiff};
use crate::artifacts::tiff::TiffSnapshot;

/// 🔺️ Diff helper for set-snapshot: sparse field-by-field `between(base, next)`.
pub async fn diff(base: &TiffSnapshot, next: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
    if base == next {
        return protocol::MutationOutcome::new(TiffDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one").await;
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, next))
}
