use crate::artifacts::stl::schema::mutations::{apply_stl_mutation, StlMutation};
use crate::artifacts::stl::StlSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut StlSnapshot, mutation: &StlMutation) {
    apply_stl_mutation(projection, mutation);
}
