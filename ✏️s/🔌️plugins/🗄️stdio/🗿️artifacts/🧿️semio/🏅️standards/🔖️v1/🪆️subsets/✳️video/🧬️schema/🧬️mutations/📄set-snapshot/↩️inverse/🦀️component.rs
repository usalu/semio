use crate::artifacts::semio::standards::v1::subsets::video::schema::mutations::SemioVideoMutation;
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &SemioVideoSnapshot, mutation: &SemioVideoMutation) -> Vec<SemioVideoMutation> {
    <SemioVideoMutation as Mutation<SemioVideoSnapshot>>::inverse(mutation, base)
}
