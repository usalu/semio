use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioDrawingSnapshot, mutation: &SemioDrawingMutation) -> Vec<SemioDrawingMutation> {
    <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::inverse(mutation, base)
}
