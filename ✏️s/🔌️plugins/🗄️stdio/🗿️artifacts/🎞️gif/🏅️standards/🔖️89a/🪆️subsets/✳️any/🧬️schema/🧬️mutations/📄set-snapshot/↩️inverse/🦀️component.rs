use crate::artifacts::gif::standards::v89a::subsets::any::schema::mutations::GifMutation;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &GifSnapshot, mutation: &GifMutation) -> Vec<GifMutation> {
    <GifMutation as Mutation<GifSnapshot>>::inverse(mutation, base)
}
