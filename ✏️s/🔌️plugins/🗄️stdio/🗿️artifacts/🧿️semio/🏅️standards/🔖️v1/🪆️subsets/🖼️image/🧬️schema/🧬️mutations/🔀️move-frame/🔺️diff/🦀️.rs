use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::Mutation;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::move_frame;

/// 🔺️ Diff helper for move-frame — index-based reordering of one existing frame; an out-of-range
/// `from` (source) or `to` (destination) is `mutation.target-missing` (Error, empty diff, checked
/// before `from == to` so a genuinely absent index is never misreported as a no-op); `from == to`
/// is `mutation.no-op` (Warning, empty diff).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioImageSnapshot, from: usize, to: usize) -> protocol::MutationOutcome<SemioImageDiff> {
    if from >= base.frames.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame index {from} does not exist."), [from.to_string()]);
    }
    if to >= base.frames.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame index {to} does not exist."), [to.to_string()]);
    }
    if from == to {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Frame {from} is already at this position."));
    }
    Mutation::diff(&SemioImageMutation::MoveFrame(move_frame::MoveFrame { from, to }), base)
}
