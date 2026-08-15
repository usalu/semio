use crate::artifacts::jpg::schema::mutations::{apply_jpg_mutation, JpgMutation};
use crate::artifacts::jpg::JpgSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut JpgSnapshot, mutation: &JpgMutation) {
    apply_jpg_mutation(projection, mutation);
}
