use crate::artifacts::gif::standards::v89a::subsets::any::schema::mutations::{apply_gif_mutation, GifMutation};
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut GifSnapshot, mutation: &GifMutation) {
    apply_gif_mutation(projection, mutation);
}
