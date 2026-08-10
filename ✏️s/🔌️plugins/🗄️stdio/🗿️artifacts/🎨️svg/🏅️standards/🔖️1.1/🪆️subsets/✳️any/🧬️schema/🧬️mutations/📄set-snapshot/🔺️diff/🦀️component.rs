//! 🧩 set_snapshot diff leaf.

use crate::artifacts::svg::schema::diff::{SvgDiff, diff_set_snapshot};
use crate::artifacts::svg::SvgSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &SvgSnapshot) -> SvgDiff {
    diff_set_snapshot(snapshot)
}
