//! 🧩 set_snapshot diff leaf.

use crate::artifacts::bcf::schema::diff::{diff_set_snapshot, BcfDiff};
use crate::artifacts::bcf::BcfSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub async fn diff(base: &BcfSnapshot, snapshot: &BcfSnapshot) -> protocol::MutationOutcome<BcfDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(BcfDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
