use crate::artifacts::stl::schema::mutations::StlMutation;
use crate::artifacts::stl::StlSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &StlSnapshot, mutation: &StlMutation) -> Vec<StlMutation> {
    <StlMutation as Mutation<StlSnapshot>>::inverse(mutation, base)
}
