//! 🧩 set_snapshot diff leaf.

use crate::artifacts::svg::schema::diff::{diff_set_snapshot, SvgDiff};
use crate::artifacts::svg::SvgSnapshot;

/// 🔺️ Diff helper for set-snapshot -- the sparse field-by-field `SvgDiff::between(base, next)`,
/// never a whole-`SvgSnapshot` replace slot.
pub async fn diff(base: &SvgSnapshot, next: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
    if base == next {
        return protocol::MutationOutcome::new(SvgDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one").await;
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, next))
}
