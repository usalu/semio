use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::artifacts::semio::standards::v1::subsets::model::schema::mutations::{SemioModelMutation, apply_semio_model_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioModelSnapshot, mutation: &SemioModelMutation) {
    let _ = apply_semio_model_mutation(projection, mutation);
}
