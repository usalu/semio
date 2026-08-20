use crate::artifacts::bcf::schema::mutations::BcfMutation;
use crate::artifacts::bcf::BcfSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &BcfSnapshot, mutation: &BcfMutation) -> Vec<BcfMutation> {
    <BcfMutation as Mutation<BcfSnapshot>>::inverse(mutation, base)
}
