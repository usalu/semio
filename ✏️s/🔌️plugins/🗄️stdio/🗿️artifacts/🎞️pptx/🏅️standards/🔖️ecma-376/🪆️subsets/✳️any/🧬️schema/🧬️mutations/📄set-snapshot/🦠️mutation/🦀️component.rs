use crate::artifacts::pptx::{PptxSnapshot};
use crate::artifacts::pptx::schema::mutations::{PptxMutation, apply_pptx_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut PptxSnapshot, mutation: &PptxMutation) {
    apply_pptx_mutation(projection, mutation);
}
