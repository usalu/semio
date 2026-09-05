use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::set_frame_pixels;

/// ▶️ Applies a set-frame-pixels mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(snapshot: &mut SemioImageSnapshot, index: usize, rgba8: Vec<u8>) -> SemioImageDiff {
    apply_semio_image_mutation(snapshot, &SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, rgba8 }))
}
