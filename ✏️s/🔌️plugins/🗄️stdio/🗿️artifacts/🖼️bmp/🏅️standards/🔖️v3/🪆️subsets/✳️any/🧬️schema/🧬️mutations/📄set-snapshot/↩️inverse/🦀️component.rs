use crate::artifacts::bmp::schema::mutations::BmpMutation;
use crate::artifacts::bmp::BmpSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &BmpSnapshot, mutation: &BmpMutation) -> Vec<BmpMutation> {
    <BmpMutation as Mutation<BmpSnapshot>>::inverse(mutation, base)
}
