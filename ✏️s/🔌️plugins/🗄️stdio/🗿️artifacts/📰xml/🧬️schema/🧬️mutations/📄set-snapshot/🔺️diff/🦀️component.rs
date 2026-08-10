//! 🧩 set_snapshot diff leaf.

use crate::artifacts::xml::schema::diff::{XmlDiff, diff_set_snapshot};
use crate::artifacts::xml::XmlSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &XmlSnapshot) -> XmlDiff {
    diff_set_snapshot(snapshot)
}
