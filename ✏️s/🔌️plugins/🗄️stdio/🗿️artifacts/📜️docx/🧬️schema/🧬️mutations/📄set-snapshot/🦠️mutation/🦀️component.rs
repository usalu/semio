use crate::artifacts::docx::{DocxSnapshot};
use crate::artifacts::docx::schema::mutations::{DocxMutation, apply_docx_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut DocxSnapshot, mutation: &DocxMutation) {
    apply_docx_mutation(projection, mutation);
}
