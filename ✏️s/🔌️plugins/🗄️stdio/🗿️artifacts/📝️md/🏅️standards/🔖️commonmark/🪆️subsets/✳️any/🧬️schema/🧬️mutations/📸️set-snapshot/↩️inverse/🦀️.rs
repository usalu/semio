use crate::schema::mutations::MdMutation;
use crate::MdSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &MdSnapshot, mutation: &MdMutation) -> Result<Vec<MdMutation>, semio_framework_value::ValueError> {
    Ok({
    <MdMutation as Mutation<MdSnapshot>>::inverse(mutation, base)?

    })
}
