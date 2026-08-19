use crate::artifacts::pptx::schema::mutations::{apply_pptx_mutation, PptxMutation};
use crate::artifacts::pptx::PptxSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut PptxSnapshot, mutation: &PptxMutation) {
    apply_pptx_mutation(projection, mutation);
}
