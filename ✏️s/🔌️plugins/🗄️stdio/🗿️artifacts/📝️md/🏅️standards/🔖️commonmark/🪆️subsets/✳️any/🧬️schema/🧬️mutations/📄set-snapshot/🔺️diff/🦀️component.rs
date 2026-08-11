//! 🧩 set_snapshot diff leaf.

use crate::artifacts::md::schema::diff::{MdDiff, diff_set_snapshot};
use crate::artifacts::md::MdSnapshot;

/// 🔺️ Diff helper for set-snapshot: the sparse field-by-field delta from `base` to `snapshot`
/// (never a full-replace slot -- see `diff_set_snapshot`'s own doc comment).
pub fn diff(base: &MdSnapshot, snapshot: &MdSnapshot) -> MdDiff {
    diff_set_snapshot(base, snapshot)
}
