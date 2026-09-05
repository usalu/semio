use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::set_frame_pixels;

/// ↩️ Inverse of set-frame-pixels.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioImageSnapshot, index: usize, rgba8: Vec<u8>) -> Vec<SemioImageMutation> {
    <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, rgba8 }), base)
}
