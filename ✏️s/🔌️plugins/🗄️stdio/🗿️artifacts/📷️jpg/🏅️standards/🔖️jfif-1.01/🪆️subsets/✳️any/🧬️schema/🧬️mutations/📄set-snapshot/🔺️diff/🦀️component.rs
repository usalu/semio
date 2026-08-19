//! 🧩 set_snapshot diff leaf.

use crate::artifacts::jpg::schema::diff::{diff_set_snapshot, JpgDiff};
use crate::artifacts::jpg::JpgSnapshot;

/// 🔺️ Diff helper for set-snapshot: sparse field-by-field `between(base, next)`.
pub async fn diff(base: &JpgSnapshot, next: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
    if base == next {
        return protocol::MutationOutcome::new(JpgDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, next))
}
