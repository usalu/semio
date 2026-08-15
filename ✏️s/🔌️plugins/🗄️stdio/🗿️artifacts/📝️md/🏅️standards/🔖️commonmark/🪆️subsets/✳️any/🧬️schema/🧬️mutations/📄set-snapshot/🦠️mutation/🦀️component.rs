use crate::artifacts::md::schema::mutations::{apply_md_mutation, MdMutation};
use crate::artifacts::md::MdSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut MdSnapshot, mutation: &MdMutation) {
    apply_md_mutation(projection, mutation);
}
