use crate::artifacts::html::standards::v5::subsets::any::schema::diff::{diff_set_snapshot, HtmlDiff};
use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &HtmlSnapshot, snapshot: &HtmlSnapshot) -> protocol::MutationOutcome<HtmlDiff> {
    if base == snapshot {
        return protocol::MutationOutcome::new(HtmlDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot))
}
