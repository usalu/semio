use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for set-dimensions.
pub fn diff(base: &SemioImageSnapshot, width: u32, height: u32) -> SemioImageDiff {
    Mutation::diff(&SemioImageMutation::SetDimensions { width, height }, base)
}
