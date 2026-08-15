//! 🧩 set_snapshot diff leaf.

use crate::artifacts::step::schema::diff::{diff_set_snapshot, StepDiff};
use crate::artifacts::step::StepSnapshot;

/// 🔺️ Diff helper for set-snapshot — the sparse field-by-field `between(base, snapshot)` (no
/// full-replace slot exists on `StepDiff` to short-circuit into).
pub fn diff(base: &StepSnapshot, snapshot: &StepSnapshot) -> StepDiff {
    diff_set_snapshot(base, snapshot)
}
