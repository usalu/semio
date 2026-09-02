use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::{apply_wav_mutation, WavMutation};
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut WavSnapshot, mutation: &WavMutation) {
    let _ = apply_wav_mutation(projection, mutation);
}
