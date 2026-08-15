use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut PdfSnapshot, mutation: &PdfMutation) {
    apply_pdf_mutation(projection, mutation);
}
