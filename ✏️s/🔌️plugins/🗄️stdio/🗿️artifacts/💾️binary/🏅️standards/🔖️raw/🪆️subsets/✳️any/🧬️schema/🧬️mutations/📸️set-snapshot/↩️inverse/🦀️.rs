//! ↩️ Inverse for `set-snapshot`.

use crate::BinarySnapshot;
use crate::schema::mutations::BinaryMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &BinarySnapshot, mutation: &BinaryMutation) -> Result<Vec<BinaryMutation>, semio_framework_value::ValueError> {
    Ok({
    <BinaryMutation as Mutation<BinarySnapshot>>::inverse(mutation, base)?

    })
}
