use crate::artifacts::glb::{GlbSnapshot};
use crate::artifacts::glb::schema::mutations::GlbMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &GlbSnapshot, mutation: &GlbMutation) -> Vec<GlbMutation> {
    <GlbMutation as Mutation<GlbSnapshot>>::inverse(mutation, base)
}
