use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut PdfSnapshot, mutation: &PdfMutation) {
    apply_pdf_mutation(projection, mutation);
}
