use crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::{apply_semio_document_mutation, SemioDocumentMutation};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut SemioDocumentSnapshot, mutation: &SemioDocumentMutation) {
    let _ = apply_semio_document_mutation(projection, mutation);
}
