use crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::{apply_semio_audio_mutation, SemioAudioMutation};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut SemioAudioSnapshot, mutation: &SemioAudioMutation) {
    let _ = apply_semio_audio_mutation(projection, mutation);
}
