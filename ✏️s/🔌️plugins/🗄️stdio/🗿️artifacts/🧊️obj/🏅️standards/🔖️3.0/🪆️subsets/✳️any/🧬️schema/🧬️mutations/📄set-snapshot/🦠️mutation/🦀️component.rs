use crate::artifacts::obj::schema::mutations::{apply_obj_mutation, ObjMutation};
use crate::artifacts::obj::{ObjDiff, ObjSnapshot};

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut ObjSnapshot, mutation: &ObjMutation) -> protocol::MutationOutcome<ObjDiff> {
    apply_obj_mutation(projection, mutation)
}
