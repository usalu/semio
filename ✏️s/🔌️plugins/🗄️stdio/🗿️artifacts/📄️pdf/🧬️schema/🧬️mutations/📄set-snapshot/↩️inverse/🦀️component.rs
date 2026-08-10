use crate::artifacts::pdf::{PdfSnapshot};
use crate::artifacts::pdf::schema::mutations::PdfMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &PdfSnapshot, mutation: &PdfMutation) -> Vec<PdfMutation> {
    <PdfMutation as Mutation<PdfSnapshot>>::inverse(mutation, base)
}
