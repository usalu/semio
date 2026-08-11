use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for set-frame-delay.
pub fn diff(base: &SemioImageSnapshot, index: usize, delay_ms: u32) -> SemioImageDiff {
    Mutation::diff(&SemioImageMutation::SetFrameDelay { index, delay_ms }, base)
}
