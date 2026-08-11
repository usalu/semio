use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::{WavMutation, apply_wav_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut WavSnapshot, mutation: &WavMutation) {
    let _ = apply_wav_mutation(projection, mutation);
}
