use crate::artifacts::html::standards::v5::subsets::any::schema::diff::{diff_set_snapshot, HtmlDiff};
use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &HtmlSnapshot, snapshot: &HtmlSnapshot) -> HtmlDiff {
    diff_set_snapshot(base, snapshot)
}
