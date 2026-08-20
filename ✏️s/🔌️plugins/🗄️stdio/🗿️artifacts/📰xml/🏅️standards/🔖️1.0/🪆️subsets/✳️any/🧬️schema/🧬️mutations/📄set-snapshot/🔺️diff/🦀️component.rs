//! 🧩 set_snapshot diff leaf.

use crate::artifacts::xml::schema::diff::{diff_set_snapshot, XmlDiff};
use crate::artifacts::xml::XmlSnapshot;

/// 🔺️ Diff helper for set-snapshot -- the sparse field-by-field `XmlDiff::between(base, next)`,
/// never a whole-`XmlSnapshot` replace slot.
pub async fn diff(base: &XmlSnapshot, next: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
    if base == next {
        return protocol::MutationOutcome::new(XmlDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one").await;
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, next))
}
