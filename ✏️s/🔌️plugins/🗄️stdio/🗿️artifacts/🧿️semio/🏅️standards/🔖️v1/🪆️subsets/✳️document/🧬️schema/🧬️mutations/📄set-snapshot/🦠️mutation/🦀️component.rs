use crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::{apply_semio_document_mutation, SemioDocumentMutation};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut SemioDocumentSnapshot, mutation: &SemioDocumentMutation) {
    let _ = apply_semio_document_mutation(projection, mutation);
}
