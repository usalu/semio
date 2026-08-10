use crate::artifacts::md::{MdSnapshot};
use crate::artifacts::md::schema::mutations::{MdMutation, apply_md_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut MdSnapshot, mutation: &MdMutation) {
    apply_md_mutation(projection, mutation);
}
