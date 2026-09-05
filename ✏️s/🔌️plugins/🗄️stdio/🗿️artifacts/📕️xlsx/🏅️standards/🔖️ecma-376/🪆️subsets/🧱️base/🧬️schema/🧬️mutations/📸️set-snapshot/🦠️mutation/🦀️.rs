use crate::artifacts::xlsx::schema::mutations::{apply_xlsx_mutation, XlsxMutation};
use crate::artifacts::xlsx::XlsxSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut XlsxSnapshot, mutation: &XlsxMutation) {
    apply_xlsx_mutation(projection, mutation);
}
