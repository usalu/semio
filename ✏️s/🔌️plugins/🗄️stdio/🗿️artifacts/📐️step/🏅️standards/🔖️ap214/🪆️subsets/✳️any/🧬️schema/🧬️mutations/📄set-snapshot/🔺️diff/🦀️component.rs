//! 🧩 set_snapshot diff leaf.

use crate::artifacts::step::schema::diff::{diff_set_snapshot, StepDiff};
use crate::artifacts::step::StepSnapshot;

/// 🔺️ Diff helper for set-snapshot — the sparse field-by-field `between(base, snapshot)` (no
/// full-replace slot exists on `StepDiff` to short-circuit into).
pub async fn diff(base: &StepSnapshot, snapshot: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(StepDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one").await;
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
