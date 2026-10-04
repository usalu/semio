use crate::schema::mutations::BcfMutation;
use crate::BcfSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &BcfSnapshot, mutation: &BcfMutation) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
    Ok({
    <BcfMutation as Mutation<BcfSnapshot>>::inverse(mutation, base)?

    })
}
