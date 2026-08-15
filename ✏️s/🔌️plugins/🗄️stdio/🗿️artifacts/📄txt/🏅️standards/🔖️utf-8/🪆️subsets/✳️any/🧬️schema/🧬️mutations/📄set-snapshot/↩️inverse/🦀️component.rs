use crate::artifacts::txt::schema::mutations::TxtMutation;
use crate::artifacts::txt::TxtSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &TxtSnapshot, mutation: &TxtMutation) -> Vec<TxtMutation> {
    <TxtMutation as Mutation<TxtSnapshot>>::inverse(mutation, base)
}
