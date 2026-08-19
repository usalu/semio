use crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::SemioAudioMutation;
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &SemioAudioSnapshot, mutation: &SemioAudioMutation) -> Vec<SemioAudioMutation> {
    <SemioAudioMutation as Mutation<SemioAudioSnapshot>>::inverse(mutation, base)
}
