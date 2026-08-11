use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::mutations::SemioCadMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioCadSnapshot, mutation: &SemioCadMutation) -> Vec<SemioCadMutation> {
    <SemioCadMutation as Mutation<SemioCadSnapshot>>::inverse(mutation, base)
}
