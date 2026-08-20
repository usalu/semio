use crate::artifacts::csv::schema::mutations::{apply_csv_mutation, CsvMutation};
use crate::artifacts::csv::CsvSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut CsvSnapshot, mutation: &CsvMutation) {
    apply_csv_mutation(projection, mutation);
}
