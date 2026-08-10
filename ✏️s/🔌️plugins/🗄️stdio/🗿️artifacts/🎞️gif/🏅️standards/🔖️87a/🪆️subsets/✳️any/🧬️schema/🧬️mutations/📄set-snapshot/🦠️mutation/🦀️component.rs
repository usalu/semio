use crate::artifacts::gif::{GifSnapshot};
use crate::artifacts::gif::schema::mutations::{GifMutation, apply_gif_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut GifSnapshot, mutation: &GifMutation) {
    apply_gif_mutation(projection, mutation);
}
