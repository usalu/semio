use crate::artifacts::csv::{CsvSnapshot};
use crate::artifacts::csv::schema::mutations::{CsvMutation, apply_csv_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut CsvSnapshot, mutation: &CsvMutation) {
    apply_csv_mutation(projection, mutation);
}
