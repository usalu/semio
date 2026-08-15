use crate::artifacts::semio::standards::v1::subsets::presentation::schema::mutations::SemioPresentationMutation;
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioPresentationSnapshot, mutation: &SemioPresentationMutation) -> Vec<SemioPresentationMutation> {
    <SemioPresentationMutation as Mutation<SemioPresentationSnapshot>>::inverse(mutation, base)
}
