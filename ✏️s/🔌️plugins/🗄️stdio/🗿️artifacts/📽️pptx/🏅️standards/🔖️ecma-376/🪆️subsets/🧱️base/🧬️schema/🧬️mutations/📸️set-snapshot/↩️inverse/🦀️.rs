use crate::schema::mutations::PptxMutation;
use crate::PptxSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &PptxSnapshot, mutation: &PptxMutation) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
    Ok({
    <PptxMutation as Mutation<PptxSnapshot>>::inverse(mutation, base)?

    })
}
