use crate::standards::v1::subsets::cad::schema::mutations::SemioCadMutation;
use crate::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioCadSnapshot, mutation: &SemioCadMutation) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioCadMutation as Mutation<SemioCadSnapshot>>::inverse(mutation, base)?

    })
}
