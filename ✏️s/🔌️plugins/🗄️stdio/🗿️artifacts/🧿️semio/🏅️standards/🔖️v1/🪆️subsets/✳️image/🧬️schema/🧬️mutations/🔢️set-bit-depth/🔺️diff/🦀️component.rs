use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for set-bit-depth.
pub fn diff(base: &SemioImageSnapshot, bit_depth: u8) -> SemioImageDiff {
    Mutation::diff(&SemioImageMutation::SetBitDepth { bit_depth }, base)
}
