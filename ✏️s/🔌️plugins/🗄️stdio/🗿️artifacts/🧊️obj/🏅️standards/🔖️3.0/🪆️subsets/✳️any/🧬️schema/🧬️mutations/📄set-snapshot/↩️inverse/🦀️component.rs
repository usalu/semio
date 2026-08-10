use crate::artifacts::obj::{ObjSnapshot};
use crate::artifacts::obj::schema::mutations::ObjMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &ObjSnapshot, mutation: &ObjMutation) -> Vec<ObjMutation> {
    <ObjMutation as Mutation<ObjSnapshot>>::inverse(mutation, base)
}
