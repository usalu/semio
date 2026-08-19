//! 🧩 set_snapshot diff leaf.

use crate::artifacts::ifc::schema::diff::{diff_set_snapshot, IfcDiff};
use crate::artifacts::ifc::IfcSnapshot;

/// 🔺️ Diff helper for set-snapshot — the sparse field-by-field `between(base, next)` (no
/// full-replace slot exists on `IfcDiff` to short-circuit into).
pub async fn diff(base: &IfcSnapshot, snapshot: &IfcSnapshot) -> protocol::MutationOutcome<IfcDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(IfcDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
