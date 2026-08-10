//! 🧩 set_snapshot diff leaf.

use crate::artifacts::dxf::schema::diff::{DxfDiff, diff_set_snapshot};
use crate::artifacts::dxf::DxfSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &DxfSnapshot) -> DxfDiff {
    diff_set_snapshot(snapshot)
}
