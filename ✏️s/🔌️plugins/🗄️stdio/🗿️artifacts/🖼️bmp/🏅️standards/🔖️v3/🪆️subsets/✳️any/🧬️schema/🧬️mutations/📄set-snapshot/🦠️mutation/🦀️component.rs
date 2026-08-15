use crate::artifacts::bmp::schema::mutations::{apply_bmp_mutation, BmpMutation};
use crate::artifacts::bmp::BmpSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut BmpSnapshot, mutation: &BmpMutation) {
    apply_bmp_mutation(projection, mutation);
}
