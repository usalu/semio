//! 🧩 set_snapshot diff leaf.

use crate::artifacts::stl::schema::diff::{diff_set_snapshot, StlDiff};
use crate::artifacts::stl::StlSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field `between(base, snapshot)`, matching
/// the recipe's "no full-replace slot" rule (no `StlDiff{snapshot: Option<StlSnapshot>}` blob).
pub async fn diff(base: &StlSnapshot, snapshot: &StlSnapshot) -> protocol::MutationOutcome<StlDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(StlDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
