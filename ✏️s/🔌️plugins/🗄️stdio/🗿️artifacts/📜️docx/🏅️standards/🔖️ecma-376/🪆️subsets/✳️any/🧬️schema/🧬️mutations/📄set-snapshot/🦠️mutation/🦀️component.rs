use crate::artifacts::docx::schema::mutations::{apply_docx_mutation, DocxMutation};
use crate::artifacts::docx::DocxSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut DocxSnapshot, mutation: &DocxMutation) {
    apply_docx_mutation(projection, mutation);
}
