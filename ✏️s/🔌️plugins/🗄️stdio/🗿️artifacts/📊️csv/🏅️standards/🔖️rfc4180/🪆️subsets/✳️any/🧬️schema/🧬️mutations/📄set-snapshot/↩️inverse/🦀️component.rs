use crate::artifacts::csv::schema::mutations::CsvMutation;
use crate::artifacts::csv::CsvSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &CsvSnapshot, mutation: &CsvMutation) -> Vec<CsvMutation> {
    <CsvMutation as Mutation<CsvSnapshot>>::inverse(mutation, base)
}
