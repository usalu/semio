use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::set_frame_delay;

/// ↩️ Inverse of set-frame-delay.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioImageSnapshot, index: usize, delay_ms: u32) -> Vec<SemioImageMutation> {
    <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index, delay_ms }), base)
}
