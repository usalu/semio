use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{PdfMutation, apply_pdf_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut PdfSnapshot, mutation: &PdfMutation) {
    apply_pdf_mutation(projection, mutation);
}
