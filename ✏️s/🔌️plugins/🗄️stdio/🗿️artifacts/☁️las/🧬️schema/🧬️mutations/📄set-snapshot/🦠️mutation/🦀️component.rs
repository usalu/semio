use crate::artifacts::las::{LasSnapshot};
use crate::artifacts::las::schema::mutations::{LasMutation, apply_las_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut LasSnapshot, mutation: &LasMutation) {
    apply_las_mutation(projection, mutation);
}
