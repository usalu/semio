//! 🧩 set_snapshot diff leaf.

use crate::artifacts::csv::schema::diff::{diff_set_snapshot, CsvDiff};
use crate::artifacts::csv::CsvSnapshot;

/// 🔺️ Diff helper for set-snapshot (sparse field-by-field delta, never a full-replace slot).
pub fn diff(base: &CsvSnapshot, next: &CsvSnapshot) -> CsvDiff {
    diff_set_snapshot(base, next)
}
