use crate::artifacts::stl::{StlSnapshot};
use crate::artifacts::stl::schema::mutations::StlMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &StlSnapshot, mutation: &StlMutation) -> Vec<StlMutation> {
    <StlMutation as Mutation<StlSnapshot>>::inverse(mutation, base)
}
