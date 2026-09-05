use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::set_icc;

/// ▶️ Applies a set-icc mutation (`icc: None` clears the embedded profile).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(snapshot: &mut SemioImageSnapshot, icc: Option<Vec<u8>>) -> SemioImageDiff {
    apply_semio_image_mutation(snapshot, &SemioImageMutation::SetIcc(set_icc::SetIcc { icc }))
}
