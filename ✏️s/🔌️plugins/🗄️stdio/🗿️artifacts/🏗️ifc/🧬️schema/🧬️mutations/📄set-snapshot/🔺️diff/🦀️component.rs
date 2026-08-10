//! 🧩 set_snapshot diff leaf.

use crate::artifacts::ifc::schema::diff::{IfcDiff, diff_set_snapshot};
use crate::artifacts::ifc::IfcSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &IfcSnapshot) -> IfcDiff {
    diff_set_snapshot(snapshot)
}
