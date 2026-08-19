use crate::artifacts::obj::schema::mutations::{apply_obj_mutation, ObjMutation};
use crate::artifacts::obj::{ObjDiff, ObjSnapshot};

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut ObjSnapshot, mutation: &ObjMutation) -> protocol::MutationOutcome<ObjDiff> {
    apply_obj_mutation(projection, mutation)
}
