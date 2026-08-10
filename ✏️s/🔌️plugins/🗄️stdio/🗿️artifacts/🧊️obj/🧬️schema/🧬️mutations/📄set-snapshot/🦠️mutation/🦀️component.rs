use crate::artifacts::obj::{ObjSnapshot};
use crate::artifacts::obj::schema::mutations::{ObjMutation, apply_obj_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut ObjSnapshot, mutation: &ObjMutation) {
    apply_obj_mutation(projection, mutation);
}
