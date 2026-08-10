use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::mutations::{GifMutation, apply_gif_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut GifSnapshot, mutation: &GifMutation) {
    apply_gif_mutation(projection, mutation);
}
