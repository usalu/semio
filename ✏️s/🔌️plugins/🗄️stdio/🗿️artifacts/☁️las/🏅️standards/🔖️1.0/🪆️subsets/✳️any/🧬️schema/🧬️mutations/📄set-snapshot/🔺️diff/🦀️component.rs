//! 🧩 set_snapshot diff leaf.

use crate::artifacts::las::schema::diff::{diff_set_snapshot, LasDiff};
use crate::artifacts::las::LasSnapshot;

/// 🔺️ Diff helper for set-snapshot — the sparse field-by-field `between(base, next)` (no
/// full-replace slot exists on `LasDiff` to short-circuit into).
pub async fn diff(base: &LasSnapshot, snapshot: &LasSnapshot) -> protocol::MutationOutcome<LasDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(LasDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
