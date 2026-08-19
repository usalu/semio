use crate::artifacts::docx::schema::mutations::{apply_docx_mutation, DocxMutation};
use crate::artifacts::docx::DocxSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut DocxSnapshot, mutation: &DocxMutation) {
    apply_docx_mutation(projection, mutation);
}
