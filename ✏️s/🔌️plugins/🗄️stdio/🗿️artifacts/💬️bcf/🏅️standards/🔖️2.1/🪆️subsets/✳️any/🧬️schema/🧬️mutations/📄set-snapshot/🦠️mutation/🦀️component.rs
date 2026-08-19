use crate::artifacts::bcf::schema::mutations::{apply_bcf_mutation, BcfMutation};
use crate::artifacts::bcf::BcfSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut BcfSnapshot, mutation: &BcfMutation) {
    apply_bcf_mutation(projection, mutation);
}
