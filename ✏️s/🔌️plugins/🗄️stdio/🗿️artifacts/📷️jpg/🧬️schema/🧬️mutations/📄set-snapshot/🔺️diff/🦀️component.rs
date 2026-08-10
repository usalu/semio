//! 🧩 set_snapshot diff leaf.

use crate::artifacts::jpg::schema::diff::{JpgDiff, diff_set_snapshot};
use crate::artifacts::jpg::JpgSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &JpgSnapshot) -> JpgDiff {
    diff_set_snapshot(snapshot)
}
