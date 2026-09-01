use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioImageFrame, SemioImageSnapshot};
use protocol::Mutation;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::insert_frame;

/// 🔺️ Diff helper for insert-frame — an `index` past the end of `base.frames` is
/// `mutation.clamped` (Warning, non-empty diff): the frame is still inserted, at the clamped
/// (append) position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioImageSnapshot, index: usize, frame: SemioImageFrame) -> protocol::MutationOutcome<SemioImageDiff> {
    let clamped = index.min(base.frames.len());
    let outcome = Mutation::diff(&SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index: clamped, frame }), base);
    if clamped == index {
        outcome
    } else {
        outcome.warn("mutation.clamped", format!("Insert index {index} was past the end; clamped to {clamped}."))
    }
}
