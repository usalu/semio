use crate::artifacts::semio::standards::v1::subsets::model::schema::mutations::SemioModelMutation;
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioModelSnapshot, mutation: &SemioModelMutation) -> Vec<SemioModelMutation> {
    <SemioModelMutation as Mutation<SemioModelSnapshot>>::inverse(mutation, base)
}
