use crate::artifacts::bmp::{BmpSnapshot};
use crate::artifacts::bmp::schema::mutations::{BmpMutation, apply_bmp_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut BmpSnapshot, mutation: &BmpMutation) {
    apply_bmp_mutation(projection, mutation);
}
