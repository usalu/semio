use crate::schema::mutations::DocxMutation;
use crate::DocxSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &DocxSnapshot, mutation: &DocxMutation) -> Vec<DocxMutation> {
    <DocxMutation as Mutation<DocxSnapshot>>::inverse(mutation, base)
}
