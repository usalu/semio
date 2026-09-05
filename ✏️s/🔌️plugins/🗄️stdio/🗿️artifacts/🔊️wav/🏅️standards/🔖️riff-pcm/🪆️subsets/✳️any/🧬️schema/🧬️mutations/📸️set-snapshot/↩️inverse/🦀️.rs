use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::WavMutation;
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &WavSnapshot, mutation: &WavMutation) -> Vec<WavMutation> {
    <WavMutation as Mutation<WavSnapshot>>::inverse(mutation, base)
}
