use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for remove-frame — an absent BASE frame `index` is `mutation.target-missing`
/// (Error, empty diff).
pub async fn diff(base: &SemioImageSnapshot, index: usize) -> protocol::MutationOutcome<SemioImageDiff> {
    if index >= base.frames.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame index {index} does not exist."), [index.to_string()]);
    }
    Mutation::diff(&SemioImageMutation::RemoveFrame { index }, base)
}
