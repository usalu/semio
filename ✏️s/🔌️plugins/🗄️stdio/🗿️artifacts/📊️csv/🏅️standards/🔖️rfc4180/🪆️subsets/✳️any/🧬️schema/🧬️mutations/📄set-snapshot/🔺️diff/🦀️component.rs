//! 🧩 set_snapshot diff leaf.

use crate::artifacts::csv::schema::diff::{CsvDiff, diff_set_snapshot};
use crate::artifacts::csv::CsvSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(snapshot: &CsvSnapshot) -> CsvDiff {
    diff_set_snapshot(snapshot)
}
