//! 🧩 set_snapshot diff leaf.

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::{Ifc2x3Diff, diff_set_snapshot};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &Ifc2x3Snapshot, snapshot: &Ifc2x3Snapshot) -> Ifc2x3Diff {
    diff_set_snapshot(base, snapshot)
}
