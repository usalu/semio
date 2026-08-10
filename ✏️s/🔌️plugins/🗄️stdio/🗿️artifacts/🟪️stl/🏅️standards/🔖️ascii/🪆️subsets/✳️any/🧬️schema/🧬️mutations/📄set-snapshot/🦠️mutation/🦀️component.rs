use crate::artifacts::stl::{StlSnapshot};
use crate::artifacts::stl::schema::mutations::{StlMutation, apply_stl_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut StlSnapshot, mutation: &StlMutation) {
    apply_stl_mutation(projection, mutation);
}
