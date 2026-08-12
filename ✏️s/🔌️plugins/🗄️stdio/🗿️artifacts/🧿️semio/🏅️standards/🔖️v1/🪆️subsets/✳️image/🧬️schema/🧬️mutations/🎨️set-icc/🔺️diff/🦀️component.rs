use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for set-icc.
pub fn diff(base: &SemioImageSnapshot, icc: Option<Vec<u8>>) -> SemioImageDiff {
    Mutation::diff(&SemioImageMutation::SetIcc { icc }, base)
}
