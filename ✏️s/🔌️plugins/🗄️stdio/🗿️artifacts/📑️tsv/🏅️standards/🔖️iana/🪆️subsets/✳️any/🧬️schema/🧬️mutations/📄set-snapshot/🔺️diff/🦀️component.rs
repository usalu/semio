use crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::{diff_set_snapshot, TsvDiff};
use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &TsvSnapshot, snapshot: &TsvSnapshot) -> TsvDiff {
    diff_set_snapshot(base, snapshot)
}
