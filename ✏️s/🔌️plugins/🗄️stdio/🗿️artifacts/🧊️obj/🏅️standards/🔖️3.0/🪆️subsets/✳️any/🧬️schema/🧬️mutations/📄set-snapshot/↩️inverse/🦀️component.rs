use crate::artifacts::obj::schema::mutations::ObjMutation;
use crate::artifacts::obj::ObjSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &ObjSnapshot, mutation: &ObjMutation) -> Vec<ObjMutation> {
    <ObjMutation as Mutation<ObjSnapshot>>::inverse(mutation, base)
}
