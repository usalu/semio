use crate::artifacts::pptx::schema::mutations::PptxMutation;
use crate::artifacts::pptx::PptxSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &PptxSnapshot, mutation: &PptxMutation) -> Vec<PptxMutation> {
    <PptxMutation as Mutation<PptxSnapshot>>::inverse(mutation, base)
}
