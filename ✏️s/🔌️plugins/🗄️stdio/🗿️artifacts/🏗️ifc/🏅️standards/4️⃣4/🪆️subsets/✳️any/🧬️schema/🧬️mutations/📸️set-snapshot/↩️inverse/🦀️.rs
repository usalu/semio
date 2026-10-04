use crate::schema::mutations::IfcMutation;
use crate::IfcSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &IfcSnapshot, mutation: &IfcMutation) -> Result<Vec<IfcMutation>, semio_framework_value::ValueError> {
    Ok({
    <IfcMutation as Mutation<IfcSnapshot>>::inverse(mutation, base)?

    })
}
