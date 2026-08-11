use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioObjectSnapshot, mutation: &SemioObjectMutation) -> Vec<SemioObjectMutation> {
    <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::inverse(mutation, base)
}
