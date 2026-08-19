use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::{apply_wav_mutation, WavMutation};
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut WavSnapshot, mutation: &WavMutation) {
    let _ = apply_wav_mutation(projection, mutation);
}
