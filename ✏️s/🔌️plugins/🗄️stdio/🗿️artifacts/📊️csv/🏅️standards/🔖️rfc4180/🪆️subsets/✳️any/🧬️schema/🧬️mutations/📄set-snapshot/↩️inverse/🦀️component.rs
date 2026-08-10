use crate::artifacts::csv::{CsvSnapshot};
use crate::artifacts::csv::schema::mutations::CsvMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &CsvSnapshot, mutation: &CsvMutation) -> Vec<CsvMutation> {
    <CsvMutation as Mutation<CsvSnapshot>>::inverse(mutation, base)
}
