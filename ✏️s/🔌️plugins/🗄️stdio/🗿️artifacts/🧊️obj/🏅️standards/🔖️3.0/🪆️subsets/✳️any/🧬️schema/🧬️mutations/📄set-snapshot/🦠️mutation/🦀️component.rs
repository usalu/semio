use crate::artifacts::obj::schema::mutations::{apply_obj_mutation, ObjMutation};
use crate::artifacts::obj::ObjSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut ObjSnapshot, mutation: &ObjMutation) {
    apply_obj_mutation(projection, mutation);
}
