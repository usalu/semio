//! 🧩 set_snapshot diff leaf.

use crate::artifacts::ifc::schema::diff::{IfcDiff, diff_set_snapshot};
use crate::artifacts::ifc::IfcSnapshot;

/// 🔺️ Diff helper for set-snapshot — the sparse field-by-field `between(base, next)` (no
/// full-replace slot exists on `IfcDiff` to short-circuit into).
pub fn diff(base: &IfcSnapshot, snapshot: &IfcSnapshot) -> IfcDiff {
    diff_set_snapshot(base, snapshot)
}
