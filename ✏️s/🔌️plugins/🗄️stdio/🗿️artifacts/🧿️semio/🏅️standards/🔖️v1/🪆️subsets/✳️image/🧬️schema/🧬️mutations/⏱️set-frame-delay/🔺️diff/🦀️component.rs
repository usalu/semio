use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::Mutation;

/// 🔺️ Diff helper for set-frame-delay — an absent BASE frame `index` is `mutation.target-missing`
/// (Error, empty diff).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioImageSnapshot, index: usize, delay_ms: u32) -> protocol::MutationOutcome<SemioImageDiff> {
    if index >= base.frames.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame index {index} does not exist."), [index.to_string()]);
    }
    Mutation::diff(&SemioImageMutation::SetFrameDelay { index, delay_ms }, base)
}
