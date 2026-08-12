use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for set-colorspace.
pub fn diff(base: &SemioImageSnapshot, colorspace: SemioColorspace) -> SemioImageDiff {
    Mutation::diff(&SemioImageMutation::SetColorspace { colorspace }, base)
}
