use crate::artifacts::csv::schema::mutations::CsvMutation;
use crate::artifacts::csv::CsvSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &CsvSnapshot, mutation: &CsvMutation) -> Vec<CsvMutation> {
    <CsvMutation as Mutation<CsvSnapshot>>::inverse(mutation, base)
}
