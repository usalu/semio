use crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::SemioAudioMutation;
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioAudioSnapshot, mutation: &SemioAudioMutation) -> Vec<SemioAudioMutation> {
    <SemioAudioMutation as Mutation<SemioAudioSnapshot>>::inverse(mutation, base)
}
