use crate::artifacts::semio::standards::v1::subsets::video::schema::mutations::{apply_semio_video_mutation, SemioVideoMutation};
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioVideoSnapshot, mutation: &SemioVideoMutation) {
    let _ = apply_semio_video_mutation(projection, mutation);
}
