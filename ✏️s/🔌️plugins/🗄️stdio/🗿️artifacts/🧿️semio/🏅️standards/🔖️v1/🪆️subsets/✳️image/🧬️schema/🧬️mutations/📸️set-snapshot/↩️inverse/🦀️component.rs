use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &SemioImageSnapshot, mutation: &SemioImageMutation) -> Vec<SemioImageMutation> {
    <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(mutation, base)
}
