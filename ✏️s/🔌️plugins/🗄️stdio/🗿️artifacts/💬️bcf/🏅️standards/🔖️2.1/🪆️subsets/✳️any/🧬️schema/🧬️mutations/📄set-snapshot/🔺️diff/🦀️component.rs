//! 🧩 set_snapshot diff leaf.

use crate::artifacts::bcf::schema::diff::{diff_set_snapshot, BcfDiff};
use crate::artifacts::bcf::BcfSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &BcfSnapshot, snapshot: &BcfSnapshot) -> BcfDiff {
    diff_set_snapshot(base, snapshot)
}
