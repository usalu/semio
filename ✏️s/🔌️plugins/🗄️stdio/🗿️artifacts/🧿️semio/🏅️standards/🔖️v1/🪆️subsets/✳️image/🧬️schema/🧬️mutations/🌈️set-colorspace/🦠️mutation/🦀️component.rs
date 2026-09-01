use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::set_colorspace;

/// ▶️ Applies a set-colorspace mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(snapshot: &mut SemioImageSnapshot, colorspace: SemioColorspace) -> SemioImageDiff {
    apply_semio_image_mutation(snapshot, &SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace }))
}
