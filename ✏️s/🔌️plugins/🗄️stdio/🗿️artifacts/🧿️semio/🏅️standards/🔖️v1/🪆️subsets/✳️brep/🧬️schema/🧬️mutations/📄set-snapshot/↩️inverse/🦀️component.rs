use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioBrepSnapshot, mutation: &SemioBrepMutation) -> Vec<SemioBrepMutation> {
    <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::inverse(mutation, base)
}
