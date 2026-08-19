use crate::artifacts::semio::standards::v1::subsets::animation::schema::mutations::SemioAnimationMutation;
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &SemioAnimationSnapshot, mutation: &SemioAnimationMutation) -> Vec<SemioAnimationMutation> {
    <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::inverse(mutation, base)
}
