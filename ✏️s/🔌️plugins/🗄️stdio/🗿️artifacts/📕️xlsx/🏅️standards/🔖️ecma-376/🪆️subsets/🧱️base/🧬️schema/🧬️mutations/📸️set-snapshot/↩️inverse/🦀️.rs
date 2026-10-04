use crate::schema::mutations::XlsxMutation;
use crate::XlsxSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &XlsxSnapshot, mutation: &XlsxMutation) -> Result<Vec<XlsxMutation>, semio_framework_value::ValueError> {
    Ok({
    <XlsxMutation as Mutation<XlsxSnapshot>>::inverse(mutation, base)?

    })
}
