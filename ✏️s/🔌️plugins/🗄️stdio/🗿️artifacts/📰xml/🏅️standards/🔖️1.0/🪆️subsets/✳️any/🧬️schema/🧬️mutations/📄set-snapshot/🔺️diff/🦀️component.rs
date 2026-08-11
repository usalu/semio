//! 🧩 set_snapshot diff leaf.

use crate::artifacts::xml::schema::diff::{XmlDiff, diff_set_snapshot};
use crate::artifacts::xml::XmlSnapshot;

/// 🔺️ Diff helper for set-snapshot -- the sparse field-by-field `XmlDiff::between(base, next)`,
/// never a whole-`XmlSnapshot` replace slot.
pub fn diff(base: &XmlSnapshot, next: &XmlSnapshot) -> XmlDiff {
    diff_set_snapshot(base, next)
}
