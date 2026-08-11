//! 🧩 set_snapshot diff leaf.

use crate::artifacts::bcf::schema::diff::{BcfDiff, diff_set_snapshot};
use crate::artifacts::bcf::BcfSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &BcfSnapshot, snapshot: &BcfSnapshot) -> BcfDiff {
    diff_set_snapshot(base, snapshot)
}
