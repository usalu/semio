//! 🧩 set_snapshot diff leaf.

use crate::artifacts::jpg::schema::diff::{diff_set_snapshot, JpgDiff};
use crate::artifacts::jpg::JpgSnapshot;

/// 🔺️ Diff helper for set-snapshot: sparse field-by-field `between(base, next)`.
pub fn diff(base: &JpgSnapshot, next: &JpgSnapshot) -> JpgDiff {
    diff_set_snapshot(base, next)
}
