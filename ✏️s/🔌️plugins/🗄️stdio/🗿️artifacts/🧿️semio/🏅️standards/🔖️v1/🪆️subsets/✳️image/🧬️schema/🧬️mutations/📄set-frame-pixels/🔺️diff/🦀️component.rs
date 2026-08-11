use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for set-frame-pixels.
pub fn diff(base: &SemioImageSnapshot, index: usize, rgba8: Vec<u8>) -> SemioImageDiff {
    Mutation::diff(&SemioImageMutation::SetFramePixels { index, rgba8 }, base)
}
