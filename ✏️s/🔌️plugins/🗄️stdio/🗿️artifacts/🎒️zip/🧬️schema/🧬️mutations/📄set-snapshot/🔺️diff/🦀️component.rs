//! 🧩 set_snapshot diff leaf.

use crate::artifacts::zip::schema::diff::{ZipDiff, diff_set_snapshot};
use crate::artifacts::zip::ZipSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &ZipSnapshot) -> ZipDiff {
    diff_set_snapshot(snapshot)
}
