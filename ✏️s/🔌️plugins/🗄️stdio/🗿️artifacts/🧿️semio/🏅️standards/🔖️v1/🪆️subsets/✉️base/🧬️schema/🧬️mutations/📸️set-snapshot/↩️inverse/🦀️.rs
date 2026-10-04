use crate::standards::v1::subsets::base::schema::mutations::SemioMutation;
use crate::standards::v1::subsets::base::schema::snapshot::SemioSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioSnapshot, mutation: &SemioMutation) -> Result<Vec<SemioMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioMutation as Mutation<SemioSnapshot>>::inverse(mutation, base)?

    })
}
