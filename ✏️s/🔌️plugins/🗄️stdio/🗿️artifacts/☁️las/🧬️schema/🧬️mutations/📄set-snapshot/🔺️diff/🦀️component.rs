//! 🧩 set_snapshot diff leaf.

use crate::artifacts::las::schema::diff::{LasDiff, diff_set_snapshot};
use crate::artifacts::las::LasSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &LasSnapshot) -> LasDiff {
    diff_set_snapshot(snapshot)
}
