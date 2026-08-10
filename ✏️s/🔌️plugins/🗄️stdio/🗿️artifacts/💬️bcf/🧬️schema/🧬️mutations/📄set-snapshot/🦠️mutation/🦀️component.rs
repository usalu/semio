use crate::artifacts::bcf::{BcfSnapshot};
use crate::artifacts::bcf::schema::mutations::{BcfMutation, apply_bcf_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut BcfSnapshot, mutation: &BcfMutation) {
    apply_bcf_mutation(projection, mutation);
}
