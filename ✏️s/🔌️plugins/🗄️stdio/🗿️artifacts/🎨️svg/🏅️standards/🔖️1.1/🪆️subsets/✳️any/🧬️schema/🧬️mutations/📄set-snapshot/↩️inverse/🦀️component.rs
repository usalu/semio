use crate::artifacts::svg::schema::mutations::SvgMutation;
use crate::artifacts::svg::SvgSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SvgSnapshot, mutation: &SvgMutation) -> Vec<SvgMutation> {
    <SvgMutation as Mutation<SvgSnapshot>>::inverse(mutation, base)
}
