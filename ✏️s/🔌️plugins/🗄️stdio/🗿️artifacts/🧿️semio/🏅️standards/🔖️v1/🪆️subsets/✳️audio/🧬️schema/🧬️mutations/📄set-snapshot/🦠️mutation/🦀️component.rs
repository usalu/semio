use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::{SemioAudioMutation, apply_semio_audio_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioAudioSnapshot, mutation: &SemioAudioMutation) {
    let _ = apply_semio_audio_mutation(projection, mutation);
}
