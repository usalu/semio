use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use crate::artifacts::semio::standards::v1::subsets::animation::schema::mutations::SemioAnimationMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioAnimationSnapshot, mutation: &SemioAnimationMutation) -> Vec<SemioAnimationMutation> {
    <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::inverse(mutation, base)
}
