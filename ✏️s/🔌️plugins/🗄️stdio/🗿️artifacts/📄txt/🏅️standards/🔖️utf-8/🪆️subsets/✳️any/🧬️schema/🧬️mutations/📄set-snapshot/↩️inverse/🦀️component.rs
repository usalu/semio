use crate::artifacts::txt::schema::mutations::TxtMutation;
use crate::artifacts::txt::TxtSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &TxtSnapshot, mutation: &TxtMutation) -> Vec<TxtMutation> {
    <TxtMutation as Mutation<TxtSnapshot>>::inverse(mutation, base)
}
