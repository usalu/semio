use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use crate::artifacts::semio::standards::v1::subsets::video::schema::mutations::{SemioVideoMutation, apply_semio_video_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioVideoSnapshot, mutation: &SemioVideoMutation) {
    let _ = apply_semio_video_mutation(projection, mutation);
}
