//! ↩️ Inverse for `set-snapshot`.

use crate::artifacts::zip::ZipSnapshot;
use crate::artifacts::zip::schema::mutations::{ZipMutation, apply_zip_mutation};
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &ZipSnapshot, mutation: &ZipMutation) -> Vec<ZipMutation> {
    <ZipMutation as Mutation<ZipSnapshot>>::inverse(mutation, base)
}
