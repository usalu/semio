use crate::artifacts::png::schema::mutations::PngMutation;
use crate::artifacts::png::PngSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &PngSnapshot, mutation: &PngMutation) -> Vec<PngMutation> {
    <PngMutation as Mutation<PngSnapshot>>::inverse(mutation, base)
}
