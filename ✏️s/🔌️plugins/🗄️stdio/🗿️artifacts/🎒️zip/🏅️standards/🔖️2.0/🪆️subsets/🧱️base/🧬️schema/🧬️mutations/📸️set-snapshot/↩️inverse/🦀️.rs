//! ↩️ Inverse for `set-snapshot`.

use crate::ZipSnapshot;
use crate::schema::mutations::ZipMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &ZipSnapshot, mutation: &ZipMutation) -> Result<Vec<ZipMutation>, semio_framework_value::ValueError> {
    Ok({
    <ZipMutation as Mutation<ZipSnapshot>>::inverse(mutation, base)?

    })
}
