use crate::standards::iana::subsets::any::schema::mutations::TsvMutation;
use crate::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &TsvSnapshot, mutation: &TsvMutation) -> Result<Vec<TsvMutation>, semio_framework_value::ValueError> {
    Ok({
    <TsvMutation as Mutation<TsvSnapshot>>::inverse(mutation, base)?

    })
}
