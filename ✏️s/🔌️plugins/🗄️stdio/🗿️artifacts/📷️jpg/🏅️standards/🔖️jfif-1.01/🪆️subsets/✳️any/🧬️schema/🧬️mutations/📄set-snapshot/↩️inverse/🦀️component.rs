use crate::artifacts::jpg::schema::mutations::JpgMutation;
use crate::artifacts::jpg::JpgSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &JpgSnapshot, mutation: &JpgMutation) -> Vec<JpgMutation> {
    <JpgMutation as Mutation<JpgSnapshot>>::inverse(mutation, base)
}
