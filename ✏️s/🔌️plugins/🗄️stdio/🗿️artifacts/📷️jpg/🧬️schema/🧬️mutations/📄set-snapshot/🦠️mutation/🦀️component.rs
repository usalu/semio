use crate::artifacts::jpg::{JpgSnapshot};
use crate::artifacts::jpg::schema::mutations::{JpgMutation, apply_jpg_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut JpgSnapshot, mutation: &JpgMutation) {
    apply_jpg_mutation(projection, mutation);
}
