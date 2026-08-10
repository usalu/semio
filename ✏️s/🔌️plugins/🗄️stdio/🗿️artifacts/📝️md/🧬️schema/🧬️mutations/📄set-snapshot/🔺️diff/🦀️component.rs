//! 🧩 set_snapshot diff leaf.

use crate::artifacts::md::schema::diff::{MdDiff, diff_set_snapshot};
use crate::artifacts::md::MdSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &MdSnapshot) -> MdDiff {
    diff_set_snapshot(snapshot)
}
