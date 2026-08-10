use crate::artifacts::ply::{PlySnapshot};
use crate::artifacts::ply::schema::mutations::{PlyMutation, apply_ply_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut PlySnapshot, mutation: &PlyMutation) {
    apply_ply_mutation(projection, mutation);
}
