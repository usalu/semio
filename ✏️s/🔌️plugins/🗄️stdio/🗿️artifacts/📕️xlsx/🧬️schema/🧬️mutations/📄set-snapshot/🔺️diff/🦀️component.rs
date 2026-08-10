//! 🧩 set_snapshot diff leaf.

use crate::artifacts::xlsx::schema::diff::{XlsxDiff, diff_set_snapshot};
use crate::artifacts::xlsx::XlsxSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &XlsxSnapshot) -> XlsxDiff {
    diff_set_snapshot(snapshot)
}
