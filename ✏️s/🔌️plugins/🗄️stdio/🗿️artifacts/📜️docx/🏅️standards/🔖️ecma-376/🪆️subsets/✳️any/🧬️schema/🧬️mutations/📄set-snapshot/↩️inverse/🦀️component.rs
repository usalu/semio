use crate::artifacts::docx::schema::mutations::DocxMutation;
use crate::artifacts::docx::DocxSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &DocxSnapshot, mutation: &DocxMutation) -> Vec<DocxMutation> {
    <DocxMutation as Mutation<DocxSnapshot>>::inverse(mutation, base)
}
