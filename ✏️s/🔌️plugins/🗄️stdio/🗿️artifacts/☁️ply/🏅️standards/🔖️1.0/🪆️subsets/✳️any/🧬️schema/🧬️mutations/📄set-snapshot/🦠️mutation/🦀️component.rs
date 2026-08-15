use crate::artifacts::ply::schema::mutations::{apply_ply_mutation, PlyMutation};
use crate::artifacts::ply::PlySnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut PlySnapshot, mutation: &PlyMutation) {
    apply_ply_mutation(projection, mutation);
}
