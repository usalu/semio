use crate::artifacts::deflate::schema::mutations::DeflateMutation;
use crate::artifacts::deflate::DeflateSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &DeflateSnapshot, mutation: &DeflateMutation) -> Vec<DeflateMutation> {
    <DeflateMutation as Mutation<DeflateSnapshot>>::inverse(mutation, base)
}
