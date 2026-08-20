use crate::artifacts::tiff::schema::mutations::TiffMutation;
use crate::artifacts::tiff::TiffSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &TiffSnapshot, mutation: &TiffMutation) -> Vec<TiffMutation> {
    <TiffMutation as Mutation<TiffSnapshot>>::inverse(mutation, base)
}
