use crate::artifacts::glb::{GlbSnapshot};
use crate::artifacts::glb::schema::mutations::{GlbMutation, apply_glb_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut GlbSnapshot, mutation: &GlbMutation) {
    apply_glb_mutation(projection, mutation);
}
