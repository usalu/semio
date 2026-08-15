use crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::SemioDocumentMutation;
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioDocumentSnapshot, mutation: &SemioDocumentMutation) -> Vec<SemioDocumentMutation> {
    <SemioDocumentMutation as Mutation<SemioDocumentSnapshot>>::inverse(mutation, base)
}
