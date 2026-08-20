use crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::SemioDocumentMutation;
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioDocumentSnapshot, mutation: &SemioDocumentMutation) -> Vec<SemioDocumentMutation> {
    <SemioDocumentMutation as Mutation<SemioDocumentSnapshot>>::inverse(mutation, base)
}
