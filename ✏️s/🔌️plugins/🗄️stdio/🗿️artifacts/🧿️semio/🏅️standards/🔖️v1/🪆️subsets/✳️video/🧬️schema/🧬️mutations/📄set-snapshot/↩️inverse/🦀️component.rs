use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use crate::artifacts::semio::standards::v1::subsets::video::schema::mutations::SemioVideoMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioVideoSnapshot, mutation: &SemioVideoMutation) -> Vec<SemioVideoMutation> {
    <SemioVideoMutation as Mutation<SemioVideoSnapshot>>::inverse(mutation, base)
}
