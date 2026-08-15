use crate::artifacts::las::schema::mutations::{apply_las_mutation, LasMutation};
use crate::artifacts::las::LasSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut LasSnapshot, mutation: &LasMutation) {
    apply_las_mutation(projection, mutation);
}
