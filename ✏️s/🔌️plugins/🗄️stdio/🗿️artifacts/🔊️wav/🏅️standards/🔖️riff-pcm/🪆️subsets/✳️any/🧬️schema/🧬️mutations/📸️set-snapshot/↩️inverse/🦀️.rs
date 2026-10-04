use crate::standards::riff_pcm::subsets::any::schema::mutations::WavMutation;
use crate::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &WavSnapshot, mutation: &WavMutation) -> Result<Vec<WavMutation>, semio_framework_value::ValueError> {
    Ok({
    <WavMutation as Mutation<WavSnapshot>>::inverse(mutation, base)?

    })
}
