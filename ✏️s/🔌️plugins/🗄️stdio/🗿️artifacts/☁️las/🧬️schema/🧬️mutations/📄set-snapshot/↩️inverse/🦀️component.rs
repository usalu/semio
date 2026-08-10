use crate::artifacts::las::{LasSnapshot};
use crate::artifacts::las::schema::mutations::LasMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &LasSnapshot, mutation: &LasMutation) -> Vec<LasMutation> {
    <LasMutation as Mutation<LasSnapshot>>::inverse(mutation, base)
}
