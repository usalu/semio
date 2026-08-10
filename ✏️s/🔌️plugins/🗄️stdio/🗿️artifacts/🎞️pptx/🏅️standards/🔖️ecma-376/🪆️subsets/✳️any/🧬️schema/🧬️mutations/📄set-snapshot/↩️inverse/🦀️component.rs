use crate::artifacts::pptx::{PptxSnapshot};
use crate::artifacts::pptx::schema::mutations::PptxMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &PptxSnapshot, mutation: &PptxMutation) -> Vec<PptxMutation> {
    <PptxMutation as Mutation<PptxSnapshot>>::inverse(mutation, base)
}
