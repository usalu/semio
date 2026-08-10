use crate::artifacts::pdf::{PdfSnapshot};
use crate::artifacts::pdf::schema::mutations::{PdfMutation, apply_pdf_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut PdfSnapshot, mutation: &PdfMutation) {
    apply_pdf_mutation(projection, mutation);
}
