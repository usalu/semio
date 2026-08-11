use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::{SemioObjectMutation, apply_semio_object_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioObjectSnapshot, mutation: &SemioObjectMutation) {
    let _ = apply_semio_object_mutation(projection, mutation);
}
