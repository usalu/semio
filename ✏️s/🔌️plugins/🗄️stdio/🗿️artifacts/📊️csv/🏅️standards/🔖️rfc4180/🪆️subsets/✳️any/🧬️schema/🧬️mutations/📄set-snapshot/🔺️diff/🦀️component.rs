//! 🧩 set_snapshot diff leaf.

use crate::artifacts::csv::schema::diff::{diff_set_snapshot, CsvDiff};
use crate::artifacts::csv::CsvSnapshot;

/// 🔺️ Diff helper for set-snapshot (sparse field-by-field delta, never a full-replace slot).
pub async fn diff(base: &CsvSnapshot, next: &CsvSnapshot) -> protocol::MutationOutcome<CsvDiff> {
    if base == next {
        return protocol::MutationOutcome::new(CsvDiff::default()).await.warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one").await;
    }
    protocol::MutationOutcome::new(diff_set_snapshot(base, next))
}
