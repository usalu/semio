use crate::artifacts::zip::{ZipSnapshot};
use crate::artifacts::zip::schema::mutations::{ZipMutation, apply_zip_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut ZipSnapshot, mutation: &ZipMutation) {
    apply_zip_mutation(projection, mutation);
}
