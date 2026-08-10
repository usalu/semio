//! 🧩 set_snapshot diff leaf.

use crate::artifacts::step::schema::diff::{StepDiff, diff_set_snapshot};
use crate::artifacts::step::StepSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &StepSnapshot) -> StepDiff {
    diff_set_snapshot(snapshot)
}
