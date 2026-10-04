//! ↩️ Inverse for `set-snapshot`.

use crate::DeflateSnapshot;
use crate::schema::mutations::DeflateMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &DeflateSnapshot, mutation: &DeflateMutation) -> Result<Vec<DeflateMutation>, semio_framework_value::ValueError> {
    Ok({
    <DeflateMutation as Mutation<DeflateSnapshot>>::inverse(mutation, base)?

    })
}
