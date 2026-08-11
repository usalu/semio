use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::WavMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &WavSnapshot, mutation: &WavMutation) -> Vec<WavMutation> {
    <WavMutation as Mutation<WavSnapshot>>::inverse(mutation, base)
}
