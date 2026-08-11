use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::{SemioDocumentMutation, apply_semio_document_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioDocumentSnapshot, mutation: &SemioDocumentMutation) {
    let _ = apply_semio_document_mutation(projection, mutation);
}
