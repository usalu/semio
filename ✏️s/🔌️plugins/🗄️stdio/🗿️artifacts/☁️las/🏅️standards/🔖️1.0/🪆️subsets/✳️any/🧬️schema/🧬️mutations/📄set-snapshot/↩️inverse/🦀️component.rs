use crate::artifacts::las::schema::mutations::LasMutation;
use crate::artifacts::las::LasSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &LasSnapshot, mutation: &LasMutation) -> Vec<LasMutation> {
    <LasMutation as Mutation<LasSnapshot>>::inverse(mutation, base)
}
