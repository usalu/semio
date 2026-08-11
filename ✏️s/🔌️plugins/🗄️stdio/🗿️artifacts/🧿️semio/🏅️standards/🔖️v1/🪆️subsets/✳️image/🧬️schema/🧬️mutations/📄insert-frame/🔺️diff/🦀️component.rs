use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioImageFrame, SemioImageSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for insert-frame.
pub fn diff(base: &SemioImageSnapshot, index: usize, frame: SemioImageFrame) -> SemioImageDiff {
    Mutation::diff(&SemioImageMutation::InsertFrame { index, frame }, base)
}
