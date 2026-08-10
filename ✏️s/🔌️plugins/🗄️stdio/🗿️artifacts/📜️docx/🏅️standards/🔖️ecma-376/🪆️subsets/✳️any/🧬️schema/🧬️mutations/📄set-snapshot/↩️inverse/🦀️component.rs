use crate::artifacts::docx::{DocxSnapshot};
use crate::artifacts::docx::schema::mutations::DocxMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &DocxSnapshot, mutation: &DocxMutation) -> Vec<DocxMutation> {
    <DocxMutation as Mutation<DocxSnapshot>>::inverse(mutation, base)
}
