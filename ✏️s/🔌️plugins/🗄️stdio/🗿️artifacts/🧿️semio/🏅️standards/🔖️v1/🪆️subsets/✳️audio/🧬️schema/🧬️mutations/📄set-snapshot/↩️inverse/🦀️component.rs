use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::SemioAudioMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioAudioSnapshot, mutation: &SemioAudioMutation) -> Vec<SemioAudioMutation> {
    <SemioAudioMutation as Mutation<SemioAudioSnapshot>>::inverse(mutation, base)
}
