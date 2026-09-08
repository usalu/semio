//! ↩️ Inverse for `set-snapshot`.

use crate::ObjSnapshot;
use crate::schema::mutations::{ObjMutation, apply_obj_mutation};
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &ObjSnapshot, mutation: &ObjMutation) -> Vec<ObjMutation> {
    <ObjMutation as Mutation<ObjSnapshot>>::inverse(mutation, base)
}
