use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for move-frame.
pub fn diff(base: &SemioImageSnapshot, from: usize, to: usize) -> SemioImageDiff {
    Mutation::diff(&SemioImageMutation::MoveFrame { from, to }, base)
}
