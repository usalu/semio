use crate::artifacts::md::schema::mutations::MdMutation;
use crate::artifacts::md::MdSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &MdSnapshot, mutation: &MdMutation) -> Vec<MdMutation> {
    <MdMutation as Mutation<MdSnapshot>>::inverse(mutation, base)
}
