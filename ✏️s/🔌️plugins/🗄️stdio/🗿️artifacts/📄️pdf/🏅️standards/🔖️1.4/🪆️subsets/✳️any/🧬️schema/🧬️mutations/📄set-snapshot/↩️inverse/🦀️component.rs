use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::PdfMutation;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &PdfSnapshot, mutation: &PdfMutation) -> Vec<PdfMutation> {
    <PdfMutation as Mutation<PdfSnapshot>>::inverse(mutation, base)
}
