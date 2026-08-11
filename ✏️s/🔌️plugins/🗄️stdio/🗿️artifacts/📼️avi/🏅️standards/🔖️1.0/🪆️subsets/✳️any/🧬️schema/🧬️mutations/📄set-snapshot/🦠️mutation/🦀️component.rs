use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::{AviMutation, apply_avi_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut AviSnapshot, mutation: &AviMutation) {
    let _ = apply_avi_mutation(projection, mutation);
}
