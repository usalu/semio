//! 🧩 set_snapshot diff leaf.

use crate::artifacts::stl::schema::diff::{StlDiff, diff_set_snapshot};
use crate::artifacts::stl::StlSnapshot;

/// 🔺️ Diff helper for set-snapshot — sparse field-by-field `between(base, snapshot)`, matching
/// the recipe's "no full-replace slot" rule (no `StlDiff{snapshot: Option<StlSnapshot>}` blob).
pub fn diff(base: &StlSnapshot, snapshot: &StlSnapshot) -> StlDiff {
    diff_set_snapshot(base, snapshot)
}
