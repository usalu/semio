use crate::artifacts::pptx::schema::mutations::{apply_pptx_mutation, PptxMutation};
use crate::artifacts::pptx::PptxSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut PptxSnapshot, mutation: &PptxMutation) {
    apply_pptx_mutation(projection, mutation);
}
