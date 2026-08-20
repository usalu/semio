use crate::artifacts::zip::schema::mutations::ZipMutation;
use crate::artifacts::zip::ZipSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &ZipSnapshot, mutation: &ZipMutation) -> Vec<ZipMutation> {
    <ZipMutation as Mutation<ZipSnapshot>>::inverse(mutation, base)
}
