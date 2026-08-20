use crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::{apply_semio_audio_mutation, SemioAudioMutation};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut SemioAudioSnapshot, mutation: &SemioAudioMutation) {
    let _ = apply_semio_audio_mutation(projection, mutation);
}
