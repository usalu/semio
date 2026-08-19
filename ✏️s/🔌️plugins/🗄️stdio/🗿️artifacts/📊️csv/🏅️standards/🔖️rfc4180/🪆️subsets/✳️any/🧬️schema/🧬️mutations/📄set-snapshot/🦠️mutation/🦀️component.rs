use crate::artifacts::csv::schema::mutations::{apply_csv_mutation, CsvMutation};
use crate::artifacts::csv::CsvSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut CsvSnapshot, mutation: &CsvMutation) {
    apply_csv_mutation(projection, mutation);
}
